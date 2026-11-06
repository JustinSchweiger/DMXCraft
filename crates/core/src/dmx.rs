/// Kanäle pro DMX-Universum.
pub const UNIVERSE_SIZE: usize = 512;

/// Ein vollständiger Ausgabe-Frame über alle Universen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DmxFrame {
    universes: Vec<[u8; UNIVERSE_SIZE]>,
}

impl DmxFrame {
    pub fn new(universes: usize) -> Self {
        Self {
            universes: vec![[0; UNIVERSE_SIZE]; universes],
        }
    }

    pub fn universe_count(&self) -> usize {
        self.universes.len()
    }

    pub fn universe(&self, index: usize) -> Option<&[u8; UNIVERSE_SIZE]> {
        self.universes.get(index)
    }

    pub fn universe_mut(&mut self, index: usize) -> Option<&mut [u8; UNIVERSE_SIZE]> {
        self.universes.get_mut(index)
    }

    /// Wert lesen; `address` ist 1-basiert wie in der Lichttechnik üblich.
    pub fn get(&self, universe: usize, address: u16) -> Option<u8> {
        let idx = usize::from(address).checked_sub(1)?;
        self.universes.get(universe)?.get(idx).copied()
    }

    /// Wert setzen; `address` ist 1-basiert. Ungültige Adressen werden ignoriert.
    pub fn set(&mut self, universe: usize, address: u16, value: u8) {
        if let Some(idx) = usize::from(address).checked_sub(1) {
            if let Some(slot) = self
                .universes
                .get_mut(universe)
                .and_then(|u| u.get_mut(idx))
            {
                *slot = value;
            }
        }
    }

    pub fn clear(&mut self) {
        for u in &mut self.universes {
            u.fill(0);
        }
    }

    /// Passt die Anzahl der Universen an (neue werden mit 0 gefüllt).
    pub fn resize(&mut self, universes: usize) {
        self.universes.resize(universes, [0; UNIVERSE_SIZE]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_are_one_based() {
        let mut f = DmxFrame::new(2);
        f.set(1, 1, 10);
        f.set(1, 512, 20);
        f.set(1, 0, 99); // ungültig
        f.set(1, 513, 99); // ungültig
        f.set(5, 1, 99); // ungültiges Universum
        assert_eq!(f.universe(1).unwrap()[0], 10);
        assert_eq!(f.get(1, 512), Some(20));
        assert_eq!(f.get(1, 0), None);
        assert_eq!(f.get(0, 1), Some(0));
    }
}
