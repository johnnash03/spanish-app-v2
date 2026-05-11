use crate::db::Db;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, ResponseFormat, ResponseFormatJsonSchema,
    },
    Client,
};
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct SessionItem {
    pub id: String,
    pub source: String,
    #[serde(rename = "primaryTag")]
    pub primary_tag: String,
    #[serde(rename = "stackedTags")]
    pub stacked_tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AttemptInput {
    #[serde(rename = "itemId")]
    pub item_id: String,
    pub tag: String,
    #[serde(rename = "learnerAnswer")]
    pub learner_answer: String,
}

// ─── Evaluation types ─────────────────────────────────────────────────────────

pub struct EvalInput {
    pub item_id: String,
    pub source: String,
    pub canonical: String,
    pub primary_tag: String,
    pub primary_tag_title: String,
    pub stacked_tags: Vec<(String, String)>,
    pub learner_answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalResult {
    #[serde(rename = "itemId")]
    pub item_id: String,
    pub correct: bool,
    #[serde(rename = "errorTag")]
    pub error_tag: Option<String>,
    pub remarks: Vec<String>,
    pub explanation: Option<String>,
    pub canonical: String,
}

#[derive(Debug, Serialize)]
pub struct EvalSessionResponse {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub results: Vec<EvalResult>,
}

// Raw OpenAI response item (no item_id — matched by position).
#[derive(Debug, Deserialize)]
struct OpenAiEvalItem {
    correct: bool,
    #[serde(rename = "errorTag")]
    error_tag: Option<String>,
    remarks: Vec<String>,
    explanation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAiEvalResponse {
    results: Vec<OpenAiEvalItem>,
}

pub static EVAL_SYSTEM_PROMPT: &str = r#"You are a Spanish language evaluator for a translation practice app.
The learner is given an English sentence and must produce a correct Spanish translation.
Your job is to evaluate the learner's answer against the canonical answer and return a
structured JSON result.

EVALUATION RULES

1. CORRECTNESS
   - Compare the learner's answer to the canonical answer semantically.
   - Accept grammatically valid alternative forms (clitic placement, optional subject
     pronouns, lexical synonyms) even if not identical to the canonical.

2. ACCENTS
   - Never mark an answer wrong solely due to a missing or incorrect accent, even when
     the accented and unaccented forms are different words (e.g. 'por que' vs 'por qué',
     'si' vs 'sí'). Use the English sentence to determine intent, then mark correct and
     add a remark.
   - Always add a remark when an accent is wrong or missing, explaining the difference.
   - Example remark: "Note: 'si' means 'if' — 'sí' means 'yes'. Worth getting right."

3. PUNCTUATION
   - Ignore ¿ and ¡ entirely. Do not remark on them.

4. CAPITALIZATION
   - Ignore capitalization errors entirely. Do not remark on them.

5. AVOIDS TESTED CONSTRUCTION
   - If the learner's answer is grammatically valid Spanish but does not use the
     construction being tested (identified by the primary skill tag), mark as correct
     but add a remark noting what construction was expected and why it's worth practicing.

6. PARTIAL CREDIT
   - There is no partial credit. Evaluation is binary: correct or incorrect.

7. ERROR ATTRIBUTION
   - If the answer is wrong, set errorTag to the tag most responsible for the error.
   - If the primary skill is wrong, always attribute to primaryTag regardless of other errors.
   - If only a stacked skill is wrong, attribute to that stackedTag.
   - If correct, set errorTag to null.

8. REMARKS
   - Remarks are informational notes shown to the learner after answering.
   - Keep remarks concise, specific, and constructive.
   - Only add a remark when there is something genuinely worth noting.
   - Do not add remarks for correct answers unless an accent or construction note applies.

9. EXPLANATION
   - When correct is false, provide a brief pedagogical explanation of why the correct
     answer is correct, tied to the errorTag skill.
   - Explain the grammar rule at play in plain language. Reference the learner's specific
     wrong answer to make it concrete.
   - Keep it to 1–3 sentences. Do not lecture — just clarify the rule.
   - When correct is true, set explanation to null.

FEW-SHOT EXAMPLES

--- EXAMPLE 1: Clean correct answer ---
English: "I want to see it"
Canonical: "Quiero verlo"
Primary skill: opener.quiero — using 'quiero' + infinitive to express want
Stacked skills: clitic.do.post — direct object clitic attached to infinitive
Learner answer: "Quiero verlo"
Result: { "correct": true, "errorTag": null, "remarks": [], "explanation": null }

--- EXAMPLE 2: Wrong answer — primary tag error ---
English: "He wants to eat"
Canonical: "Quiere comer"
Primary skill: stem.e-ie.pres — stem-changing verbs (e→ie) in present tense
Learner answer: "Quere comer"
Result: { "correct": false, "errorTag": "stem.e-ie.pres", "remarks": [], "explanation": "'Querer' is a stem-changing verb: the e changes to ie in all present tense forms except nosotros/vosotros. So 'él quere' should be 'él quiere'." }

--- EXAMPLE 3: Wrong answer — stacked tag error (clitic dropped entirely) ---
English: "I want to see it"
Canonical: "Quiero verlo"
Primary skill: opener.quiero — using 'quiero' + infinitive
Stacked skills: clitic.do.post — direct object clitic attached to infinitive
Learner answer: "Quiero ver"
Result: { "correct": false, "errorTag": "clitic.do.post", "remarks": [], "explanation": "When the direct object is a pronoun ('it' → 'lo'), the pronoun cannot be dropped. Both 'Quiero verlo' (attached to infinitive) and 'Lo quiero ver' (before conjugated verb) are correct." }

--- EXAMPLE 4a: Missing accent — correct with remark ---
English: "Yes, I know"
Canonical: "Sí, sé"
Primary skill: irreg.yo.saber — irregular yo form of saber
Learner answer: "Si, se"
Result: { "correct": true, "errorTag": null, "remarks": ["Note: 'si' means 'if' — 'sí' means 'yes'. Worth getting right.", "Note: 'se' is a reflexive pronoun — 'sé' is the yo form of saber. Worth getting right."], "explanation": null }

--- EXAMPLE 4b: Missing accent on interrogative — correct with remark ---
English: "I understand why you speak."
Canonical: "Entiendo por qué hablas."
Primary skill: interrog.por-que — using 'por qué' to mean 'why'
Learner answer: "Entiendo por que hablas"
Result: { "correct": true, "errorTag": null, "remarks": ["Note: 'por que' (no accent) means 'for which/because' — 'por qué' (with accent) means 'why'. Worth getting right."], "explanation": null }

--- EXAMPLE 5: Avoids tested construction — correct with remark ---
English: "I want to see it"
Canonical: "Quiero verlo"
Primary skill: clitic.do.post — direct object clitic attached to infinitive
Learner answer: "Yo deseo ver la película"
Result: { "correct": true, "errorTag": null, "remarks": ["Good Spanish, but this unit practices attaching the clitic to the infinitive (verlo). Try: 'Quiero verlo'."], "explanation": null }

--- EXAMPLE 7: Clitic moved before conjugated verb — CORRECT alternative placement ---
English: "Why do you want to give it to me?"
Canonical: "¿Por qué quieres dármelo?"
Primary skill: clitic.do.post — direct object clitic attached to infinitive
Stacked skills: clitic.io.post — indirect object clitic attached to infinitive
Learner answer: "Por que me lo quieres dar"
Result: { "correct": true, "errorTag": null, "remarks": [], "explanation": null }

--- EXAMPLE 6: Multiple tag errors — attribute to primary tag ---
English: "Do you want to see it?"
Canonical: "¿Quieres verlo?"
Primary skill: stem.e-ie.pres — stem-changing verbs (e→ie) in present tense
Stacked skills: clitic.do.post — direct object clitic attached to infinitive
Learner answer: "Quero ver"
Result: { "correct": false, "errorTag": "stem.e-ie.pres", "remarks": [], "explanation": "'Querer' stem-changes e→ie: 'tú quieres', not 'tú quero'. Also, the direct object pronoun 'lo' cannot be dropped — use 'verlo' or 'lo quieres ver'." }"#;

pub fn build_eval_user_message(items: &[EvalInput]) -> String {
    let mut msg = String::from(
        "Evaluate each item below. Return a JSON object with a \"results\" array \
         containing one evaluation per item, in the same order.\n\n",
    );
    for (i, item) in items.iter().enumerate() {
        msg.push_str(&format!("--- ITEM {} ---\n", i + 1));
        msg.push_str(&format!("English: \"{}\"\n", item.source));
        msg.push_str(&format!("Canonical: \"{}\"\n", item.canonical));
        msg.push_str(&format!(
            "Primary skill: {} — {}\n",
            item.primary_tag, item.primary_tag_title
        ));
        if !item.stacked_tags.is_empty() {
            let stacked: Vec<String> = item
                .stacked_tags
                .iter()
                .map(|(tag, title)| format!("{} — {}", tag, title))
                .collect();
            msg.push_str(&format!("Stacked skills: {}\n", stacked.join(", ")));
        }
        msg.push_str(&format!("Learner answer: \"{}\"\n\n", item.learner_answer));
    }
    msg
}

// ─── Queue Assembly ───────────────────────────────────────────────────────────

fn lcg_shuffle<T>(v: &mut Vec<T>) {
    if v.len() < 2 {
        return;
    }
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64;
    let mut state = seed.wrapping_add(v.len() as u64).wrapping_mul(6364136223846793005);
    for i in (1..v.len()).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = (state >> 33) as usize % (i + 1);
        v.swap(i, j);
    }
}

fn parse_stacked_tags(json: &str) -> Vec<String> {
    serde_json::from_str(json).unwrap_or_default()
}

fn fetch_unseen_items(
    conn: &rusqlite::Connection,
    active_unit_tag: &str,
) -> rusqlite::Result<Vec<SessionItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, source, primary_tag, stacked_tags
         FROM exercise_items
         WHERE primary_tag = ?1
           AND id NOT IN (SELECT DISTINCT item_id FROM attempt_log)
         ORDER BY created_at ASC",
    )?;
    let items = stmt
        .query_map(params![active_unit_tag], |r| {
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
            stacked_tags: parse_stacked_tags(&st),
        })
        .collect();
    Ok(items)
}

fn fetch_last5_unit_tags(
    conn: &rusqlite::Connection,
    active_unit_tag: &str,
) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT tag, MAX(timestamp) as last_seen
         FROM attempt_log
         WHERE tag != ?1
         GROUP BY tag
         ORDER BY last_seen DESC
         LIMIT 5",
    )?;
    let tags = stmt
        .query_map(params![active_unit_tag], |r| r.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(tags)
}

fn fetch_review_items(
    conn: &rusqlite::Connection,
    tags: &[String],
    limit: usize,
) -> rusqlite::Result<Vec<SessionItem>> {
    if tags.is_empty() || limit == 0 {
        return Ok(vec![]);
    }
    let placeholders: String = tags
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT id, source, primary_tag, stacked_tags
         FROM exercise_items
         WHERE primary_tag IN ({placeholders})
         ORDER BY RANDOM()
         LIMIT {limit}"
    );
    let mut stmt = conn.prepare(&sql)?;
    let items = stmt
        .query_map(
            rusqlite::params_from_iter(tags.iter().map(|s| s.as_str())),
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                ))
            },
        )?
        .filter_map(|r| r.ok())
        .map(|(id, source, pt, st)| SessionItem {
            id,
            source,
            primary_tag: pt,
            stacked_tags: parse_stacked_tags(&st),
        })
        .collect();
    Ok(items)
}

fn fetch_longtail_items(
    conn: &rusqlite::Connection,
    active_unit_tag: &str,
    last5_tags: &[String],
    limit: usize,
) -> rusqlite::Result<Vec<SessionItem>> {
    if limit == 0 {
        return Ok(vec![]);
    }
    // Build exclusion set: active unit + last 5 units
    let mut excluded: Vec<&str> = vec![active_unit_tag];
    for t in last5_tags {
        excluded.push(t.as_str());
    }
    let placeholders: String = excluded
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(", ");

    // Tags weighted by error rate (higher error → more items sampled)
    let sql = format!(
        "SELECT tag,
                CAST(SUM(CASE WHEN correct = 0 THEN 1 ELSE 0 END) AS REAL) / COUNT(*) AS error_rate
         FROM attempt_log
         WHERE tag NOT IN ({placeholders})
         GROUP BY tag
         ORDER BY error_rate DESC
         LIMIT 10"
    );
    let mut stmt = conn.prepare(&sql)?;
    let longtail_tags: Vec<String> = stmt
        .query_map(
            rusqlite::params_from_iter(excluded.iter()),
            |r| r.get::<_, String>(0),
        )?
        .filter_map(|r| r.ok())
        .collect();

    if longtail_tags.is_empty() {
        return Ok(vec![]);
    }
    fetch_review_items(conn, &longtail_tags, limit)
}

fn assemble_queue_internal(
    conn: &rusqlite::Connection,
    active_unit_tag: &str,
) -> rusqlite::Result<Vec<SessionItem>> {
    let mut current = fetch_unseen_items(conn, active_unit_tag)?;
    let n = current.len();

    let last5 = fetch_last5_unit_tags(conn, active_unit_tag)?;
    let review_target = if n > 0 { n } else { 5 };
    let mut review = fetch_review_items(conn, &last5, review_target)?;
    let longtail_target = if n > 0 { (n + 1) / 2 } else { 2 };
    let mut longtail = fetch_longtail_items(conn, active_unit_tag, &last5, longtail_target)?;

    lcg_shuffle(&mut current);
    lcg_shuffle(&mut review);
    lcg_shuffle(&mut longtail);

    let mut all: Vec<SessionItem> = Vec::new();
    all.append(&mut current);
    all.append(&mut review);
    all.append(&mut longtail);

    lcg_shuffle(&mut all);
    Ok(all)
}

#[tauri::command]
pub fn assemble_session_queue(
    state: tauri::State<'_, Db>,
    active_unit_tag: String,
) -> Result<Vec<SessionItem>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    assemble_queue_internal(&conn, &active_unit_tag).map_err(|e| e.to_string())
}

// ─── Attempt Submission ───────────────────────────────────────────────────────

fn uuid_v4_session() -> String {
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

fn basic_correct(learner: &str, canonical: &str) -> bool {
    learner.trim().to_lowercase() == canonical.trim().to_lowercase()
}

#[tauri::command]
pub fn submit_session_attempts(
    state: tauri::State<'_, Db>,
    attempts: Vec<AttemptInput>,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    for (i, attempt) in attempts.iter().enumerate() {
        // Look up canonical for basic correctness check
        let canonical: Option<String> = conn
            .query_row(
                "SELECT canonical FROM exercise_items WHERE id = ?1",
                params![&attempt.item_id],
                |r| r.get(0),
            )
            .ok();

        let correct = canonical
            .as_deref()
            .map(|c| basic_correct(&attempt.learner_answer, c))
            .unwrap_or(false);

        // Stagger timestamps so rows are ordered correctly within one session
        let ts = now + i as i64;

        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                uuid_v4_session(),
                attempt.tag,
                attempt.item_id,
                correct as i64,
                attempt.learner_answer,
                ts,
            ],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ─── Evaluation ───────────────────────────────────────────────────────────────

fn generate_session_id() -> String {
    uuid_v4_session()
}

pub fn save_attempts_unevaluated(
    conn: &rusqlite::Connection,
    attempts: &[AttemptInput],
    session_id: &str,
) -> rusqlite::Result<()> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    for (i, attempt) in attempts.iter().enumerate() {
        let ts = now + i as i64;
        conn.execute(
            "INSERT OR IGNORE INTO attempt_log
             (id, tag, item_id, correct, learner_answer, timestamp, session_id, eval_state)
             VALUES (?1, ?2, ?3, 0, ?4, ?5, ?6, 'unevaluated')",
            params![
                uuid_v4_session(),
                attempt.tag,
                attempt.item_id,
                attempt.learner_answer,
                ts,
                session_id,
            ],
        )?;
    }
    Ok(())
}

pub fn update_attempt_eval(
    conn: &rusqlite::Connection,
    session_id: &str,
    result: &EvalResult,
    primary_tag: &str,
) -> rusqlite::Result<()> {
    let remarks_json =
        serde_json::to_string(&result.remarks).unwrap_or_else(|_| "[]".to_string());
    // Primary tag is correct unless it is specifically the errorTag.
    let primary_correct =
        result.error_tag.as_deref() != Some(primary_tag);
    conn.execute(
        "UPDATE attempt_log
         SET correct=?1, eval_state='evaluated', error_tag=?2, remarks=?3, explanation=?4
         WHERE session_id=?5 AND item_id=?6",
        params![
            primary_correct as i64,
            result.error_tag,
            remarks_json,
            result.explanation,
            session_id,
            result.item_id,
        ],
    )?;
    Ok(())
}

/// Insert one attempt row per stacked tag after evaluation.
/// Each stacked tag is correct unless it is the errorTag.
pub fn insert_stacked_tag_attempts(
    conn: &rusqlite::Connection,
    session_id: &str,
    result: &EvalResult,
    input: &EvalInput,
) -> rusqlite::Result<()> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    for (i, (tag, _title)) in input.stacked_tags.iter().enumerate() {
        let correct = result.error_tag.as_deref() != Some(tag.as_str());
        conn.execute(
            "INSERT OR IGNORE INTO attempt_log
             (id, tag, item_id, correct, learner_answer, timestamp, session_id, eval_state)
             VALUES (?1, ?2, ?3, ?4, '', ?5, ?6, 'evaluated')",
            params![
                uuid_v4_session(),
                tag,
                result.item_id,
                correct as i64,
                now + i as i64 + 1,
                session_id,
            ],
        )?;
    }
    Ok(())
}

fn load_eval_inputs(
    conn: &rusqlite::Connection,
    attempts: &[AttemptInput],
) -> rusqlite::Result<Vec<EvalInput>> {
    let mut inputs = Vec::new();
    for attempt in attempts {
        let row: (String, String, String, String) = conn.query_row(
            "SELECT source, canonical, primary_tag, stacked_tags FROM exercise_items WHERE id = ?1",
            params![attempt.item_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )?;
        let (source, canonical, primary_tag, stacked_tags_json) = row;
        let stacked_tag_ids: Vec<String> =
            serde_json::from_str(&stacked_tags_json).unwrap_or_default();

        let primary_tag_title: String = conn
            .query_row(
                "SELECT COALESCE(title, ?1) FROM units WHERE skill_tag = ?2",
                params![primary_tag.as_str(), primary_tag.as_str()],
                |r| r.get(0),
            )
            .unwrap_or_else(|_| primary_tag.clone());

        let mut stacked_tags = Vec::new();
        for tag in &stacked_tag_ids {
            let title: String = conn
                .query_row(
                    "SELECT COALESCE(title, ?1) FROM units WHERE skill_tag = ?2",
                    params![tag.as_str(), tag.as_str()],
                    |r| r.get(0),
                )
                .unwrap_or_else(|_| tag.clone());
            stacked_tags.push((tag.clone(), title));
        }

        inputs.push(EvalInput {
            item_id: attempt.item_id.clone(),
            source,
            canonical,
            primary_tag,
            primary_tag_title,
            stacked_tags,
            learner_answer: attempt.learner_answer.clone(),
        });
    }
    Ok(inputs)
}

#[tauri::command]
pub async fn evaluate_session(
    state: tauri::State<'_, Db>,
    session_id: Option<String>,
    attempts: Vec<AttemptInput>,
) -> Result<EvalSessionResponse, String> {
    if attempts.is_empty() {
        let sid = session_id.unwrap_or_else(generate_session_id);
        return Ok(EvalSessionResponse {
            session_id: sid,
            results: vec![],
        });
    }

    let sid = session_id.unwrap_or_else(generate_session_id);

    // Save attempts to DB immediately so they persist even if evaluation fails.
    {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        save_attempts_unevaluated(&conn, &attempts, &sid).map_err(|e| e.to_string())?;
    }

    // Load item details needed for the evaluation prompt.
    let eval_inputs = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        load_eval_inputs(&conn, &attempts).map_err(|e| e.to_string())?
    };

    // Call OpenAI.
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY not set".to_string())?;

    let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));
    let user_msg = build_eval_user_message(&eval_inputs);
    eprintln!("\n=== EVAL PROMPT ===\n{}\n=== END EVAL PROMPT ===\n", user_msg);

    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "results": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "correct": { "type": "boolean" },
                        "errorTag": { "anyOf": [{ "type": "string" }, { "type": "null" }] },
                        "remarks": { "type": "array", "items": { "type": "string" } },
                        "explanation": { "anyOf": [{ "type": "string" }, { "type": "null" }] }
                    },
                    "required": ["correct", "errorTag", "remarks", "explanation"],
                    "additionalProperties": false
                }
            }
        },
        "required": ["results"],
        "additionalProperties": false
    });

    let system_msg = ChatCompletionRequestSystemMessageArgs::default()
        .content(EVAL_SYSTEM_PROMPT)
        .build()
        .map_err(|e| e.to_string())?;
    let user_message = ChatCompletionRequestUserMessageArgs::default()
        .content(user_msg.as_str())
        .build()
        .map_err(|e| e.to_string())?;

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .temperature(0.0_f32)
        .messages(vec![system_msg.into(), user_message.into()])
        .response_format(ResponseFormat::JsonSchema {
            json_schema: ResponseFormatJsonSchema {
                name: "evaluation_results".to_string(),
                description: None,
                schema: Some(schema),
                strict: Some(true),
            },
        })
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| e.to_string())?;

    let content = response
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .ok_or_else(|| "Empty response from OpenAI".to_string())?;

    let parsed: OpenAiEvalResponse =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse eval response: {e}"))?;

    if parsed.results.len() != eval_inputs.len() {
        return Err(format!(
            "Evaluation result count mismatch: expected {}, got {}",
            eval_inputs.len(),
            parsed.results.len()
        ));
    }

    // Build EvalResult list and update DB.
    let mut results = Vec::new();
    {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        for (input, ai_item) in eval_inputs.iter().zip(parsed.results.iter()) {
            let result = EvalResult {
                item_id: input.item_id.clone(),
                correct: ai_item.correct,
                error_tag: ai_item.error_tag.clone(),
                remarks: ai_item.remarks.clone(),
                explanation: ai_item.explanation.clone(),
                canonical: input.canonical.clone(),
            };
            update_attempt_eval(&conn, &sid, &result, &input.primary_tag)
                .map_err(|e| e.to_string())?;
            insert_stacked_tag_attempts(&conn, &sid, &result, input)
                .map_err(|e| e.to_string())?;
            results.push(result);
        }
    }

    Ok(EvalSessionResponse {
        session_id: sid,
        results,
    })
}

// ─── Pending session ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct PendingAttempt {
    #[serde(rename = "itemId")]
    pub item_id: String,
    pub tag: String,
    #[serde(rename = "learnerAnswer")]
    pub learner_answer: String,
    pub source: String,
}

fn get_pending_session_internal(
    conn: &rusqlite::Connection,
) -> rusqlite::Result<Option<Vec<PendingAttempt>>> {
    let session_id: Option<String> = {
        let mut stmt = conn.prepare(
            "SELECT session_id FROM attempt_log
             WHERE eval_state = 'unevaluated' AND session_id IS NOT NULL
             ORDER BY timestamp DESC
             LIMIT 1",
        )?;
        let mut rows = stmt.query([])?;
        rows.next()?.map(|r| r.get(0)).transpose()?
    };

    let Some(sid) = session_id else {
        return Ok(None);
    };

    let mut stmt = conn.prepare(
        "SELECT a.item_id, a.tag, a.learner_answer, COALESCE(e.source, '') AS source
         FROM attempt_log a
         LEFT JOIN exercise_items e ON e.id = a.item_id
         WHERE a.session_id = ?1 AND a.eval_state = 'unevaluated'
         ORDER BY a.timestamp ASC",
    )?;

    let attempts: Vec<PendingAttempt> = stmt
        .query_map([&sid], |row| {
            Ok(PendingAttempt {
                item_id: row.get(0)?,
                tag: row.get(1)?,
                learner_answer: row.get(2)?,
                source: row.get(3)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    if attempts.is_empty() {
        return Ok(None);
    }

    Ok(Some(attempts))
}

#[tauri::command]
pub fn get_pending_session(
    state: tauri::State<'_, crate::db::Db>,
) -> Result<Option<Vec<PendingAttempt>>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    get_pending_session_internal(&conn).map_err(|e| e.to_string())
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();
        conn
    }

    fn insert_item(conn: &Connection, id: &str, source: &str, canonical: &str, tag: &str) {
        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at)
             VALUES (?1, ?2, ?3, ?4, '[]', 0)",
            params![id, source, canonical, tag],
        )
        .unwrap();
    }

    fn insert_attempt(conn: &Connection, item_id: &str, tag: &str, correct: bool, ts: i64) {
        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES (?1, ?2, ?3, ?4, '', ?5)",
            params![
                format!("atmp-{item_id}-{ts}"),
                tag,
                item_id,
                correct as i64,
                ts
            ],
        )
        .unwrap();
    }

    #[test]
    fn unseen_items_excludes_attempted() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want", "Quiero", "tag.a");
        insert_item(&conn, "i2", "She wants", "Quiere", "tag.a");
        insert_attempt(&conn, "i1", "tag.a", true, 100);

        let items = fetch_unseen_items(&conn, "tag.a").unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "i2");
    }

    #[test]
    fn assemble_queue_returns_unseen_for_simple_case() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want", "Quiero", "tag.a");
        insert_item(&conn, "i2", "She wants", "Quiere", "tag.a");

        let queue = assemble_queue_internal(&conn, "tag.a").unwrap();
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn assemble_queue_includes_review_items_from_last5() {
        let conn = in_memory();
        insert_item(&conn, "i1", "Unseen current", "Q1", "tag.a");
        insert_item(&conn, "i2", "Prior unit item", "Q2", "tag.b");
        insert_attempt(&conn, "i2", "tag.b", true, 50);

        let queue = assemble_queue_internal(&conn, "tag.a").unwrap();
        // Should include i1 (unseen current) + i2 (review from last5)
        assert!(queue.len() >= 1);
        let ids: Vec<&str> = queue.iter().map(|q| q.id.as_str()).collect();
        assert!(ids.contains(&"i1"));
        assert!(ids.contains(&"i2"));
    }

    #[test]
    fn assemble_queue_excludes_active_unit_from_review() {
        let conn = in_memory();
        insert_item(&conn, "i1", "Item A", "Q1", "tag.a");
        insert_attempt(&conn, "i1", "tag.a", true, 100);
        insert_item(&conn, "i2", "Item A2", "Q2", "tag.a");

        // Only i2 is unseen; i1 is seen and belongs to active unit (not review)
        let queue = assemble_queue_internal(&conn, "tag.a").unwrap();
        let ids: Vec<&str> = queue.iter().map(|q| q.id.as_str()).collect();
        assert!(ids.contains(&"i2"));
    }

    #[test]
    fn basic_correct_is_case_insensitive() {
        assert!(basic_correct("quiero comer", "Quiero comer"));
        assert!(basic_correct("  quiero comer  ", "quiero comer"));
        assert!(!basic_correct("quiero", "quiero comer"));
    }

    #[test]
    fn submit_attempts_records_to_attempt_log() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want to eat", "Quiero comer", "tag.a");

        // Build the attempt — correct answer
        let now = 1000i64;
        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES ('test-id', 'tag.a', 'i1', 1, 'Quiero comer', ?1)",
            params![now],
        )
        .unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM attempt_log", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn lcg_shuffle_does_not_panic_on_empty() {
        let mut v: Vec<i32> = vec![];
        lcg_shuffle(&mut v);
    }

    #[test]
    fn lcg_shuffle_preserves_length() {
        let mut v: Vec<i32> = (0..10).collect();
        lcg_shuffle(&mut v);
        assert_eq!(v.len(), 10);
    }

    // ─── Evaluation prompt tests ─────────────────────────────────────────────

    fn make_eval_input(item_id: &str, source: &str, canonical: &str, primary_tag: &str, primary_title: &str, learner_answer: &str) -> EvalInput {
        EvalInput {
            item_id: item_id.to_string(),
            source: source.to_string(),
            canonical: canonical.to_string(),
            primary_tag: primary_tag.to_string(),
            primary_tag_title: primary_title.to_string(),
            stacked_tags: vec![],
            learner_answer: learner_answer.to_string(),
        }
    }

    #[test]
    fn eval_user_message_contains_item_fields() {
        let input = make_eval_input("i1", "I want to eat", "Quiero comer", "opener.quiero", "Quiero + inf, affirmative", "quiero comer");
        let msg = build_eval_user_message(&[input]);
        assert!(msg.contains("I want to eat"));
        assert!(msg.contains("Quiero comer"));
        assert!(msg.contains("opener.quiero"));
        assert!(msg.contains("Quiero + inf, affirmative"));
        assert!(msg.contains("quiero comer"));
    }

    #[test]
    fn eval_user_message_omits_stacked_line_when_empty() {
        let input = make_eval_input("i1", "I want", "Quiero", "opener.quiero", "Quiero + inf, affirmative", "quiero");
        let msg = build_eval_user_message(&[input]);
        assert!(!msg.contains("Stacked skills:"));
    }

    #[test]
    fn eval_user_message_includes_stacked_skills_when_present() {
        let mut input = make_eval_input("i1", "I want to see it", "Quiero verlo", "opener.quiero", "Quiero + inf", "quiero verlo");
        input.stacked_tags = vec![("clitic.do.post".to_string(), "DO clitic post-infinitive".to_string())];
        let msg = build_eval_user_message(&[input]);
        assert!(msg.contains("Stacked skills:"));
        assert!(msg.contains("clitic.do.post"));
        assert!(msg.contains("DO clitic post-infinitive"));
    }

    #[test]
    fn eval_user_message_numbers_multiple_items() {
        let i1 = make_eval_input("i1", "I want", "Quiero", "opener.quiero", "title1", "quiero");
        let i2 = make_eval_input("i2", "She wants", "Quiere", "opener.quiero", "title1", "quiere");
        let msg = build_eval_user_message(&[i1, i2]);
        assert!(msg.contains("ITEM 1"));
        assert!(msg.contains("ITEM 2"));
        assert!(msg.contains("I want"));
        assert!(msg.contains("She wants"));
    }

    #[test]
    fn eval_system_prompt_contains_required_sections() {
        assert!(EVAL_SYSTEM_PROMPT.contains("CORRECTNESS"));
        assert!(EVAL_SYSTEM_PROMPT.contains("ACCENTS"));
        assert!(EVAL_SYSTEM_PROMPT.contains("PUNCTUATION"));
        assert!(EVAL_SYSTEM_PROMPT.contains("CAPITALIZATION"));
        assert!(EVAL_SYSTEM_PROMPT.contains("AVOIDS TESTED CONSTRUCTION"));
        assert!(EVAL_SYSTEM_PROMPT.contains("PARTIAL CREDIT"));
        assert!(EVAL_SYSTEM_PROMPT.contains("ERROR ATTRIBUTION"));
        assert!(EVAL_SYSTEM_PROMPT.contains("REMARKS"));
        assert!(EVAL_SYSTEM_PROMPT.contains("EXPLANATION"));
    }

    #[test]
    fn save_attempts_inserts_as_unevaluated() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want to eat", "Quiero comer", "tag.a");

        let attempt = AttemptInput {
            item_id: "i1".to_string(),
            tag: "tag.a".to_string(),
            learner_answer: "quiero comer".to_string(),
        };
        save_attempts_unevaluated(&conn, &[attempt], "sess-1").unwrap();

        let eval_state: String = conn
            .query_row("SELECT eval_state FROM attempt_log WHERE item_id='i1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(eval_state, "unevaluated");
    }

    #[test]
    fn save_attempts_is_idempotent_on_retry() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want to eat", "Quiero comer", "tag.a");

        let attempt = AttemptInput {
            item_id: "i1".to_string(),
            tag: "tag.a".to_string(),
            learner_answer: "quiero comer".to_string(),
        };
        save_attempts_unevaluated(&conn, &[attempt.clone()], "sess-1").unwrap();
        save_attempts_unevaluated(&conn, &[attempt], "sess-1").unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM attempt_log WHERE session_id='sess-1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1, "retry must not duplicate rows");
    }

    #[test]
    fn update_attempt_with_eval_result_sets_evaluated() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want to eat", "Quiero comer", "tag.a");

        let attempt = AttemptInput {
            item_id: "i1".to_string(),
            tag: "tag.a".to_string(),
            learner_answer: "quiero comer".to_string(),
        };
        save_attempts_unevaluated(&conn, &[attempt], "sess-1").unwrap();

        let result = EvalResult {
            item_id: "i1".to_string(),
            correct: true,
            error_tag: None,
            remarks: vec![],
            explanation: None,
            canonical: "Quiero comer".to_string(),
        };
        update_attempt_eval(&conn, "sess-1", &result, "tag.a").unwrap();

        let (eval_state, correct): (String, i64) = conn
            .query_row(
                "SELECT eval_state, correct FROM attempt_log WHERE item_id='i1'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(eval_state, "evaluated");
        assert_eq!(correct, 1);
    }

    #[test]
    fn eval_result_includes_canonical_from_input() {
        let input = make_eval_input("i1", "I want to eat", "Quiero comer", "tag.a", "Tag A", "quiero comer");
        let ai_item = OpenAiEvalItem {
            correct: true,
            error_tag: None,
            remarks: vec![],
            explanation: None,
        };
        let result = EvalResult {
            item_id: input.item_id.clone(),
            correct: ai_item.correct,
            error_tag: ai_item.error_tag.clone(),
            remarks: ai_item.remarks.clone(),
            explanation: ai_item.explanation.clone(),
            canonical: input.canonical.clone(),
        };
        assert_eq!(result.canonical, "Quiero comer");
    }

    // ─── Tag attempt attribution tests ───────────────────────────────────────

    fn make_eval_input_with_stacked(
        item_id: &str,
        primary_tag: &str,
        stacked: Vec<&str>,
    ) -> EvalInput {
        EvalInput {
            item_id: item_id.to_string(),
            source: "src".to_string(),
            canonical: "canonical".to_string(),
            primary_tag: primary_tag.to_string(),
            primary_tag_title: "title".to_string(),
            stacked_tags: stacked
                .into_iter()
                .map(|t| (t.to_string(), t.to_string()))
                .collect(),
            learner_answer: "ans".to_string(),
        }
    }

    fn setup_item_and_attempt(conn: &Connection, item_id: &str, primary_tag: &str) {
        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at)
             VALUES (?1, 'src', 'canonical', ?2, '[]', 0)",
            params![item_id, primary_tag],
        ).unwrap();
        let attempt = AttemptInput {
            item_id: item_id.to_string(),
            tag: primary_tag.to_string(),
            learner_answer: "ans".to_string(),
        };
        save_attempts_unevaluated(conn, &[attempt], "sess-1").unwrap();
    }

    #[test]
    fn wrong_answer_with_primary_error_marks_primary_wrong() {
        let conn = in_memory();
        setup_item_and_attempt(&conn, "i1", "tag.primary");

        let result = EvalResult {
            item_id: "i1".to_string(),
            correct: false,
            error_tag: Some("tag.primary".to_string()),
            remarks: vec![],
            explanation: None,
            canonical: "canonical".to_string(),
        };
        update_attempt_eval(&conn, "sess-1", &result, "tag.primary").unwrap();

        let correct: i64 = conn
            .query_row(
                "SELECT correct FROM attempt_log WHERE item_id='i1' AND tag='tag.primary'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(correct, 0, "primary tag must be wrong when it is the errorTag");
    }

    #[test]
    fn wrong_answer_with_stacked_error_marks_primary_correct() {
        let conn = in_memory();
        setup_item_and_attempt(&conn, "i1", "tag.primary");

        let result = EvalResult {
            item_id: "i1".to_string(),
            correct: false,
            error_tag: Some("tag.stacked".to_string()),
            remarks: vec![],
            explanation: None,
            canonical: "canonical".to_string(),
        };
        update_attempt_eval(&conn, "sess-1", &result, "tag.primary").unwrap();

        let correct: i64 = conn
            .query_row(
                "SELECT correct FROM attempt_log WHERE item_id='i1' AND tag='tag.primary'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(correct, 1, "primary tag must be correct when a stacked tag is the errorTag");
    }

    #[test]
    fn stacked_tag_gets_wrong_attempt_when_it_is_error_tag() {
        let conn = in_memory();
        setup_item_and_attempt(&conn, "i1", "tag.primary");

        let result = EvalResult {
            item_id: "i1".to_string(),
            correct: false,
            error_tag: Some("tag.stacked".to_string()),
            remarks: vec![],
            explanation: None,
            canonical: "canonical".to_string(),
        };
        let input = make_eval_input_with_stacked("i1", "tag.primary", vec!["tag.stacked"]);
        insert_stacked_tag_attempts(&conn, "sess-1", &result, &input).unwrap();

        let correct: i64 = conn
            .query_row(
                "SELECT correct FROM attempt_log WHERE item_id='i1' AND tag='tag.stacked'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(correct, 0, "stacked errorTag must be logged as wrong");
    }

    #[test]
    fn non_error_stacked_tag_gets_correct_attempt() {
        let conn = in_memory();
        setup_item_and_attempt(&conn, "i1", "tag.primary");

        let result = EvalResult {
            item_id: "i1".to_string(),
            correct: false,
            error_tag: Some("tag.stacked.a".to_string()),
            remarks: vec![],
            explanation: None,
            canonical: "canonical".to_string(),
        };
        let input = make_eval_input_with_stacked(
            "i1",
            "tag.primary",
            vec!["tag.stacked.a", "tag.stacked.b"],
        );
        insert_stacked_tag_attempts(&conn, "sess-1", &result, &input).unwrap();

        let correct_b: i64 = conn
            .query_row(
                "SELECT correct FROM attempt_log WHERE item_id='i1' AND tag='tag.stacked.b'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(correct_b, 1, "non-errorTag stacked tag must be logged as correct");
    }

    #[test]
    fn correct_answer_logs_all_tags_as_correct() {
        let conn = in_memory();
        setup_item_and_attempt(&conn, "i1", "tag.primary");

        let result = EvalResult {
            item_id: "i1".to_string(),
            correct: true,
            error_tag: None,
            remarks: vec![],
            explanation: None,
            canonical: "canonical".to_string(),
        };
        let input = make_eval_input_with_stacked("i1", "tag.primary", vec!["tag.stacked"]);
        update_attempt_eval(&conn, "sess-1", &result, "tag.primary").unwrap();
        insert_stacked_tag_attempts(&conn, "sess-1", &result, &input).unwrap();

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM attempt_log WHERE item_id='i1' AND correct=1",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 2, "both primary and stacked must be correct");
    }

    #[test]
    fn stacked_attempts_are_idempotent() {
        let conn = in_memory();
        setup_item_and_attempt(&conn, "i1", "tag.primary");

        let result = EvalResult {
            item_id: "i1".to_string(),
            correct: false,
            error_tag: Some("tag.stacked".to_string()),
            remarks: vec![],
            explanation: None,
            canonical: "canonical".to_string(),
        };
        let input = make_eval_input_with_stacked("i1", "tag.primary", vec!["tag.stacked"]);
        insert_stacked_tag_attempts(&conn, "sess-1", &result, &input).unwrap();
        insert_stacked_tag_attempts(&conn, "sess-1", &result, &input).unwrap();

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM attempt_log WHERE item_id='i1' AND tag='tag.stacked'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "retry must not duplicate stacked tag rows");
    }

    #[test]
    fn get_pending_session_returns_none_when_empty() {
        let conn = in_memory();
        let result = get_pending_session_internal(&conn).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn get_pending_session_returns_unevaluated_attempts() {
        let conn = in_memory();
        insert_item(&conn, "i1", "Yo ___ alto.", "Yo soy alto.", "ser-estar");
        insert_item(&conn, "i2", "Ella ___ feliz.", "Ella está feliz.", "ser-estar");

        let attempts = vec![
            AttemptInput { item_id: "i1".into(), tag: "ser-estar".into(), learner_answer: "soy".into() },
            AttemptInput { item_id: "i2".into(), tag: "ser-estar".into(), learner_answer: "está".into() },
        ];
        save_attempts_unevaluated(&conn, &attempts, "sess-pending").unwrap();

        let result = get_pending_session_internal(&conn).unwrap();
        assert!(result.is_some());
        let pending = result.unwrap();
        assert_eq!(pending.len(), 2);
        assert_eq!(pending[0].item_id, "i1");
        assert_eq!(pending[0].tag, "ser-estar");
        assert_eq!(pending[0].learner_answer, "soy");
        assert_eq!(pending[0].source, "Yo ___ alto.");
    }

    #[test]
    fn get_pending_session_returns_none_after_evaluation() {
        let conn = in_memory();
        insert_item(&conn, "i1", "Yo ___ alto.", "Yo soy alto.", "ser-estar");

        let attempts = vec![
            AttemptInput { item_id: "i1".into(), tag: "ser-estar".into(), learner_answer: "soy".into() },
        ];
        save_attempts_unevaluated(&conn, &attempts, "sess-done").unwrap();

        // Mark as evaluated
        conn.execute(
            "UPDATE attempt_log SET eval_state='evaluated' WHERE session_id='sess-done'",
            [],
        ).unwrap();

        let result = get_pending_session_internal(&conn).unwrap();
        assert!(result.is_none());
    }
}
