//! UI translation.
//!
//! English is the source language: texts are written in English in the code
//! and go through [`t`] (fixed texts) or [`tr!`] (texts with placeholders
//! `{name}`). For German, [`t`] looks the English text up in
//! [`de::ENTRIES`]. If an entry is missing the English text is shown – a test
//! (`all_texts_translated`) makes sure that does not happen.

mod de;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::LazyLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    De,
    En,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::En, Lang::De];

    /// Key used in the settings.
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
            .unwrap_or(Lang::En)
    }

    /// Name of the language in itself (not translated).
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

static DE: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| de::ENTRIES.iter().copied().collect());

/// Translates a fixed English text into the active language.
pub fn t(en: &'static str) -> &'static str {
    match lang() {
        Lang::En => en,
        Lang::De => DE.get(en).copied().unwrap_or(en),
    }
}

/// Fills values into a (translated) text with placeholders `{name}`.
pub fn fill(template: &str, args: &[(&str, String)]) -> String {
    let mut out = template.to_string();
    for (k, v) in args {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

/// Translated text with placeholders:
/// `tr!("{n} fixtures", n = count)` → “3 fixtures” / „3 Geräte“.
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

    /// All texts from `t("…")` and `tr!("…")` in the source code.
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
            for marker in ["t(", "tr!("] {
                let mut rest = src.as_str();
                while let Some(i) = rest.find(marker) {
                    // only as a separate word (not `set(`, `hint(`, `include_str!(` …),
                    // the literal may follow on the next line (rustfmt)
                    let before = rest[..i].chars().last();
                    let ok = !before.is_some_and(|c| c.is_alphanumeric() || c == '_');
                    let after = rest[i + marker.len()..].trim_start();
                    if ok && after.starts_with('"') {
                        let start = rest.len() - after.len() + 1;
                        let end = start + rest[start..].find('"').unwrap();
                        found.push((
                            rest[start..end].replace("\\n", "\n"),
                            f.display().to_string(),
                        ));
                        rest = &rest[end..];
                    } else {
                        rest = &rest[i + marker.len()..];
                    }
                }
            }
        }
        found
    }

    #[test]
    fn all_texts_translated() {
        let missing: Vec<_> = texts_in_source()
            .into_iter()
            .filter(|(en, _)| !DE.contains_key(en.as_str()))
            .collect();
        assert!(missing.is_empty(), "not translated: {missing:#?}");
    }

    /// Texts that come from other crates or constants and only go through
    /// `t()` where they are used.
    #[test]
    fn labels_from_core_translated() {
        let mut labels: Vec<&str> = dmxcraft_core::PlanShape::ALL
            .iter()
            .map(|s| s.label())
            .chain(dmxcraft_core::ChannelKind::ALL.iter().map(|k| k.label()))
            .chain(crate::ui::theme::Theme::ALL.iter().map(|t| t.label()))
            .chain(dmxcraft_output::OutputKind::ALL.iter().map(|k| k.label()))
            .chain(dmxcraft_core::fx::Transform::ALL.iter().map(|k| k.label()))
            .chain(
                dmxcraft_core::fx::feature::CurveShape::ALL
                    .iter()
                    .map(|k| k.label()),
            )
            .chain(
                dmxcraft_core::fx::motion::MoveShape::ALL
                    .iter()
                    .map(|k| k.label()),
            )
            .collect();
        labels.extend(crate::views::fixture_menu::SIZES.iter().map(|(l, _)| *l));
        for l in labels {
            assert!(DE.contains_key(l), "not translated: {l}");
        }
    }

    #[test]
    fn no_duplicate_entries() {
        let mut seen = HashSet::new();
        for (en, _) in de::ENTRIES {
            assert!(seen.insert(en), "duplicate: {en}");
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
        for (en, de) in de::ENTRIES {
            assert_eq!(names(en), names(de), "placeholders: {en} / {de}");
        }
    }

    #[test]
    fn fill_replaces_names() {
        assert_eq!(
            fill("{n} of {m}", &[("n", "1".into()), ("m", "2".into())]),
            "1 of 2"
        );
    }
}
