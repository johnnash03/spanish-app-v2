//! V2 core (PRD #31). New curriculum, generation, and evaluation subsystems
//! are built here slice by slice (S1–S14). State lives in the v2 database
//! (`spanish-app-v2.db`), entirely separate from the legacy v1 database.

pub mod curriculum;
pub mod db;
pub mod fixtures;
