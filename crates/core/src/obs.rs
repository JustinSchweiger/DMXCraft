//! OBS-Anbindung (obs-websocket 5): Verbindungsdaten der Show und Aktionen,
//! die Blöcke einer SUPER SCENE in OBS auslösen.

use serde::{Deserialize, Serialize};

/// Verbindung zu OBS (wird mit der Show gespeichert).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ObsSettings {
    /// Verbinden, solange die Show offen ist.
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    /// Passwort des WebSocket-Servers von OBS (leer = ohne Anmeldung).
    pub password: String,
    /// Medienquelle, in der Videoblöcke abgespielt werden (wird bei Bedarf
    /// in der OBS-Szene des Blocks bzw. der aktuellen Szene angelegt).
    pub video_source: String,
}

impl Default for ObsSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            host: "localhost".into(),
            port: 4455,
            password: String::new(),
            video_source: "DMXCraft Video".into(),
        }
    }
}

impl ObsSettings {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

/// Aktion eines OBS-Blocks, ausgelöst beim Erreichen des Blocks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObsAction {
    /// Zur OBS-Szene wechseln.
    Scene(String),
    /// Quelle in einer Szene ein- bzw. ausblenden.
    Source {
        scene: String,
        source: String,
        visible: bool,
    },
}

impl Default for ObsAction {
    fn default() -> Self {
        Self::Scene(String::new())
    }
}

/// Länge einer MP4/MOV-Datei aus dem `mvhd`-Kopf (ohne den Inhalt zu
/// dekodieren). `None`, wenn der Kopf nicht gefunden wird.
pub fn mp4_duration_ms<R: std::io::Read + std::io::Seek>(r: &mut R) -> Option<u64> {
    use std::io::SeekFrom;
    let end = r.seek(SeekFrom::End(0)).ok()?;
    find_box(r, 0, end, b"moov").and_then(|(s, e)| {
        let (ms, _) = find_box(r, s, e, b"mvhd")?;
        r.seek(SeekFrom::Start(ms)).ok()?;
        let mut head = [0u8; 4];
        r.read_exact(&mut head).ok()?;
        let (scale, dur) = if head[0] == 1 {
            let mut b = [0u8; 28];
            r.read_exact(&mut b).ok()?;
            (
                u32::from_be_bytes(b[16..20].try_into().ok()?) as u64,
                u64::from_be_bytes(b[20..28].try_into().ok()?),
            )
        } else {
            let mut b = [0u8; 16];
            r.read_exact(&mut b).ok()?;
            (
                u32::from_be_bytes(b[8..12].try_into().ok()?) as u64,
                u32::from_be_bytes(b[12..16].try_into().ok()?) as u64,
            )
        };
        (scale > 0).then(|| dur.saturating_mul(1000) / scale)
    })
}

/// Box `kind` zwischen `start` und `end` suchen: Beginn und Ende ihres Inhalts.
fn find_box<R: std::io::Read + std::io::Seek>(
    r: &mut R,
    start: u64,
    end: u64,
    kind: &[u8; 4],
) -> Option<(u64, u64)> {
    use std::io::SeekFrom;
    let mut pos = start;
    while pos + 8 <= end {
        r.seek(SeekFrom::Start(pos)).ok()?;
        let mut h = [0u8; 8];
        r.read_exact(&mut h).ok()?;
        let mut size = u32::from_be_bytes(h[0..4].try_into().ok()?) as u64;
        let mut head = 8;
        if size == 1 {
            let mut l = [0u8; 8];
            r.read_exact(&mut l).ok()?;
            size = u64::from_be_bytes(l);
            head = 16;
        } else if size == 0 {
            size = end - pos;
        }
        if size < head {
            return None;
        }
        if &h[4..8] == kind {
            return Some((pos + head, (pos + size).min(end)));
        }
        pos += size;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bx(kind: &[u8; 4], body: &[u8]) -> Vec<u8> {
        let mut v = ((body.len() + 8) as u32).to_be_bytes().to_vec();
        v.extend_from_slice(kind);
        v.extend_from_slice(body);
        v
    }

    #[test]
    fn reads_mp4_length() {
        // mvhd v0: Version/Flags, Erstellt, Geändert, Zeitbasis 1000, Dauer 12 345
        let mut mvhd = vec![0u8; 12];
        mvhd.extend_from_slice(&1000u32.to_be_bytes());
        mvhd.extend_from_slice(&12_345u32.to_be_bytes());
        mvhd.extend_from_slice(&[0u8; 80]);
        let mut file = bx(b"ftyp", b"isom0000");
        file.extend(bx(b"mdat", &[0u8; 100]));
        file.extend(bx(b"moov", &bx(b"mvhd", &mvhd)));
        let mut c = std::io::Cursor::new(file);
        assert_eq!(mp4_duration_ms(&mut c), Some(12_345));
        let mut junk = std::io::Cursor::new(vec![0u8; 5]);
        assert_eq!(mp4_duration_ms(&mut junk), None);
    }
}
