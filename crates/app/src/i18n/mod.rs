//! Übersetzung der Oberfläche.
//!
//! Deutsch ist die Quellsprache: Texte stehen im Code auf Deutsch und laufen
//! über [`t`] (feste Texte) oder [`tr!`] (Texte mit Platzhaltern
//! `{name}`). Für Englisch schlägt [`t`] den deutschen Text in [`en::ENTRIES`]
//! nach. Fehlt ein Eintrag, bleibt der deutsche Text stehen – ein Test
//! (`all_texts_translated`) sorgt dafür, dass das nicht passiert.

mod en;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    De,
    En,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::De, Lang::En];

    /// Schlüssel für die Einstellungen.
    pub fn key(self) -> &'static str {
        match self {
            Lang::De => "de",
            Lang::En => "en",
        }
    }

    pub fn from_key(key: &str) -> Lang {
        Lang::ALL
            .into_iter()
            .find(|l| l.key() == key)
            .unwrap_or(Lang::De)
    }

    /// Name der Sprache in ihr selbst (wird nicht übersetzt).
    pub fn native_name(self) -> &'static str {
        match self {
            Lang::De => "Deutsch",
            Lang::En => "English",
        }
    }
}

static LANG: AtomicU8 = AtomicU8::new(0);

pub fn set_lang(l: Lang) {
    let i = Lang::ALL.iter().position(|x| *x == l).unwrap_or(0) as u8;
    LANG.store(i, Ordering::Relaxed);
}

pub fn lang() -> Lang {
    Lang::ALL[usize::from(LANG.load(Ordering::Relaxed)) % Lang::ALL.len()]
}

static EN: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| en::ENTRIES.iter().copied().collect());

/// Übersetzt einen festen deutschen Text in die aktive Sprache.
pub fn t(de: &'static str) -> &'static str {
    match lang() {
        Lang::De => de,
        Lang::En => EN.get(de).copied().unwrap_or(de),
    }
}

/// Setzt Werte in einen (übersetzten) Text mit Platzhaltern `{name}` ein.
pub fn fill(template: &str, args: &[(&str, String)]) -> String {
    let mut out = template.to_string();
    for (k, v) in args {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

/// Übersetzter Text mit Platzhaltern:
/// `tr!("{n} Geräte", n = count)` → „3 Geräte“ / „3 fixtures“.
#[macro_export]
macro_rules! tr {
    ($tpl:literal $(, $k:ident = $v:expr)* $(,)?) => {
        $crate::i18n::fill(
            $crate::i18n::t($tpl),
            &[$((stringify!($k), ($v).to_string())),*],
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Alle Texte aus `t("…")` und `tr!("…")` im Quellcode.
    fn texts_in_source() -> Vec<(String, String)> {
        fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    walk(&p, out);
                } else if p.extension().is_some_and(|x| x == "rs") {
                    out.push(p);
                }
            }
        }
        let mut files = Vec::new();
        walk(
            &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
            &mut files,
        );
        let mut found = Vec::new();
        for f in files {
            if f.components().any(|c| c.as_os_str() == "i18n") {
                continue;
            }
            let src = std::fs::read_to_string(&f).unwrap();
            for marker in ["t(\"", "tr!(\""] {
                let mut rest = src.as_str();
                while let Some(i) = rest.find(marker) {
                    // `t(` nur als eigenes Wort (nicht `set(`, `hint(` …)
                    let before = rest[..i].chars().last();
                    let ok = marker != "t(\""
                        || !before.is_some_and(|c| c.is_alphanumeric() || c == '_');
                    let start = i + marker.len();
                    let end = start + rest[start..].find('"').unwrap();
                    if ok {
                        found.push((
                            rest[start..end].replace("\\n", "\n"),
                            f.display().to_string(),
                        ));
                    }
                    rest = &rest[end..];
                }
            }
        }
        found
    }

    #[test]
    fn all_texts_translated() {
        let missing: Vec<_> = texts_in_source()
            .into_iter()
            .filter(|(de, _)| !EN.contains_key(de.as_str()))
            .collect();
        assert!(missing.is_empty(), "ohne Übersetzung: {missing:#?}");
    }

    /// Texte, die aus anderen Crates oder Konstanten kommen und erst an der
    /// Verwendungsstelle über `t()` laufen.
    #[test]
    fn labels_from_core_translated() {
        let mut labels: Vec<&str> = dmxcraft_core::PlanShape::ALL
            .iter()
            .map(|s| s.label())
            .chain(dmxcraft_core::ChannelKind::ALL.iter().map(|k| k.label()))
            .chain(crate::ui::theme::Theme::ALL.iter().map(|t| t.label()))
            .chain(dmxcraft_output::OutputKind::ALL.iter().map(|k| k.label()))
            .chain(dmxcraft_core::fx::Transform::ALL.iter().map(|k| k.label()))
            .collect();
        labels.extend(crate::views::fixture_menu::SIZES.iter().map(|(l, _)| *l));
        for l in labels {
            assert!(EN.contains_key(l), "ohne Übersetzung: {l}");
        }
    }

    #[test]
    fn no_duplicate_entries() {
        let mut seen = HashSet::new();
        for (de, _) in en::ENTRIES {
            assert!(seen.insert(de), "doppelt: {de}");
        }
    }

    #[test]
    fn placeholders_match() {
        let names = |s: &str| {
            let mut v: Vec<String> = s
                .split('{')
                .skip(1)
                .filter_map(|p| p.split_once('}').map(|(n, _)| n.to_string()))
                .collect();
            v.sort();
            v
        };
        for (de, en) in en::ENTRIES {
            assert_eq!(names(de), names(en), "Platzhalter: {de} / {en}");
        }
    }

    #[test]
    fn fill_replaces_names() {
        assert_eq!(
            fill("{n} von {m}", &[("n", "1".into()), ("m", "2".into())]),
            "1 von 2"
        );
    }
}
