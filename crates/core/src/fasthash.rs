//! Fast hashing for the small integer keys of the engine (channels,
//! fixtures). The standard hasher protects against crafted keys, which the
//! engine does not need; it builds several channel maps per frame.

use std::hash::{BuildHasherDefault, Hasher};

/// Multiply-rotate hasher (the scheme of rustc's FxHasher).
#[derive(Default, Clone, Copy)]
pub struct FxHasher(u64);

const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

impl FxHasher {
    #[inline]
    fn add(&mut self, v: u64) {
        self.0 = (self.0.rotate_left(5) ^ v).wrapping_mul(SEED);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut b = [0u8; 8];
            b[..chunk.len()].copy_from_slice(chunk);
            self.add(u64::from_le_bytes(b));
        }
    }
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(u64::from(i));
    }
    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add(u64::from(i));
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add(u64::from(i));
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

pub type FastBuild = BuildHasherDefault<FxHasher>;
/// HashMap with the fast hasher (create with `default()`).
pub type FastMap<K, V> = std::collections::HashMap<K, V, FastBuild>;
/// HashSet with the fast hasher (create with `default()`).
pub type FastSet<T> = std::collections::HashSet<T, FastBuild>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_as_map() {
        let mut m: FastMap<(u32, u16), u8> = FastMap::default();
        for i in 0..1000u32 {
            m.insert((i, (i % 7) as u16), (i % 255) as u8);
        }
        assert_eq!(m.len(), 1000);
        assert_eq!(m[&(500, 3)], (500 % 255) as u8);
    }
}
