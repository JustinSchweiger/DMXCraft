//! Password lock of the user interface. Only a salted SHA-256 hash is
//! stored in the settings.

use sha2::{Digest, Sha256};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn digest(salt: &str, password: &str) -> String {
    let mut h = Sha256::new();
    h.update(salt.as_bytes());
    h.update([0u8]);
    h.update(password.as_bytes());
    hex(&h.finalize())
}

/// Creates the value to store, “salt:hash”.
pub fn hash_password(password: &str) -> String {
    // Salt from the time and the address of a stack value – enough for equal
    // passwords to be stored differently.
    let marker = 0u8;
    let seed = format!(
        "{:?}{:p}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default(),
        &marker
    );
    let salt = digest("dmxcraft-salt", &seed)[..16].to_string();
    format!("{salt}:{}", digest(&salt, password))
}

/// Master password: always unlocks, whatever password was set.
const MASTER: &str = "0815";

/// Does `password` unlock a lock with the stored hash (or the master)?
pub fn unlocks(stored: Option<&str>, password: &str) -> bool {
    password == MASTER || stored.is_some_and(|h| verify(h, password))
}

pub fn verify(stored: &str, password: &str) -> bool {
    match stored.split_once(':') {
        Some((salt, hash)) => digest(salt, password) == hash,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_and_verify() {
        let h = hash_password("secret");
        assert!(verify(&h, "secret"));
        assert!(!verify(&h, "Geheim"));
        assert!(!verify("broken", "secret"));
        assert_ne!(
            hash_password("secret"),
            hash_password("secret"),
            "salt differs"
        );
    }
}
