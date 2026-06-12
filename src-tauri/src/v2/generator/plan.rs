//! Bank planning: per-item slot specs and the one-unknown rule's input
//! partition (S5, #36).
//!
//! The one-unknown rule (PRD #31, load-bearing): every item may carry
//! difficulty on exactly one axis. Code — not the model — computes which
//! words are legal for each item before generation, and the validator
//! rejects any item that strays outside its partition. Without this rule,
//! vocabulary failures masquerade as grammar failures.

use crate::v2::curriculum::types::TargetAtom;
use crate::v2::curriculum::Curriculum;
use crate::v2::validator::{Polarity, SentenceType, SlotSpec};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// The learner's word and skill state at generation time. Until the Words
/// track lands (S10/S11) the live pipeline runs on `Default::default()`
/// (empty window, nothing mastered); the partition logic and its contract
/// are fixed here.
#[derive(Debug, Clone, Default)]
pub struct LearnerSnapshot {
    /// Consolidated vocabulary beyond the curriculum: graduated words still
    /// cycling through exercises, plus window words with at least one prior
    /// successful use.
    pub consolidated: BTreeSet<String>,
    /// Window words in their early encounters (no successful use yet) —
    /// each may appear only as the single unknown of an otherwise
    /// fully-mastered item.
    pub early_window: BTreeSet<String>,
    /// Skill (unit) ids currently at mastery.
    pub mastered_skills: BTreeSet<String>,
}

/// Which axis an item's one allowed unknown lives on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnknownAxis {
    /// The item drills a not-yet-mastered structure: content words must
    /// come from consolidated vocabulary only.
    Structure,
    /// The item carries one early-window word; every structure in it is at
    /// mastery.
    Word(String),
}

/// The words legal for one item beyond the unit's licensed vocabulary
/// (passed to the judge as the item's window — anything else is rejected
/// as [`crate::v2::validator::Violation::UnlicensedVocab`]).
pub fn legal_window(unknown: &UnknownAxis, learner: &LearnerSnapshot) -> BTreeSet<String> {
    let mut legal = learner.consolidated.clone();
    if let UnknownAxis::Word(lemma) = unknown {
        legal.insert(lemma.clone());
    }
    legal
}

/// The skill tags an item is authored against, persisted with it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemTags {
    pub target_skill: String,
    #[serde(default)]
    pub stacked: Vec<String>,
}

/// One planned bank slot: what the generator is asked to produce and what
/// the validator will hold it to.
#[derive(Debug, Clone)]
pub struct ItemPlan {
    pub slot_id: u32,
    pub spec: SlotSpec,
    pub tags: ItemTags,
    pub unknown: UnknownAxis,
    /// Words legal for this item beyond the unit's licensed vocabulary
    /// ([`legal_window`] of `unknown`).
    pub legal_window: BTreeSet<String>,
}

/// Grammatical persons in curriculum order (no vosotros/voseo, matching
/// `FORM_SLOTS`).
const PERSON_ORDER: [&str; 5] = ["1sg", "2sg", "3sg", "1pl", "3pl"];

/// Sense variations for polysemous curriculum verbs (PRD #31: "sense
/// variation for polysemous verbs"). A hint enters an item's slot spec
/// only when the lemma is licensed for the unit.
const POLYSEMY: &[(&str, &[&str])] = &[
    ("esperar", &["to wait (for)", "to hope"]),
    ("saber", &["to know a fact", "to know how (saber + infinitive)"]),
    ("seguir", &["to follow", "to continue (doing something)"]),
    ("servir", &["to serve", "to be useful"]),
    ("tomar", &["to take", "to drink"]),
    ("dejar", &["to leave (something behind)", "to let / allow"]),
];

/// Items before this index isolate the target skill (minimum pairs).
const FIRST_STACKED_ITEM: usize = 3;
/// Items from this index on may stack two prior skills.
const FIRST_DOUBLE_STACKED_ITEM: usize = 10;
const MAX_STACK_POOL: usize = 4;

/// Plans a bank of `n` items for a unit: deterministic slot specs varying
/// person, polarity, sentence type, and sense across the licensed space
/// (user story 7), stacked prior skills, and early-window words scheduled
/// under the one-unknown rule (user story 38). `None` for an unknown unit.
pub fn plan_bank(
    c: &Curriculum,
    unit_id: &str,
    learner: &LearnerSnapshot,
    n: usize,
) -> Option<Vec<ItemPlan>> {
    let licensing = c.effective_licensing(unit_id)?;

    let persons: Vec<&str> = PERSON_ORDER
        .iter()
        .filter(|p| {
            let suffix = format!(".{p}");
            licensing.verb_forms.iter().any(|vf| vf.form.ends_with(&suffix))
                || (!licensing.vocab.is_empty()
                    && licensing
                        .vocab_verb_forms
                        .iter()
                        .any(|g| g.form.ends_with(&suffix)))
        })
        .copied()
        .collect();

    let negation_licensed = licensing.constructions.iter().any(|t| t.starts_with("neg."));
    let questions_licensed = licensing
        .constructions
        .iter()
        .any(|t| t.starts_with("question."));
    // A unit whose own target is a question construction produces only
    // questions; otherwise questions are one slot axis among others.
    let target_is_question = c.target_spec(unit_id).is_some_and(|spec| {
        spec.groups.iter().flatten().any(
            |a| matches!(a, TargetAtom::Construction(t) if t.starts_with("question.")),
        )
    });

    let stack_pool = stack_pool(c, unit_id);

    // Sense hints for licensed polysemous lemmas, assigned round-robin to
    // every third item.
    let sense_hints: Vec<String> = POLYSEMY
        .iter()
        .filter(|(lemma, _)| {
            licensing.vocab.contains(*lemma)
                || licensing.verb_forms.iter().any(|vf| vf.lemma == *lemma)
        })
        .flat_map(|(lemma, senses)| senses.iter().map(move |s| format!("{lemma} = {s}")))
        .collect();

    let unit_mastered = learner.mastered_skills.contains(unit_id);
    let mut early_words = learner.early_window.iter().cloned().cycle();
    let has_early_words = !learner.early_window.is_empty();

    let mut plans = Vec::with_capacity(n);
    for idx in 0..n {
        let stacked: Vec<String> = if stack_pool.is_empty() || idx < FIRST_STACKED_ITEM {
            vec![]
        } else {
            let take = if idx >= FIRST_DOUBLE_STACKED_ITEM && stack_pool.len() >= 2 {
                2
            } else {
                1
            };
            (0..take)
                .map(|k| stack_pool[(idx - FIRST_STACKED_ITEM + k) % stack_pool.len()].clone())
                .collect()
        };

        // One-unknown rule: an early-window word may only ride an item
        // whose every structure — the unit's and the stacked ones — is at
        // mastery. Schedule words into every other eligible item so new
        // vocabulary doesn't saturate the bank.
        let all_structures_mastered = unit_mastered
            && stacked.iter().all(|s| learner.mastered_skills.contains(s));
        let unknown = if all_structures_mastered && has_early_words && idx % 2 == 0 {
            UnknownAxis::Word(early_words.next().expect("cycle of non-empty set"))
        } else {
            UnknownAxis::Structure
        };

        let spec = SlotSpec {
            person: (!persons.is_empty()).then(|| persons[idx % persons.len()].to_string()),
            polarity: Some(if negation_licensed && idx % 3 == 2 {
                Polarity::Negative
            } else {
                Polarity::Affirmative
            }),
            sentence_type: Some(if target_is_question || (questions_licensed && idx % 4 == 3) {
                SentenceType::Question
            } else {
                SentenceType::Declarative
            }),
            sense: (!sense_hints.is_empty() && idx % 3 == 1)
                .then(|| sense_hints[(idx / 3) % sense_hints.len()].clone()),
            required_lemma: match &unknown {
                UnknownAxis::Word(w) => Some(w.clone()),
                UnknownAxis::Structure => None,
            },
        };

        plans.push(ItemPlan {
            slot_id: idx as u32,
            legal_window: legal_window(&unknown, learner),
            spec,
            tags: ItemTags {
                target_skill: unit_id.to_string(),
                stacked: stacked.clone(),
            },
            unknown,
        });
    }
    Some(plans)
}

/// Prior skills a unit's items may stack: ancestors (nearest first) whose
/// target is checkable as pure constructions and that belong to a
/// different skill family. Form-target units (the openers) underlie every
/// later sentence anyway, and same-family stacks (a wh-question that is
/// also a yes/no question) are linguistically unsatisfiable — demanding
/// them would only burn repair rounds.
fn stack_pool(c: &Curriculum, unit_id: &str) -> Vec<String> {
    let family = |id: &str| id.split('.').next().unwrap_or(id).to_string();
    let unit_family = family(unit_id);

    let mut pool = Vec::new();
    let mut seen: BTreeSet<String> = [unit_id.to_string()].into();
    let mut queue: std::collections::VecDeque<String> = c
        .unit(unit_id)
        .map(|u| u.prereqs.clone())
        .unwrap_or_default()
        .into();
    while let Some(id) = queue.pop_front() {
        if !seen.insert(id.clone()) {
            continue;
        }
        if let Some(unit) = c.unit(&id) {
            queue.extend(unit.prereqs.iter().cloned());
            let construction_only = c.target_spec(&id).is_some_and(|spec| {
                spec.groups.iter().all(|g| {
                    g.iter().any(|a| matches!(a, TargetAtom::Construction(_)))
                })
            });
            if construction_only && family(&id) != unit_family {
                pool.push(id);
            }
        }
    }
    pool.truncate(MAX_STACK_POOL);
    pool
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;

    fn learner() -> LearnerSnapshot {
        LearnerSnapshot {
            consolidated: ["gato".to_string(), "casa".to_string()].into(),
            early_window: ["perro".to_string(), "libro".to_string()].into(),
            mastered_skills: ["opener.quiero".to_string()].into(),
        }
    }

    #[test]
    fn structure_unknown_items_get_consolidated_words_only() {
        // An item drilling an unmastered structure must never carry an
        // early-encounter window word: its one unknown is the structure.
        let legal = legal_window(&UnknownAxis::Structure, &learner());
        assert_eq!(legal, ["gato".to_string(), "casa".to_string()].into());
        assert!(!legal.contains("perro"));
    }

    #[test]
    fn word_unknown_items_add_exactly_the_scheduled_word() {
        let legal = legal_window(&UnknownAxis::Word("perro".into()), &learner());
        assert!(legal.contains("perro"));
        assert!(!legal.contains("libro"), "the other early word stays illegal");
        assert!(legal.contains("gato"), "consolidated words remain legal");
    }

    // --- plan_bank ---

    #[test]
    fn unit_one_plan_stays_inside_the_licensed_space() {
        // opener.quiero licenses exactly one finite person (1sg), no
        // questions, and has no stackable ancestors — the plan must not
        // demand anything the licensing set cannot satisfy.
        let c = curriculum::load_embedded().unwrap();
        let plans = plan_bank(&c, "opener.quiero", &LearnerSnapshot::default(), 12).unwrap();
        assert_eq!(plans.len(), 12);
        for (idx, p) in plans.iter().enumerate() {
            assert_eq!(p.slot_id, idx as u32);
            assert_eq!(p.spec.person.as_deref(), Some("1sg"));
            assert_eq!(p.spec.sentence_type, Some(SentenceType::Declarative));
            assert!(p.tags.stacked.is_empty(), "no stackable ancestors in unit 1");
            assert_eq!(p.tags.target_skill, "opener.quiero");
            assert_eq!(p.unknown, UnknownAxis::Structure);
            assert!(p.legal_window.is_empty());
            assert!(p.spec.required_lemma.is_none());
        }
        // Negation is ambient-licensed, so polarity still varies.
        assert!(plans.iter().any(|p| p.spec.polarity == Some(Polarity::Negative)));
        assert!(plans.iter().any(|p| p.spec.polarity == Some(Polarity::Affirmative)));
        // esperar is licensed unit-1 vocab — its two senses get scheduled.
        assert!(plans
            .iter()
            .any(|p| p.spec.sense.as_deref() == Some("esperar = to hope")));
    }

    #[test]
    fn question_unit_plans_only_questions_and_varies_person() {
        let c = curriculum::load_embedded().unwrap();
        let plans = plan_bank(&c, "question.yes-no", &LearnerSnapshot::default(), 8).unwrap();
        assert!(plans
            .iter()
            .all(|p| p.spec.sentence_type == Some(SentenceType::Question)));
        let persons: BTreeSet<&str> =
            plans.iter().filter_map(|p| p.spec.person.as_deref()).collect();
        assert!(persons.contains("2sg"), "tú forms are the unit's grant");
        assert!(persons.len() >= 2, "person must vary, got {persons:?}");
    }

    #[test]
    fn stacking_starts_after_minimum_pairs_and_crosses_skill_families() {
        // clitic.io.attach's ancestors include the construction-targeted
        // openers (tengo-que, voy-a) and the same-family clitic.do units;
        // only cross-family construction targets are stackable.
        let c = curriculum::load_embedded().unwrap();
        let plans = plan_bank(&c, "clitic.io.attach", &LearnerSnapshot::default(), 12).unwrap();
        for p in &plans[..FIRST_STACKED_ITEM] {
            assert!(p.tags.stacked.is_empty(), "items 0–2 are minimum pairs");
        }
        let stacked: BTreeSet<&str> = plans
            .iter()
            .flat_map(|p| p.tags.stacked.iter().map(String::as_str))
            .collect();
        assert!(!stacked.is_empty(), "later items must stack prior skills");
        for tag in &stacked {
            assert!(
                tag.starts_with("opener."),
                "only cross-family construction targets may stack, got {tag}"
            );
        }
    }

    #[test]
    fn early_words_are_scheduled_only_into_fully_mastered_structures() {
        let c = curriculum::load_embedded().unwrap();
        let mut learner = LearnerSnapshot::default();
        learner.early_window.insert("mensaje".into());

        // Unit not mastered: the structure is the unknown everywhere, so
        // no early word may enter any item.
        let unmastered = plan_bank(&c, "opener.quiero", &learner, 8).unwrap();
        assert!(unmastered.iter().all(|p| p.unknown == UnknownAxis::Structure));
        assert!(unmastered.iter().all(|p| p.spec.required_lemma.is_none()));

        // Unit mastered: words ride some items, declared as the unknown,
        // required by the slot spec, and legal only in their own item.
        learner.mastered_skills.insert("opener.quiero".into());
        let mastered = plan_bank(&c, "opener.quiero", &learner, 8).unwrap();
        let word_items: Vec<_> = mastered
            .iter()
            .filter(|p| p.unknown == UnknownAxis::Word("mensaje".into()))
            .collect();
        assert!(!word_items.is_empty(), "mastered unit must schedule the window word");
        assert!(word_items.len() < mastered.len(), "not every item carries the new word");
        for p in word_items {
            assert_eq!(p.spec.required_lemma.as_deref(), Some("mensaje"));
            assert!(p.legal_window.contains("mensaje"));
        }
        let structure_items = mastered.iter().filter(|p| p.unknown == UnknownAxis::Structure);
        for p in structure_items {
            assert!(!p.legal_window.contains("mensaje"));
        }
    }

    #[test]
    fn plan_bank_rejects_unknown_units() {
        let c = curriculum::load_embedded().unwrap();
        assert!(plan_bank(&c, "nope", &LearnerSnapshot::default(), 5).is_none());
    }
}
