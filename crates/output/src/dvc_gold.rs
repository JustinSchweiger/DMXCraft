//! Platzhalter für die USB-Anbindung der Daslight DVC Gold.
//!
//! Stand (USB-Mitschnitte vom 06.11.2026, Daslight 5 unter Windows):
//!
//! * Gerät: „Smart universal DMX interface (SIUDI11A)“, meldet sich als
//!   „DVC GOLD“, VID `0x6244`, PID `0x0650`, eigener Herstellertreiber.
//! * Verbinden: Vendor-Control-Requests mit Zeitstempel, 32-Byte-Challenge
//!   der Box, 236-Byte-Antwort des PCs und mehreren Challenge-Response-Runden
//!   – eine kryptografische Authentisierung der Software.
//! * DMX-Daten: Bulk-OUT Endpoint 1, 50 Pakete/s zu 536 Byte
//!   (`01 01 <Zähler> 24 00 00 01 00` + 528 Byte). Der Nutzteil ist auch bei
//!   lauter Nullen zufällig verteilt und ändert sich mit jedem Paket, ist also
//!   mit einem Sitzungsschlüssel verschlüsselt.
//!
//! Damit lässt sich die Box nicht ohne die Schlüssel von Daslight ansprechen.
//! Diese aus der Software zu holen hieße, eine technische Schutzmaßnahme zu
//! umgehen – das machen wir nicht. Offen bleiben ein offizielles SDK von
//! Nicolaudie oder ein anderes Interface (Art-Net, Enttec-kompatibel).
//! Dieses Backend meldet deshalb „nicht verfügbar“.

use dmxcraft_core::DmxFrame;

use crate::{DmxOutput, OutputError};

/// Vendor-ID der Nicolaudie-Interfaces (aus dem Geräte-Manager bestätigt).
pub const NICOLAUDIE_VENDOR_ID_GUESS: u16 = 0x6244;
/// Product-ID der DVC Gold (SIUDI11A).
pub const DVC_GOLD_PRODUCT_ID: u16 = 0x0650;

/// Die DVC Gold hat im Live-Modus 2 Universen (erweiterbar auf 4).
pub const DVC_GOLD_UNIVERSES: usize = 2;

pub struct DvcGoldUsb {
    connected: bool,
}

impl DvcGoldUsb {
    pub fn new() -> Self {
        Self { connected: false }
    }
}

impl Default for DvcGoldUsb {
    fn default() -> Self {
        Self::new()
    }
}

impl DmxOutput for DvcGoldUsb {
    fn name(&self) -> &str {
        "Daslight DVC Gold (USB)"
    }

    fn open(&mut self) -> Result<(), OutputError> {
        self.connected = false;
        Err(OutputError::NotImplemented(
            "Die DVC Gold verschlüsselt ihre USB-Daten – ohne offizielles SDK nicht ansprechbar",
        ))
    }

    fn send(&mut self, _frame: &DmxFrame) -> Result<(), OutputError> {
        if !self.connected {
            return Err(OutputError::NotFound("DVC Gold nicht verbunden".into()));
        }
        Err(OutputError::NotImplemented("DVC-Gold-Paketformat"))
    }

    fn close(&mut self) {
        self.connected = false;
    }

    fn max_universes(&self) -> usize {
        DVC_GOLD_UNIVERSES
    }
}
