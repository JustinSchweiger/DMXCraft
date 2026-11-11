//! OBS connection (obs-websocket 5): connection data of the show and
//! actions that blocks of a SUPER SCENE trigger in OBS.

use serde::{Deserialize, Serialize};

/// Connection to OBS (saved with the show).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ObsSettings {
    /// Connect while the show is open.
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    /// Password of OBS's WebSocket server (empty = no authentication).
    pub password: String,
    /// Media source in which video blocks are played (created if needed in the
    /// block's OBS scene or the current scene).
    pub video_source: String,
    /// How video blocks reach OBS.
    pub video_output: VideoOutput,
    /// Browser source that shows DMXCraft's video output.
    pub browser_source: String,
    /// Shows the picture this much later (ms, negative = earlier), to match
    /// the sound.
    pub video_delay_ms: i32,
}

/// How video blocks reach OBS.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoOutput {
    /// DMXCraft serves a page that OBS shows in a browser source; the page
    /// follows DMXCraft's clock (picture in sync with sound and light).
    #[default]
    Browser,
    /// OBS plays the file in a media source and DMXCraft corrects drift.
    Media,
}

impl Default for ObsSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            host: "localhost".into(),
            port: 4455,
            password: String::new(),
            video_source: "DMXCraft Video".into(),
            video_output: VideoOutput::Browser,
            browser_source: "DMXCraft Video Output".into(),
            video_delay_ms: 0,
        }
    }
}

impl ObsSettings {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

/// Action of an OBS block, triggered when the block is reached.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObsAction {
    /// Switch to the OBS scene.
    Scene(String),
    /// Show or hide a source in a scene.
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

/// Length of an MP4/MOV file from the `mvhd` header (without decoding the
/// content). `None` if the header is not found.
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

/// Find the box `kind` between `start` and `end`: start and end of its
/// content.
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
        // mvhd v0: version/flags, created, modified, timescale 1000, duration 12 345
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
