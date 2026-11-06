//! DIP-Schalter-Stellung für eine DMX-Startadresse.
//!
//! Übliche Belegung an Geräten: Schalter 1–9 binär (1, 2, 4, … 256),
//! Schalter 10 ist meist eine Sonderfunktion und bleibt aus.

/// Anzahl der dargestellten Schalter.
pub const SWITCHES: usize = 10;

/// `true` = Schalter AN. Index 0 ist Schalter 1 (Wert 1).
pub fn switches(address: u16) -> [bool; SWITCHES] {
    let mut s = [false; SWITCHES];
    for (i, on) in s.iter_mut().enumerate().take(9) {
        *on = address & (1 << i) != 0;
    }
    s
}

/// Nummern der eingeschalteten Schalter, z. B. Adresse 101 → [1, 3, 6, 7].
pub fn on_switches(address: u16) -> Vec<usize> {
    switches(address)
        .iter()
        .enumerate()
        .filter(|(_, on)| **on)
        .map(|(i, _)| i + 1)
        .collect()
}

/// Wert eines Schalters (Schalter 1 = 1, Schalter 9 = 256).
pub fn switch_value(switch: usize) -> u16 {
    1 << (switch.clamp(1, 9) - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_addresses() {
        assert_eq!(on_switches(1), vec![1]);
        assert_eq!(on_switches(101), vec![1, 3, 6, 7]); // 1+4+32+64
        assert_eq!(on_switches(512), Vec::<usize>::new()); // 512 passt nicht in 9 Bit
        assert_eq!(on_switches(511), (1..=9).collect::<Vec<_>>());
        assert!(!switches(511)[9]);
        assert_eq!(switch_value(9), 256);
    }

    #[test]
    fn roundtrip() {
        for a in 1..=511u16 {
            let sum: u16 = on_switches(a).iter().map(|s| switch_value(*s)).sum();
            assert_eq!(sum, a);
        }
    }
}
