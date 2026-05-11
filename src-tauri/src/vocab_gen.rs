use serde::Serialize;

// ─── Constants ────────────────────────────────────────────────────────────────

/// Minimum unseen vocab exercises in pool before triggering replenishment.
pub const POOL_MIN_SIZE: i64 = 10;
const ITEMS_PER_BATCH: u32 = 20;
const MAX_TARGET_WORDS: usize = 5;
const MAX_CONSOLIDATING_WORDS: usize = 10;
const MAX_GRAMMAR_TAGS: usize = 10;

// ─── Stable system prompt (cached prefix) ────────────────────────────────────

/// Never changes between batches — enables OpenAI prompt caching.
pub static VOCAB_STABLE_SYSTEM_PROMPT: &str = r#"You are a Spanish language exercise author for a translation practice app.
Your job is to generate English → Spanish translation exercises for a vocabulary
practice session. Each exercise introduces exactly one new target vocabulary word
(the "1T" element) embedded in a sentence that also uses 2–3 consolidating words.

THE 1T PRINCIPLE
"1T" means one new Thing per sentence. The target word is the ONE new element.
Everything else — grammar structures and other vocabulary — must come from what
the learner already knows:
  - Grammar: ONLY structures listed in "Available grammar structures"
  - Other vocabulary: ONLY words listed in the Target or Consolidating sections

EXERCISE REQUIREMENTS
Each exercise must:
1. Introduce exactly ONE target vocabulary word (set as primaryTag)
2. Weave in 2–3 consolidating vocabulary words naturally in the same sentence
3. Use ONLY grammar structures from the "Available grammar structures" list
4. Present a clear, contextually grounded English cue
5. Provide a natural, idiomatic canonical Spanish answer

CRITICAL — GRAMMAR CONSTRAINT:
Use ONLY grammar structures from the "Available grammar structures" list.
Do not use any grammar the learner has not yet encountered.
This curriculum is non-standard — trust the provided list, not pedagogical intuition.

CRITICAL — TAG NAMES:
- "primaryTag" must be the EXACT lemma of the target word (e.g., "comprar", "mercado")
- "stackedTags" may include exact lemmas of consolidating words used and grammar tag names used
- Never invent tag names — use only lemmas from the word lists and tags from the grammar list

DISTRIBUTION RULE:
Distribute exercises evenly across all target words provided.
Each target word must appear as primaryTag in at least 2 exercises.

STYLE RULES:
1. Tone: neutral everyday English — conversational, not formal or slangy.
2. Length: short to medium sentences. The sentence should naturally showcase the target word.
3. Context: the English cue must make the target word's meaning deducible from context.
4. Person: vary grammatical person naturally across items (I / you / she / we / they).
5. Canonical format: omit subject pronouns by default ("Quiero comer", not "Yo quiero comer").
6. Ambiguity: prefer clear cues. Add context when a sentence could translate two valid ways.
7. Dialect: neutral Latin American Spanish. Use 'ustedes' not 'vosotros'.

Respond with a JSON array of objects. Each object must have exactly these fields:
- "source": string — the English cue
- "canonical": string — the correct Spanish answer
- "primaryTag": string — the exact lemma of the target vocabulary word
- "stackedTags": array of strings — lemmas of consolidating words used + grammar tags used

Output raw JSON only — no markdown, no explanation, no wrapper object."#;

// ─── Data structures ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct VocabWord {
    pub lemma: String,
    pub translation: String,
    pub part_of_speech: String,
}

#[derive(Debug, Clone)]
pub struct GrammarTag {
    pub tag: String,
    pub title: String,
}

pub struct VocabGenInfo {
    pub target_words: Vec<VocabWord>,
    pub consolidating_words: Vec<VocabWord>,
    pub grammar_tags: Vec<GrammarTag>,
    pub existing_sources: Vec<String>,
    pub items_to_generate: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct VocabPoolStats {
    pub state: String,
    #[serde(rename = "poolSize")]
    pub pool_size: i64,
}

// ─── Prompt building ──────────────────────────────────────────────────────────

pub fn build_vocab_user_message(info: &VocabGenInfo) -> String {
    let mut msg = format!("Items to generate: {}", info.items_to_generate);

    msg.push_str("\n\nTarget vocabulary words (each must appear as primaryTag in at least 2 exercises):");
    for w in &info.target_words {
        msg.push_str(&format!("\n- {} ({}) [{}]", w.lemma, w.translation, w.part_of_speech));
    }

    if !info.consolidating_words.is_empty() {
        msg.push_str("\n\nConsolidating vocabulary words (weave 2–3 naturally into each sentence):");
        for w in &info.consolidating_words {
            msg.push_str(&format!("\n- {} ({}) [{}]", w.lemma, w.translation, w.part_of_speech));
        }
    }

    if !info.grammar_tags.is_empty() {
        msg.push_str("\n\nAvailable grammar structures (use ONLY these — the learner has encountered no other grammar):");
        for t in &info.grammar_tags {
            msg.push_str(&format!("\n- {} — {}", t.tag, t.title));
        }
    }

    if !info.existing_sources.is_empty() {
        msg.push_str("\n\nExisting English cues to avoid:");
        for s in &info.existing_sources {
            msg.push_str(&format!("\n- \"{}\"", s));
        }
    }

    msg
}

// ─── Pool state management ────────────────────────────────────────────────────

/// Count of unseen vocab exercises still available in the pool.
pub fn get_vocab_pool_size(conn: &rusqlite::Connection) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM exercise_items
         WHERE category = 'vocab'
           AND NOT EXISTS (
             SELECT 1 FROM attempt_log WHERE item_id = exercise_items.id
           )",
        [],
        |r| r.get(0),
    )
}

fn get_pool_state(conn: &rusqlite::Connection) -> String {
    conn.query_row(
        "SELECT state FROM vocab_pool_state WHERE id = 1",
        [],
        |r| r.get::<_, String>(0),
    )
    .unwrap_or_else(|_| "idle".to_string())
}

fn set_pool_state(conn: &rusqlite::Connection, state: &str) -> rusqlite::Result<()> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    conn.execute(
        "UPDATE vocab_pool_state SET state = ?1, updated_at = ?2 WHERE id = 1",
        rusqlite::params![state, now],
    )?;
    Ok(())
}

// ─── DB info loading ──────────────────────────────────────────────────────────

fn load_vocab_gen_info(conn: &rusqlite::Connection) -> rusqlite::Result<VocabGenInfo> {
    // Target words: prefer 'new' state, then 'learning', by frequency rank ascending.
    let mut stmt = conn.prepare(
        "SELECT lemma, translation, part_of_speech FROM vocab_words
         WHERE pipeline_state IN ('new', 'learning')
         ORDER BY CASE pipeline_state WHEN 'new' THEN 0 ELSE 1 END, frequency_rank ASC
         LIMIT ?1",
    )?;
    let target_words: Vec<VocabWord> = stmt
        .query_map(rusqlite::params![MAX_TARGET_WORDS as i64], |r| {
            Ok(VocabWord {
                lemma: r.get(0)?,
                translation: r.get(1)?,
                part_of_speech: r.get(2)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    let target_lemmas: Vec<String> = target_words.iter().map(|w| w.lemma.clone()).collect();

    // Consolidating words: 'learning' words not already chosen as targets.
    let mut stmt2 = conn.prepare(
        "SELECT lemma, translation, part_of_speech FROM vocab_words
         WHERE pipeline_state = 'learning'
         ORDER BY frequency_rank ASC
         LIMIT ?1",
    )?;
    let all_learning: Vec<VocabWord> = stmt2
        .query_map(
            rusqlite::params![(MAX_CONSOLIDATING_WORDS + MAX_TARGET_WORDS) as i64],
            |r| {
                Ok(VocabWord {
                    lemma: r.get(0)?,
                    translation: r.get(1)?,
                    part_of_speech: r.get(2)?,
                })
            },
        )?
        .filter_map(|r| r.ok())
        .collect();

    let consolidating_words: Vec<VocabWord> = all_learning
        .into_iter()
        .filter(|w| !target_lemmas.contains(&w.lemma))
        .take(MAX_CONSOLIDATING_WORDS)
        .collect();

    // Grammar tags: units with generation_state = 'ready', most recent first.
    let mut stmt3 = conn.prepare(
        "SELECT skill_tag, title FROM units
         WHERE generation_state = 'ready'
         ORDER BY unit_number DESC
         LIMIT ?1",
    )?;
    let grammar_tags: Vec<GrammarTag> = stmt3
        .query_map(rusqlite::params![MAX_GRAMMAR_TAGS as i64], |r| {
            Ok(GrammarTag {
                tag: r.get(0)?,
                title: r.get(1)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

    // Existing vocab exercise sources to avoid (dedup).
    let mut stmt4 = conn.prepare(
        "SELECT source FROM exercise_items WHERE category = 'vocab'",
    )?;
    let existing_sources: Vec<String> = stmt4
        .query_map([], |r| r.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(VocabGenInfo {
        target_words,
        consolidating_words,
        grammar_tags,
        existing_sources,
        items_to_generate: ITEMS_PER_BATCH,
    })
}

// ─── Persist ─────────────────────────────────────────────────────────────────

fn persist_vocab_item(
    conn: &rusqlite::Connection,
    item: &crate::generate::GeneratedItem,
) -> rusqlite::Result<()> {
    let id = vocab_uuid_v4();
    let stacked_json = serde_json::to_string(&item.stacked_tags).unwrap_or_default();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    conn.execute(
        "INSERT OR IGNORE INTO exercise_items
         (id, source, canonical, primary_tag, stacked_tags, created_at, category)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'vocab')",
        rusqlite::params![id, item.source, item.canonical, item.primary_tag, stacked_json, now],
    )?;
    Ok(())
}

fn vocab_uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let tid = std::thread::current().id();
    format!(
        "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        t,
        (t >> 16) & 0xffff,
        (t >> 4) & 0xfff,
        0x8000 | ((t ^ format!("{:?}", tid).len() as u32) & 0x3fff),
        (t as u64).wrapping_mul(0x9e3779b97f4a7c15),
    )
}

// ─── Generation pipeline ─────────────────────────────────────────────────────

use crate::db::Db;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures_util::StreamExt;
use tauri::{AppHandle, Manager};

async fn run_vocab_generation(app: AppHandle) {
    let api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            let db = app.state::<Db>();
            let conn = db.0.lock().unwrap();
            let _ = set_pool_state(&conn, "failed");
            return;
        }
    };

    let gen_info = {
        let db = app.state::<Db>();
        let conn = db.0.lock().unwrap();
        match load_vocab_gen_info(&conn) {
            Ok(info) => info,
            Err(_) => {
                let _ = set_pool_state(&conn, "failed");
                return;
            }
        }
    };

    if gen_info.target_words.is_empty() {
        // No active vocab — nothing to generate; reset to idle.
        let db = app.state::<Db>();
        let conn = db.0.lock().unwrap();
        let _ = set_pool_state(&conn, "idle");
        return;
    }

    let user_msg = build_vocab_user_message(&gen_info);
    eprintln!("\n=== VOCAB GENERATION PROMPT ===\n{}\n=== END PROMPT ===\n", user_msg);

    let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));

    let request = match CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .temperature(0.7_f32)
        .messages(
            [
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(VOCAB_STABLE_SYSTEM_PROMPT)
                    .build()
                    .map(|m| m.into()),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_msg.as_str())
                    .build()
                    .map(|m| m.into()),
            ]
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_default(),
        )
        .build()
    {
        Ok(r) => r,
        Err(_) => {
            let db = app.state::<Db>();
            let conn = db.0.lock().unwrap();
            let _ = set_pool_state(&conn, "failed");
            return;
        }
    };

    let mut stream = match client.chat().create_stream(request).await {
        Ok(s) => s,
        Err(_) => {
            let db = app.state::<Db>();
            let conn = db.0.lock().unwrap();
            let _ = set_pool_state(&conn, "failed");
            return;
        }
    };

    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(c) => {
                for choice in &c.choices {
                    if let Some(content) = &choice.delta.content {
                        buffer.push_str(content);
                        let (items, consumed) =
                            crate::generate::extract_complete_items(&buffer);
                        if !items.is_empty() {
                            let db = app.state::<Db>();
                            let conn = db.0.lock().unwrap();
                            for item in &items {
                                let _ = persist_vocab_item(&conn, item);
                            }
                            buffer.drain(..consumed);
                        }
                    }
                }
            }
            Err(_) => {
                let db = app.state::<Db>();
                let conn = db.0.lock().unwrap();
                let _ = set_pool_state(&conn, "failed");
                return;
            }
        }
    }

    let (remaining_items, _) = crate::generate::extract_complete_items(&buffer);
    {
        let db = app.state::<Db>();
        let conn = db.0.lock().unwrap();
        for item in &remaining_items {
            let _ = persist_vocab_item(&conn, item);
        }
        // Return to idle — pool grows incrementally; callers use get_vocab_pool_state
        let _ = set_pool_state(&conn, "idle");
    }
}

// ─── Tauri commands ───────────────────────────────────────────────────────────

/// Check pool size and spawn background replenishment if needed.
/// `force = true` skips the threshold check — used when a grammar unit is unlocked.
#[tauri::command]
pub async fn trigger_vocab_replenishment(
    state: tauri::State<'_, Db>,
    app: AppHandle,
    force: bool,
) -> Result<VocabPoolStats, String> {
    let (current_state, pool_size) = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let s = get_pool_state(&conn);
        let n = get_vocab_pool_size(&conn).unwrap_or(0);
        (s, n)
    };

    let should_generate =
        current_state != "generating" && (pool_size < POOL_MIN_SIZE || force);

    if should_generate {
        {
            let conn = state.0.lock().map_err(|e| e.to_string())?;
            set_pool_state(&conn, "generating").map_err(|e| e.to_string())?;
        }
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            run_vocab_generation(app_clone).await;
        });
        return Ok(VocabPoolStats {
            state: "generating".to_string(),
            pool_size,
        });
    }

    Ok(VocabPoolStats {
        state: current_state,
        pool_size,
    })
}

/// Poll current vocab pool state and size.
#[tauri::command]
pub fn get_vocab_pool_state(
    state: tauri::State<'_, Db>,
) -> Result<VocabPoolStats, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    Ok(VocabPoolStats {
        state: get_pool_state(&conn),
        pool_size: get_vocab_pool_size(&conn).unwrap_or(0),
    })
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_word(lemma: &str, translation: &str, pos: &str) -> VocabWord {
        VocabWord {
            lemma: lemma.to_string(),
            translation: translation.to_string(),
            part_of_speech: pos.to_string(),
        }
    }

    fn make_tag(tag: &str, title: &str) -> GrammarTag {
        GrammarTag {
            tag: tag.to_string(),
            title: title.to_string(),
        }
    }

    // ── Prompt building ───────────────────────────────────────────────────────

    #[test]
    fn user_message_contains_item_count() {
        let info = VocabGenInfo {
            target_words: vec![make_word("comprar", "to buy", "verb")],
            consolidating_words: vec![],
            grammar_tags: vec![],
            existing_sources: vec![],
            items_to_generate: 20,
        };
        let msg = build_vocab_user_message(&info);
        assert!(msg.contains("Items to generate: 20"));
    }

    #[test]
    fn user_message_lists_target_words_with_translation_and_pos() {
        let info = VocabGenInfo {
            target_words: vec![
                make_word("comprar", "to buy", "verb"),
                make_word("mercado", "market", "noun"),
            ],
            consolidating_words: vec![],
            grammar_tags: vec![],
            existing_sources: vec![],
            items_to_generate: 20,
        };
        let msg = build_vocab_user_message(&info);
        assert!(msg.contains("comprar (to buy) [verb]"));
        assert!(msg.contains("mercado (market) [noun]"));
        assert!(msg.contains("Target vocabulary words"));
    }

    #[test]
    fn user_message_lists_consolidating_words() {
        let info = VocabGenInfo {
            target_words: vec![make_word("comprar", "to buy", "verb")],
            consolidating_words: vec![make_word("comida", "food", "noun")],
            grammar_tags: vec![],
            existing_sources: vec![],
            items_to_generate: 20,
        };
        let msg = build_vocab_user_message(&info);
        assert!(msg.contains("Consolidating vocabulary words"));
        assert!(msg.contains("comida (food) [noun]"));
    }

    #[test]
    fn user_message_omits_consolidating_section_when_empty() {
        let info = VocabGenInfo {
            target_words: vec![make_word("comprar", "to buy", "verb")],
            consolidating_words: vec![],
            grammar_tags: vec![],
            existing_sources: vec![],
            items_to_generate: 20,
        };
        let msg = build_vocab_user_message(&info);
        assert!(!msg.contains("Consolidating vocabulary words"));
    }

    #[test]
    fn user_message_lists_grammar_tags() {
        let info = VocabGenInfo {
            target_words: vec![make_word("comprar", "to buy", "verb")],
            consolidating_words: vec![],
            grammar_tags: vec![make_tag("opener.quiero", "Quiero + inf, affirmative")],
            existing_sources: vec![],
            items_to_generate: 20,
        };
        let msg = build_vocab_user_message(&info);
        assert!(msg.contains("Available grammar structures"));
        assert!(msg.contains("opener.quiero — Quiero + inf, affirmative"));
    }

    #[test]
    fn user_message_omits_grammar_section_when_empty() {
        let info = VocabGenInfo {
            target_words: vec![make_word("comprar", "to buy", "verb")],
            consolidating_words: vec![],
            grammar_tags: vec![],
            existing_sources: vec![],
            items_to_generate: 20,
        };
        let msg = build_vocab_user_message(&info);
        assert!(!msg.contains("Available grammar structures"));
    }

    #[test]
    fn user_message_includes_existing_sources_to_avoid() {
        let info = VocabGenInfo {
            target_words: vec![make_word("comprar", "to buy", "verb")],
            consolidating_words: vec![],
            grammar_tags: vec![],
            existing_sources: vec!["I want to buy food".to_string()],
            items_to_generate: 20,
        };
        let msg = build_vocab_user_message(&info);
        assert!(msg.contains("Existing English cues to avoid"));
        assert!(msg.contains("\"I want to buy food\""));
    }

    #[test]
    fn user_message_omits_avoid_section_when_empty() {
        let info = VocabGenInfo {
            target_words: vec![make_word("comprar", "to buy", "verb")],
            consolidating_words: vec![],
            grammar_tags: vec![],
            existing_sources: vec![],
            items_to_generate: 20,
        };
        let msg = build_vocab_user_message(&info);
        assert!(!msg.contains("Existing English cues to avoid"));
    }

    // ── System prompt completeness ────────────────────────────────────────────

    #[test]
    fn vocab_stable_system_prompt_has_required_sections() {
        assert!(VOCAB_STABLE_SYSTEM_PROMPT.contains("1T"), "must mention 1T principle");
        assert!(VOCAB_STABLE_SYSTEM_PROMPT.contains("primaryTag"), "must mention primaryTag");
        assert!(VOCAB_STABLE_SYSTEM_PROMPT.contains("stackedTags"), "must mention stackedTags");
        assert!(VOCAB_STABLE_SYSTEM_PROMPT.contains("CRITICAL — GRAMMAR CONSTRAINT"), "must have grammar constraint");
        assert!(VOCAB_STABLE_SYSTEM_PROMPT.contains("CRITICAL — TAG NAMES"), "must have tag names constraint");
        assert!(VOCAB_STABLE_SYSTEM_PROMPT.contains("DISTRIBUTION RULE"), "must have distribution rule");
        assert!(VOCAB_STABLE_SYSTEM_PROMPT.contains("STYLE RULES"), "must have style rules");
        assert!(VOCAB_STABLE_SYSTEM_PROMPT.contains("JSON array"), "must specify output format");
    }

    // ── Pool size counting ────────────────────────────────────────────────────

    #[test]
    fn pool_size_counts_only_vocab_items_not_yet_seen() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();

        // Insert one vocab exercise and one grammar exercise.
        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at, category)
             VALUES ('v1', 'I want food', 'Quiero comida', 'comida', '[]', 1, 'vocab')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at, category)
             VALUES ('g1', 'I want to eat', 'Quiero comer', 'opener.quiero', '[]', 1, 'grammar')",
            [],
        ).unwrap();

        // Pool should be 1 (only the vocab item).
        assert_eq!(get_vocab_pool_size(&conn).unwrap(), 1);
    }

    #[test]
    fn pool_size_excludes_already_seen_vocab_items() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();

        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at, category)
             VALUES ('v1', 'I want food', 'Quiero comida', 'comida', '[]', 1, 'vocab')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at, category)
             VALUES ('v2', 'I can buy', 'Puedo comprar', 'comprar', '[]', 2, 'vocab')",
            [],
        ).unwrap();

        // Log an attempt for v1 — it has been seen.
        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES ('a1', 'comida', 'v1', 1, 'Quiero comida', 1)",
            [],
        ).unwrap();

        // Pool should be 1 (only v2 unseen).
        assert_eq!(get_vocab_pool_size(&conn).unwrap(), 1);
    }

    #[test]
    fn pool_size_is_zero_when_pool_is_empty() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();
        assert_eq!(get_vocab_pool_size(&conn).unwrap(), 0);
    }

    #[test]
    fn pool_size_does_not_stale_for_older_learning_word_exercises() {
        // Exercises targeting words that were once 'new' remain counted
        // even if the word is now 'learning' — pool doesn't stale.
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();

        // Simulate: 'comprar' was 'new', generated exercise, now moved to 'learning'.
        conn.execute(
            "INSERT INTO vocab_words (lemma, translation, frequency_rank, part_of_speech, pipeline_state)
             VALUES ('comprar', 'to buy', 1, 'verb', 'learning')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at, category)
             VALUES ('v1', 'I want to buy food', 'Quiero comprar comida', 'comprar', '[]', 1, 'vocab')",
            [],
        ).unwrap();

        // Exercise is still counted in pool (not stale).
        assert_eq!(get_vocab_pool_size(&conn).unwrap(), 1);
    }

    // ── Pool state management ─────────────────────────────────────────────────

    #[test]
    fn pool_state_defaults_to_idle() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();
        assert_eq!(get_pool_state(&conn), "idle");
    }

    #[test]
    fn set_pool_state_updates_state() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();
        set_pool_state(&conn, "generating").unwrap();
        assert_eq!(get_pool_state(&conn), "generating");
        set_pool_state(&conn, "idle").unwrap();
        assert_eq!(get_pool_state(&conn), "idle");
    }
}
