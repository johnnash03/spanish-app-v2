//! LEGACY (v1) — quarantined in S1 (#32). V1 exercise generation
//! (prompt-prose constraints, no enforcement layer). Replaced by the S5
//! generator with slot specs and the one-unknown rule (#36), gated by the
//! S4 validator (#35). Do not extend. Deleted in S17 (#48).

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
- stackedTags must contain tags from the available stacking tags, up to the
  "Max stacking tags per item" limit provided in the user message
- Combine the primary skill with multiple prior skills simultaneously

CRITICAL — TAG NAMES:
stackedTags values must be taken ONLY from the "Available stacking tags" list in the
user message. Never invent tag names. If no stacking tags are provided, stackedTags
must be [] for every item.

CRITICAL — CURRICULUM SEQUENCE:
This curriculum is deliberately non-standard. Do not apply conventional Spanish pedagogy
assumptions. The learner has encountered ONLY the grammar and vocabulary introduced by
the units listed in "Available stacking tags." Even if you consider a concept basic
(articles, prepositions, personal 'a'), if it does not appear in that list, the learner
has not seen it and you must not use it. The stacking tags list is the only source of
truth about what has been taught.

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
    pub existing_sources: Vec<String>,
    pub item_count: u32,
}

pub struct TagDescription {
    pub tag: String,
    pub title: String,
}

pub fn build_user_message(unit: &UnitInfo) -> String {
    // Clamp ratio to 0 when there are no valid prior skills to stack with.
    let effective_ratio = if unit.stacking_tags.is_empty() {
        0
    } else {
        stack_ratio(unit.phase, &unit.skill_tag)
    };
    let max_tags_per_item = unit.stacking_tags.len().min(3);

    let mut msg = format!(
        "Unit skill: {} — {}\nPhase: {}\nStack ratio: {}% of items should be stacked\nItems to generate: {}",
        unit.skill_tag, unit.title, unit.phase, effective_ratio, unit.item_count
    );

    if !unit.stacking_tags.is_empty() {
        msg.push_str(&format!("\nMax stacking tags per item: {}", max_tags_per_item));
        msg.push_str("\n\nAvailable stacking tags (deliberately test these — include in stackedTags):");
        for t in &unit.stacking_tags {
            msg.push_str(&format!("\n- {} — {}", t.tag, t.title));
        }
    }

    if unit.skill_tag.starts_with("lex.cognate") {
        msg.push_str(
            "\n\nFORMAT CONSTRAINT (cognate vocabulary unit):\
             \nSentences must follow the pattern: [article] cognate noun + es/son + cognate adjective or adverb.\
             \nExamples: \"La preparación es importante\" | \"El sistema es complejo\" | \"Las diferencias son inevitables\"\
             \nArticle gender rules: use 'la' for nouns ending in -ción, -idad, -encia, -anza, -dad; \
             use 'el' for nouns ending in -ma, -ema.\
             \nUse plural articles (los/las) and plural verbs (son) when the subject noun is plural.\
             \nDo NOT use any verbs other than es / son. Keep Spanish translations to 4-7 words.",
        );
        if unit.skill_tag == "lex.cognate.tion" {
            msg.push_str(
                "\nADDITIONAL PATTERN (-tion verb derivation): also include sentences where the \
                 derived -ar infinitive is the subject: infinitive + es + cognate adjective.\
                 \nExamples: \"To prepare is important\" → \"Preparar es importante\" | \
                 \"To organize is necessary\" → \"Organizar es necesario\"\
                 \nAim for roughly half noun-subject sentences and half infinitive-subject sentences.",
            );
        }
    }

    if unit.skill_tag == "gram.personal-a" {
        msg.push_str(
            "\n\nFORMAT CONSTRAINT (personal 'a' unit):\
             \nDrill the ANIMATE vs INANIMATE contrast using a minimum-pair structure: same verb, \
             different object type. Animate objects require personal 'a'; inanimate objects do not.\
             \nUse only openers the learner knows: quiero, puedo, debo, tengo que, voy a.\
             \nAnimate objects: people and pets (María, mi madre, el médico, el perro).\
             \nInanimate objects: things (el libro, la música, el coche).\
             \nNote: 'al' = a + el (e.g. 'ver al médico'). Include at least two al-contraction examples.\
             \nExamples: \"I want to invite María\" → \"Quiero invitar a María\" | \
             \"I want to read the book\" → \"Quiero leer el libro\" | \
             \"Can you see the doctor?\" → \"¿Puedes ver al médico?\"",
        );
    }

    if unit.skill_tag == "gram.prep-basic" {
        msg.push_str(
            "\n\nFORMAT CONSTRAINT (basic prepositions unit):\
             \nDrill the four prepositions en, con, de, a in simple noun-phrase contexts.\
             \nUse only these verbs as vehicles: ser (soy/es/somos), trabajar (trabajo/trabaja), \
             vivir (vivo/vive), hablar (hablo/habla).\
             \nDistribute items evenly across all four prepositions. Include both bare noun-phrase \
             translations (e.g. 'with Pablo → con Pablo') and short full sentences.\
             \nCRITICAL: 'a' here means directional/locative destination (e.g. 'Voy a la tienda'). \
             It is NOT personal 'a' and NOT the 'a' in 'voy a + infinitive'. \
             Make this distinction clear by using only noun destinations, not infinitives or animate objects.\
             \nExamples: \"I work in the office\" → \"Trabajo en la oficina\" | \
             \"I'm from Madrid\" → \"Soy de Madrid\" | \
             \"I talk with Pablo\" → \"Hablo con Pablo\" | \
             \"She lives in the house\" → \"Vive en la casa\"",
        );
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

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

static GENERATING_TAGS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn generating_tags() -> &'static Mutex<HashSet<String>> {
    GENERATING_TAGS.get_or_init(|| Mutex::new(HashSet::new()))
}

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

const ITEMS_PER_UNIT: u32 = 20;
/// Most recent prior units to include as stacking pool.
const MAX_STACKING_TAGS: usize = 15;

/// Load unit info from DB for prompt building.
fn load_unit_info(
    conn: &rusqlite::Connection,
    skill_tag: &str,
) -> rusqlite::Result<UnitInfo> {
    let (title, phase): (String, u32) = conn.query_row(
        "SELECT title, phase FROM units WHERE skill_tag = ?1",
        rusqlite::params![skill_tag],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;

    // Stacking tags: the 15 most recent prior units (by unit_number).
    // These are all skills the learner has already encountered — valid to stack with.
    let mut stmt = conn.prepare(
        "SELECT skill_tag, title FROM units
         WHERE unit_number < (SELECT unit_number FROM units WHERE skill_tag = ?1)
           AND skill_tag != ?1
         ORDER BY unit_number DESC
         LIMIT ?2",
    )?;
    let stacking_tags: Vec<TagDescription> = stmt
        .query_map(rusqlite::params![skill_tag, MAX_STACKING_TAGS as i64], |r| {
            Ok(TagDescription {
                tag: r.get(0)?,
                title: r.get(1)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();

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
    if !generating_tags().lock().unwrap().insert(skill_tag.clone()) {
        return;
    }
    run_generation_inner(app, skill_tag.clone(), is_prefetch).await;
    generating_tags().lock().unwrap().remove(&skill_tag);
}

async fn run_generation_inner(app: AppHandle, skill_tag: String, is_prefetch: bool) {
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

    // DEBUG: print prompt to terminal
    eprintln!("\n=== GENERATION PROMPT for {} ===\n{}\n=== END PROMPT ===\n", skill_tag, user_msg);

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

    // Prefetch for adjacent unit disabled during testing — generate on visit only.
    let _ = next_skill_tag;

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

    #[test]
    fn stack_ratio_phase_0_regular() {
        // raw = 30 + (0-1)*2 = 28 → snap to 30
        assert_eq!(stack_ratio(0, "lex.cognate.al-ent"), 30);
    }

    #[test]
    fn stack_ratio_phase_0_mixed_is_100() {
        assert_eq!(stack_ratio(0, "lex.cognate.mixed"), 100);
    }

    // User message builder tests
    #[test]
    fn user_message_contains_skill_tag_and_phase() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero".to_string(),
            title: "Quiero + inf, affirmative".to_string(),
            phase: 1,
            stacking_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("opener.quiero"));
        assert!(msg.contains("Phase: 1"));
        assert!(msg.contains("Items to generate: 20"));
    }

    #[test]
    fn user_message_clamps_ratio_to_zero_when_no_stacking_tags() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero".to_string(),
            title: "Quiero + inf, affirmative".to_string(),
            phase: 1,
            stacking_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("Stack ratio: 0%"), "ratio must be 0 when pool is empty");
    }

    #[test]
    fn user_message_includes_stacking_tags_and_max_tags() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero.neg".to_string(),
            title: "Quiero + inf, negative".to_string(),
            phase: 1,
            stacking_tags: vec![TagDescription {
                tag: "opener.quiero".to_string(),
                title: "Quiero + inf, affirmative".to_string(),
            }],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("Available stacking tags"));
        assert!(msg.contains("opener.quiero"));
        assert!(msg.contains("Max stacking tags per item: 1"));
    }

    #[test]
    fn user_message_omits_stacking_section_when_no_prior_units() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero".to_string(),
            title: "Quiero + inf, affirmative".to_string(),
            phase: 1,
            stacking_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(!msg.contains("Available stacking tags"));
        assert!(!msg.contains("Max stacking tags per item"));
    }

    #[test]
    fn user_message_includes_existing_sources() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero".to_string(),
            title: "Quiero + inf, affirmative".to_string(),
            phase: 1,
            stacking_tags: vec![],
            existing_sources: vec!["I want to eat".to_string()],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("Existing English cues to avoid"));
        assert!(msg.contains("I want to eat"));
    }

    #[test]
    fn user_message_caps_max_tags_at_3() {
        let unit = UnitInfo {
            skill_tag: "some.advanced.tag".to_string(),
            title: "Advanced unit".to_string(),
            phase: 10,
            stacking_tags: (0..10).map(|i| TagDescription {
                tag: format!("tag.{}", i),
                title: format!("Tag {}", i),
            }).collect(),
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("Max stacking tags per item: 3"));
    }

    #[test]
    fn cognate_unit_user_message_contains_format_constraint() {
        let unit = UnitInfo {
            skill_tag: "lex.cognate.ible-able".to_string(),
            title: "-ible/-able stays".to_string(),
            phase: 0,
            stacking_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("FORMAT CONSTRAINT"), "cognate unit must include format constraint");
        assert!(msg.contains("es/son"), "cognate constraint must mention es/son");
        assert!(msg.contains("-ción"), "cognate constraint must mention article gender rules");
        assert!(!msg.contains("ADDITIONAL PATTERN"), "non-tion unit must not include verb derivation section");
    }

    #[test]
    fn cognate_tion_unit_includes_verb_derivation_pattern() {
        let unit = UnitInfo {
            skill_tag: "lex.cognate.tion".to_string(),
            title: "-tion → -ción + verb derivation".to_string(),
            phase: 0,
            stacking_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("FORMAT CONSTRAINT"));
        assert!(msg.contains("ADDITIONAL PATTERN"), "tion unit must include infinitive-subject pattern");
        assert!(msg.contains("Preparar es importante"), "must include an infinitive-subject example");
    }

    #[test]
    fn non_cognate_unit_user_message_omits_format_constraint() {
        let unit = UnitInfo {
            skill_tag: "opener.quiero".to_string(),
            title: "Quiero + inf, affirmative".to_string(),
            phase: 1,
            stacking_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(!msg.contains("FORMAT CONSTRAINT"), "non-cognate unit must not include format constraint");
    }

    #[test]
    fn gram_personal_a_unit_includes_animate_contrast_constraint() {
        let unit = UnitInfo {
            skill_tag: "gram.personal-a".to_string(),
            title: "Personal \"a\" before animate direct objects".to_string(),
            phase: 2,
            stacking_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("FORMAT CONSTRAINT"), "gram.personal-a must include format constraint");
        assert!(msg.contains("ANIMATE vs INANIMATE"), "must mention animate vs inanimate contrast");
        assert!(msg.contains("al-contraction"), "must mention al contraction");
    }

    #[test]
    fn gram_prep_basic_unit_includes_four_prepositions_constraint() {
        let unit = UnitInfo {
            skill_tag: "gram.prep-basic".to_string(),
            title: "Basic prepositions in noun phrases: en, con, de, a".to_string(),
            phase: 4,
            stacking_tags: vec![],
            existing_sources: vec![],
            item_count: 20,
        };
        let msg = build_user_message(&unit);
        assert!(msg.contains("FORMAT CONSTRAINT"), "gram.prep-basic must include format constraint");
        assert!(msg.contains("en, con, de, a"), "must mention all four prepositions");
        assert!(msg.contains("NOT personal 'a'"), "must clarify distinction from personal 'a'");
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
        assert!(STABLE_SYSTEM_PROMPT.contains("CRITICAL — TAG NAMES"), "must have CRITICAL tag constraint");
        assert!(STABLE_SYSTEM_PROMPT.contains("CRITICAL — CURRICULUM SEQUENCE"), "must have curriculum sequence constraint");
        assert!(STABLE_SYSTEM_PROMPT.contains("non-standard"), "must warn against conventional pedagogy assumptions");
        assert!(!STABLE_SYSTEM_PROMPT.contains("Background vocabulary"), "background vocab concept removed");
    }
}
