//! Generates `crates/ofl/data/ofl-library.json.gz` from a checkout of the
//! Open Fixture Library.
//!
//! ```sh
//! git clone --depth 1 https://github.com/OpenLightingProject/open-fixture-library ofl
//! cargo run -p dmxcraft-ofl --bin ofl-bundle -- ofl crates/ofl/data/ofl-library.json.gz
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;

use dmxcraft_ofl::{convert_fixture, Library, OflError};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ofl-bundle <OFL checkout> <output.json.gz>");
        std::process::exit(2);
    }
    let root = PathBuf::from(&args[1]);
    let out = PathBuf::from(&args[2]);
    match run(&root, &out) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}

fn run(root: &Path, out: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let fixtures_dir = root.join("fixtures");
    let manufacturers: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(
        fixtures_dir.join("manufacturers.json"),
    )?)?;

    let mut lib = Library {
        source_commit: git(root, &["rev-parse", "HEAD"]),
        source_date: git(root, &["log", "-1", "--format=%cs"]),
        fixtures: Vec::new(),
    };
    let (mut skipped, mut failed) = (0, 0);

    let mut man_dirs: Vec<_> = std::fs::read_dir(&fixtures_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .collect();
    man_dirs.sort_by_key(|e| e.file_name());

    for man in man_dirs {
        let man_key = man.file_name().to_string_lossy().to_string();
        let man_name = manufacturers
            .get(&man_key)
            .and_then(|m| m.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or(&man_key)
            .to_string();
        let mut files: Vec<_> = std::fs::read_dir(man.path())?
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "json"))
            .collect();
        files.sort();
        for file in files {
            let key = file
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let json = std::fs::read_to_string(&file)?;
            match convert_fixture(&man_key, &man_name, &key, &json) {
                Ok(f) => lib.fixtures.push(f),
                Err(OflError::Skip(_)) => skipped += 1,
                Err(e) => {
                    failed += 1;
                    eprintln!("  {man_key}/{key}: {e}");
                }
            }
        }
    }

    let bytes = lib.to_gzip()?;
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(out, &bytes)?;
    let modes: usize = lib.fixtures.iter().map(|f| f.modes.len()).sum();
    println!(
        "{} fixtures, {} modes ({} skipped, {} failed) → {} ({} kB), OFL {} from {}",
        lib.fixtures.len(),
        modes,
        skipped,
        failed,
        out.display(),
        bytes.len() / 1024,
        lib.source_commit.get(..10).unwrap_or(""),
        lib.source_date
    );
    Ok(())
}

fn git(dir: &Path, args: &[&str]) -> String {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
