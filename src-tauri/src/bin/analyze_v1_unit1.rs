//! Offline dev command (S4, #35): run the real analyzer over the v1
//! unit-1 exercise banks and write the inventories to
//! `fixtures/v1_unit1_analyses.json`.
//!
//! The committed output is the input to the hermetic
//! `v1_regression` test, which judges these analyses against the v2
//! unit-1 licensing sets and must catch the documented v1 ordering leaks
//! (quiere/queremos/ser inside unit 1 banks). Re-run only to refresh the
//! fixture after analyzer/prompt changes:
//!
//! ```sh
//! cargo run --bin analyze_v1_unit1
//! ```

use futures_util::{stream, StreamExt};
use serde::Deserialize;
use spanish_app_lib::v2::validator::{Analyzer, CandidateItem, OpenAiAnalyzer};

const V1_ITEMS: &str = include_str!("../../fixtures/v1_exercise_items.json");
const OUT_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/v1_unit1_analyses.json"
);
/// The v1 banks for the two units the documented leaks live in. The v1
/// tags happen to match the v2 unit ids.
const UNIT1_TAGS: [&str; 2] = ["opener.quiero", "opener.quiero.neg"];
const CONCURRENCY: usize = 5;

#[derive(Deserialize)]
struct V1Item {
    id: String,
    source: String,
    canonical: String,
    primary_tag: String,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let analyzer = OpenAiAnalyzer::from_env().expect("OPENAI_API_KEY must be set (.env)");

    let items: Vec<V1Item> = serde_json::from_str(V1_ITEMS).expect("v1 fixture parses");
    let unit1: Vec<&V1Item> = items
        .iter()
        .filter(|i| UNIT1_TAGS.contains(&i.primary_tag.as_str()))
        .collect();
    eprintln!("analyzing {} unit-1 items…", unit1.len());

    let analyses: Vec<serde_json::Value> = stream::iter(unit1)
        .map(|item| {
            let analyzer = &analyzer;
            async move {
                let candidate = CandidateItem {
                    source: item.source.clone(),
                    canonical: item.canonical.clone(),
                };
                let analysis = analyzer
                    .analyze(&candidate)
                    .await
                    .unwrap_or_else(|e| panic!("analysis of `{}` failed: {e}", item.canonical));
                eprintln!("  ✓ {}", item.canonical);
                serde_json::json!({
                    "item_id": item.id,
                    "primary_tag": item.primary_tag,
                    "source": item.source,
                    "canonical": item.canonical,
                    "analysis": analysis,
                })
            }
        })
        .buffered(CONCURRENCY)
        .collect()
        .await;

    let out = serde_json::json!({
        "analyzer_model": spanish_app_lib::v2::validator::analyzer::analyzer_model(),
        "analyses": analyses,
    });
    std::fs::write(OUT_PATH, serde_json::to_string_pretty(&out).unwrap() + "\n")
        .expect("write fixture");
    eprintln!("wrote {OUT_PATH}");
}
