//! Rückgängig/Wiederherstellen für die Show.
//!
//! Statt jede Änderung einzeln zu erfassen, vergleicht `track` nach jedem
//! Frame die Show mit dem letzten Stand. Solange die Maus gedrückt ist oder
//! ein Textfeld den Fokus hat, wird nicht übernommen – so wird ein Ziehen
//! oder ein Umbenennen zu genau einem Schritt.

use dmxcraft_core::Show;

const LIMIT: usize = 100;

#[derive(Default)]
pub struct History {
    undo: Vec<Show>,
    redo: Vec<Show>,
    last: Option<Show>,
}

impl History {
    /// Neuer Ausgangsstand (neue Show geladen): Verlauf leeren.
    pub fn reset(&mut self, show: &Show) {
        self.undo.clear();
        self.redo.clear();
        self.last = Some(show.clone());
    }

    /// Nach jedem Frame aufrufen. `busy` = Maus gedrückt oder Texteingabe.
    pub fn track(&mut self, show: &Show, busy: bool) {
        if busy {
            return;
        }
        match &self.last {
            Some(last) if last == show => {}
            Some(_) => {
                let prev = self.last.replace(show.clone()).expect("checked above");
                self.undo.push(prev);
                if self.undo.len() > LIMIT {
                    self.undo.remove(0);
                }
                self.redo.clear();
            }
            None => self.last = Some(show.clone()),
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo.is_empty()
    }

    /// Liefert den vorherigen Stand; `current` ist der aktuelle.
    pub fn undo(&mut self, current: &Show) -> Option<Show> {
        let prev = self.undo.pop()?;
        self.redo.push(current.clone());
        self.last = Some(prev.clone());
        Some(prev)
    }

    pub fn redo(&mut self, current: &Show) -> Option<Show> {
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
        let mut s = Show::new("A", 8);
        h.reset(&s);
        s.name = "B".into();
        h.track(&s, true); // beschäftigt: nichts übernehmen
        assert!(!h.can_undo());
        h.track(&s, false);
        assert!(h.can_undo());
        let prev = h.undo(&s).unwrap();
        assert_eq!(prev.name, "A");
        let next = h.redo(&prev).unwrap();
        assert_eq!(next.name, "B");
        // Unveränderte Show erzeugt keinen Schritt
        h.track(&next, false);
        assert!(!h.can_redo());
        assert_eq!(h.undo.len(), 1);
    }
}
