use crate::db::Db;
use crate::generate::extract_complete_items;
use crate::session::SessionItem;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures_util::StreamExt;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tauri::{AppHandle, Manager};

const DP_WINDOW: usize = 10;
const DP_WRONG_THRESHOLD: usize = 3;
const DP_MIN_BANK: i64 = 5;
const DP_MAX_TAGS_PER_CALL: usize = 3;
const DP_TOTAL_TARGET: usize = 25;

// ─── Prompts ──────────────────────────────────────────────────────────────────

pub static DP_SYSTEM_PROMPT: &str = r#"You are a Spanish language exercise author for a targeted remediation app.
The learner has demonstrated consistent errors on specific Spanish skills.
Your job is to generate minimum-pair English → Spanish translation exercises that
directly target each failing construction.

For each skill provided, study the learner's actual errors to understand the specific
mistake pattern, then generate exercises that confront that pattern directly.

GENERATION RULES:
1. Generate 5–8 exercises per skill.
2. All exercises must be minimum-pair: stackedTags must always be empty [].
   Do not combine multiple skills — isolate each failing construction cleanly.
3. Each exercise must set primaryTag to the skill it targets.
4. Study the learner's error examples carefully. Generate items that specifically
   address the pattern of mistakes shown — not generic exercises for the tag.
5. Vary sentence subjects, objects, and contexts across items to prevent pattern
   memorization.

STYLE RULES:
1. Tone: neutral everyday English — conversational, not formal or slangy.
2. Vocabulary: simple A2-B1 level. Vocabulary should not be an additional challenge.
3. Length: natural, not artificially stripped or padded. Length follows from stacking complexity.
4. Person: vary grammatical person naturally across items. Don't default to first person only.
5. Canonical format: omit subject pronouns by default ("Quiero comer", not "Yo quiero comer").
6. Ambiguity: prefer clear, contextually grounded cues. Add context when a sentence could
   translate two valid ways that test different skills.
7. Dialect: neutral Latin American Spanish. Use 'ustedes' not 'vosotros', 'tú' not 'vos'.
   Avoid regionally marked vocabulary.

Respond with a JSON array of objects. Each object must have exactly these fields:
- "source": string — the English cue
- "canonical": string — the correct Spanish answer
- "primaryTag": string — must match one of the weak skill tags in this call
- "stackedTags": array of strings — must always be empty []

Output raw JSON only — no markdown, no explanation, no wrapper object."#;

// ─── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct ErrorExample {
    pub source: String,
    pub canonical: String,
    pub learner_answer: String,
}

#[derive(Debug, Clone)]
pub struct DpWeakTag {
    pub tag: String,
    pub name: String,
    pub error_rate: f64,
    pub error_examples: Vec<ErrorExample>,
    pub existing_sources: Vec<String>,
}

// ─── Core logic ───────────────────────────────────────────────────────────────

/// True if ≥3 of the supplied attempts are wrong.
/// `last10` should be the last ≤10 attempts for a tag (order does not matter).
pub fn is_dp_weak(last10: &[bool]) -> bool {
    let wrong = last10.iter().filter(|&&c| !c).count();
    wrong >= DP_WRONG_THRESHOLD
}

fn fetch_last10(conn: &Connection, tag: &str) -> rusqlite::Result<Vec<bool>> {
    let mut stmt = conn.prepare_cached(
        "SELECT correct FROM attempt_log WHERE tag = ?1 ORDER BY timestamp DESC LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![tag, DP_WINDOW as i64], |r| r.get::<_, i64>(0))?;
    rows.map(|r| r.map(|v| v != 0)).collect()
}

fn fetch_error_examples(conn: &Connection, tag: &str) -> rusqlite::Result<Vec<ErrorExample>> {
    let mut stmt = conn.prepare(
        "SELECT ei.source, ei.canonical, al.learner_answer
         FROM attempt_log al
         JOIN exercise_items ei ON al.item_id = ei.id
         WHERE al.tag = ?1 AND al.correct = 0
         ORDER BY al.timestamp DESC
         LIMIT 5",
    )?;
    let rows = stmt.query_map(params![tag], |r| {
        Ok(ErrorExample {
            source: r.get(0)?,
            canonical: r.get(1)?,
            learner_answer: r.get(2)?,
        })
    })?;
    rows.map(|r| r.map_err(|e| e)).collect()
}

fn fetch_existing_sources(conn: &Connection, tag: &str) -> rusqlite::Result<Vec<String>> {
    let mut stmt =
        conn.prepare("SELECT source FROM exercise_items WHERE primary_tag = ?1")?;
    let rows = stmt.query_map(params![tag], |r| r.get::<_, String>(0))?;
    rows.map(|r| r.map_err(|e| e)).collect()
}

/// Return all weak tags (≥3/10 wrong in attempt_log), sorted by error rate descending.
pub fn compute_dp_weak_tags(
    conn: &Connection,
    tag_name_map: &HashMap<String, String>,
) -> rusqlite::Result<Vec<DpWeakTag>> {
    let mut stmt = conn.prepare("SELECT DISTINCT tag FROM attempt_log")?;
    let tags: Vec<String> = stmt
        .query_map([], |r| r.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .collect();

    let mut weak = Vec::new();
    for tag in tags {
        let last10 = fetch_last10(conn, &tag)?;
        if last10.is_empty() || !is_dp_weak(&last10) {
            continue;
        }
        let wrong = last10.iter().filter(|&&c| !c).count();
        let error_rate = wrong as f64 / last10.len() as f64;
        let name = tag_name_map.get(&tag).cloned().unwrap_or_else(|| tag.clone());
        let error_examples = fetch_error_examples(conn, &tag).unwrap_or_default();
        let existing_sources = fetch_existing_sources(conn, &tag).unwrap_or_default();
        weak.push(DpWeakTag {
            tag,
            name,
            error_rate,
            error_examples,
            existing_sources,
        });
    }

    weak.sort_by(|a, b| {
        b.error_rate
            .partial_cmp(&a.error_rate)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    Ok(weak)
}

/// Build the OpenAI user message for a batch of ≤3 weak tags.
pub fn build_dp_user_message(batch: &[DpWeakTag]) -> String {
    let mut msg =
        String::from("Generate 5\u{2013}8 minimum-pair exercises for each of the following weak skills:\n");

    for (i, tag) in batch.iter().enumerate() {
        msg.push_str(&format!("\nSKILL {}: {} \u{2014} {}\n", i + 1, tag.tag, tag.name));
        if !tag.error_examples.is_empty() {
            msg.push_str("Learner errors:\n");
            for ex in &tag.error_examples {
                msg.push_str(&format!(
                    "- Asked: \"{}\" \u{2192} Correct: \"{}\" \u{2192} Learner wrote: \"{}\"\n",
                    ex.source, ex.canonical, ex.learner_answer
                ));
            }
        }
    }

    // Merge existing sources across all tags in this batch
    let all_existing: Vec<&str> = batch
        .iter()
        .flat_map(|t| t.existing_sources.iter().map(|s| s.as_str()))
        .collect();
    if !all_existing.is_empty() {
        msg.push_str("\nExisting English cues to avoid:\n");
        for s in all_existing {
            msg.push_str(&format!("- \"{}\"\n", s));
        }
    }

    msg
}

// ─── Queue assembly ───────────────────────────────────────────────────────────

fn dp_parse_stacked(json: &str) -> Vec<String> {
    serde_json::from_str(json).unwrap_or_default()
}

fn dp_lcg_shuffle<T>(v: &mut Vec<T>) {
    if v.len() < 2 {
        return;
    }
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64;
    let mut state = seed
        .wrapping_add(v.len() as u64)
        .wrapping_mul(6364136223846793005);
    for i in (1..v.len()).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = (state >> 33) as usize % (i + 1);
        v.swap(i, j);
    }
}

fn fetch_failed_items(conn: &Connection, tag: &str) -> rusqlite::Result<Vec<SessionItem>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT ei.id, ei.source, ei.primary_tag, ei.stacked_tags
         FROM exercise_items ei
         JOIN attempt_log al ON al.item_id = ei.id
         WHERE al.tag = ?1 AND al.correct = 0 AND ei.primary_tag = ?1",
    )?;
    let items = stmt
        .query_map(params![tag], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .map(|(id, source, pt, st)| SessionItem {
            id,
            source,
            primary_tag: pt,
            stacked_tags: dp_parse_stacked(&st),
        })
        .collect();
    Ok(items)
}

fn fetch_unseen_items_for_tag(conn: &Connection, tag: &str) -> rusqlite::Result<Vec<SessionItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, source, primary_tag, stacked_tags
         FROM exercise_items
         WHERE primary_tag = ?1
           AND id NOT IN (SELECT DISTINCT item_id FROM attempt_log)",
    )?;
    let items = stmt
        .query_map(params![tag], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .map(|(id, source, pt, st)| SessionItem {
            id,
            source,
            primary_tag: pt,
            stacked_tags: dp_parse_stacked(&st),
        })
        .collect();
    Ok(items)
}

/// Assemble the deliberate practice queue.
/// Items per tag are proportional to each tag's error rate; total target ≈ DP_TOTAL_TARGET.
pub fn assemble_dp_queue_internal(conn: &Connection) -> rusqlite::Result<Vec<SessionItem>> {
    let mut stmt =
        conn.prepare("SELECT skill_tag, title FROM units WHERE unit_number IS NOT NULL")?;
    let tag_name_map: HashMap<String, String> = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))?
        .filter_map(|r| r.ok())
        .collect();

    let weak_tags = compute_dp_weak_tags(conn, &tag_name_map)?;
    if weak_tags.is_empty() {
        return Ok(vec![]);
    }

    let total_error_rate: f64 = weak_tags.iter().map(|t| t.error_rate).sum();
    let mut result = Vec::new();

    for tag in &weak_tags {
        let share = tag.error_rate / total_error_rate;
        let target = ((share * DP_TOTAL_TARGET as f64).round() as usize).max(3);

        let mut failed = fetch_failed_items(conn, &tag.tag)?;
        let mut unseen = fetch_unseen_items_for_tag(conn, &tag.tag)?;

        dp_lcg_shuffle(&mut failed);
        dp_lcg_shuffle(&mut unseen);

        // Merge: failed items first, then unseen, deduplicated by id
        let mut seen_ids = HashSet::new();
        let mut tag_items: Vec<SessionItem> = Vec::new();
        for item in failed.into_iter().chain(unseen.into_iter()) {
            if seen_ids.insert(item.id.clone()) {
                tag_items.push(item);
            }
        }

        result.extend(tag_items.into_iter().take(target));
    }

    dp_lcg_shuffle(&mut result);
    Ok(result)
}

// ─── Generation ───────────────────────────────────────────────────────────────

fn dp_uuid() -> String {
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

fn persist_dp_item(conn: &Connection, item: &crate::generate::GeneratedItem) -> rusqlite::Result<()> {
    let id = dp_uuid();
    let stacked_json = serde_json::to_string(&item.stacked_tags).unwrap_or_default();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    conn.execute(
        "INSERT OR IGNORE INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![id, item.source, item.canonical, item.primary_tag, stacked_json, now],
    )?;
    Ok(())
}

fn dp_bank_count(conn: &Connection, tag: &str) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM exercise_items WHERE primary_tag = ?1",
        params![tag],
        |r| r.get(0),
    )
}

async fn run_dp_generation(app: AppHandle, batch: Vec<DpWeakTag>) {
    let api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(k) => k,
        Err(_) => return,
    };

    let user_msg = build_dp_user_message(&batch);
    eprintln!(
        "\n=== DP GENERATION PROMPT ===\n{}\n=== END DP PROMPT ===\n",
        user_msg
    );

    let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));

    let request = match CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .temperature(0.4_f32)
        .messages(
            [
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(DP_SYSTEM_PROMPT)
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

    // Valid primary tags for this batch — reject any stray model output
    let valid_tags: HashSet<&str> = batch.iter().map(|t| t.tag.as_str()).collect();

    let mut buffer = String::new();
    while let Some(chunk) = stream.next().await {
        if let Ok(c) = chunk {
            for choice in &c.choices {
                if let Some(content) = &choice.delta.content {
                    buffer.push_str(content);
                    let (items, consumed) = extract_complete_items(&buffer);
                    if !items.is_empty() {
                        let db = app.state::<Db>();
                        let conn = db.0.lock().unwrap();
                        for item in &items {
                            if valid_tags.contains(item.primary_tag.as_str()) {
                                let _ = persist_dp_item(&conn, item);
                            }
                        }
                        buffer.drain(..consumed);
                    }
                }
            }
        }
    }

    let (remaining, _) = extract_complete_items(&buffer);
    let db = app.state::<Db>();
    let conn = db.0.lock().unwrap();
    for item in &remaining {
        if valid_tags.contains(item.primary_tag.as_str()) {
            let _ = persist_dp_item(&conn, item);
        }
    }
}

// ─── Tauri commands ───────────────────────────────────────────────────────────

/// Assemble the deliberate practice queue from available exercise items.
#[tauri::command]
pub fn assemble_dp_queue(state: tauri::State<'_, Db>) -> Result<Vec<SessionItem>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    assemble_dp_queue_internal(&conn).map_err(|e| e.to_string())
}

/// Trigger generation for weak tags that have fewer than DP_MIN_BANK items in the bank.
/// Batches tags ≤3 per call, prioritising highest error rate first.
#[tauri::command]
pub async fn trigger_dp_generation(
    state: tauri::State<'_, Db>,
    app: AppHandle,
) -> Result<(), String> {
    let weak_tags = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT skill_tag, title FROM units WHERE unit_number IS NOT NULL")
            .map_err(|e| e.to_string())?;
        let tag_name_map: HashMap<String, String> = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();
        compute_dp_weak_tags(&conn, &tag_name_map).map_err(|e| e.to_string())?
    };

    // Filter to only tags whose exercise bank is below the minimum threshold
    let needs_gen: Vec<DpWeakTag> = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        weak_tags
            .into_iter()
            .filter(|t| dp_bank_count(&conn, &t.tag).unwrap_or(0) < DP_MIN_BANK)
            .collect()
    };

    if needs_gen.is_empty() {
        return Ok(());
    }

    // Already sorted by error_rate desc — batch into groups of ≤3
    for batch in needs_gen.chunks(DP_MAX_TAGS_PER_CALL) {
        let batch_vec = batch.to_vec();
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            run_dp_generation(app_clone, batch_vec).await;
        });
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
        crate::db::run_migrations_for_test(&conn).unwrap();
        conn
    }

    fn insert_attempt(conn: &Connection, tag: &str, item_id: &str, correct: bool, learner_answer: &str, ts: i64) {
        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                format!("{}-{}", tag, ts),
                tag,
                item_id,
                correct as i64,
                learner_answer,
                ts
            ],
        )
        .unwrap();
    }

    fn insert_item(conn: &Connection, id: &str, source: &str, canonical: &str, tag: &str) {
        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at)
             VALUES (?1, ?2, ?3, ?4, '[]', 0)",
            params![id, source, canonical, tag],
        )
        .unwrap();
    }

    // ── is_dp_weak ────────────────────────────────────────────────────────────

    #[test]
    fn tag_with_3_of_10_wrong_is_weak() {
        let attempts = vec![true, true, true, true, true, true, true, false, false, false];
        assert!(is_dp_weak(&attempts));
    }

    #[test]
    fn tag_with_2_of_10_wrong_is_not_weak() {
        let attempts = vec![true, true, true, true, true, true, true, true, false, false];
        assert!(!is_dp_weak(&attempts));
    }

    #[test]
    fn tag_with_all_wrongs_is_weak() {
        let attempts = vec![false, false, false];
        assert!(is_dp_weak(&attempts));
    }

    #[test]
    fn empty_attempts_not_weak() {
        assert!(!is_dp_weak(&[]));
    }

    #[test]
    fn exactly_threshold_wrongs_is_weak() {
        let attempts = vec![true, true, true, true, true, true, true, false, false, false];
        assert!(is_dp_weak(&attempts));
    }

    // ── DP_SYSTEM_PROMPT ──────────────────────────────────────────────────────

    #[test]
    fn dp_system_prompt_contains_required_sections() {
        assert!(DP_SYSTEM_PROMPT.contains("GENERATION RULES"), "must have GENERATION RULES");
        assert!(DP_SYSTEM_PROMPT.contains("STYLE RULES"), "must have STYLE RULES");
        assert!(DP_SYSTEM_PROMPT.contains("minimum-pair"), "must mention minimum-pair");
        assert!(DP_SYSTEM_PROMPT.contains("stackedTags"), "must mention stackedTags");
        assert!(DP_SYSTEM_PROMPT.contains("primaryTag"), "must mention primaryTag");
        assert!(DP_SYSTEM_PROMPT.contains("remediation"), "must describe remediation purpose");
        assert!(DP_SYSTEM_PROMPT.contains("5\u{2013}8"), "must specify 5-8 items per skill");
        assert!(DP_SYSTEM_PROMPT.contains("always be empty []"), "must require empty stackedTags");
    }

    // ── build_dp_user_message ─────────────────────────────────────────────────

    #[test]
    fn dp_user_message_contains_skill_tag_and_name() {
        let tag = DpWeakTag {
            tag: "ser-estar".to_string(),
            name: "Ser vs Estar".to_string(),
            error_rate: 0.4,
            error_examples: vec![],
            existing_sources: vec![],
        };
        let msg = build_dp_user_message(&[tag]);
        assert!(msg.contains("ser-estar"), "must include skill tag");
        assert!(msg.contains("Ser vs Estar"), "must include skill name");
        assert!(msg.contains("SKILL 1:"), "must number skills");
    }

    #[test]
    fn dp_user_message_includes_learner_error_examples() {
        let tag = DpWeakTag {
            tag: "ser-estar".to_string(),
            name: "Ser vs Estar".to_string(),
            error_rate: 0.4,
            error_examples: vec![ErrorExample {
                source: "I am tired".to_string(),
                canonical: "Estoy cansado".to_string(),
                learner_answer: "Soy cansado".to_string(),
            }],
            existing_sources: vec![],
        };
        let msg = build_dp_user_message(&[tag]);
        assert!(msg.contains("I am tired"));
        assert!(msg.contains("Estoy cansado"));
        assert!(msg.contains("Soy cansado"));
        assert!(msg.contains("Learner errors:"));
    }

    #[test]
    fn dp_user_message_omits_existing_cues_section_when_empty() {
        let tag = DpWeakTag {
            tag: "tag.a".to_string(),
            name: "Tag A".to_string(),
            error_rate: 0.3,
            error_examples: vec![],
            existing_sources: vec![],
        };
        let msg = build_dp_user_message(&[tag]);
        assert!(
            !msg.contains("Existing English cues to avoid"),
            "must omit cues section when no existing sources"
        );
    }

    #[test]
    fn dp_user_message_includes_existing_cues_when_present() {
        let tag = DpWeakTag {
            tag: "tag.a".to_string(),
            name: "Tag A".to_string(),
            error_rate: 0.3,
            error_examples: vec![],
            existing_sources: vec!["I want to eat".to_string()],
        };
        let msg = build_dp_user_message(&[tag]);
        assert!(msg.contains("Existing English cues to avoid"));
        assert!(msg.contains("I want to eat"));
    }

    #[test]
    fn dp_user_message_numbers_multiple_skills() {
        let tags = vec![
            DpWeakTag {
                tag: "a".to_string(),
                name: "A".to_string(),
                error_rate: 0.5,
                error_examples: vec![],
                existing_sources: vec![],
            },
            DpWeakTag {
                tag: "b".to_string(),
                name: "B".to_string(),
                error_rate: 0.3,
                error_examples: vec![],
                existing_sources: vec![],
            },
        ];
        let msg = build_dp_user_message(&tags);
        assert!(msg.contains("SKILL 1:"), "must have SKILL 1");
        assert!(msg.contains("SKILL 2:"), "must have SKILL 2");
    }

    // ── compute_dp_weak_tags ──────────────────────────────────────────────────

    #[test]
    fn compute_weak_tags_includes_tag_with_3_of_10_wrong() {
        let conn = setup();
        for ts in 0..7 {
            insert_attempt(&conn, "tag.x", "item-x", true, "", ts);
        }
        for ts in 7..10 {
            insert_attempt(&conn, "tag.x", "item-x", false, "wrong", ts);
        }
        let map = HashMap::new();
        let result = compute_dp_weak_tags(&conn, &map).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].tag, "tag.x");
        assert!((result[0].error_rate - 0.3).abs() < 0.001, "error rate must be 0.3");
    }

    #[test]
    fn compute_weak_tags_excludes_tag_with_2_of_10_wrong() {
        let conn = setup();
        for ts in 0..8 {
            insert_attempt(&conn, "tag.y", "item-y", true, "", ts);
        }
        for ts in 8..10 {
            insert_attempt(&conn, "tag.y", "item-y", false, "wrong", ts);
        }
        let map = HashMap::new();
        let result = compute_dp_weak_tags(&conn, &map).unwrap();
        assert!(result.is_empty(), "2/10 wrong must not be considered weak");
    }

    #[test]
    fn compute_weak_tags_sorted_by_error_rate_descending() {
        let conn = setup();
        // tag.a: 3/10 wrong
        for ts in 0..7 {
            insert_attempt(&conn, "tag.a", "ia", true, "", ts);
        }
        for ts in 7..10 {
            insert_attempt(&conn, "tag.a", "ia", false, "x", ts);
        }
        // tag.b: 5/10 wrong
        for ts in 0..5 {
            insert_attempt(&conn, "tag.b", "ib", true, "", ts);
        }
        for ts in 5..10 {
            insert_attempt(&conn, "tag.b", "ib", false, "y", ts);
        }

        let map = HashMap::new();
        let result = compute_dp_weak_tags(&conn, &map).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].tag, "tag.b", "highest error rate must come first");
        assert_eq!(result[1].tag, "tag.a");
    }

    #[test]
    fn compute_weak_tags_uses_name_map() {
        let conn = setup();
        for ts in 0..7 {
            insert_attempt(&conn, "tag.x", "item-x", true, "", ts);
        }
        for ts in 7..10 {
            insert_attempt(&conn, "tag.x", "item-x", false, "wrong", ts);
        }
        let mut map = HashMap::new();
        map.insert("tag.x".to_string(), "My Skill Name".to_string());
        let result = compute_dp_weak_tags(&conn, &map).unwrap();
        assert_eq!(result[0].name, "My Skill Name");
    }

    #[test]
    fn compute_weak_tags_falls_back_to_tag_when_no_name() {
        let conn = setup();
        for ts in 0..7 {
            insert_attempt(&conn, "tag.x", "item-x", true, "", ts);
        }
        for ts in 7..10 {
            insert_attempt(&conn, "tag.x", "item-x", false, "wrong", ts);
        }
        let map = HashMap::new();
        let result = compute_dp_weak_tags(&conn, &map).unwrap();
        assert_eq!(result[0].name, "tag.x", "must fall back to tag string when no name map entry");
    }

    #[test]
    fn compute_weak_tags_empty_when_no_attempts() {
        let conn = setup();
        let map = HashMap::new();
        let result = compute_dp_weak_tags(&conn, &map).unwrap();
        assert!(result.is_empty());
    }

    // ── assemble_dp_queue_internal ────────────────────────────────────────────

    #[test]
    fn dp_queue_empty_when_no_weak_tags() {
        let conn = setup();
        let queue = assemble_dp_queue_internal(&conn).unwrap();
        assert!(queue.is_empty());
    }

    #[test]
    fn dp_queue_includes_failed_items() {
        let conn = setup();
        // Make tag.a weak: 3/10 wrong
        for ts in 0..7 {
            insert_attempt(&conn, "tag.a", "item-seen", true, "", ts);
        }
        insert_item(&conn, "item-failed", "I am tired", "Estoy cansado", "tag.a");
        for ts in 7..10 {
            insert_attempt(&conn, "tag.a", "item-failed", false, "wrong answer", ts);
        }

        let queue = assemble_dp_queue_internal(&conn).unwrap();
        let ids: Vec<&str> = queue.iter().map(|q| q.id.as_str()).collect();
        assert!(ids.contains(&"item-failed"), "must include failed items");
    }

    #[test]
    fn dp_queue_includes_unseen_items() {
        let conn = setup();
        // Make tag.a weak: 3/10 wrong
        for ts in 0..7 {
            insert_attempt(&conn, "tag.a", "item-seen", true, "", ts);
        }
        for ts in 7..10 {
            insert_attempt(&conn, "tag.a", "item-seen", false, "wrong", ts);
        }
        insert_item(&conn, "item-unseen", "She is tired", "Está cansada", "tag.a");

        let queue = assemble_dp_queue_internal(&conn).unwrap();
        let ids: Vec<&str> = queue.iter().map(|q| q.id.as_str()).collect();
        assert!(ids.contains(&"item-unseen"), "must include unseen items");
    }

    #[test]
    fn dp_queue_no_duplicates() {
        let conn = setup();
        for ts in 0..7 {
            insert_attempt(&conn, "tag.a", "item-x", true, "", ts);
        }
        insert_item(&conn, "item-x", "I want to eat", "Quiero comer", "tag.a");
        for ts in 7..10 {
            insert_attempt(&conn, "tag.a", "item-x", false, "wrong", ts);
        }

        let queue = assemble_dp_queue_internal(&conn).unwrap();
        let id_count = queue.iter().filter(|q| q.id == "item-x").count();
        assert_eq!(id_count, 1, "failed + unseen overlap must not produce duplicates");
    }

    #[test]
    fn dp_queue_proportional_distribution_favors_higher_error_rate() {
        let conn = setup();
        // tag.a: 5/10 wrong (higher error rate)
        for ts in 0..5 {
            insert_attempt(&conn, "tag.a", "ia", true, "", ts);
        }
        for ts in 5..10 {
            insert_attempt(&conn, "tag.a", "ia", false, "w", ts);
        }
        // tag.b: 3/10 wrong (lower error rate)
        for ts in 0..7 {
            insert_attempt(&conn, "tag.b", "ib", true, "", ts);
        }
        for ts in 7..10 {
            insert_attempt(&conn, "tag.b", "ib", false, "w", ts);
        }
        // Each has 5 unseen items
        for i in 0..5 {
            insert_item(&conn, &format!("ua{}", i), &format!("src a{}", i), "can", "tag.a");
            insert_item(&conn, &format!("ub{}", i), &format!("src b{}", i), "can", "tag.b");
        }

        let queue = assemble_dp_queue_internal(&conn).unwrap();
        let a_count = queue.iter().filter(|q| q.primary_tag == "tag.a").count();
        let b_count = queue.iter().filter(|q| q.primary_tag == "tag.b").count();
        assert!(
            a_count >= b_count,
            "tag.a with 50% error rate should get at least as many items as tag.b with 30%: got a={a_count} b={b_count}"
        );
    }
}
