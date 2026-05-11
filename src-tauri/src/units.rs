use crate::db::Db;
use crate::mastery;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UnitRow {
    pub n: u32,
    pub name: String,
    pub phase: u32,
    #[serde(rename = "skillTag")]
    pub skill_tag: String,
    #[serde(rename = "generationState")]
    pub generation_state: String,
    pub status: String,
    pub prerequisites: Vec<String>,
}

#[tauri::command]
pub fn list_units(state: tauri::State<'_, Db>) -> Result<Vec<UnitRow>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT unit_number, title, phase, skill_tag, generation_state, prerequisites
             FROM units
             WHERE unit_number IS NOT NULL
             ORDER BY unit_number ASC",
        )
        .map_err(|e| e.to_string())?;

    let rows: Vec<(u32, String, u32, String, String, String)> = stmt
        .query_map([], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get(5)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    // Fetch all attempts at once for efficiency, grouped by tag.
    // Sorted by tag then timestamp so we can split per-tag slices.
    let all_attempts: Vec<(String, bool)> = {
        let mut s = conn
            .prepare(
                "SELECT tag, correct FROM attempt_log ORDER BY tag ASC, timestamp ASC",
            )
            .map_err(|e| e.to_string())?;
        let rows: Vec<(String, bool)> = s
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .map(|(tag, c)| (tag, c != 0))
            .collect();
        rows
    };

    // Build tag → Vec<bool> map.
    let mut attempts_by_tag: std::collections::HashMap<String, Vec<bool>> =
        std::collections::HashMap::new();
    for (tag, correct) in all_attempts {
        attempts_by_tag.entry(tag).or_default().push(correct);
    }

    let units = rows
        .into_iter()
        .map(|(n, name, phase, skill_tag, gen_state, prereqs_json)| {
            let attempts = attempts_by_tag
                .get(&skill_tag)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            let status = mastery::derive_unit_status(attempts).to_string();
            let prerequisites: Vec<String> =
                serde_json::from_str(&prereqs_json).unwrap_or_default();
            UnitRow {
                n,
                name,
                phase,
                skill_tag,
                generation_state: gen_state,
                status,
                prerequisites,
            }
        })
        .collect();

    Ok(units)
}

#[tauri::command]
pub fn get_unit_by_n(
    state: tauri::State<'_, Db>,
    n: u32,
) -> Result<Option<UnitRow>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    let result = conn.query_row(
        "SELECT unit_number, title, phase, skill_tag, generation_state, prerequisites
         FROM units WHERE unit_number = ?1",
        params![n],
        |r| {
            Ok((
                r.get::<_, u32>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, u32>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
            ))
        },
    );

    match result {
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
        Ok((num, name, phase, skill_tag, gen_state, prereqs_json)) => {
            let attempts = mastery::fetch_attempts(&conn, &skill_tag)
                .unwrap_or_default();
            let status = mastery::derive_unit_status(&attempts).to_string();
            let prerequisites: Vec<String> =
                serde_json::from_str(&prereqs_json).unwrap_or_default();
            Ok(Some(UnitRow {
                n: num,
                name,
                phase,
                skill_tag,
                generation_state: gen_state,
                status,
                prerequisites,
            }))
        }
    }
}

/// Returns the unit_number of the most recently active unit from attempt_log,
/// falling back to 1 when no attempts exist yet.
#[tauri::command]
pub fn get_current_unit_number(state: tauri::State<'_, Db>) -> Result<u32, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let n: Option<u32> = conn
        .query_row(
            "SELECT u.unit_number
             FROM attempt_log al
             JOIN units u ON u.skill_tag = al.tag
             WHERE u.unit_number IS NOT NULL
             ORDER BY al.timestamp DESC
             LIMIT 1",
            [],
            |r| r.get(0),
        )
        .ok();
    Ok(n.unwrap_or(1))
}
