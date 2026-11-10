//! Generates `crates/ofl/data/qlc-library.json.gz` from a checkout of QLC+.
//!
//! ```sh
//! git clone --depth 1 --filter=blob:none --sparse https://github.com/mcallegari/qlcplus qlcplus
//! (cd qlcplus && git sparse-checkout set resources/fixtures)
//! cargo run -p dmxcraft-ofl --bin qlc-bundle -- qlcplus crates/ofl/data/qlc-library.json.gz
//! ```

use std::path::{Path, PathBuf};
use std::process::Command;

use dmxcraft_ofl::{convert_qxf, Library, LibraryFixture};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: qlc-bundle <QLC+ checkout> <output.json.gz>");
        std::process::exit(2);
    }
    if let Err(e) = run(Path::new(&args[1]), Path::new(&args[2])) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn qxf_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for e in std::fs::read_dir(dir)? {
        let p = e?.path();
        if p.is_dir() {
            qxf_files(&p, out)?;
        } else if p.extension().is_some_and(|x| x == "qxf") {
            out.push(p);
        }
    }
    Ok(())
}

fn run(root: &Path, out: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    qxf_files(&root.join("resources/fixtures"), &mut files)?;
    files.sort();

    let mut lib = Library {
        source_commit: git(root, &["rev-parse", "HEAD"]),
        source_date: git(root, &["log", "-1", "--format=%cs"]),
        fixtures: Vec::new(),
    };
    let mut failed = 0;
    for file in &files {
        let xml = std::fs::read_to_string(file)?;
        match convert_qxf(&xml) {
            Ok(f) => lib.fixtures.push(f),
            Err(e) => {
                failed += 1;
                eprintln!("  {}: {e}", file.display());
            }
        }
    }
    dedupe_keys(&mut lib.fixtures);
    let bytes = lib.to_gzip()?;
    std::fs::write(out, &bytes)?;
    let modes: usize = lib.fixtures.iter().map(|f| f.modes.len()).sum();
    println!(
        "{} fixtures, {modes} modes, {failed} failed → {} ({} KB)",
        lib.fixtures.len(),
        out.display(),
        bytes.len() / 1024
    );
    Ok(())
}

/// The same manufacturer/model names in several files get unique keys
/// (`…-2`) so shows find their type again.
fn dedupe_keys(fixtures: &mut [LibraryFixture]) {
    let mut seen = std::collections::HashMap::<String, usize>::new();
    for f in fixtures {
        let base = format!("{}/{}", f.manufacturer_key, f.key);
        let n = seen.entry(base).or_insert(0);
        *n += 1;
        if *n > 1 {
            let suffix = format!("-{n}");
            f.key.push_str(&suffix);
            for m in &mut f.modes {
                m.key = m.key.replacen(
                    &format!(
                        "qlc/{}/{}/",
                        f.manufacturer_key,
                        &f.key[..f.key.len() - suffix.len()]
                    ),
                    &format!("qlc/{}/{}/", f.manufacturer_key, f.key),
                    1,
                );
            }
        }
    }
}

fn git(root: &Path, args: &[&str]) -> String {
    Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}
