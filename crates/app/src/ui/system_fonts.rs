//! Fonts of the operating system for characters the built-in fonts lack
//! (other scripts, emoji in names …).
//!
//! The system font fallback asks the sans-serif fonts first, and those often
//! have single-color emoji (e.g. Segoe UI Symbol, Unifont). Emoji should come
//! in color, so for characters with emoji presentation the emoji fonts of the
//! system (Segoe UI Emoji, Apple Color Emoji, Noto Color Emoji) are asked
//! first; everything else goes to the regular system fallback.

use std::sync::{Arc, Mutex, OnceLock};

use epaint::text::{
    has_emoji_presentation, FallbackRequest, FontData, FontInsert, FontPriority, FontProvider,
    InsertFontFamily,
};
use fontique::{
    Collection, CollectionOptions, GenericFamily, QueryFamily, QueryStatus, SourceCache,
};

struct Emoji {
    collection: Collection,
    cache: SourceCache,
}

pub struct SystemFonts {
    system: egui_system_font_provider::SystemFontProvider,
    /// Enumerated on first use (an emoji in a text).
    emoji: OnceLock<Mutex<Emoji>>,
}

impl SystemFonts {
    pub fn new() -> Self {
        Self {
            system: egui_system_font_provider::SystemFontProvider::new(),
            emoji: OnceLock::new(),
        }
    }

    fn emoji_font(&self, request: &FallbackRequest<'_>) -> Option<FontInsert> {
        let c = request.cluster.chars().next()?;
        let emoji = self.emoji.get_or_init(|| {
            Mutex::new(Emoji {
                collection: Collection::new(CollectionOptions {
                    shared: false,
                    system_fonts: true,
                }),
                cache: SourceCache::default(),
            })
        });
        let mut guard = emoji.lock().unwrap_or_else(|p| p.into_inner());
        let Emoji { collection, cache } = &mut *guard;
        let mut found = None;
        let mut query = collection.query(cache);
        query.set_families([QueryFamily::Generic(GenericFamily::Emoji)]);
        query.matches_with(|font| {
            let has = font
                .charmap()
                .and_then(|m| m.map(c))
                .is_some_and(|g| g != 0);
            if has {
                found = Some(font.clone());
                QueryStatus::Stop
            } else {
                QueryStatus::Continue
            }
        });
        drop(query);
        let font = found?;
        let family = collection.family_name(font.family.0).unwrap_or("emoji");
        let name = format!("system-emoji:{family}:{}:{}", font.family.1, font.index);
        let (blob, _) = font.blob.into_raw_parts();
        Some(FontInsert {
            name,
            data: FontData::from_blob(blob, font.index),
            families: vec![InsertFontFamily {
                family: request.family.clone(),
                priority: FontPriority::Lowest,
            }],
        })
    }
}

impl FontProvider for SystemFonts {
    fn font_for(&self, request: &FallbackRequest<'_>) -> Option<FontInsert> {
        if has_emoji_presentation(request.cluster) {
            if let Some(font) = self.emoji_font(request) {
                return Some(font);
            }
        }
        self.system.font_for(request)
    }
}

/// Install the provider (once per context).
pub fn install(ctx: &eframe::egui::Context) {
    ctx.add_font_provider(Arc::new(SystemFonts::new()));
}
