//! DMX over the network: Art-Net (ArtDmx) and sACN (ANSI E1.31).
//!
//! Universe mapping: DMXCraft universe 1 is Art-Net port address 0
//! (net 0, sub-net 0, universe 0), universe 2 is port address 1, …; and
//! sACN universe 1, 2, … (sACN starts at 1).
//!
//! Art-Net goes to the configured destination (broadcast by default, or the
//! address of a node), sACN to the standard multicast group of the universe
//! (`239.255.<hi>.<lo>`). Both are sent at ~40 Hz from their own thread.

use std::hash::{BuildHasher, Hasher};
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs, UdpSocket};
use std::sync::Arc;
use std::time::{Duration, Instant};

use dmxcraft_core::UNIVERSE_SIZE;

use crate::devices::{DmxDevice, Driver, Shared, ARTNET_KEY, SACN_KEY};

/// UDP port of Art-Net.
pub const ARTNET_PORT: u16 = 6454;
/// UDP port of sACN.
pub const SACN_PORT: u16 = 5568;
/// Default Art-Net destination.
pub const ARTNET_BROADCAST: &str = "255.255.255.255";

/// Length of an ArtDmx packet with a full universe.
pub const ARTNET_LEN: usize = 18 + UNIVERSE_SIZE;
/// Length of an E1.31 data packet with a full universe.
pub const SACN_LEN: usize = 126 + UNIVERSE_SIZE;

/// Time per frame (~40 Hz).
const FRAME: Duration = Duration::from_millis(25);
/// Wait time after a socket error.
const RETRY: Duration = Duration::from_secs(1);

/// The network outputs for the device list.
pub fn list() -> Vec<DmxDevice> {
    [
        (ARTNET_KEY, "Art-Net", Driver::ArtNet),
        (SACN_KEY, "sACN / E1.31", Driver::Sacn),
    ]
    .into_iter()
    .map(|(key, label, driver)| DmxDevice {
        key: key.into(),
        port_name: key.into(),
        label: label.into(),
        ftdi: false,
        driver,
        recognized: true,
    })
    .collect()
}

/// ArtDmx packet for the universe with index `universe` (0-based = port
/// address).
pub fn artnet_packet(
    universe: usize,
    sequence: u8,
    channels: &[u8; UNIVERSE_SIZE],
) -> [u8; ARTNET_LEN] {
    let port_address = universe as u16 & 0x7FFF;
    let mut p = [0u8; ARTNET_LEN];
    p[..8].copy_from_slice(b"Art-Net\0");
    p[8..10].copy_from_slice(&0x5000u16.to_le_bytes()); // OpDmx
    p[10..12].copy_from_slice(&14u16.to_be_bytes()); // protocol version
    p[12] = sequence;
    p[13] = 0; // physical port
    p[14] = (port_address & 0xFF) as u8; // SubUni
    p[15] = (port_address >> 8) as u8; // Net
    p[16..18].copy_from_slice(&(UNIVERSE_SIZE as u16).to_be_bytes());
    p[18..].copy_from_slice(channels);
    p
}

/// Flags (0x7) and length of a PDU that runs from `offset` to the end of
/// the packet.
fn flags_length(offset: usize) -> [u8; 2] {
    (0x7000 | (SACN_LEN - offset) as u16).to_be_bytes()
}

/// E1.31 data packet for the universe with index `universe` (0-based; sACN
/// universe = index + 1).
pub fn sacn_packet(
    universe: usize,
    sequence: u8,
    cid: &[u8; 16],
    channels: &[u8; UNIVERSE_SIZE],
) -> [u8; SACN_LEN] {
    let mut p = [0u8; SACN_LEN];
    // Root layer
    p[0..2].copy_from_slice(&0x0010u16.to_be_bytes()); // preamble size
    p[4..16].copy_from_slice(b"ASC-E1.17\0\0\0");
    p[16..18].copy_from_slice(&flags_length(16));
    p[18..22].copy_from_slice(&4u32.to_be_bytes()); // VECTOR_ROOT_E131_DATA
    p[22..38].copy_from_slice(cid);
    // Framing layer
    p[38..40].copy_from_slice(&flags_length(38));
    p[40..44].copy_from_slice(&2u32.to_be_bytes()); // VECTOR_E131_DATA_PACKET
    let name = b"DMXCraft";
    p[44..44 + name.len()].copy_from_slice(name); // source name, 64 bytes
    p[108] = 100; // priority
    p[111] = sequence;
    p[112] = 0; // options
    p[113..115].copy_from_slice(&(sacn_universe(universe)).to_be_bytes());
    // DMP layer
    p[115..117].copy_from_slice(&flags_length(115));
    p[117] = 0x02; // VECTOR_DMP_SET_PROPERTY
    p[118] = 0xA1; // address type & data type
    p[121..123].copy_from_slice(&1u16.to_be_bytes()); // address increment
    p[123..125].copy_from_slice(&((UNIVERSE_SIZE + 1) as u16).to_be_bytes());
    p[125] = 0; // start code
    p[126..].copy_from_slice(channels);
    p
}

/// sACN universe number of a universe index.
pub fn sacn_universe(universe: usize) -> u16 {
    (universe + 1).min(63_999) as u16
}

/// Multicast group of an sACN universe.
pub fn sacn_group(universe: usize) -> Ipv4Addr {
    let [hi, lo] = sacn_universe(universe).to_be_bytes();
    Ipv4Addr::new(239, 255, hi, lo)
}

/// Random component ID of this program run (identifies the sender for
/// receivers).
fn new_cid() -> [u8; 16] {
    let state = std::collections::hash_map::RandomState::new();
    let mut cid = [0u8; 16];
    for (i, part) in cid.chunks_mut(8).enumerate() {
        let mut h = state.build_hasher();
        h.write_usize(i);
        h.write_u128(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or_default(),
        );
        part.copy_from_slice(&h.finish().to_be_bytes());
    }
    cid[6] = (cid[6] & 0x0F) | 0x40; // UUID version 4
    cid[8] = (cid[8] & 0x3F) | 0x80;
    cid
}

/// Destination of Art-Net packets: `host` or `host:port`.
pub fn artnet_destination(target: &str) -> std::io::Result<SocketAddr> {
    let target = target.trim();
    let target = if target.is_empty() {
        ARTNET_BROADCAST
    } else {
        target
    };
    let with_port = if target.contains(':') {
        target.to_string()
    } else {
        format!("{target}:{ARTNET_PORT}")
    };
    with_port
        .to_socket_addrs()?
        .find(SocketAddr::is_ipv4)
        .ok_or_else(|| std::io::Error::other("no IPv4 address"))
}

/// Sends packets built by `packet` to `dest` at the frame rate until the
/// output stops.
fn send_loop(
    universe: usize,
    what: &str,
    shared: &Shared,
    dest: impl Fn() -> std::io::Result<SocketAddr>,
    mut packet: impl FnMut(u8, &[u8; UNIVERSE_SIZE]) -> Vec<u8>,
) {
    let mut sequence = 0u8;
    while !shared.stopped() {
        let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).and_then(|s| {
            s.set_broadcast(true)?;
            Ok(s)
        });
        let (socket, dest) = match socket.and_then(|s| Ok((s, dest()?))) {
            Ok(x) => x,
            Err(e) => {
                shared.set_error(format!("Universe {}: {what}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                continue;
            }
        };
        while !shared.stopped() {
            let start = Instant::now();
            // 0 means "no sequence" in both protocols
            sequence = sequence.wrapping_add(1).max(1);
            match socket.send_to(&packet(sequence, &shared.channels()), dest) {
                Ok(_) => shared.set_connected(),
                Err(e) => {
                    // e.g. no network yet: keep trying with the same socket
                    shared.set_error(format!("Universe {}: {what} {dest}: {e}", universe + 1));
                    std::thread::sleep(RETRY);
                    continue;
                }
            }
            let elapsed = start.elapsed();
            if elapsed < FRAME {
                std::thread::sleep(FRAME - elapsed);
            }
        }
    }
}

/// Send thread of Art-Net for one universe.
pub(crate) fn run_artnet(target: String, universe: usize, shared: Arc<Shared>) {
    send_loop(
        universe,
        "Art-Net",
        &shared,
        || artnet_destination(&target),
        |seq, ch| artnet_packet(universe, seq, ch).to_vec(),
    );
}

/// Send thread of sACN for one universe.
pub(crate) fn run_sacn(universe: usize, shared: Arc<Shared>) {
    let cid = new_cid();
    send_loop(
        universe,
        "sACN",
        &shared,
        || Ok(SocketAddr::from((sacn_group(universe), SACN_PORT))),
        |seq, ch| sacn_packet(universe, seq, &cid, ch).to_vec(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn channels() -> [u8; UNIVERSE_SIZE] {
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[0] = 255;
        ch[511] = 7;
        ch
    }

    #[test]
    fn artnet_packet_layout() {
        let p = artnet_packet(0x123, 9, &channels());
        assert_eq!(p.len(), 530);
        assert_eq!(&p[..8], b"Art-Net\0");
        assert_eq!(p[8..10], [0x00, 0x50], "OpDmx, little endian");
        assert_eq!(p[10..12], [0, 14], "protocol version 14");
        assert_eq!(p[12], 9, "sequence");
        assert_eq!(p[14..16], [0x23, 0x01], "SubUni, Net");
        assert_eq!(p[16..18], [0x02, 0x00], "length 512, big endian");
        assert_eq!((p[18], p[529]), (255, 7));
    }

    #[test]
    fn sacn_packet_layout() {
        let cid = [0xAB; 16];
        let p = sacn_packet(0, 3, &cid, &channels());
        assert_eq!(p.len(), 638);
        assert_eq!(p[0..4], [0x00, 0x10, 0x00, 0x00]);
        assert_eq!(&p[4..16], b"ASC-E1.17\0\0\0");
        assert_eq!(p[16..18], [0x72, 0x6E], "root flags + length 622");
        assert_eq!(p[18..22], [0, 0, 0, 4]);
        assert_eq!(p[22..38], cid);
        assert_eq!(p[38..40], [0x72, 0x58], "framing flags + length 600");
        assert_eq!(p[40..44], [0, 0, 0, 2]);
        assert_eq!(&p[44..52], b"DMXCraft");
        assert_eq!((p[108], p[111]), (100, 3), "priority, sequence");
        assert_eq!(p[113..115], [0, 1], "universe 1");
        assert_eq!(p[115..117], [0x72, 0x0B], "DMP flags + length 523");
        assert_eq!(p[117..125], [0x02, 0xA1, 0, 0, 0, 1, 0x02, 0x01]);
        assert_eq!(p[125], 0, "start code");
        assert_eq!((p[126], p[637]), (255, 7));
    }

    #[test]
    fn sacn_multicast_group() {
        assert_eq!(sacn_group(0), Ipv4Addr::new(239, 255, 0, 1));
        assert_eq!(sacn_group(299), Ipv4Addr::new(239, 255, 1, 44));
    }

    #[test]
    fn artnet_destination_parsing() {
        assert_eq!(
            artnet_destination("").unwrap(),
            "255.255.255.255:6454".parse().unwrap()
        );
        assert_eq!(
            artnet_destination(" 2.0.0.10 ").unwrap(),
            "2.0.0.10:6454".parse().unwrap()
        );
        assert_eq!(
            artnet_destination("127.0.0.1:7000").unwrap(),
            "127.0.0.1:7000".parse().unwrap()
        );
    }

    #[test]
    fn cid_is_a_v4_uuid_and_differs() {
        let (a, b) = (new_cid(), new_cid());
        assert_ne!(a, b);
        assert_eq!(a[6] >> 4, 4);
    }

    /// End to end over the loopback interface.
    #[test]
    fn artnet_arrives_at_the_destination() {
        let rx = UdpSocket::bind("127.0.0.1:0").unwrap();
        rx.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        let target = rx.local_addr().unwrap().to_string();
        let shared = Arc::new(Shared::new());
        *shared.data.lock().unwrap() = channels();
        let s = shared.clone();
        let t = std::thread::spawn(move || run_artnet(target, 1, s));
        let mut buf = [0u8; 1024];
        let n = rx.recv(&mut buf).unwrap();
        shared
            .stop
            .store(true, std::sync::atomic::Ordering::Relaxed);
        t.join().unwrap();
        assert_eq!(n, ARTNET_LEN);
        assert_eq!(buf[14], 1, "universe 2 = port address 1");
        assert_eq!((buf[18], buf[529]), (255, 7));
    }
}
