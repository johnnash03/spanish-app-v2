//! SQLite persistence for the generated bank (S5, #36): validated items,
//! and the per-unit generation state machine
//! (idle → generating → ready/failed, carried over from v1).

use super::pipeline::BankSink;
use super::types::BankItem;
use crate::v2::db::DbV2;
use crate::v2::validator::ExistingItem;
use rusqlite::{params, Connection};

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

pub fn persist_item(conn: &Connection, item: &BankItem) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO bank_items
         (id, unit_id, source, canonical, variants, slot_spec, tags, analysis, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            item.id,
            item.unit_id,
            item.source,
            item.canonical,
            serde_json::to_string(&item.variants).unwrap(),
            serde_json::to_string(&item.slot).unwrap(),
            serde_json::to_string(&item.tags).unwrap(),
            serde_json::to_string(&item.analysis).unwrap(),
            now(),
        ],
    )?;
    Ok(())
}

/// Banked items of a unit, as near-duplication context for further
/// generation runs.
pub fn existing_items(conn: &Connection, unit_id: &str) -> rusqlite::Result<Vec<ExistingItem>> {
    let mut stmt =
        conn.prepare("SELECT id, canonical FROM bank_items WHERE unit_id = ?1 ORDER BY created_at")?;
    let items = stmt
        .query_map(params![unit_id], |r| {
            Ok(ExistingItem {
                id: r.get(0)?,
                canonical: r.get(1)?,
            })
        })?
        .collect::<Result<_, _>>()?;
    Ok(items)
}

/// English cues of a unit's banked items (the prompt's avoid-list).
pub fn existing_sources(conn: &Connection, unit_id: &str) -> rusqlite::Result<Vec<String>> {
    let mut stmt =
        conn.prepare("SELECT source FROM bank_items WHERE unit_id = ?1 ORDER BY created_at")?;
    let sources = stmt
        .query_map(params![unit_id], |r| r.get(0))?
        .collect::<Result<_, _>>()?;
    Ok(sources)
}

pub fn bank_count(conn: &Connection, unit_id: &str) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM bank_items WHERE unit_id = ?1",
        params![unit_id],
        |r| r.get(0),
    )
}

pub fn generation_state(conn: &Connection, unit_id: &str) -> String {
    conn.query_row(
        "SELECT state FROM unit_generation WHERE unit_id = ?1",
        params![unit_id],
        |r| r.get(0),
    )
    .unwrap_or_else(|_| "idle".to_string())
}

pub fn set_generation_state(
    conn: &Connection,
    unit_id: &str,
    state: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO unit_generation (unit_id, state, updated_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(unit_id) DO UPDATE SET state = ?2, updated_at = ?3",
        params![unit_id, state, now()],
    )?;
    Ok(())
}

/// [`BankSink`] over the shared v2 database: each passing item is written
/// the moment the pipeline hands it over (streaming persistence).
pub struct SqliteBankSink<'a>(pub &'a DbV2);

impl BankSink for SqliteBankSink<'_> {
    fn persist(&self, item: &BankItem) -> Result<(), String> {
        let conn = self.0 .0.lock().map_err(|e| e.to_string())?;
        persist_item(&conn, item).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::db::run_migrations;
    use crate::v2::generator::plan::ItemTags;
    use crate::v2::validator::{ItemAnalysis, SlotSpec};

    fn in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    fn bank_item(id: &str, canonical: &str) -> BankItem {
        BankItem {
            id: id.into(),
            unit_id: "opener.quiero".into(),
            source: "I want to eat.".into(),
            canonical: canonical.into(),
            variants: vec![],
            slot: SlotSpec::default(),
            tags: ItemTags {
                target_skill: "opener.quiero".into(),
                stacked: vec![],
            },
            analysis: ItemAnalysis::default(),
        }
    }

    #[test]
    fn persists_and_reads_back_bank_items() {
        let conn = in_memory();
        persist_item(&conn, &bank_item("a", "Quiero comer.")).unwrap();
        persist_item(&conn, &bank_item("b", "Quiero bailar.")).unwrap();

        assert_eq!(bank_count(&conn, "opener.quiero").unwrap(), 2);
        assert_eq!(bank_count(&conn, "opener.puedo").unwrap(), 0);

        let existing = existing_items(&conn, "opener.quiero").unwrap();
        assert_eq!(existing.len(), 2);
        assert_eq!(existing[0].canonical, "Quiero comer.");

        let sources = existing_sources(&conn, "opener.quiero").unwrap();
        assert_eq!(sources, vec!["I want to eat.", "I want to eat."]);

        // The stored JSON columns parse back into their types.
        let (variants, slot, tags, analysis): (String, String, String, String) = conn
            .query_row(
                "SELECT variants, slot_spec, tags, analysis FROM bank_items WHERE id = 'a'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
            )
            .unwrap();
        serde_json::from_str::<Vec<crate::v2::generator::ValidatedVariant>>(&variants).unwrap();
        serde_json::from_str::<SlotSpec>(&slot).unwrap();
        serde_json::from_str::<ItemTags>(&tags).unwrap();
        serde_json::from_str::<ItemAnalysis>(&analysis).unwrap();
    }

    #[test]
    fn generation_state_defaults_to_idle_and_round_trips() {
        let conn = in_memory();
        assert_eq!(generation_state(&conn, "opener.quiero"), "idle");
        set_generation_state(&conn, "opener.quiero", "generating").unwrap();
        assert_eq!(generation_state(&conn, "opener.quiero"), "generating");
        set_generation_state(&conn, "opener.quiero", "ready").unwrap();
        assert_eq!(generation_state(&conn, "opener.quiero"), "ready");
    }
}
