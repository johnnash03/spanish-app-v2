use serde::{Deserialize, Serialize};

/// Stack ratio (0–100) for a unit: percentage of items that are stacked.
pub fn stack_ratio(phase: u32, skill_tag: &str) -> u32 {
    if skill_tag.ends_with(".mixed") {
        return 100;
    }
    let raw = if phase <= 16 {
        30.0 + (phase as f64 - 1.0) * 2.0
    } else {
        60.0 + (phase as f64 - 16.0) * 1.54
    };
    let snapped = ((raw / 5.0).round() * 5.0) as u32;
    snapped.min(100)
}

/// One exercise item as returned by the generation model (no id — assigned server-side).
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct GeneratedItem {
    pub source: String,
    pub canonical: String,
    #[serde(rename = "primaryTag")]
    pub primary_tag: String,
    #[serde(rename = "stackedTags")]
    pub stacked_tags: Vec<String>,
}

// ─── Prompt building ─────────────────────────────────────────────────────────

/// Stable system prompt prefix — never changes between units, enabling prompt caching.
pub static STABLE_SYSTEM_PROMPT: &str = r#"You are a Spanish language exercise author for a translation practice app.
Your job is to generate a set of English → Spanish translation exercises for a single
drill unit. Each exercise has an English cue (shown to the learner) and a canonical
Spanish answer (used server-side for evaluation).

The learner translates English sentences into Spanish. Exercises target one primary skill
and optionally combine it with prior skills (stacking).

Generate items in the following order:

Items 1–3: MINIMUM-PAIR
- stackedTags must be empty []
- Only the primary skill varies
- Sentences should be simple and isolate the target construction cleanly

Items 4–10: LIGHT STACKING
- stackedTags must contain exactly one tag from the available stacking tags
- Introduce one prior skill alongside the primary skill

Items 11+: FULL STACKING
- stackedTags must contain 2–3 tags from the available stacking tags
- Combine the primary skill with multiple prior skills simultaneously

BACKGROUND VOCABULARY RULE:
The learner has mastered everything up to this unit. Background vocabulary (any
construction listed under "Background vocabulary" in the user message) may appear
freely in any item — including minimum-pair items — without being added to stackedTags.
Vary background vocabulary naturally across items. Do not repeat the same opener,
verb, or construction in every sentence just because it appears in the stacking tags.

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
- "primaryTag": string — must match the unit's skill tag exactly
- "stackedTags": array of strings — prior skill tags mixed in; empty [] for minimum-pair items

Output raw JSON only — no markdown, no explanation, no wrapper object."#;

pub struct UnitInfo {
    pub skill_tag: String,
    pub title: String,
    pub phase: u32,
    pub stacking_tags: Vec<TagDescription>,
    pub background_tags: Vec<TagDescription>,
    pub existing_sources: Vec<String>,
    pub item_count: u32,
}

pub struct TagDescription {
    pub tag: String,
    pub title: String,
}

pub fn build_user_message(unit: &UnitInfo) -> String {
    let ratio = stack_ratio(unit.phase, &unit.skill_tag);
    let mut msg = format!(
        "Unit skill: {} — {}\nPhase: {}\nStack ratio: {}% of items should be stacked\nItems to generate: {}",
        unit.skill_tag, unit.title, unit.phase, ratio, unit.item_count
    );

    if !unit.stacking_tags.is_empty() {
        msg.push_str("\n\nAvailable stacking tags (deliberately test these — include in stackedTags):");
        for t in &unit.stacking_tags {
            msg.push_str(&format!("\n- {} — {}", t.tag, t.title));
        }
    }

    if !unit.background_tags.is_empty() {
        msg.push_str("\n\nBackground vocabulary (use freely in sentences, do NOT include in stackedTags):");
        for t in &unit.background_tags {
            msg.push_str(&format!("\n- {} — {}", t.tag, t.title));
        }
    }

    if !unit.existing_sources.is_empty() {
        msg.push_str("\n\nExisting English cues to avoid:");
        for s in &unit.existing_sources {
            msg.push_str(&format!("\n- \"{}\"", s));
        }
    }

    msg
}

// ─── Incremental JSON item extraction ────────────────────────────────────────

/// Scan `buffer` for complete `{...}` objects parseable as `GeneratedItem`.
/// Returns (items found, number of bytes consumed from the start of buffer).
pub fn extract_complete_items(buffer: &str) -> (Vec<GeneratedItem>, usize) {
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
            if let Ok(item) = serde_json::from_str::<GeneratedItem>(obj_str) {
                items.push(item);
                consumed = end_idx + 1;
                i = end_idx + 1;
            } else {
                i += 1;
            }
        } else {
            break; // Incomplete object — stop scanning
        }
    }

    (items, consumed)
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

const ITEMS_PER_UNIT: u32 = 20;
/// Maximum number of stacking prereqs to include in prompt (keeps token count bounded).
const MAX_STACKING_TAGS: usize = 8;
/// Maximum number of background tags to include.
const MAX_BACKGROUND_TAGS: usize = 15;

/// Load unit info from DB for prompt building.
fn load_unit_info(
    conn: &rusqlite::Connection,
    skill_tag: &str,
) -> rusqlite::Result<UnitInfo> {
    let (title, phase, prereqs_json): (String, u32, String) = conn.query_row(
        "SELECT title, phase, prerequisites FROM units WHERE skill_tag = ?1",
        rusqlite::params![skill_tag],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    )?;

    let prereq_tags: Vec<String> =
        serde_json::from_str(&prereqs_json).unwrap_or_default();

    // Stacking tags: direct prerequisites (capped for prompt size)
    let mut stacking_tags = Vec::new();
    for tag in prereq_tags.iter().take(MAX_STACKING_TAGS) {
        if let Ok(prereq_title) = conn.query_row(
            "SELECT title FROM units WHERE skill_tag = ?1",
            rusqlite::params![tag],
            |r| r.get::<_, String>(0),
        ) {
            stacking_tags.push(TagDescription {
                tag: tag.clone(),
                title: prereq_title,
            });
        }
    }

    // Background tags: units that come before this one but aren't direct prereqs
    // We use unit_number ordering and exclude direct prereqs + primary tag
    let prereq_set: std::collections::HashSet<&str> =
        prereq_tags.iter().map(String::as_str).collect();
    let mut stmt = conn.prepare(
        "SELECT skill_tag, title FROM units
         WHERE unit_number < (SELECT unit_number FROM units WHERE skill_tag = ?1)
           AND skill_tag != ?1
         ORDER BY unit_number DESC
         LIMIT ?2",
    )?;
    let bg_rows = stmt.query_map(
        rusqlite::params![skill_tag, MAX_BACKGROUND_TAGS as i64 + prereq_tags.len() as i64],
        |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
    )?;

    let mut background_tags = Vec::new();
    for row in bg_rows {
        let (tag, ttl) = row?;
        if !prereq_set.contains(tag.as_str()) && background_tags.len() < MAX_BACKGROUND_TAGS {
            background_tags.push(TagDescription { tag, title: ttl });
        }
    }

    // Existing sources to avoid
    let mut stmt2 = conn.prepare(
        "SELECT source FROM exercise_items WHERE primary_tag = ?1",
    )?;
    let existing_sources: Vec<String> = stmt2
        .query_map(rusqlite::params![skill_tag], |r| r.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(UnitInfo {
        skill_tag: skill_tag.to_string(),
        title,
        phase,
        stacking_tags,
        background_tags,
        existing_sources,
        item_count: ITEMS_PER_UNIT,
    })
}

/// Returns true if the exercise bank for `skill_tag` is empty.
fn bank_is_empty(conn: &rusqlite::Connection, skill_tag: &str) -> rusqlite::Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM exercise_items WHERE primary_tag = ?1",
        rusqlite::params![skill_tag],
        |r| r.get(0),
    )?;
    Ok(count == 0)
}

/// Persist a generated item to the exercise_items table.
fn persist_item(
    conn: &rusqlite::Connection,
    item: &GeneratedItem,
) -> rusqlite::Result<()> {
    let id = uuid_v4();
    let stacked_json = serde_json::to_string(&item.stacked_tags).unwrap_or_default();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    conn.execute(
        "INSERT OR IGNORE INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![id, item.source, item.canonical, item.primary_tag, stacked_json, now],
    )?;
    Ok(())
}

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    // Simple UUID v4 using random bytes from OS via timestamp + thread id mixing.
    // Not cryptographically random, but sufficient for exercise IDs.
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let tid = std::thread::current().id();
    format!("{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        t,
        (t >> 16) & 0xffff,
        (t >> 4) & 0xfff,
        0x8000 | ((t ^ format!("{:?}", tid).len() as u32) & 0x3fff),
        (t as u64).wrapping_mul(0x9e3779b97f4a7c15),
    )
}

fn set_generation_state(
    conn: &rusqlite::Connection,
    skill_tag: &str,
    state: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE units SET generation_state = ?1 WHERE skill_tag = ?2",
        rusqlite::params![state, skill_tag],
    )?;
    Ok(())
}

/// Run the full generation pipeline for a unit.
/// `is_prefetch` = true means failures are silent (no state → "failed").
async fn run_generation(app: AppHandle, skill_tag: String, is_prefetch: bool) {
    let api_key = match std::env::var("OPENAI_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            if !is_prefetch {
                let db = app.state::<Db>();
                let conn = db.0.lock().unwrap();
                let _ = set_generation_state(&conn, &skill_tag, "failed");
            }
            return;
        }
    };

    // Build prompt
    let unit_info = {
        let db = app.state::<Db>();
        let conn = db.0.lock().unwrap();
        match load_unit_info(&conn, &skill_tag) {
            Ok(info) => info,
            Err(_) => {
                if !is_prefetch {
                    let _ = set_generation_state(&conn, &skill_tag, "failed");
                }
                return;
            }
        }
    };

    let user_msg = build_user_message(&unit_info);

    let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));

    let request = match CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .temperature(0.7_f32)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(STABLE_SYSTEM_PROMPT)
                .build()
                .map(|m| m.into()),
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_msg.as_str())
                .build()
                .map(|m| m.into()),
        ]
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default())
        .build()
    {
        Ok(r) => r,
        Err(_) => {
            if !is_prefetch {
                let db = app.state::<Db>();
                let conn = db.0.lock().unwrap();
                let _ = set_generation_state(&conn, &skill_tag, "failed");
            }
            return;
        }
    };

    // Stream the response
    let mut stream = match client.chat().create_stream(request).await {
        Ok(s) => s,
        Err(_) => {
            if !is_prefetch {
                let db = app.state::<Db>();
                let conn = db.0.lock().unwrap();
                let _ = set_generation_state(&conn, &skill_tag, "failed");
            }
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
                        // Try to extract and persist complete items
                        let (items, consumed) = extract_complete_items(&buffer);
                        if !items.is_empty() {
                            let db = app.state::<Db>();
                            let conn = db.0.lock().unwrap();
                            for item in &items {
                                let _ = persist_item(&conn, item);
                            }
                            buffer.drain(..consumed);
                        }
                    }
                }
            }
            Err(_) => {
                if !is_prefetch {
                    let db = app.state::<Db>();
                    let conn = db.0.lock().unwrap();
                    let _ = set_generation_state(&conn, &skill_tag, "failed");
                }
                return;
            }
        }
    }

    // Parse any remaining items in buffer
    let (remaining_items, _) = extract_complete_items(&buffer);
    {
        let db = app.state::<Db>();
        let conn = db.0.lock().unwrap();
        for item in &remaining_items {
            let _ = persist_item(&conn, item);
        }
        let _ = set_generation_state(&conn, &skill_tag, "ready");
    }
}

// ─── Tauri commands ───────────────────────────────────────────────────────────

/// Check bank for `skill_tag`; if empty and not already generating, kick off generation.
/// Also triggers prefetch for the adjacent (N+1) unit.
/// Returns the current generation state.
#[tauri::command]
pub async fn trigger_generation(
    state: tauri::State<'_, Db>,
    app: AppHandle,
    skill_tag: String,
) -> Result<String, String> {
    let (current_state, next_skill_tag) = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;

        let gen_state: String = conn
            .query_row(
                "SELECT COALESCE(generation_state, 'idle') FROM units WHERE skill_tag = ?1",
                rusqlite::params![&skill_tag],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| "idle".to_string());

        // Find adjacent unit (next by unit_number) for prefetch
        let next: Option<String> = conn
            .query_row(
                "SELECT skill_tag FROM units
                 WHERE unit_number > (SELECT unit_number FROM units WHERE skill_tag = ?1)
                 ORDER BY unit_number ASC LIMIT 1",
                rusqlite::params![&skill_tag],
                |r| r.get(0),
            )
            .ok();

        (gen_state, next)
    };

    // If bank is empty and state is idle, start generation
    if current_state == "idle" || current_state == "failed" {
        let empty = {
            let conn = state.0.lock().map_err(|e| e.to_string())?;
            bank_is_empty(&conn, &skill_tag).unwrap_or(true)
        };

        if empty || current_state == "failed" {
            {
                let conn = state.0.lock().map_err(|e| e.to_string())?;
                let _ = set_generation_state(&conn, &skill_tag, "generating");
            }
            let app_clone = app.clone();
            let tag_clone = skill_tag.clone();
            tauri::async_runtime::spawn(async move {
                run_generation(app_clone, tag_clone, false).await;
            });
        }
    }

    // Kick off prefetch for adjacent unit (background, silent failure)
    if let Some(next_tag) = next_skill_tag {
        let next_state: String = {
            let conn = state.0.lock().map_err(|e| e.to_string())?;
            conn.query_row(
                "SELECT COALESCE(generation_state, 'idle') FROM units WHERE skill_tag = ?1",
                rusqlite::params![&next_tag],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| "idle".to_string())
        };

        if next_state == "idle" {
            let empty = {
                let conn = state.0.lock().map_err(|e| e.to_string())?;
                bank_is_empty(&conn, &next_tag).unwrap_or(true)
            };
            if empty {
                let app_clone = app.clone();
                tauri::async_runtime::spawn(async move {
                    run_generation(app_clone, next_tag, true).await;
                });
            }
        }
    }

    // Return current state (may have just changed to "generating")
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let final_state: String = conn
        .query_row(
            "SELECT COALESCE(generation_state, 'idle') FROM units WHERE skill_tag = ?1",
            rusqlite::params![&skill_tag],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "idle".to_string());
    Ok(final_state)
}

/// Poll generation state for a unit.
#[tauri::command]
pub fn get_unit_generation_state(
    state: tauri::State<'_, Db>,
    skill_tag: String,
) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let gen_state: String = conn
        .query_row(
            "SELECT COALESCE(generation_state, 'idle') FROM units WHERE skill_tag = ?1",
            rusqlite::params![&skill_tag],
            |r| r.get(0),
        )
        .unwrap_or_else(|_| "idle".to_string());
    Ok(gen_state)
}

/// Retry a failed generation (same as trigger but always resets state).
#[tauri::command]
pub async fn retry_generation(
    state: tauri::State<'_, Db>,
    app: AppHandle,
    skill_tag: String,
) -> Result<(), String> {
    {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        set_generation_state(&conn, &skill_tag, "generating").map_err(|e| e.to_string())?;
    }
    let app_clone = app.clone();
    let tag_clone = skill_tag.clone();
    tauri::async_runtime::spawn(async move {
        run_generation(app_clone, tag_clone, false).await;
    });
    Ok(())
}

/// Inner logic for prewarm — callable from both the Tauri command and app setup.
pub async fn prewarm_units_internal(app: AppHandle) -> Result<(), String> {
    // First 5 idle units with empty banks, ordered by curriculum position.
    let candidates: Vec<String> = {
        let db = app.state::<Db>();
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT u.skill_tag FROM units u
                 WHERE u.generation_state = 'idle'
                   AND NOT EXISTS (
                     SELECT 1 FROM exercise_items e WHERE e.primary_tag = u.skill_tag
                   )
                 ORDER BY u.unit_number ASC
                 LIMIT 5",
            )
            .map_err(|e| e.to_string())?;
        let result: Vec<String> = stmt
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect();
        result
    };

    for skill_tag in candidates {
        {
            let db = app.state::<Db>();
            let conn = db.0.lock().map_err(|e| e.to_string())?;
            let _ = set_generation_state(&conn, &skill_tag, "generating");
        }
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            run_generation(app_clone, skill_tag, true).await;
        });
    }

    Ok(())
}

/// App-open pre-warm sweep: Tauri command wrapper.
#[tauri::command]
pub async fn prewarm_units(app: AppHandle) -> Result<(), String> {
    prewarm_units_internal(app).await
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Stack ratio tests
    #[test]
    fn stack_ratio_phase_1_regular() {
        assert_eq!(stack_ratio(1, "opener.quiero"), 30);
    }

    #[test]
    fn stack_ratio_phase_5_regular() {
        // raw = 30 + (5-1)*2 = 38 → snap to 40
        assert_eq!(stack_ratio(5, "some.tag"), 40);
    }

    #[test]
    fn stack_ratio_phase_8_regular() {
        // raw = 30 + 7*2 = 44 → snap to 45
        assert_eq!(stack_ratio(8, "some.tag"), 45);
    }

    #[test]
    fn stack_ratio_phase_16_regular() {
        // raw = 30 + 15*2 = 60 → snap to 60
        assert_eq!(stack_ratio(16, "some.tag"), 60);
    }

    #[test]
    fn stack_ratio_phase_20_regular() {
        // raw = 60 + (20-16)*1.54 = 60 + 6.16 = 66.16 → snap to 65
        assert_eq!(stack_ratio(20, "some.tag"), 65);
    }

    #[test]
    fn stack_ratio_phase_42_regular() {
        // raw = 60 + 26*1.54 = 100.04 → snap to 100, cap at 100
        assert_eq!(stack_ratio(42, "some.tag"), 100);
    }

    #[test]
    fn stack_ratio_mixed_unit_is_100_regardless_of_phase() {
        assert_eq!(stack_ratio(1, "opener.mixed"), 100);
        assert_eq!(stack_ratio(5, "stem.pres.mixed"), 100);
        assert_eq!(stack_ratio(16, "conj.pres.regular.mixed"), 100);
    }

    // User message builder tests
    #[test]
    fn user_message_contains_skill_tag_and_phase() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero".to_string(),
            title: "Quiero + inf, affirmative".to_string(),
            phase: 1,
            stacking_tags: vec![],
            background_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("opener.quiero"));
        assert!(msg.contains("Phase: 1"));
        assert!(msg.contains("30%"));
        assert!(msg.contains("Items to generate: 20"));
    }

    #[test]
    fn user_message_includes_stacking_tags() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero.neg".to_string(),
            title: "Quiero + inf, negative".to_string(),
            phase: 1,
            stacking_tags: vec![TagDescription {
                tag: "opener.quiero".to_string(),
                title: "Quiero + inf, affirmative".to_string(),
            }],
            background_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("Available stacking tags"));
        assert!(msg.contains("opener.quiero"));
    }

    #[test]
    fn user_message_omits_stacking_section_when_no_prereqs() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero".to_string(),
            title: "Quiero + inf, affirmative".to_string(),
            phase: 1,
            stacking_tags: vec![],
            background_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(!msg.contains("Available stacking tags"));
    }

    #[test]
    fn user_message_includes_existing_sources() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero".to_string(),
            title: "Quiero + inf, affirmative".to_string(),
            phase: 1,
            stacking_tags: vec![],
            background_tags: vec![],
            existing_sources: vec!["I want to eat".to_string()],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("Existing English cues to avoid"));
        assert!(msg.contains("I want to eat"));
    }

    // Incremental JSON extractor tests
    #[test]
    fn extract_single_complete_item() {
        let json = r#"[{"source":"I want to eat","canonical":"Quiero comer","primaryTag":"opener.quiero","stackedTags":[]}]"#;
        let (items, consumed) = extract_complete_items(json);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "I want to eat");
        assert_eq!(items[0].canonical, "Quiero comer");
        assert_eq!(items[0].stacked_tags, Vec::<String>::new());
        assert!(consumed > 0);
    }

    #[test]
    fn extract_multiple_items() {
        let json = r#"[
  {"source":"I want to eat","canonical":"Quiero comer","primaryTag":"opener.quiero","stackedTags":[]},
  {"source":"She wants to sleep","canonical":"Quiere dormir","primaryTag":"opener.quiero","stackedTags":[]}
]"#;
        let (items, _) = extract_complete_items(json);
        assert_eq!(items.len(), 2);
        assert_eq!(items[1].canonical, "Quiere dormir");
    }

    #[test]
    fn extract_partial_buffer_returns_complete_items_only() {
        // Simulate streaming: first item is complete, second is cut off mid-object
        let partial = r#"[{"source":"I want to eat","canonical":"Quiero comer","primaryTag":"opener.quiero","stackedTags":[]},
  {"source":"She wants to slee"#;
        let (items, consumed) = extract_complete_items(partial);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].source, "I want to eat");
        // Consumed bytes stop after the first complete item
        assert!(consumed < partial.len());
    }

    #[test]
    fn extract_empty_buffer_returns_nothing() {
        let (items, consumed) = extract_complete_items("");
        assert_eq!(items.len(), 0);
        assert_eq!(consumed, 0);
    }

    #[test]
    fn stable_system_prompt_contains_required_sections() {
        assert!(STABLE_SYSTEM_PROMPT.contains("MINIMUM-PAIR"));
        assert!(STABLE_SYSTEM_PROMPT.contains("LIGHT STACKING"));
        assert!(STABLE_SYSTEM_PROMPT.contains("FULL STACKING"));
        assert!(STABLE_SYSTEM_PROMPT.contains("STYLE RULES"));
        assert!(STABLE_SYSTEM_PROMPT.contains("stackedTags"));
        assert!(STABLE_SYSTEM_PROMPT.contains("primaryTag"));
    }
}
