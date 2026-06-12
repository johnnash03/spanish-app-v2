//! Offline dev command (S5, #36): run the real generate → validate →
//! repair pipeline for one unit and write the resulting bank to
//! `fixtures/generated_bank_<unit>.json` for inspection.
//!
//! The committed opener.quiero output is the input to the hermetic
//! `s5_acceptance` test, which re-judges every banked item against the
//! unit's licensing set (acceptance: "a validated bank for a Phase 1 unit
//! with zero licensing violations"). Re-run to refresh after prompt or
//! pipeline changes:
//!
//! ```sh
//! cargo run --bin generate_unit -- [unit-id] [items]
//! ```

use spanish_app_lib::v2::generator::{
    generate_unit_bank, source::generator_model, source::OpenAiItemSource, BankItem, BankSink,
    LearnerSnapshot, PipelineConfig,
};
use spanish_app_lib::v2::validator::{analyzer::analyzer_model, OpenAiAnalyzer};
use spanish_app_lib::v2::curriculum;
use std::sync::Mutex;

#[derive(Default)]
struct MemorySink(Mutex<Vec<BankItem>>);

impl BankSink for MemorySink {
    fn persist(&self, item: &BankItem) -> Result<(), String> {
        self.0.lock().unwrap().push(item.clone());
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let mut args = std::env::args().skip(1);
    let unit_id = args.next().unwrap_or_else(|| "opener.quiero".to_string());
    let items_per_unit: usize = args
        .next()
        .map(|n| n.parse().expect("items must be a number"))
        .unwrap_or(20);

    let c = curriculum::load_embedded().expect("committed curriculum loads");
    let source = OpenAiItemSource::from_env().expect("OPENAI_API_KEY must be set (.env)");
    let analyzer = OpenAiAnalyzer::from_env().expect("OPENAI_API_KEY must be set (.env)");
    let sink = MemorySink::default();

    eprintln!("generating {items_per_unit} items for `{unit_id}`…");
    let outcome = generate_unit_bank(
        &source,
        &analyzer,
        &sink,
        &c,
        &unit_id,
        &LearnerSnapshot::default(),
        vec![],
        vec![],
        &PipelineConfig {
            items_per_unit,
            ..Default::default()
        },
    )
    .await
    .expect("pipeline runs");

    let items = sink.0.into_inner().unwrap();
    let out_path = format!(
        "{}/fixtures/generated_bank_{}.json",
        env!("CARGO_MANIFEST_DIR"),
        unit_id.replace(['.', '-'], "_")
    );
    let out = serde_json::json!({
        "unit_id": unit_id,
        "generator_model": generator_model(),
        "analyzer_model": analyzer_model(),
        "banked": outcome.banked,
        "rounds": outcome.rounds,
        "abandoned_slots": outcome
            .abandoned
            .iter()
            .map(|f| serde_json::json!({
                "slot_id": f.plan.slot_id,
                "violations": f.violations,
            }))
            .collect::<Vec<_>>(),
        "items": items,
    });
    std::fs::write(&out_path, serde_json::to_string_pretty(&out).unwrap() + "\n")
        .expect("write fixture");

    eprintln!("\n=== {} items banked in {} round(s) ===", outcome.banked, outcome.rounds);
    for item in &items {
        eprintln!(
            "  [{}] {} → {}  (+{} variants)",
            item.tags.target_skill,
            item.source,
            item.canonical,
            item.variants.len()
        );
    }
    if !outcome.abandoned.is_empty() {
        eprintln!("=== {} slot(s) abandoned ===", outcome.abandoned.len());
    }
    eprintln!("wrote {out_path}");
}
