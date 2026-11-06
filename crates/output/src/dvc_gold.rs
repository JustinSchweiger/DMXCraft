//! Platzhalter für die USB-Anbindung der Daslight DVC Gold.
//!
//! Stand: Das Protokoll ist noch nicht entschlüsselt. Dieses Backend meldet
//! deshalb beim Öffnen „noch nicht implementiert“; Engine und Oberfläche
//! funktionieren trotzdem (der Runtime-Thread versucht es periodisch erneut).
//!
//! Plan für die Umsetzung (nach Auswertung der USB-Mitschnitte):
//!
//! 1. Gerät per `nusb` finden. Hypothese aus älteren Daslight-Boxen
//!    (DVC128, Projekt „derlicht“): Vendor-ID `0x6244`. Die Product-ID der
//!    DVC Gold muss aus dem Geräte-Manager / Mitschnitt kommen.
//! 2. Interface claimen, Endpoints aus den Descriptors bestimmen
//!    (Bulk/Interrupt OUT für DMX-Daten, ggf. IN für Antworten).
//! 3. Handshake nachspielen, den Daslight 5 beim Verbinden sendet
//!    (Lizenz-/Seriennummernabfrage?). Prüfen, ob er statisch ist oder
//!    Challenge-Response.
//! 4. `send`: Frame pro Universum in das Paketformat der Box packen
//!    (Header? Universumsnummer? Prüfsumme?) und schreiben.
//!    Der Runtime-Thread ruft `send` mit der eingestellten Rate auf.
//!
//! Wichtig unter Windows: den Daslight-Treiber NICHT durch WinUSB/Zadig
//! ersetzen, solange Daslight 5 parallel weiter funktionieren soll.

use dmxcraft_core::DmxFrame;

use crate::{DmxOutput, OutputError};

/// Bekannte Vendor-ID älterer Daslight/Nicolaudie-Interfaces (Hypothese).
pub const NICOLAUDIE_VENDOR_ID_GUESS: u16 = 0x6244;

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
            "USB-Protokoll der DVC Gold ist noch nicht entschlüsselt",
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
