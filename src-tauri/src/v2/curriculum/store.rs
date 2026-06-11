//! Persists the validated curriculum into the v2 database so licensing
//! sets are stored, versioned, and inspectable (PRD #31: "stored,
//! versioned, inspectable"). The database copy is a replaceable artifact —
//! the committed JSON files remain the source of truth, and every startup
//! rewrites the tables from the freshly validated load.

use super::loader::Curriculum;
use rusqlite::{params, Connection};

pub fn persist(conn: &mut Connection, curriculum: &Curriculum) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM curriculum_artifacts", [])?;
    tx.execute(
        "INSERT INTO curriculum_artifacts (kind, version, json) VALUES ('ambient_set', ?1, ?2)",
        params![
            curriculum.ambient.version,
            serde_json::to_string(&curriculum.ambient).unwrap()
        ],
    )?;
    tx.execute(
        "INSERT INTO curriculum_artifacts (kind, version, json) VALUES ('power_verbs', ?1, ?2)",
        params![
            curriculum.power_verbs.version,
            serde_json::to_string(&curriculum.power_verbs).unwrap()
        ],
    )?;
    tx.execute(
        "INSERT INTO curriculum_artifacts (kind, version, json) VALUES ('cognate_notes', ?1, ?2)",
        params![
            curriculum.cognate_notes.version,
            serde_json::to_string(&curriculum.cognate_notes).unwrap()
        ],
    )?;

    tx.execute("DELETE FROM curriculum_units", [])?;
    for (position, unit) in curriculum.units.iter().enumerate() {
        tx.execute(
            "INSERT INTO curriculum_units (id, position, phase, title, json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                unit.id,
                position as i64,
                unit.phase,
                unit.title,
                serde_json::to_string(unit).unwrap()
            ],
        )?;
    }

    tx.execute("DELETE FROM licensing_sets", [])?;
    for eff in curriculum.effective_licensing_all() {
        tx.execute(
            "INSERT INTO licensing_sets (unit_id, curriculum_version, ambient_version, json)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                eff.unit_id,
                eff.curriculum_version,
                eff.ambient_version,
                serde_json::to_string(eff).unwrap()
            ],
        )?;
    }

    tx.execute(
        "INSERT OR REPLACE INTO meta (key, value) VALUES ('curriculum_version', ?1)",
        params![curriculum.version.to_string()],
    )?;

    tx.commit()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum::load_embedded;
    use crate::v2::db::run_migrations;

    fn db_with_curriculum() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        let curriculum = load_embedded().unwrap();
        persist(&mut conn, &curriculum).unwrap();
        conn
    }

    #[test]
    fn persists_units_artifacts_and_licensing_sets() {
        let conn = db_with_curriculum();
        let curriculum = load_embedded().unwrap();

        let unit_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM curriculum_units", [], |r| r.get(0))
            .unwrap();
        assert_eq!(unit_count as usize, curriculum.units.len());

        let licensing_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM licensing_sets", [], |r| r.get(0))
            .unwrap();
        assert_eq!(
            licensing_count as usize,
            curriculum.units.len(),
            "every unit must have a stored effective licensing set"
        );

        let kinds: Vec<String> = conn
            .prepare("SELECT kind FROM curriculum_artifacts ORDER BY kind")
            .unwrap()
            .query_map([], |r| r.get(0))
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(kinds, vec!["ambient_set", "cognate_notes", "power_verbs"]);

        let curriculum_version: String = conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'curriculum_version'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(curriculum_version, curriculum.version.to_string());
    }

    #[test]
    fn persist_is_idempotent() {
        let mut conn = db_with_curriculum();
        let curriculum = load_embedded().unwrap();
        persist(&mut conn, &curriculum).unwrap();

        let unit_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM curriculum_units", [], |r| r.get(0))
            .unwrap();
        assert_eq!(unit_count as usize, curriculum.units.len());
    }

    #[test]
    fn stored_licensing_set_json_is_readable() {
        let conn = db_with_curriculum();
        let json: String = conn
            .query_row(
                "SELECT json FROM licensing_sets WHERE unit_id = 'opener.quiero'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["unit_id"], "opener.quiero");
        let forms = value["verb_forms"].as_array().unwrap();
        assert!(forms
            .iter()
            .any(|f| f["lemma"] == "querer" && f["form"] == "pres.1sg"));
    }
}
