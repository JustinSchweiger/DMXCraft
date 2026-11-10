//! DIP switch setting for a DMX start address.
//!
//! Usual layout on fixtures: switches 1–9 binary (1, 2, 4, … 256), switch 10
//! is usually a special function and stays off.

/// Number of switches shown.
pub const SWITCHES: usize = 10;

/// `true` = switch ON. Index 0 is switch 1 (value 1).
pub fn switches(address: u16) -> [bool; SWITCHES] {
    let mut s = [false; SWITCHES];
    for (i, on) in s.iter_mut().enumerate().take(9) {
        *on = address & (1 << i) != 0;
    }
    s
}

/// Numbers of the switches that are on, e.g. address 101 → [1, 3, 6, 7].
pub fn on_switches(address: u16) -> Vec<usize> {
    switches(address)
        .iter()
        .enumerate()
        .filter(|(_, on)| **on)
        .map(|(i, _)| i + 1)
        .collect()
}

/// Value of a switch (switch 1 = 1, switch 9 = 256).
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
        assert_eq!(on_switches(512), Vec::<usize>::new()); // 512 does not fit into 9 bits
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
