//! Undo/redo for the show and the live values.
//!
//! Instead of recording every change individually, `track` compares the
//! show with the last state after every frame. While the mouse is pressed
//! or a text field has focus nothing is taken over – so a drag or a rename
//! becomes exactly one step.

use dmxcraft_core::{ChannelRef, Engine, Show};

/// State that can be undone.
#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub show: Show,
    /// Live values, sorted (not part of the saved show).
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

    /// Does the state match the engine (without copying the show)?
    fn matches(&self, e: &Engine) -> bool {
        self.show == *e.show() && self.live == Self::live_of(e)
    }

    /// Apply the state to the engine.
    pub fn restore(self, e: &mut Engine) {
        // following of LIVE TRACKING is a live state: undo keeps it
        let following: Vec<u32> = e
            .show()
            .trackers
            .iter()
            .filter(|t| t.active)
            .map(|t| t.id)
            .collect();
        *e.show_mut() = self.show;
        for t in &mut e.show_mut().trackers {
            t.active = following.contains(&t.id);
        }
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
    /// New initial state (new show loaded): clear the history.
    pub fn reset(&mut self, show: &Snapshot) {
        self.undo.clear();
        self.redo.clear();
        self.last = Some(show.clone());
    }

    /// Call after every frame. `busy` = mouse pressed or text input.
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

    /// Returns the previous state; `current` is the current one.
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
    fn following_is_no_undo_step() {
        let mut h = History::default();
        let mut e = Engine::new(Show::new("A", 8));
        let tr = e.show_mut().add_tracker("T", Vec::new());
        h.reset(&Snapshot::of(&e));
        e.show_mut().tracker_mut(tr).unwrap().active = true;
        h.track(&e, false);
        assert!(!h.can_undo(), "switching following is not a change");
        e.show_mut().name = "B".into();
        h.track(&e, false);
        let prev = h.undo(&Snapshot::of(&e)).unwrap();
        prev.restore(&mut e);
        assert_eq!(e.show().name, "A");
        assert!(e.show().tracker(tr).unwrap().active, "undo keeps following");
    }

    #[test]
    fn undo_redo_roundtrip() {
        let mut h = History::default();
        let mut e = Engine::new(Show::new("A", 8));
        h.reset(&Snapshot::of(&e));
        e.show_mut().name = "B".into();
        h.track(&e, true); // busy: take nothing over
        assert!(!h.can_undo());
        h.track(&e, false);
        assert!(h.can_undo());
        let prev = h.undo(&Snapshot::of(&e)).unwrap();
        assert_eq!(prev.show.name, "A");
        prev.restore(&mut e);
        let next = h.redo(&Snapshot::of(&e)).unwrap();
        assert_eq!(next.show.name, "B");
        next.restore(&mut e);
        // An unchanged state does not create a step
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
