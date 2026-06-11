//! LEGACY (v1) — quarantined in S1 (#32) as part of the v2 strangler-fig
//! rewrite (PRD #31). Everything in this module serves the v1 UI, which
//! remains the default until the v2 home lands (S14, #45), then demotes to
//! a Legacy menu entry. State lives in the legacy v1 database
//! (`spanish-app.db`), which v2 never touches.
//!
//! Do not extend any module in here. The whole namespace is deleted in
//! S17: Legacy teardown (#48). Each submodule's header names the v2 slice
//! that replaces it.

pub mod combined;
pub mod db;
pub mod deliberate_practice;
pub mod generate;
pub mod mastery;
pub mod session;
pub mod srs;
pub mod units;
pub mod vocab;
