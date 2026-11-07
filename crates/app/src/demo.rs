//! Demo-Show, damit man ohne Gerät und ohne eigene Datei loslegen kann.

use dmxcraft_core::library::builtin_fixture_types;
use dmxcraft_core::{ChannelValue, FixtureId, Show, Step};

fn cv(fixture: FixtureId, channel: u16, value: u8) -> ChannelValue {
    ChannelValue {
        fixture,
        channel,
        value,
    }
}

/// Leere Show mit den eingebauten Gerätetypen.
pub fn empty_show() -> Show {
    let mut s = Show::new("Neue Show", 2);
    ensure_builtin_types(&mut s);
    s
}

/// Ergänzt fehlende eingebaute Gerätetypen (vorhandene bleiben unverändert).
pub fn ensure_builtin_types(show: &mut Show) {
    for t in builtin_fixture_types() {
        if show.fixture_type(&t.key).is_none() {
            show.fixture_types.push(t);
        }
    }
}

pub fn demo_show() -> Show {
    let mut s = Show::new("Demo", 2);
    ensure_builtin_types(&mut s);

    let pars: Vec<FixtureId> = (0..4)
        .map(|i| {
            s.add_fixture(
                format!("PAR {}", i + 1),
                "generic/rgbw-par-6ch",
                0,
                1 + i * 6,
            )
            .expect("Demo-Patch")
        })
        .collect();
    let heads: Vec<FixtureId> = (0..2)
        .map(|i| {
            s.add_fixture(
                format!("Moving Head {}", i + 1),
                "generic/moving-head-spot-11ch",
                0,
                101 + i * 11,
            )
            .expect("Demo-Patch")
        })
        .collect();

    // Bühnenplan: PARs auf der Traverse, Moving Heads davor
    for (i, id) in pars.iter().enumerate() {
        if let Some(f) = s.fixture_mut(*id) {
            f.plan_pos = Some([0.25 + i as f32 * 0.17, 0.25]);
        }
    }
    for (i, id) in heads.iter().enumerate() {
        if let Some(f) = s.fixture_mut(*id) {
            f.plan_pos = Some([0.33 + i as f32 * 0.34, 0.65]);
        }
    }
    s.add_group("PARs", pars.clone());
    s.add_group("Moving Heads", heads.clone());

    let farben = s.add_zone("Farben");
    let bewegung = s.add_zone("Bewegung");

    // PAR-Kanäle: 0 Dimmer, 1 R, 2 G, 3 B, 4 W, 5 Strobe
    let color = |rgbw: [u8; 4]| -> Vec<ChannelValue> {
        pars.iter()
            .flat_map(|&p| {
                [
                    cv(p, 0, 255),
                    cv(p, 1, rgbw[0]),
                    cv(p, 2, rgbw[1]),
                    cv(p, 3, rgbw[2]),
                    cv(p, 4, rgbw[3]),
                ]
            })
            .collect()
    };

    s.add_scene("Rot", Some(farben), vec![Step::new(color([255, 0, 0, 0]))]);
    s.add_scene("Blau", Some(farben), vec![Step::new(color([0, 0, 255, 0]))]);
    s.add_scene(
        "Warmweiß",
        Some(farben),
        vec![Step::new(color([255, 120, 20, 255]))],
    );

    let rainbow: Vec<Step> = [
        [255, 0, 0, 0],
        [255, 160, 0, 0],
        [0, 255, 0, 0],
        [0, 120, 255, 0],
        [160, 0, 255, 0],
    ]
    .into_iter()
    .map(|c| {
        let mut st = Step::new(color(c));
        st.fade_ms = 1500;
        st.hold_ms = 500;
        st
    })
    .collect();
    s.add_scene("Farbwechsel", Some(farben), rainbow);

    // Lauflicht über die PARs (frei kombinierbar, nur Dimmer → HTP)
    let chase: Vec<Step> = pars
        .iter()
        .map(|&p| {
            let mut st = Step::new(vec![cv(p, 0, 255)]);
            st.fade_ms = 100;
            st.hold_ms = 250;
            st
        })
        .collect();
    s.add_scene("Lauflicht", Some(farben), chase);

    // Moving Heads: 0 Pan, 2 Tilt, 5 Dimmer
    let pos = |pan: u8, tilt: u8| -> Vec<ChannelValue> {
        heads
            .iter()
            .flat_map(|&h| [cv(h, 0, pan), cv(h, 2, tilt), cv(h, 5, 255)])
            .collect()
    };
    s.add_scene("Mitte", Some(bewegung), vec![Step::new(pos(128, 128))]);
    let sweep: Vec<Step> = [(60, 90), (200, 90), (200, 170), (60, 170)]
        .into_iter()
        .map(|(p, t)| {
            let mut st = Step::new(pos(p, t));
            st.fade_ms = 1200;
            st.hold_ms = 0;
            st
        })
        .collect();
    s.add_scene("Schwenk", Some(bewegung), sweep);

    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn demo_is_valid() {
        let s = super::demo_show();
        assert_eq!(s.fixtures.len(), 6);
        assert!(s.scenes.len() >= 6);
        let json = s.to_json().unwrap();
        assert_eq!(dmxcraft_core::Show::from_json(&json).unwrap(), s);
    }
}
