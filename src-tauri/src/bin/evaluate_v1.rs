//! Offline dev command (S7, #38): run the real Tier 1 evaluator over all
//! 131 archived v1 evaluations and write the decomposed analyses to
//! `fixtures/v1_eval_tier1.json`.
//!
//! The committed output is the input to the hermetic `eval::v1_regression`
//! test, which resolves these analyses with deterministic code and must
//! correct the documented unjust v1 verdicts ("Los puedes ver" rejected as
//! invalid Spanish). Re-run only to refresh the fixture after
//! evaluator/prompt changes:
//!
//! ```sh
//! cargo run --bin evaluate_v1
//! ```

use futures_util::{stream, StreamExt};
use serde::Deserialize;
use spanish_app_lib::v2::eval::{EvalInput, Evaluator, OpenAiEvaluator};
use std::collections::HashMap;

const V1_ITEMS: &str = include_str!("../../fixtures/v1_exercise_items.json");
const V1_EVALUATIONS: &str = include_str!("../../fixtures/v1_evaluations.json");
const OUT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/v1_eval_tier1.json");
const CONCURRENCY: usize = 5;

/// Target-structure glosses for every v1 tag in the evaluation log. The
/// v1 curriculum predates the v2 units file, so the descriptions are
/// authored here rather than derived.
const V1_TAG_GLOSSES: &[(&str, &str)] = &[
    ("opener.quiero", "querer (conjugated) followed directly by an infinitive (quiero comer)"),
    ("opener.puedo", "poder (conjugated) followed directly by an infinitive (puedo comer)"),
    ("opener.voy-a", "ir a + infinitive for the near future (voy a comer)"),
    ("opener.tengo-que", "tener que + infinitive for obligation (tengo que comer)"),
    ("clitic.do.sg.attach", "singular direct-object pronoun lo/la attached to an infinitive (quiero verlo)"),
    ("clitic.do.person.attach", "personal direct-object pronoun me/te/nos attached to an infinitive"),
    ("clitic.do.attach.mixed", "a direct-object pronoun attached to an infinitive, any opener"),
    ("clitic.do.before-finite", "a direct-object pronoun placed before the finite verb (lo veo)"),
    ("clitic.io.before-finite", "an indirect-object pronoun le/les placed before the finite verb"),
    ("clitic.both.before-finite", "two object pronouns (indirect then direct) before the finite verb (me lo da)"),
    ("clitic.placement.choice", "an object pronoun in either valid position (before the finite verb or attached to the infinitive)"),
    ("question.yes-no", "a yes/no question formed by intonation, written ¿…?"),
    ("question.wh", "a fronted question word (qué/quién/dónde/cuándo/cómo/por qué/cuánto)"),
    ("question.embedded", "an embedded question after saber (sé si/qué/dónde…)"),
    ("conj.pres.er.mixed", "present-tense conjugation of regular -er verbs"),
    ("conj.pres.ir.mixed", "present-tense conjugation of regular -ir verbs"),
    ("cond.regular", "the regular conditional tense"),
    ("gram.personal-a", "the personal a before a human direct object"),
    ("lex.cognate.tion", "English -tion words as Spanish -ción cognates"),
    ("lex.cognate.ible-able", "English -ible/-able words as Spanish cognates"),
];

#[derive(Deserialize)]
struct V1Item {
    id: String,
    source: String,
    canonical: String,
}

#[derive(Deserialize)]
struct V1Evaluation {
    id: String,
    tag: String,
    item_id: String,
    correct: i64,
    learner_answer: String,
    error_tag: Option<String>,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let evaluator = OpenAiEvaluator::from_env().expect("OPENAI_API_KEY must be set (.env)");

    let items: Vec<V1Item> = serde_json::from_str(V1_ITEMS).expect("v1 items fixture parses");
    let by_id: HashMap<&str, &V1Item> = items.iter().map(|i| (i.id.as_str(), i)).collect();
    let evals: Vec<V1Evaluation> =
        serde_json::from_str(V1_EVALUATIONS).expect("v1 evaluations fixture parses");
    let glosses: HashMap<&str, &str> = V1_TAG_GLOSSES.iter().copied().collect();
    eprintln!("evaluating {} archived v1 attempts…", evals.len());

    let analyses: Vec<serde_json::Value> = stream::iter(evals.iter())
        .map(|e| {
            let evaluator = &evaluator;
            let item = by_id[e.item_id.as_str()];
            let target = *glosses
                .get(e.tag.as_str())
                .unwrap_or_else(|| panic!("v1 tag `{}` has no gloss", e.tag));
            async move {
                let analysis = evaluator
                    .evaluate(&EvalInput {
                        cue: item.source.clone(),
                        answer: e.learner_answer.clone(),
                        target_description: target.to_string(),
                    })
                    .await
                    .unwrap_or_else(|err| {
                        panic!("evaluation of `{}` failed: {err}", e.learner_answer)
                    });
                eprintln!("  ✓ {}", e.learner_answer);
                serde_json::json!({
                    "v1_id": e.id,
                    "v1_tag": e.tag,
                    "v1_correct": e.correct,
                    "v1_error_tag": e.error_tag,
                    "cue": item.source,
                    "answer": e.learner_answer,
                    "canonical": item.canonical,
                    "analysis": analysis,
                })
            }
        })
        .buffered(CONCURRENCY)
        .collect()
        .await;

    let out = serde_json::json!({
        "evaluator_model": spanish_app_lib::v2::eval::tier1::evaluator_model(),
        "evaluations": analyses,
    });
    std::fs::write(OUT_PATH, serde_json::to_string_pretty(&out).unwrap() + "\n")
        .expect("write fixture");
    eprintln!("wrote {OUT_PATH}");
}
