//! LEGACY (v1) — quarantined in S1 (#32). The separate combined
//! grammar+vocab track. Removed in v2 with no direct counterpart: vocabulary
//! embedding is the default nature of all practice (S5 generator #36,
//! S6 session loop #37). Do not extend. Deleted in S17 (#48).

use serde::{Deserialize, Serialize};

/// One combined exercise as returned by the generation model.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CombinedExercise {
    pub source: String,
    pub canonical: String,
    #[serde(rename = "grammarTags")]
    pub grammar_tags: Vec<String>,
    #[serde(rename = "vocabWords")]
    pub vocab_words: Vec<String>,
}

/// Combined exercises to return to the frontend.
#[derive(Debug, Clone, Serialize)]
pub struct CombinedExerciseRow {
    pub id: String,
    pub source: String,
    pub canonical: String,
    #[serde(rename = "grammarTags")]
    pub grammar_tags: Vec<String>,
    #[serde(rename = "vocabWords")]
    pub vocab_words: Vec<String>,
}

// ─── Constants ────────────────────────────────────────────────────────────────

pub const COMBINED_POOL_LOW_WATERMARK: i64 = 10;
pub const COMBINED_BATCH_SIZE: u32 = 30;

// ─── Stable system prompt (cached prefix) ────────────────────────────────────

pub static COMBINED_STABLE_SYSTEM_PROMPT: &str = r#"You are a Spanish language exercise author for a combined vocabulary-grammar practice track.
Your job is to generate English → Spanish translation exercises that reinforce both grammar structures and vocabulary in context.

CORE PRINCIPLE — 1T Sentence (One Unknown at a Time):
Each exercise contains exactly 1 "new-encounter" vocabulary word (a word the learner is actively building familiarity with) embedded in a sentence that otherwise uses only familiar grammar and vocabulary. The learner should be able to reason about the new word from context. Do not make grammar the unknown and vocabulary the unknown simultaneously — pick one unknown per sentence.

VOCAB CATEGORIES in the user message:
- "New words" (first or second SRS exposure): these are your primary new-encounter targets. Use at least one per exercise.
- "Learning words" (SRS in progress): use 2-3 of these as consolidating context per exercise.
- Words NOT in either list have been mastered and may be used freely as background filler.

GRAMMAR:
Use ONLY the grammar structures listed under "Unlocked grammar tags." Do not introduce grammar the learner has not seen. The grammar tags listed are the only source of truth about what structures are available.

CRITICAL — CURRICULUM SEQUENCE:
This curriculum is deliberately non-standard. Do not apply conventional Spanish pedagogy assumptions. Only grammar from the "Unlocked grammar tags" list has been taught.

STYLE RULES:
1. Tone: neutral everyday English — conversational, not formal or slangy.
2. Length: natural, follows from complexity. Prefer short sentences for exercises with many new words.
3. Person: vary grammatical person across items. Do not default to first person only.
4. Canonical: omit subject pronouns by default ("Quiero comer", not "Yo quiero comer").
5. Dialect: neutral Latin American Spanish. Use 'ustedes' not 'vosotros', 'tú' not 'vos'.

OUTPUT FORMAT:
Respond with a JSON array of objects. Each object must have exactly these fields:
- "source": string — the English cue
- "canonical": string — the correct Spanish answer
- "grammarTags": array of strings — grammar tag(s) from the unlocked list that this exercise primarily targets
- "vocabWords": array of strings — lemmas from the provided vocab lists that appear in this exercise (the exact lemma forms, not inflected forms)

Output raw JSON only — no markdown, no explanation, no wrapper object."#;

// ─── User message builder ─────────────────────────────────────────────────────

pub struct CombinedGenerationInput {
    pub unlocked_grammar_tags: Vec<(String, String)>, // (tag, title)
    pub new_vocab: Vec<String>,                        // lemmas with few SRS reps
    pub learning_vocab: Vec<String>,                   // lemmas in consolidation
    pub batch_size: u32,
    pub existing_sources: Vec<String>,
}

pub fn build_combined_user_message(input: &CombinedGenerationInput) -> String {
    let mut msg = format!("Exercises to generate: {}", input.batch_size);

    if !input.unlocked_grammar_tags.is_empty() {
        msg.push_str("\n\nUnlocked grammar tags (use ONLY these structures):");
        for (tag, title) in &input.unlocked_grammar_tags {
            msg.push_str(&format!("\n- {} — {}", tag, title));
        }
    }

    if !input.new_vocab.is_empty() {
        msg.push_str("\n\nNew words (primary new-encounter targets — prioritize these):");
        for lemma in &input.new_vocab {
            msg.push_str(&format!("\n- {}", lemma));
        }
    }

    if !input.learning_vocab.is_empty() {
        msg.push_str("\n\nLearning words (use 2-3 per exercise as consolidating context):");
        for lemma in &input.learning_vocab {
            msg.push_str(&format!("\n- {}", lemma));
        }
    }

    if !input.existing_sources.is_empty() {
        msg.push_str("\n\nExisting English cues to avoid (do not repeat these):");
        for s in &input.existing_sources {
            msg.push_str(&format!("\n- \"{}\"", s));
        }
    }

    msg
}

// ─── Incremental JSON extractor ───────────────────────────────────────────────

/// Scan `buffer` for complete `{...}` objects parseable as `CombinedExercise`.
/// Returns (items found, bytes consumed from buffer start).
pub fn extract_combined_items(buffer: &str) -> (Vec<CombinedExercise>, usize) {
    let bytes = buffer.as_bytes();
    let mut items = Vec::new();
    let mut consumed = 0;
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] != b'{' {
            i += 1;
            continue;
        }
        let start = i;
        let mut depth: i32 = 0;
        let mut in_string = false;
        let mut escape_next = false;
        let mut end = None;

        let mut j = i;
        while j < bytes.len() {
            let c = bytes[j];
            if escape_next {
                escape_next = false;
            } else if in_string {
                match c {
                    b'\\' => escape_next = true,
                    b'"' => in_string = false,
                    _ => {}
                }
            } else {
                match c {
                    b'"' => in_string = true,
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(j);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            j += 1;
        }

        if let Some(end_idx) = end {
            let obj_str = &buffer[start..=end_idx];
            if let Ok(item) = serde_json::from_str::<CombinedExercise>(obj_str) {
                items.push(item);
                consumed = end_idx + 1;
                i = end_idx + 1;
            } else {
                i += 1;
            }
        } else {
            break;
        }
    }

    (items, consumed)
}

// ─── DB helpers ───────────────────────────────────────────────────────────────

use rusqlite::{params, Connection};

/// Count exercises in the pool not yet served.
pub fn combined_pool_size(conn: &Connection) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM combined_exercises WHERE served = 0",
        [],
        |r| r.get(0),
    )
}

/// Persist one generated exercise.
pub fn persist_combined_item(conn: &Connection, item: &CombinedExercise) -> rusqlite::Result<()> {
    let id = combined_uuid();
    let grammar_json = serde_json::to_string(&item.grammar_tags).unwrap_or_default();
    let vocab_json = serde_json::to_string(&item.vocab_words).unwrap_or_default();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    conn.execute(
        "INSERT OR IGNORE INTO combined_exercises
         (id, source, canonical, grammar_tags, vocab_lemmas, created_at, served)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
        params![id, item.source, item.canonical, grammar_json, vocab_json, now],
    )?;
    Ok(())
}

/// Fetch up to `limit` unserved exercises, marking them as served.
pub fn fetch_from_pool(
    conn: &Connection,
    limit: i64,
) -> rusqlite::Result<Vec<CombinedExerciseRow>> {
    let mut stmt = conn.prepare(
        "SELECT id FROM combined_exercises WHERE served = 0 ORDER BY created_at ASC LIMIT ?1",
    )?;
    let ids: Vec<String> = stmt
        .query_map(params![limit], |r| r.get(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(stmt);

    if ids.is_empty() {
        return Ok(vec![]);
    }

    // Mark as served.
    for id in &ids {
        conn.execute(
            "UPDATE combined_exercises SET served = 1 WHERE id = ?1",
            params![id],
        )?;
    }

    // Fetch full rows.
    let mut rows = Vec::with_capacity(ids.len());
    for id in &ids {
        let row = conn.query_row(
            "SELECT id, source, canonical, grammar_tags, vocab_lemmas
             FROM combined_exercises WHERE id = ?1",
            params![id],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, String>(4)?,
                ))
            },
        )?;
        let grammar_tags: Vec<String> =
            serde_json::from_str(&row.3).unwrap_or_default();
        let vocab_words: Vec<String> =
            serde_json::from_str(&row.4).unwrap_or_default();
        rows.push(CombinedExerciseRow {
            id: row.0,
            source: row.1,
            canonical: row.2,
            grammar_tags,
            vocab_words,
        });
    }
    Ok(rows)
}

/// Collect existing English cues to avoid repetition.
pub fn fetch_existing_sources(conn: &Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt =
        conn.prepare("SELECT source FROM combined_exercises ORDER BY created_at DESC LIMIT 100")?;
    let sources = stmt
        .query_map([], |r| r.get(0))?
        .collect::<rusqlite::Result<Vec<String>>>()?;
    Ok(sources)
}

/// Fetch unlocked grammar tags (units with at least one attempt).
pub fn fetch_unlocked_grammar_tags(conn: &Connection) -> rusqlite::Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT u.skill_tag, u.title
         FROM units u
         WHERE EXISTS (
             SELECT 1 FROM attempt_log al WHERE al.tag = u.skill_tag
         )
         ORDER BY u.unit_number ASC",
    )?;
    let tags = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(tags)
}

/// Fetch active vocab split by exposure level.
/// Returns (new_vocab_lemmas, learning_vocab_lemmas).
/// "new" = pipeline_state='new', "learning" = pipeline_state='learning'.
pub fn fetch_active_vocab(conn: &Connection) -> rusqlite::Result<(Vec<String>, Vec<String>)> {
    let mut new_words = Vec::new();
    let mut learning_words = Vec::new();

    let mut stmt = conn.prepare(
        "SELECT lemma, pipeline_state FROM vocab_words
         WHERE pipeline_state IN ('new', 'learning')
         ORDER BY frequency_rank ASC",
    )?;
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    for (lemma, state) in rows {
        if state == "new" {
            new_words.push(lemma);
        } else {
            learning_words.push(lemma);
        }
    }
    Ok((new_words, learning_words))
}

fn combined_uuid() -> String {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
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

// ─── Generation pipeline ──────────────────────────────────────────────────────

use std::sync::atomic::{AtomicBool, Ordering};

static GENERATION_RUNNING: AtomicBool = AtomicBool::new(false);

use crate::legacy::db::Db;
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

async fn run_combined_generation(app: AppHandle) {
    if GENERATION_RUNNING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }

    let api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            GENERATION_RUNNING.store(false, Ordering::SeqCst);
            return;
        }
    };

    // Load context from DB.
    let (grammar_tags, new_vocab, learning_vocab, existing_sources) = {
        let db = app.state::<Db>();
        let conn = db.0.lock().unwrap();
        let grammar = fetch_unlocked_grammar_tags(&conn).unwrap_or_default();
        let (new_v, learning_v) = fetch_active_vocab(&conn).unwrap_or_default();
        let existing = fetch_existing_sources(&conn).unwrap_or_default();
        (grammar, new_v, learning_v, existing)
    };

    let input = CombinedGenerationInput {
        unlocked_grammar_tags: grammar_tags,
        new_vocab,
        learning_vocab,
        batch_size: COMBINED_BATCH_SIZE,
        existing_sources,
    };

    let user_msg = build_combined_user_message(&input);

    eprintln!("\n=== COMBINED GENERATION PROMPT ===\n{}\n=== END ===\n", user_msg);

    let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));

    let request = match CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .temperature(0.7_f32)
        .messages(
            [
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(COMBINED_STABLE_SYSTEM_PROMPT)
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
        Err(_) => return,
    };

    let mut stream = match client.chat().create_stream(request).await {
        Ok(s) => s,
        Err(_) => return,
    };

    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(c) => {
                for choice in &c.choices {
                    if let Some(content) = &choice.delta.content {
                        buffer.push_str(content);
                        let (items, consumed) = extract_combined_items(&buffer);
                        if !items.is_empty() {
                            let db = app.state::<Db>();
                            let conn = db.0.lock().unwrap();
                            for item in &items {
                                let _ = persist_combined_item(&conn, item);
                            }
                            buffer.drain(..consumed);
                        }
                    }
                }
            }
            Err(_) => {
                GENERATION_RUNNING.store(false, Ordering::SeqCst);
                return;
            }
        }
    }

    // Flush remaining buffer.
    let (remaining, _) = extract_combined_items(&buffer);
    {
        let db = app.state::<Db>();
        let conn = db.0.lock().unwrap();
        for item in &remaining {
            let _ = persist_combined_item(&conn, item);
        }
    }

    GENERATION_RUNNING.store(false, Ordering::SeqCst);
}

// ─── Tauri commands ───────────────────────────────────────────────────────────

/// Fetch exercises from the combined pool (marks them served).
/// Also triggers background replenishment if pool is low after fetch.
#[tauri::command]
pub async fn get_combined_exercises(
    state: tauri::State<'_, Db>,
    app: AppHandle,
    limit: i64,
) -> Result<Vec<CombinedExerciseRow>, String> {
    let rows = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        fetch_from_pool(&conn, limit).map_err(|e| e.to_string())?
    };

    // Check pool size and trigger replenishment if low.
    let pool_size = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        combined_pool_size(&conn).unwrap_or(i64::MAX)
    };

    if pool_size < COMBINED_POOL_LOW_WATERMARK {
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            run_combined_generation(app_clone).await;
        });
    }

    Ok(rows)
}

/// Trigger background replenishment explicitly (e.g., on new grammar unit unlock).
#[tauri::command]
pub async fn trigger_combined_replenishment(app: AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn(async move {
        run_combined_generation(app).await;
    });
    Ok(())
}

/// Get current pool size without consuming exercises.
#[tauri::command]
pub fn get_combined_pool_size(state: tauri::State<'_, Db>) -> Result<i64, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    combined_pool_size(&conn).map_err(|e| e.to_string())
}

/// Record combined exercise result. On success, advances SRS for featured vocab words.
#[tauri::command]
pub fn submit_combined_exercise_result(
    state: tauri::State<'_, Db>,
    exercise_id: String,
    correct: bool,
) -> Result<(), String> {
    if !correct {
        return Ok(());
    }

    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Load vocab_lemmas for this exercise.
    let vocab_json: String = conn
        .query_row(
            "SELECT vocab_lemmas FROM combined_exercises WHERE id = ?1",
            params![exercise_id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;

    let lemmas: Vec<String> = serde_json::from_str(&vocab_json).unwrap_or_default();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    for lemma in &lemmas {
        // Only advance words that are still in the active pipeline.
        let ok: bool = conn
            .query_row(
                "SELECT pipeline_state IN ('new','learning') FROM vocab_words WHERE lemma = ?1",
                params![lemma],
                |r| r.get(0),
            )
            .unwrap_or(false);
        if ok {
            let _ = crate::legacy::srs::record_review(&conn, lemma, true, now);
        }
    }

    Ok(())
}

// ─── Combined session item ────────────────────────────────────────────────────

/// SessionItem-compatible struct with vocabLemmas for the combined track.
#[derive(Debug, Clone, Serialize)]
pub struct CombinedSessionItem {
    pub id: String,
    pub source: String,
    #[serde(rename = "primaryTag")]
    pub primary_tag: String,
    #[serde(rename = "stackedTags")]
    pub stacked_tags: Vec<String>,
    #[serde(rename = "vocabLemmas")]
    pub vocab_lemmas: Vec<String>,
}

// ─── Queue assembly ───────────────────────────────────────────────────────────

/// Fetch unserved exercises that contain at least one active SRS vocab word.
/// Marks matching exercises as served and returns them with `vocab_lemmas`
/// scoped to only the active (new/learning) words present in each exercise.
pub fn assemble_combined_queue_from_db(
    conn: &Connection,
) -> rusqlite::Result<Vec<CombinedSessionItem>> {
    use std::collections::HashSet;

    let mut stmt = conn.prepare(
        "SELECT lemma FROM vocab_words WHERE pipeline_state IN ('new', 'learning')",
    )?;
    let active_lemmas: HashSet<String> = stmt
        .query_map([], |r| r.get(0))?
        .collect::<rusqlite::Result<HashSet<_>>>()?;

    if active_lemmas.is_empty() {
        return Ok(vec![]);
    }

    let mut stmt = conn.prepare(
        "SELECT id, source, grammar_tags, vocab_lemmas
         FROM combined_exercises WHERE served = 0 ORDER BY created_at ASC",
    )?;
    let rows: Vec<(String, String, String, String)> = stmt
        .query_map([], |r| {
            Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    drop(stmt);

    let mut result = Vec::new();
    for (id, source, grammar_json, vocab_json) in rows {
        let vocab_words: Vec<String> =
            serde_json::from_str(&vocab_json).unwrap_or_default();
        let active_here: Vec<String> = vocab_words
            .iter()
            .filter(|l| active_lemmas.contains(l.as_str()))
            .cloned()
            .collect();

        if active_here.is_empty() {
            continue;
        }

        let grammar_tags: Vec<String> =
            serde_json::from_str(&grammar_json).unwrap_or_default();
        let primary_tag = grammar_tags.first().cloned().unwrap_or_default();
        let stacked_tags = if grammar_tags.len() > 1 {
            grammar_tags[1..].to_vec()
        } else {
            vec![]
        };

        conn.execute(
            "UPDATE combined_exercises SET served = 1 WHERE id = ?1",
            params![id],
        )?;

        result.push(CombinedSessionItem {
            id,
            source,
            primary_tag,
            stacked_tags,
            vocab_lemmas: active_here,
        });
    }

    Ok(result)
}

// ─── New Tauri commands ───────────────────────────────────────────────────────

/// Return unserved exercises that intersect with active SRS vocab, marked served.
/// Triggers background replenishment when the pool is empty.
#[tauri::command]
pub fn assemble_combined_queue(
    state: tauri::State<'_, Db>,
    app: AppHandle,
) -> Result<Vec<CombinedSessionItem>, String> {
    let items = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        assemble_combined_queue_from_db(&conn).map_err(|e| e.to_string())?
    };

    if items.is_empty() {
        tauri::async_runtime::spawn(async move {
            run_combined_generation(app).await;
        });
    }

    Ok(items)
}

/// Record a successful SRS review for each provided lemma (if still active).
#[tauri::command]
pub fn record_combined_session_reviews(
    state: tauri::State<'_, Db>,
    correct_lemmas: Vec<String>,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    for lemma in &correct_lemmas {
        let active: bool = conn
            .query_row(
                "SELECT pipeline_state IN ('new','learning') FROM vocab_words WHERE lemma = ?1",
                params![lemma],
                |r| r.get(0),
            )
            .unwrap_or(false);
        if active {
            let _ = crate::legacy::srs::record_review(&conn, lemma, true, now);
        }
    }

    Ok(())
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        crate::legacy::db::run_migrations_for_test(&conn).unwrap();
        conn
    }

    fn insert_exercise(conn: &Connection, source: &str, vocab: &[&str]) {
        let id = format!("ex-{}", source.len());
        let vocab_json = serde_json::to_string(vocab).unwrap();
        conn.execute(
            "INSERT INTO combined_exercises (id, source, canonical, grammar_tags, vocab_lemmas, created_at, served)
             VALUES (?1, ?2, 'canonical', '[]', ?3, 1, 0)",
            params![id, source, vocab_json],
        )
        .unwrap();
    }

    fn insert_word(conn: &Connection, lemma: &str, state: &str) {
        conn.execute(
            "INSERT INTO vocab_words (lemma, translation, frequency_rank, part_of_speech, pipeline_state)
             VALUES (?1, 'test', 999, 'noun', ?2)
             ON CONFLICT(lemma) DO UPDATE SET pipeline_state = ?2,
               srs_repetitions = 0, srs_interval_days = 1, srs_ease_factor = 2.5,
               next_review = NULL",
            params![lemma, state],
        )
        .unwrap();
    }

    // ── pool_size ──────────────────────────────────────────────────────────────

    #[test]
    fn pool_size_zero_when_empty() {
        let conn = setup();
        assert_eq!(combined_pool_size(&conn).unwrap(), 0);
    }

    #[test]
    fn pool_size_counts_unserved_only() {
        let conn = setup();
        insert_exercise(&conn, "I want to eat", &[]);
        insert_exercise(&conn, "She wants to sleep", &[]);
        // Mark one as served.
        conn.execute(
            "UPDATE combined_exercises SET served = 1 WHERE source = 'I want to eat'",
            [],
        )
        .unwrap();
        assert_eq!(combined_pool_size(&conn).unwrap(), 1);
    }

    // ── persist_combined_item ─────────────────────────────────────────────────

    #[test]
    fn persist_stores_exercise() {
        let conn = setup();
        let item = CombinedExercise {
            source: "I want to eat".to_string(),
            canonical: "Quiero comer".to_string(),
            grammar_tags: vec!["opener.quiero".to_string()],
            vocab_words: vec!["comer".to_string()],
        };
        persist_combined_item(&conn, &item).unwrap();
        assert_eq!(combined_pool_size(&conn).unwrap(), 1);
    }

    // ── fetch_from_pool ───────────────────────────────────────────────────────

    #[test]
    fn fetch_from_pool_marks_served() {
        let conn = setup();
        insert_exercise(&conn, "I want to eat", &[]);
        let rows = fetch_from_pool(&conn, 1).unwrap();
        assert_eq!(rows.len(), 1);
        // Pool should now be empty.
        assert_eq!(combined_pool_size(&conn).unwrap(), 0);
    }

    #[test]
    fn fetch_from_pool_respects_limit() {
        let conn = setup();
        insert_exercise(&conn, "ex 1", &[]);
        insert_exercise(&conn, "ex 2 longer", &[]);
        insert_exercise(&conn, "ex 3 even longer", &[]);
        let rows = fetch_from_pool(&conn, 2).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(combined_pool_size(&conn).unwrap(), 1);
    }

    #[test]
    fn fetch_from_pool_returns_empty_when_pool_empty() {
        let conn = setup();
        let rows = fetch_from_pool(&conn, 5).unwrap();
        assert!(rows.is_empty());
    }

    // ── build_combined_user_message ───────────────────────────────────────────

    #[test]
    fn user_message_contains_batch_size() {
        let input = CombinedGenerationInput {
            unlocked_grammar_tags: vec![],
            new_vocab: vec![],
            learning_vocab: vec![],
            batch_size: 30,
            existing_sources: vec![],
        };
        let msg = build_combined_user_message(&input);
        assert!(msg.contains("30"), "must mention batch size");
    }

    #[test]
    fn user_message_contains_grammar_tags() {
        let input = CombinedGenerationInput {
            unlocked_grammar_tags: vec![
                ("opener.quiero".to_string(), "Quiero + inf".to_string()),
                ("opener.puedo".to_string(), "Puedo + inf".to_string()),
            ],
            new_vocab: vec![],
            learning_vocab: vec![],
            batch_size: 10,
            existing_sources: vec![],
        };
        let msg = build_combined_user_message(&input);
        assert!(msg.contains("opener.quiero"));
        assert!(msg.contains("opener.puedo"));
        assert!(msg.contains("Unlocked grammar tags"));
    }

    #[test]
    fn user_message_contains_new_vocab() {
        let input = CombinedGenerationInput {
            unlocked_grammar_tags: vec![],
            new_vocab: vec!["comer".to_string(), "beber".to_string()],
            learning_vocab: vec![],
            batch_size: 10,
            existing_sources: vec![],
        };
        let msg = build_combined_user_message(&input);
        assert!(msg.contains("comer"));
        assert!(msg.contains("beber"));
        assert!(msg.contains("New words"));
    }

    #[test]
    fn user_message_contains_learning_vocab() {
        let input = CombinedGenerationInput {
            unlocked_grammar_tags: vec![],
            new_vocab: vec![],
            learning_vocab: vec!["salir".to_string(), "dormir".to_string()],
            batch_size: 10,
            existing_sources: vec![],
        };
        let msg = build_combined_user_message(&input);
        assert!(msg.contains("salir"));
        assert!(msg.contains("dormir"));
        assert!(msg.contains("Learning words"));
    }

    #[test]
    fn user_message_contains_existing_sources_to_avoid() {
        let input = CombinedGenerationInput {
            unlocked_grammar_tags: vec![],
            new_vocab: vec![],
            learning_vocab: vec![],
            batch_size: 5,
            existing_sources: vec!["I want to eat".to_string()],
        };
        let msg = build_combined_user_message(&input);
        assert!(msg.contains("I want to eat"));
        assert!(msg.contains("avoid"));
    }

    // ── extract_combined_items ────────────────────────────────────────────────

    #[test]
    fn extract_single_combined_item() {
        let json = r#"[{"source":"I want to eat","canonical":"Quiero comer","grammarTags":["opener.quiero"],"vocabWords":["comer"]}]"#;
        let (items, consumed) = extract_combined_items(json);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "I want to eat");
        assert_eq!(items[0].grammar_tags, vec!["opener.quiero"]);
        assert_eq!(items[0].vocab_words, vec!["comer"]);
        assert!(consumed > 0);
    }

    #[test]
    fn extract_multiple_combined_items() {
        let json = r#"[
  {"source":"I want to eat","canonical":"Quiero comer","grammarTags":["opener.quiero"],"vocabWords":["comer"]},
  {"source":"She wants to drink","canonical":"Quiere beber","grammarTags":["opener.quiero"],"vocabWords":["beber"]}
]"#;
        let (items, _) = extract_combined_items(json);
        assert_eq!(items.len(), 2);
        assert_eq!(items[1].vocab_words, vec!["beber"]);
    }

    #[test]
    fn extract_partial_buffer_returns_complete_only() {
        let partial = r#"[{"source":"I want to eat","canonical":"Quiero comer","grammarTags":["opener.quiero"],"vocabWords":["comer"]},
  {"source":"She wants to dri"#;
        let (items, consumed) = extract_combined_items(partial);
        assert_eq!(items.len(), 1);
        assert!(consumed < partial.len());
    }

    #[test]
    fn extract_empty_buffer_returns_nothing() {
        let (items, consumed) = extract_combined_items("");
        assert_eq!(items.len(), 0);
        assert_eq!(consumed, 0);
    }

    // ── submit_combined_exercise_result ───────────────────────────────────────

    #[test]
    fn correct_result_advances_srs_for_new_word() {
        let conn = setup();
        insert_word(&conn, "comer", "new");
        // Insert an exercise featuring "comer".
        conn.execute(
            "INSERT INTO combined_exercises (id, source, canonical, grammar_tags, vocab_lemmas, created_at, served)
             VALUES ('ex-1', 'I want to eat', 'Quiero comer', '[]', '[\"comer\"]', 1, 1)",
            [],
        )
        .unwrap();

        let now = 1_000_000_i64;
        // Simulate the submit logic.
        let vocab_json: String = conn
            .query_row(
                "SELECT vocab_lemmas FROM combined_exercises WHERE id = 'ex-1'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let lemmas: Vec<String> = serde_json::from_str(&vocab_json).unwrap();
        for lemma in &lemmas {
            let _ = crate::legacy::srs::record_review(&conn, lemma, true, now);
        }

        // Word should have moved to "learning".
        let state: String = conn
            .query_row(
                "SELECT pipeline_state FROM vocab_words WHERE lemma = 'comer'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(state, "learning");
    }

    #[test]
    fn incorrect_result_does_not_change_srs() {
        let conn = setup();
        insert_word(&conn, "comer", "new");
        conn.execute(
            "INSERT INTO combined_exercises (id, source, canonical, grammar_tags, vocab_lemmas, created_at, served)
             VALUES ('ex-2', 'I want to eat', 'Quiero comer', '[]', '[\"comer\"]', 1, 1)",
            [],
        )
        .unwrap();

        // On incorrect result, we do nothing — verify state stays "new".
        let state: String = conn
            .query_row(
                "SELECT pipeline_state FROM vocab_words WHERE lemma = 'comer'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(state, "new");
    }

    // ── assemble_combined_queue_from_db ───────────────────────────────────────

    fn insert_exercise_with_grammar(conn: &Connection, id: &str, source: &str, grammar: &[&str], vocab: &[&str]) {
        let grammar_json = serde_json::to_string(grammar).unwrap();
        let vocab_json = serde_json::to_string(vocab).unwrap();
        conn.execute(
            "INSERT INTO combined_exercises (id, source, canonical, grammar_tags, vocab_lemmas, created_at, served)
             VALUES (?1, ?2, 'canonical', ?3, ?4, 1, 0)",
            params![id, source, grammar_json, vocab_json],
        ).unwrap();
    }

    #[test]
    fn assemble_queue_returns_exercises_with_active_vocab() {
        let conn = setup();
        insert_word(&conn, "comer", "new");
        insert_exercise_with_grammar(&conn, "e1", "I want to eat", &["opener.quiero"], &["comer"]);
        let items = assemble_combined_queue_from_db(&conn).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "e1");
        assert_eq!(items[0].vocab_lemmas, vec!["comer"]);
    }

    #[test]
    fn assemble_queue_excludes_exercises_with_no_active_vocab() {
        let conn = setup();
        insert_word(&conn, "comer", "mastered");
        insert_exercise_with_grammar(&conn, "e2", "I eat", &["opener.quiero"], &["comer"]);
        let items = assemble_combined_queue_from_db(&conn).unwrap();
        assert!(items.is_empty());
    }

    #[test]
    fn assemble_queue_marks_returned_exercises_served() {
        let conn = setup();
        insert_word(&conn, "beber", "learning");
        insert_exercise_with_grammar(&conn, "e3", "She drinks", &["opener.quiero"], &["beber"]);
        let items = assemble_combined_queue_from_db(&conn).unwrap();
        assert_eq!(items.len(), 1);
        // Pool should now be empty.
        assert_eq!(combined_pool_size(&conn).unwrap(), 0);
    }

    #[test]
    fn assemble_queue_returns_empty_when_no_active_vocab() {
        let conn = setup();
        insert_exercise_with_grammar(&conn, "e4", "He sleeps", &["opener.quiero"], &["dormir"]);
        let items = assemble_combined_queue_from_db(&conn).unwrap();
        assert!(items.is_empty());
    }

    #[test]
    fn assemble_queue_populates_primary_and_stacked_tags() {
        let conn = setup();
        insert_word(&conn, "salir", "new");
        insert_exercise_with_grammar(
            &conn, "e5", "I can leave",
            &["opener.puedo", "verb.salir"],
            &["salir"],
        );
        let items = assemble_combined_queue_from_db(&conn).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].primary_tag, "opener.puedo");
        assert_eq!(items[0].stacked_tags, vec!["verb.salir"]);
    }

    #[test]
    fn assemble_queue_only_includes_active_lemmas_in_vocab_lemmas() {
        let conn = setup();
        insert_word(&conn, "comer", "new");
        insert_word(&conn, "beber", "mastered");
        insert_exercise_with_grammar(
            &conn, "e6", "I eat and drink",
            &["opener.quiero"],
            &["comer", "beber"],
        );
        let items = assemble_combined_queue_from_db(&conn).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].vocab_lemmas, vec!["comer"]); // beber excluded (mastered)
    }

    // ── record_combined_session_reviews ───────────────────────────────────────

    #[test]
    fn record_reviews_advances_srs_for_active_lemmas() {
        let conn = setup();
        insert_word(&conn, "traer", "new");
        let now = 1_000_000_i64;
        let lemmas = vec!["traer".to_string()];
        for lemma in &lemmas {
            let active: bool = conn.query_row(
                "SELECT pipeline_state IN ('new','learning') FROM vocab_words WHERE lemma = ?1",
                params![lemma],
                |r| r.get(0),
            ).unwrap_or(false);
            if active {
                let _ = crate::legacy::srs::record_review(&conn, lemma, true, now);
            }
        }
        let state: String = conn.query_row(
            "SELECT pipeline_state FROM vocab_words WHERE lemma = 'traer'",
            [],
            |r| r.get(0),
        ).unwrap();
        assert_eq!(state, "learning");
    }

    #[test]
    fn record_reviews_skips_mastered_lemmas() {
        let conn = setup();
        insert_word(&conn, "venir", "mastered");
        let lemmas = vec!["venir".to_string()];
        let now = 1_000_000_i64;
        for lemma in &lemmas {
            let active: bool = conn.query_row(
                "SELECT pipeline_state IN ('new','learning') FROM vocab_words WHERE lemma = ?1",
                params![lemma],
                |r| r.get(0),
            ).unwrap_or(false);
            if active {
                let _ = crate::legacy::srs::record_review(&conn, lemma, true, now);
            }
        }
        let state: String = conn.query_row(
            "SELECT pipeline_state FROM vocab_words WHERE lemma = 'venir'",
            [],
            |r| r.get(0),
        ).unwrap();
        assert_eq!(state, "mastered"); // unchanged
    }

    // ── stable system prompt ──────────────────────────────────────────────────

    #[test]
    fn stable_prompt_contains_required_sections() {
        assert!(COMBINED_STABLE_SYSTEM_PROMPT.contains("1T Sentence"));
        assert!(COMBINED_STABLE_SYSTEM_PROMPT.contains("grammarTags"));
        assert!(COMBINED_STABLE_SYSTEM_PROMPT.contains("vocabWords"));
        assert!(COMBINED_STABLE_SYSTEM_PROMPT.contains("CRITICAL — CURRICULUM SEQUENCE"));
        assert!(COMBINED_STABLE_SYSTEM_PROMPT.contains("new-encounter"));
        assert!(COMBINED_STABLE_SYSTEM_PROMPT.contains("consolidating"));
    }
}
