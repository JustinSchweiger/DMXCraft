//! Rückgängig/Wiederherstellen für die Show und die Live-Werte.
//!
//! Statt jede Änderung einzeln zu erfassen, vergleicht `track` nach jedem
//! Frame die Show mit dem letzten Stand. Solange die Maus gedrückt ist oder
//! ein Textfeld den Fokus hat, wird nicht übernommen – so wird ein Ziehen
//! oder ein Umbenennen zu genau einem Schritt.

use dmxcraft_core::{ChannelRef, Engine, Show};

/// Stand, der rückgängig gemacht werden kann.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub show: Show,
    /// Live-Werte, sortiert (gehören nicht zur gespeicherten Show).
    pub live: Vec<(ChannelRef, u8)>,
}

impl Snapshot {
    pub fn of(e: &Engine) -> Self {
        Self {
            show: e.show().clone(),
            live: Self::live_of(e),
        }
    }

    fn live_of(e: &Engine) -> Vec<(ChannelRef, u8)> {
        let mut live: Vec<(ChannelRef, u8)> = e.live().iter().map(|(c, v)| (*c, *v)).collect();
        live.sort();
        live
    }

    /// Entspricht der Stand der Engine (ohne die Show zu kopieren)?
    fn matches(&self, e: &Engine) -> bool {
        self.show == *e.show() && self.live == Self::live_of(e)
    }

    /// Stand in die Engine übernehmen.
    pub fn restore(self, e: &mut Engine) {
        *e.show_mut() = self.show;
        e.replace_live(self.live.into_iter().collect());
        e.sync();
    }
}

const LIMIT: usize = 100;

#[derive(Default)]
pub struct History {
    undo: Vec<Snapshot>,
    redo: Vec<Snapshot>,
    last: Option<Snapshot>,
}

impl History {
    /// Neuer Ausgangsstand (neue Show geladen): Verlauf leeren.
    pub fn reset(&mut self, show: &Snapshot) {
        self.undo.clear();
        self.redo.clear();
        self.last = Some(show.clone());
    }

    /// Nach jedem Frame aufrufen. `busy` = Maus gedrückt oder Texteingabe.
    pub fn track(&mut self, e: &Engine, busy: bool) {
        if busy {
            return;
        }
        match &self.last {
            Some(last) if last.matches(e) => {}
            Some(_) => {
                let prev = self.last.replace(Snapshot::of(e)).expect("checked above");
                self.undo.push(prev);
                if self.undo.len() > LIMIT {
                    self.undo.remove(0);
                }
                self.redo.clear();
            }
            None => self.last = Some(Snapshot::of(e)),
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Liefert den vorherigen Stand; `current` ist der aktuelle.
    pub fn undo(&mut self, current: &Snapshot) -> Option<Snapshot> {
        let prev = self.undo.pop()?;
        self.redo.push(current.clone());
        self.last = Some(prev.clone());
        Some(prev)
    }

    pub fn redo(&mut self, current: &Snapshot) -> Option<Snapshot> {
        let next = self.redo.pop()?;
        self.undo.push(current.clone());
        self.last = Some(next.clone());
        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn undo_redo_roundtrip() {
        let mut h = History::default();
        let mut e = Engine::new(Show::new("A", 8));
        h.reset(&Snapshot::of(&e));
        e.show_mut().name = "B".into();
        h.track(&e, true); // beschäftigt: nichts übernehmen
        assert!(!h.can_undo());
        h.track(&e, false);
        assert!(h.can_undo());
        let prev = h.undo(&Snapshot::of(&e)).unwrap();
        assert_eq!(prev.show.name, "A");
        prev.restore(&mut e);
        let next = h.redo(&Snapshot::of(&e)).unwrap();
        assert_eq!(next.show.name, "B");
        next.restore(&mut e);
        // Unveränderter Stand erzeugt keinen Schritt
        h.track(&e, false);
        assert!(!h.can_redo());
        assert_eq!(h.undo.len(), 1);
    }

    #[test]
    fn live_values_are_undoable() {
        let mut h = History::default();
        let mut e = Engine::new(Show::new("A", 8));
        h.reset(&Snapshot::of(&e));
        let ch = ChannelRef {
            fixture: dmxcraft_core::FixtureId(1),
            channel: 0,
        };
        e.set_live(ch, 200);
        h.track(&e, false);
        h.undo(&Snapshot::of(&e)).unwrap().restore(&mut e);
        assert!(e.live().is_empty());
    }
}
