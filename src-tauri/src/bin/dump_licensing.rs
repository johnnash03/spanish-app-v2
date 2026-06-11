//! Dev command (S2, #33): dump a unit's effective licensing set for
//! inspection, straight from the committed curriculum data.
//!
//! Usage, from `src-tauri/`:
//!
//! ```sh
//! cargo run --bin dump_licensing            # list unit ids
//! cargo run --bin dump_licensing -- <unit>  # dump one unit's licensing set
//! cargo run --bin dump_licensing -- --all   # dump every unit
//! ```

use spanish_app_lib::v2::curriculum;

fn main() {
    let curriculum = match curriculum::load_embedded() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("curriculum failed validation: {e}");
            std::process::exit(1);
        }
    };

    match std::env::args().nth(1).as_deref() {
        None => {
            println!(
                "curriculum v{} — {} units. Pass a unit id (or --all) to dump its effective licensing set:",
                curriculum.version,
                curriculum.units.len()
            );
            for unit in &curriculum.units {
                println!("  {}  (phase {}: {})", unit.id, unit.phase, unit.title);
            }
        }
        Some("--all") => {
            let all: Vec<_> = curriculum.effective_licensing_all().collect();
            println!("{}", serde_json::to_string_pretty(&all).unwrap());
        }
        Some(unit_id) => match curriculum::dump_effective(&curriculum, unit_id) {
            Ok(json) => println!("{}", serde_json::to_string_pretty(&json).unwrap()),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(2);
            }
        },
    }
}
