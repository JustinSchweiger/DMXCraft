//! Werkzeuge des MCP-Servers. Jedes Werkzeug bekommt seine Argumente als
//! JSON und liefert JSON zurück (oder eine verständliche Fehlermeldung).

use dmxcraft_core::fixture::ChannelKind;
use dmxcraft_core::timeline::{ClipContent, Timeline};
use dmxcraft_core::{
    ChannelRef, ChannelValue, Effect, EffectKind, Engine, FixtureId, FixtureType, GroupId,
    LoopMode, SceneId, Show, Step, ZoneId, UNIVERSE_SIZE,
};
use serde_json::{json, Map, Value};

use crate::{AppEvent, Host};

type R = Result<Value, String>;

/// Beschreibung eines Werkzeugs für `tools/list`.
struct Tool {
    name: &'static str,
    title: &'static str,
    description: &'static str,
    schema: fn() -> Value,
    read_only: bool,
}

macro_rules! props {
    ($($k:literal : $v:expr),* $(,)?) => {{
        let mut m = Map::new();
        $( m.insert($k.to_string(), $v); )*
        Value::Object(m)
    }};
}

fn obj(properties: Value, required: &[&str]) -> Value {
    json!({ "type": "object", "properties": properties, "required": required })
}
fn int(d: &str) -> Value {
    json!({ "type": "integer", "description": d })
}
fn num(d: &str) -> Value {
    json!({ "type": "number", "description": d })
}
fn string(d: &str) -> Value {
    json!({ "type": "string", "description": d })
}
fn boolean(d: &str) -> Value {
    json!({ "type": "boolean", "description": d })
}
fn ids(d: &str) -> Value {
    json!({ "type": "array", "items": { "type": "integer" }, "description": d })
}
fn any_obj(d: &str) -> Value {
    json!({ "type": "object", "description": d })
}

/// Werte für Kanäle: entweder einzeln oder je Kanalart für mehrere Geräte.
fn values_schema() -> Value {
    json!({
        "type": "array",
        "description": "Kanalwerte. Jeder Eintrag: {fixture: ID, channel: Index (ab 0) ODER Kanalart (\"Dimmer\", \"Red\", \"Green\", \"Blue\", \"White\", \"Amber\", \"Uv\", \"Pan\", \"Tilt\", \"Strobe\", \"ColorWheel\", \"Gobo\", \"Speed\") ODER Kanalname, value: 0–255}.",
        "items": {
            "type": "object",
            "properties": {
                "fixture": { "type": "integer" },
                "channel": { "type": ["integer", "string"] },
                "value": { "type": "integer", "minimum": 0, "maximum": 255 }
            },
            "required": ["fixture", "channel", "value"]
        }
    })
}

fn step_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "hold_ms": { "type": "integer", "description": "Haltezeit (Standard 5000 bei einem Schritt, sonst 1000)" },
            "fade_ms": { "type": "integer", "description": "Überblendzeit vom vorherigen Schritt (Standard 0)" },
            "values": values_schema(),
            "fixtures": { "type": "array", "items": { "type": "integer" }, "description": "Geräte, auf die `set` angewendet wird" },
            "set": { "type": "object", "description": "Kanalart → Wert für alle `fixtures`, z. B. {\"Dimmer\":255,\"Red\":255,\"Green\":0,\"Blue\":0}. Statt RGB geht auch \"color\":\"#ff8800\"." }
        }
    })
}

fn venue_object_schema() -> Value {
    let v3 = |d: &str| json!({ "type": "array", "items": { "type": "number" }, "description": d });
    obj(
        props! {
            "id": int("Objekt-ID (nur beim Ändern)"),
            "name": string("Name"),
            "shape": any_obj("Form, siehe Beschreibung"),
            "parent": int("Gruppe, in die das neue Objekt kommt (optional)"),
            "position": v3("[x, y, z] in m"),
            "rotation": v3("Grad, X dann Y dann Z"),
            "scale": v3("Skalierung"),
            "color": string("#rrggbb"),
            "opacity": num("Deckkraft 0–1"),
            "gloss": num("Glanz 0–1"),
            "emissive": num("Selbstleuchten 0–1"),
            "locked": boolean("gegen Verschieben sperren"),
            "hidden": boolean("ausblenden"),
        },
        &[],
    )
}

fn camera_schema() -> Value {
    json!({
        "type": "object",
        "description": "Kamera (alles optional): point = gespeicherter Kamerapunkt (Name oder Nummer ab 1, siehe get_visualizer) oder view = \"front\" | \"back\" | \"left\" | \"right\" | \"top\" | \"overview\" (Grundstellung) als Ausgangspunkt, dann yaw (Grad um die Hochachse; 0 = von vorn, also von der Publikumsseite +z; 90 = von rechts +x), pitch (Grad, positiv = von oben, -89–89), distance (m), target [x, y, z] (Blickpunkt in m), fov (Öffnungswinkel in Grad).",
        "properties": {
            "point": { "type": ["string", "integer"] },
            "view": { "type": "string" },
            "yaw": { "type": "number" },
            "pitch": { "type": "number" },
            "distance": { "type": "number" },
            "target": { "type": "array", "items": { "type": "number" } },
            "fov": { "type": "number" }
        }
    })
}

fn clip_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "track": { "type": "integer", "description": "Spur (ab 0); fehlende Spuren werden angelegt" },
            "scene": { "type": "integer", "description": "Szenen-ID für einen Szenenblock" },
            "audio_path": { "type": "string", "description": "Pfad einer Audiodatei für einen Audioblock" },
            "start_ms": { "type": "integer", "description": "Beginn auf der Zeitleiste" },
            "duration_ms": { "type": "integer", "description": "Länge (Standard: Länge der Szene bzw. der Datei)" },
            "fade_in_ms": { "type": "integer" },
            "fade_out_ms": { "type": "integer" },
            "level": { "type": "number", "description": "Pegel 0–1 (Dimmer bzw. Lautstärke)" },
            "speed": { "type": "number", "description": "Geschwindigkeit des Blocks (1 = normal)" }
        },
        "required": ["start_ms"]
    })
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "get_overview",
        title: "Überblick",
        description: "Überblick über die geöffnete Show: Name, Anzahl Geräte, Bänke mit ihren Szenen, Gruppen, laufende Szenen, Master, Blackout, Tempo.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "list_fixtures",
        title: "Geräte auflisten",
        description: "Alle gepatchten Geräte mit ID, Name, Typ, Universum (ab 1), Startadresse und ihren Kanälen (Index, Name, Art).",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "search_fixture_library",
        title: "Gerätebibliothek durchsuchen",
        description: "Sucht in der eingebauten Gerätebibliothek (generische Typen, Open Fixture Library, QLC+) nach Hersteller/Modell/Kategorie. Liefert je Modus den `type_key` für add_fixtures.",
        schema: || obj(props! { "query": string("Suchbegriffe, z. B. \"rgbw par\" oder \"moving head\""), "limit": int("max. Treffer (Standard 15)") }, &["query"]),
        read_only: true,
    },
    Tool {
        name: "add_fixtures",
        title: "Geräte patchen",
        description: "Patcht ein oder mehrere Geräte eines Typs an die nächsten freien Adressen (ab `address` im `universe`). Namen werden durchnummeriert.",
        schema: || obj(props! {
            "type_key": string("Typ aus search_fixture_library oder list_fixtures, z. B. generic/rgbw-par-6ch"),
            "count": int("Anzahl (Standard 1)"),
            "name": string("Grundname, z. B. \"PAR\" → PAR 1, PAR 2 … (Standard: Modellname)"),
            "universe": int("Universum ab 1 (Standard 1)"),
            "address": int("erste Startadresse (Standard 1 = erste freie)"),
        }, &["type_key"]),
        read_only: false,
    },
    Tool {
        name: "update_fixture",
        title: "Gerät ändern",
        description: "Ändert Name, Patch-Adresse oder Position im Bühnenplan (x/y 0–1) eines Geräts.",
        schema: || obj(props! {
            "id": int("Geräte-ID"), "name": string("neuer Name"),
            "universe": int("Universum ab 1"), "address": int("Startadresse 1–512"),
            "plan_x": num("Bühnenplan x 0–1 (links → rechts)"), "plan_y": num("Bühnenplan y 0–1 (hinten → vorn)"),
        }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "delete_fixtures",
        title: "Geräte löschen",
        description: "Entfernt Geräte aus dem Patch (und aus Gruppen, Szenen und Effekten).",
        schema: || obj(props! { "ids": ids("Geräte-IDs") }, &["ids"]),
        read_only: false,
    },
    Tool {
        name: "create_group",
        title: "Gruppe anlegen",
        description: "Legt eine Gerätegruppe an.",
        schema: || obj(props! { "name": string("Name"), "fixtures": ids("Geräte-IDs") }, &["name", "fixtures"]),
        read_only: false,
    },
    Tool {
        name: "delete_group",
        title: "Gruppe löschen",
        description: "Löscht eine Gruppe (die Geräte bleiben).",
        schema: || obj(props! { "id": int("Gruppen-ID") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "create_bank",
        title: "Bank anlegen",
        description: "Legt eine neue Bank (Spalte mit Szenen) an.",
        schema: || obj(props! { "name": string("Name"), "color": string("Farbe als #rrggbb (optional)") }, &["name"]),
        read_only: false,
    },
    Tool {
        name: "update_bank",
        title: "Bank ändern",
        description: "Benennt eine Bank um, ändert ihre Farbe oder blendet sie aus/ein.",
        schema: || obj(props! { "id": int("Bank-ID"), "name": string("neuer Name"), "color": string("#rrggbb oder \"auto\""), "hidden": boolean("ausblenden") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "delete_bank",
        title: "Bank löschen",
        description: "Löscht eine Bank samt ihrer Szenen.",
        schema: || obj(props! { "id": int("Bank-ID") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "list_scenes",
        title: "Szenen auflisten",
        description: "Szenen (optional nur einer Bank) mit ID, Name, Bank, Art (STATISCH/FX/SUPER SCENE), Dauer und ob sie läuft.",
        schema: || obj(props! { "bank": int("nur diese Bank (optional)") }, &[]),
        read_only: true,
    },
    Tool {
        name: "get_scene",
        title: "Szene lesen",
        description: "Alle Details einer Szene: Einstellungen, Schritte mit Kanalwerten, Effekte (als JSON) und bei SUPER SCENES die Zeitleiste.",
        schema: || obj(props! { "id": int("Szenen-ID") }, &["id"]),
        read_only: true,
    },
    Tool {
        name: "create_scene",
        title: "Szene anlegen",
        description: "Legt eine Szene in einer Bank an. Ein Schritt = statische Szene; mehrere Schritte = Ablauf (Steps). Werte über `values` oder bequem über `fixtures` + `set` je Schritt.",
        schema: || obj(props! {
            "name": string("Name"), "bank": int("Bank-ID"),
            "steps": json!({ "type": "array", "items": step_schema(), "description": "Schritte (mindestens einer)" }),
            "color": string("Farbe der Karte #rrggbb (optional)"),
            "loop": boolean("Ablauf endlos wiederholen (Standard true)"),
            "settings": any_obj("Szeneneinstellungen, z. B. {\"fade_in_ms\":500,\"fade_out_ms\":500,\"dimmer\":255,\"speed_pct\":100,\"flash\":false}"),
            "select": boolean("in der Oberfläche auswählen (Standard true)"),
        }, &["name", "bank", "steps"]),
        read_only: false,
    },
    Tool {
        name: "update_scene",
        title: "Szene ändern",
        description: "Ändert eine Szene: Name, Farbe, Bank, ausgeblendet, Endlosschleife, Einstellungen (werden zusammengeführt) oder ersetzt die Schritte.",
        schema: || obj(props! {
            "id": int("Szenen-ID"), "name": string("neuer Name"), "color": string("#rrggbb oder \"auto\""),
            "bank": int("in diese Bank verschieben"), "hidden": boolean("ausblenden"),
            "loop": boolean("endlos wiederholen"), "settings": any_obj("Einstellungen (nur genannte Felder ändern sich)"),
            "steps": json!({ "type": "array", "items": step_schema(), "description": "neue Schritte (ersetzt alle)" }),
        }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "delete_scenes",
        title: "Szenen löschen",
        description: "Löscht Szenen.",
        schema: || obj(props! { "ids": ids("Szenen-IDs") }, &["ids"]),
        read_only: false,
    },
    Tool {
        name: "describe_effect",
        title: "Effekt beschreiben",
        description: "Liefert die Standard-Einstellungen eines Effekttyps als JSON (Vorlage für add_effect/update_effect). Typen: ColorFx, ChaserFx, MoveFx, ValueFx, CurveFx, Mappings, ColorMappings.",
        schema: || obj(props! { "kind": string("Effekttyp") }, &["kind"]),
        read_only: true,
    },
    Tool {
        name: "add_effect",
        title: "Effekt hinzufügen",
        description: "Fügt einer Szene einen Effekt hinzu (macht sie zur FX-Szene). `fixtures` in der gewünschten Reihenfolge (= Lauf-/Farbreihenfolge). `params` überschreibt Felder der Vorlage aus describe_effect.",
        schema: || obj(props! {
            "scene": int("Szenen-ID"), "kind": string("Effekttyp, z. B. ColorFx"),
            "fixtures": ids("Geräte in Effekt-Reihenfolge"), "params": any_obj("Felder wie in describe_effect"),
        }, &["scene", "kind", "fixtures"]),
        read_only: false,
    },
    Tool {
        name: "update_effect",
        title: "Effekt ändern",
        description: "Ändert einen Effekt einer Szene (Felder werden zusammengeführt).",
        schema: || obj(props! {
            "scene": int("Szenen-ID"), "effect": int("Effekt-ID aus get_scene"),
            "fixtures": ids("neue Geräte (optional)"), "params": any_obj("geänderte Felder"),
        }, &["scene", "effect"]),
        read_only: false,
    },
    Tool {
        name: "remove_effect",
        title: "Effekt entfernen",
        description: "Entfernt einen Effekt aus einer Szene.",
        schema: || obj(props! { "scene": int("Szenen-ID"), "effect": int("Effekt-ID") }, &["scene", "effect"]),
        read_only: false,
    },
    Tool {
        name: "create_super_scene",
        title: "SUPER SCENE anlegen",
        description: "Legt eine SUPER SCENE an: eine Zeitleiste, auf deren Spuren Szenen- und Audioblöcke liegen (spätere Spuren überdecken frühere). Ohne `clips` entsteht eine leere Zeitleiste.",
        schema: || obj(props! {
            "name": string("Name"), "bank": int("Bank-ID"),
            "clips": json!({ "type": "array", "items": clip_schema() }),
            "track_names": json!({ "type": "array", "items": { "type": "string" }, "description": "Namen der Spuren (optional)" }),
            "bpm": num("Tempo für das Beat-Raster (optional)"),
        }, &["name", "bank"]),
        read_only: false,
    },
    Tool {
        name: "add_timeline_clips",
        title: "Blöcke einfügen",
        description: "Fügt einer SUPER SCENE Szenen- oder Audioblöcke hinzu.",
        schema: || obj(props! { "scene": int("ID der SUPER SCENE"), "clips": json!({ "type": "array", "items": clip_schema() }) }, &["scene", "clips"]),
        read_only: false,
    },
    Tool {
        name: "remove_timeline_clips",
        title: "Blöcke entfernen",
        description: "Entfernt Blöcke (IDs aus get_scene) aus einer SUPER SCENE.",
        schema: || obj(props! { "scene": int("ID der SUPER SCENE"), "clips": ids("Block-IDs") }, &["scene", "clips"]),
        read_only: false,
    },
    Tool {
        name: "play_scene",
        title: "Szene starten",
        description: "Startet eine Szene (andere Szenen derselben Bank stoppen). SUPER SCENES laufen ab `position_ms`.",
        schema: || obj(props! { "id": int("Szenen-ID"), "position_ms": int("Startposition einer SUPER SCENE (optional)") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "stop_scene",
        title: "Szene stoppen",
        description: "Stoppt eine Szene (mit ihrer Ausblendzeit). Ohne `id`: alle Szenen.",
        schema: || obj(props! { "id": int("Szenen-ID (leer = alle)") }, &[]),
        read_only: false,
    },
    Tool {
        name: "pause_scene",
        title: "Pause",
        description: "Pausiert eine laufende Szene bzw. setzt sie fort.",
        schema: || obj(props! { "id": int("Szenen-ID") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "set_output",
        title: "Master/Blackout/Tempo",
        description: "Setzt Master (0–1), Blackout und globales Tempo (1 = normal).",
        schema: || obj(props! { "master": num("0–1"), "blackout": boolean("Blackout an/aus"), "speed": num("Tempo, 1 = normal") }, &[]),
        read_only: false,
    },
    Tool {
        name: "set_live_values",
        title: "Live-Werte setzen",
        description: "Setzt Kanäle direkt im Live-Betrieb (liegt über allen Szenen, wird nicht gespeichert). `clear: true` entfernt vorher alle Live-Werte.",
        schema: || obj(props! {
            "values": values_schema(),
            "fixtures": ids("Geräte für `set`"),
            "set": any_obj("Kanalart → Wert, z. B. {\"Dimmer\":255,\"color\":\"#00ffaa\"}"),
            "clear": boolean("vorher alle Live-Werte löschen"),
        }, &[]),
        read_only: false,
    },
    Tool {
        name: "clear_live_values",
        title: "Live-Werte löschen",
        description: "Entfernt alle Live-Werte.",
        schema: || obj(json!({}), &[]),
        read_only: false,
    },
    Tool {
        name: "get_dmx_output",
        title: "DMX-Ausgang lesen",
        description: "Aktuelle DMX-Werte eines Universums (oder je Gerät: `fixtures`).",
        schema: || obj(props! {
            "universe": int("Universum ab 1 (Standard 1)"),
            "start": int("erste Adresse (Standard 1)"), "count": int("Anzahl (Standard 512)"),
            "fixtures": ids("statt Adressbereich: Werte dieser Geräte je Kanal"),
        }, &[]),
        read_only: true,
    },
    Tool {
        name: "select_scene",
        title: "Szene anzeigen",
        description: "Wählt eine Szene in der Oberfläche aus, damit der Benutzer sie sieht.",
        schema: || obj(props! { "id": int("Szenen-ID") }, &["id"]),
        read_only: true,
    },
    Tool {
        name: "get_venue",
        title: "Veranstaltungsort lesen",
        description: "3D-Veranstaltungsort: Raumgröße, Umgebung (Raumlicht, Strahlen, Nebel), alle Objekte (Quader, Flächen, Zylinder, Traversen, Gruppen) mit Lage und Weltposition sowie die 3D-Lage jedes Geräts. Einheiten: Meter, y nach oben.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "set_venue",
        title: "Veranstaltungsort einstellen",
        description: "Ändert Name, Raumgröße [Breite, Höhe, Tiefe] und Umgebung (ambient, beams, fog je 0–1).",
        schema: || obj(props! {
            "name": string("Name"), "size": json!({ "type": "array", "items": { "type": "number" }, "description": "[Breite x, Höhe y, Tiefe z] in m" }),
            "ambient": num("Raumlicht 0–1"), "beams": num("Sichtbarkeit der Strahlen 0–1"), "fog": num("Nebel 0–1"),
        }, &[]),
        read_only: false,
    },
    Tool {
        name: "add_venue_object",
        title: "Objekt in den Ort setzen",
        description: "Fügt ein Objekt hinzu. `shape` z. B. {\"type\":\"box\",\"size\":[2,0.4,1]}, {\"type\":\"plane\",\"size\":[4,3]}, {\"type\":\"cylinder\",\"radius\":0.05,\"height\":3}, {\"type\":\"truss\",\"length\":3,\"profile\":\"square\"} (entlang x) oder {\"type\":\"group\"}. Mittelpunkt des Objekts liegt bei `position`.",
        schema: venue_object_schema,
        read_only: false,
    },
    Tool {
        name: "update_venue_object",
        title: "Objekt ändern",
        description: "Ändert Form, Lage, Farbe, Deckkraft … eines Objekts (nur angegebene Felder).",
        schema: venue_object_schema,
        read_only: false,
    },
    Tool {
        name: "delete_venue_objects",
        title: "Objekte entfernen",
        description: "Entfernt Objekte (Gruppen samt Inhalt).",
        schema: || obj(props! { "ids": ids("Objekt-IDs") }, &["ids"]),
        read_only: false,
    },
    Tool {
        name: "place_fixtures",
        title: "Geräte im Raum platzieren",
        description: "Setzt die 3D-Lage von Geräten: `position` (bei mehreren mit `spacing` je Gerät versetzt), optional an die nächste Traverse hängen, `rotation` oder auf einen Punkt `aim_at` ausrichten. `automatic`: wieder aus dem Bühnenplan ableiten.",
        schema: || obj(props! {
            "fixtures": ids("Geräte-IDs"),
            "position": json!({ "type": "array", "items": { "type": "number" }, "description": "[x, y, z] in m für das erste Gerät" }),
            "spacing": json!({ "type": "array", "items": { "type": "number" }, "description": "Versatz je weiterem Gerät [x, y, z]" }),
            "hang_on_truss": boolean("unter die nächste Traverse hängen"),
            "rotation": json!({ "type": "array", "items": { "type": "number" }, "description": "Euler-Winkel in Grad; [0,0,0] strahlt senkrecht nach unten" }),
            "aim_at": json!({ "type": "array", "items": { "type": "number" }, "description": "Zielpunkt [x, y, z], auf den das Gerät strahlt" }),
            "automatic": boolean("Lage wieder automatisch"),
        }, &["fixtures"]),
        read_only: false,
    },
    Tool {
        name: "get_visualizer",
        title: "3D-Visualisierer abfragen",
        description: "Zustand des 3D-Visualisierers: Fenster offen, Qualität, Kamera (yaw, pitch, distance, target, fov), gespeicherte Kamerapunkte und Raumgröße. Den Ort selbst (Bauten, Geräte-Lagen) liefert get_venue; Umgebungslicht, Strahlen und Nebel stellt set_venue ein.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "set_visualizer",
        title: "3D-Visualisierer einstellen",
        description: "Fenster öffnen/schließen, Qualität (\"low\", \"medium\", \"high\", \"ultra\") und die Kamera der Ansicht setzen (bei offenem Fenster mit Kamerafahrt). Kamerapunkte (in der Leiste des Visualisierers per Klick bzw. Taste 1–9 erreichbar, in der Show gespeichert) mit `save_point` anlegen bzw. überschreiben (aktuelle Kamera oder `camera`) und mit `delete_point` entfernen.",
        schema: || obj(props! {
            "open": boolean("Fenster öffnen (true) oder schließen (false)"),
            "quality": string("low | medium | high | ultra"),
            "camera": camera_schema(),
            "save_point": string("Kamerapunkt mit diesem Namen speichern (vorhandener gleichen Namens wird überschrieben)"),
            "delete_point": string("Kamerapunkt (Name oder Nummer) entfernen"),
        }, &[]),
        read_only: false,
    },
    Tool {
        name: "visualizer_screenshot",
        title: "Bild des 3D-Visualisierers",
        description: "Rendert die 3D-Ansicht mit dem aktuellen DMX-Ausgang als PNG-Bild (Lichtstrahlen, Lichtflecken, Bauten) – z. B. um Positionen, Ausrichtung oder Farben einer Szene zu prüfen. Ohne `camera` aus Sicht der aktuellen Kamera. Das Fenster muss dafür nicht offen sein.",
        schema: || obj(props! {
            "camera": camera_schema(),
            "width": int("Breite in Pixel (Standard 1280, 320–2560)"),
            "height": int("Höhe in Pixel (Standard 720, 240–1600)"),
        }, &[]),
        read_only: true,
    },
    Tool {
        name: "list_profiles",
        title: "Eigene Geräteprofile",
        description: "Eigene Geräteprofile aus dem Profile Builder mit Modi, Gerätetyp-Schlüsseln (type_key für add_fixtures) und offenen Problemen.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "get_profile",
        title: "Geräteprofil lesen",
        description: "Ein eigenes Geräteprofil vollständig (Kanäle mit Bereichen, Modi, physische Daten) – zum Ändern lesen, anpassen und mit save_profile speichern.",
        schema: || obj(props! { "key": string("Schlüssel aus list_profiles") }, &["key"]),
        read_only: true,
    },
    Tool {
        name: "describe_profile_format",
        title: "Format der Geräteprofile",
        description: "Vorlage (Beispielprofil), Hinweise und alle Bereichsarten (capability kinds) mit ihren Werten für save_profile.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "save_profile",
        title: "Geräteprofil speichern",
        description: "Legt ein eigenes Geräteprofil an (ohne key) oder ersetzt eines (mit key) – wie Speichern im Profile Builder. Bereiche werden lückenlos gemacht; bei Fehlern (fehlender Name, Modus, Winkel …) wird nicht gespeichert und die Probleme werden genannt. Bereits gepatchte Geräte des Profils werden aktualisiert.",
        schema: || obj(props! { "profile": any_obj("Profil wie bei get_profile bzw. describe_profile_format") }, &["profile"]),
        read_only: false,
    },
    Tool {
        name: "delete_profile",
        title: "Geräteprofil entfernen",
        description: "Entfernt ein eigenes Geräteprofil aus der Bibliothek (die Datei kommt in den Ordner „geloescht“). Gepatchte Geräte behalten ihren Gerätetyp.",
        schema: || obj(props! { "key": string("Schlüssel aus list_profiles") }, &["key"]),
        read_only: false,
    },
    Tool {
        name: "save_show",
        title: "Show speichern",
        description: "Speichert die Show (ohne `path` unter dem aktuellen Pfad).",
        schema: || obj(props! { "path": string("Dateipfad, Endung .dmxcraft (optional)") }, &[]),
        read_only: false,
    },
];

/// Werkzeugliste für `tools/list`.
pub fn list() -> Vec<Value> {
    TOOLS
        .iter()
        .map(|t| {
            json!({
                "name": t.name,
                "title": t.title,
                "description": t.description,
                "inputSchema": (t.schema)(),
                "annotations": { "readOnlyHint": t.read_only, "destructiveHint": t.name.starts_with("delete") || t.name.starts_with("remove") },
            })
        })
        .collect()
}

/// Führt ein Werkzeug aus.
pub fn call(host: &Host, name: &str, a: &Value) -> R {
    let read_only = TOOLS
        .iter()
        .find(|t| t.name == name)
        .is_some_and(|t| t.read_only);
    if !read_only && host.locked.load(std::sync::atomic::Ordering::Relaxed) {
        return Err("DMXCraft ist gesperrt (Schloss in der Titelleiste) – nur lesende Werkzeuge sind erlaubt".into());
    }
    // 3D-Visualisierer: läuft in der Oberfläche, die dafür die Engine braucht –
    // also ohne Sperre der Engine aufrufen
    match name {
        "get_visualizer" => return (host.viz)("get", a).map(|r| r.0),
        "set_visualizer" => return (host.viz)("set", a).map(|r| r.0),
        "visualizer_screenshot" => {
            let (mut info, png) = (host.viz)("screenshot", a)?;
            let png = png.ok_or("Kein Bild erhalten")?;
            info["__image_png"] = Value::String(base64(&png));
            return Ok(info);
        }
        "list_profiles" => return crate::profiles::list(host.profiles.as_ref()),
        "get_profile" => return crate::profiles::get(host.profiles.as_ref(), &req_str(a, "key")?),
        "describe_profile_format" => return crate::profiles::format(),
        _ => {}
    }
    let user_types = if matches!(name, "search_fixture_library" | "add_fixtures") {
        crate::profiles::user_types(host.profiles.as_ref())
    } else {
        Vec::new()
    };
    let mut guard = host
        .engine
        .lock()
        .map_err(|_| "Engine nicht verfügbar".to_string())?;
    let e: &mut Engine = &mut guard;
    let out = match name {
        "get_overview" => overview(e),
        "list_fixtures" => Ok(
            json!({ "fixtures": e.show().fixtures.iter().map(|f| fixture_json(e.show(), f.id)).collect::<Vec<_>>() }),
        ),
        "search_fixture_library" => search_library(a, &user_types),
        "add_fixtures" => add_fixtures(e, a, &user_types),
        "save_profile" => {
            let r = crate::profiles::save(host.profiles.as_ref(), e, a);
            if r.is_ok() {
                (host.notify)(AppEvent::ProfilesChanged);
            }
            r
        }
        "delete_profile" => {
            let r = crate::profiles::delete(host.profiles.as_ref(), e, &req_str(a, "key")?);
            if r.is_ok() {
                (host.notify)(AppEvent::ProfilesChanged);
            }
            r
        }
        "update_fixture" => update_fixture(e, a),
        "delete_fixtures" => {
            let list = id_list(a, "ids")?;
            for id in &list {
                e.show_mut().remove_fixture(FixtureId(*id));
                for s in &mut e.show_mut().scenes {
                    for st in &mut s.steps {
                        st.values.retain(|v| v.fixture.0 != *id);
                    }
                    for fx in &mut s.effects {
                        fx.fixtures.retain(|f| f.0 != *id);
                    }
                }
            }
            e.sync();
            Ok(json!({ "deleted": list }))
        }
        "create_group" => {
            let name = req_str(a, "name")?;
            let fx = existing_fixtures(e.show(), &id_list(a, "fixtures")?)?;
            let id = e.show_mut().add_group(name, fx);
            Ok(json!({ "group": id.0 }))
        }
        "delete_group" => {
            let id = GroupId(req_u32(a, "id")?);
            e.show()
                .groups
                .iter()
                .find(|g| g.id == id)
                .ok_or("Gruppe nicht gefunden")?;
            e.show_mut().remove_group(id);
            Ok(json!({ "deleted": id.0 }))
        }
        "create_bank" => {
            let id = e.show_mut().add_zone(req_str(a, "name")?);
            if let Some(c) = a.get("color") {
                e.show_mut().zone_mut(id).ok_or("Bank fehlt")?.color = parse_color_opt(c)?;
            }
            Ok(json!({ "bank": id.0 }))
        }
        "update_bank" => {
            let id = ZoneId(req_u32(a, "id")?);
            let z = e.show_mut().zone_mut(id).ok_or("Bank nicht gefunden")?;
            if let Some(n) = opt_str(a, "name") {
                z.name = n;
            }
            if let Some(c) = a.get("color") {
                z.color = parse_color_opt(c)?;
            }
            if let Some(h) = a.get("hidden").and_then(Value::as_bool) {
                z.hidden = h;
            }
            Ok(json!({ "bank": id.0 }))
        }
        "delete_bank" => {
            let id = ZoneId(req_u32(a, "id")?);
            e.show().zone(id).ok_or("Bank nicht gefunden")?;
            e.show_mut().remove_zone_with_scenes(id);
            e.sync();
            Ok(json!({ "deleted": id.0 }))
        }
        "list_scenes" => list_scenes(e, a),
        "get_scene" => get_scene(e, SceneId(req_u32(a, "id")?)),
        "create_scene" => create_scene(host, e, a),
        "update_scene" => update_scene(e, a),
        "delete_scenes" => {
            let list = id_list(a, "ids")?;
            for id in &list {
                e.release(SceneId(*id));
                e.show_mut().remove_scene(SceneId(*id));
            }
            e.sync();
            Ok(json!({ "deleted": list }))
        }
        "describe_effect" => {
            let kind = parse_kind(&req_str(a, "kind")?)?;
            Ok(
                json!({ "kind": kind_name(kind), "template": serde_json::to_value(Effect::new(1, kind)).unwrap_or_default() }),
            )
        }
        "add_effect" => add_effect(e, a),
        "update_effect" => update_effect(e, a),
        "remove_effect" => {
            let sid = SceneId(req_u32(a, "scene")?);
            let fid = req_u32(a, "effect")?;
            let s = e.show_mut().scene_mut(sid).ok_or("Szene nicht gefunden")?;
            let n = s.effects.len();
            s.effects.retain(|f| f.id != fid);
            if s.effects.len() == n {
                return Err("Effekt nicht gefunden".into());
            }
            Ok(json!({ "scene": sid.0, "removed": fid }))
        }
        "create_super_scene" => create_super_scene(host, e, a),
        "add_timeline_clips" => {
            let sid = SceneId(req_u32(a, "scene")?);
            let clips = a
                .get("clips")
                .and_then(Value::as_array)
                .ok_or("`clips` fehlt")?
                .clone();
            let ids = add_clips(host, e, sid, &clips)?;
            Ok(json!({ "scene": sid.0, "clips": ids }))
        }
        "remove_timeline_clips" => {
            let sid = SceneId(req_u32(a, "scene")?);
            let list = id_list(a, "clips")?;
            let s = e.show_mut().scene_mut(sid).ok_or("Szene nicht gefunden")?;
            let tl = s.timeline_mut().ok_or("Keine SUPER SCENE")?;
            for c in &list {
                tl.remove_clip(*c);
            }
            s.sync_timeline_length();
            Ok(json!({ "scene": sid.0, "removed": list }))
        }
        "play_scene" => {
            let id = SceneId(req_u32(a, "id")?);
            let s = e.show().scene(id).ok_or("Szene nicht gefunden")?;
            if s.steps.is_empty() {
                return Err("Szene ist leer".into());
            }
            e.activate(id);
            if let Some(ms) = a.get("position_ms").and_then(Value::as_f64) {
                e.seek_timeline(id, ms);
            }
            Ok(json!({ "playing": id.0 }))
        }
        "stop_scene" => match a.get("id").and_then(Value::as_u64) {
            Some(id) => {
                e.release(SceneId(id as u32));
                Ok(json!({ "stopped": id }))
            }
            None => {
                e.release_all();
                Ok(json!({ "stopped": "all" }))
            }
        },
        "pause_scene" => {
            let id = SceneId(req_u32(a, "id")?);
            if !e.is_active(id) {
                return Err("Szene läuft nicht".into());
            }
            e.toggle_pause(id);
            Ok(json!({ "scene": id.0, "paused": e.progress(id).is_some_and(|p| p.paused) }))
        }
        "set_output" => {
            if let Some(m) = a.get("master").and_then(Value::as_f64) {
                e.set_master(m.clamp(0.0, 1.0) as f32);
            }
            if let Some(b) = a.get("blackout").and_then(Value::as_bool) {
                e.set_blackout(b);
            }
            if let Some(s) = a.get("speed").and_then(Value::as_f64) {
                e.set_speed(s as f32);
            }
            Ok(json!({ "master": e.master(), "blackout": e.blackout(), "speed": e.speed() }))
        }
        "set_live_values" => {
            if a.get("clear").and_then(Value::as_bool).unwrap_or(false) {
                e.clear_live();
            }
            let vals = collect_values(e.show(), a)?;
            for v in &vals {
                e.set_live(v.channel_ref(), v.value);
            }
            Ok(json!({ "set": vals.len() }))
        }
        "clear_live_values" => {
            e.clear_live();
            Ok(json!({ "cleared": true }))
        }
        "get_dmx_output" => dmx_output(e, a),
        "select_scene" => {
            let id = SceneId(req_u32(a, "id")?);
            e.show().scene(id).ok_or("Szene nicht gefunden")?;
            (host.notify)(AppEvent::SelectScene(id));
            Ok(json!({ "selected": id.0 }))
        }
        "save_show" => {
            let path = opt_str(a, "path")
                .or_else(|| {
                    host.show_path
                        .lock()
                        .ok()
                        .map(|p| p.clone())
                        .filter(|p| !p.is_empty())
                })
                .ok_or("Die Show hat noch keinen Dateipfad – bitte `path` angeben")?;
            if let Some(stem) = std::path::Path::new(&path).file_stem() {
                e.show_mut().name = stem.to_string_lossy().into_owned();
            }
            e.show().save(&path).map_err(|err| err.to_string())?;
            (host.notify)(AppEvent::Saved(path.clone()));
            Ok(json!({ "saved": path }))
        }
        "get_venue" => crate::venue::get_venue(e),
        "set_venue" => crate::venue::set_venue(e, a),
        "add_venue_object" => crate::venue::add_object(e, a),
        "update_venue_object" => crate::venue::update_object(e, a),
        "delete_venue_objects" => crate::venue::delete_objects(e, &id_list(a, "ids")?),
        "place_fixtures" => crate::venue::place_fixtures(e, a),
        _ => Err(format!("Unbekanntes Werkzeug: {name}")),
    };
    drop(guard);
    out
}

// ---------- Hilfen: Argumente ----------

fn req_u32(a: &Value, k: &str) -> Result<u32, String> {
    a.get(k)
        .and_then(Value::as_u64)
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| format!("`{k}` fehlt oder ist keine Zahl"))
}
fn opt_u64(a: &Value, k: &str) -> Option<u64> {
    a.get(k).and_then(Value::as_u64)
}
fn req_str(a: &Value, k: &str) -> Result<String, String> {
    opt_str(a, k).ok_or_else(|| format!("`{k}` fehlt"))
}
fn opt_str(a: &Value, k: &str) -> Option<String> {
    a.get(k).and_then(Value::as_str).map(str::to_string)
}
fn id_list(a: &Value, k: &str) -> Result<Vec<u32>, String> {
    a.get(k)
        .and_then(Value::as_array)
        .map(|v| {
            v.iter()
                .filter_map(Value::as_u64)
                .filter_map(|n| u32::try_from(n).ok())
                .collect()
        })
        .ok_or_else(|| format!("`{k}` fehlt (Liste von IDs)"))
}

fn existing_fixtures(show: &Show, list: &[u32]) -> Result<Vec<FixtureId>, String> {
    list.iter()
        .map(|id| {
            let f = FixtureId(*id);
            show.fixture(f)
                .map(|_| f)
                .ok_or(format!("Gerät {id} nicht gefunden"))
        })
        .collect()
}

/// „#rrggbb“ → RGB.
pub(crate) fn parse_hex(s: &str) -> Result<[u8; 3], String> {
    let h = s.trim().trim_start_matches('#');
    if h.len() != 6 {
        return Err(format!("Farbe „{s}“: erwartet #rrggbb"));
    }
    let p = |i: usize| {
        u8::from_str_radix(&h[i..i + 2], 16).map_err(|_| format!("Farbe „{s}“ ungültig"))
    };
    Ok([p(0)?, p(2)?, p(4)?])
}

/// Farbe einer Bank/Szene: `"auto"`/null = keine eigene Farbe.
fn parse_color_opt(v: &Value) -> Result<Option<[u8; 3]>, String> {
    match v {
        Value::Null => Ok(None),
        Value::String(s) if s.eq_ignore_ascii_case("auto") || s.is_empty() => Ok(None),
        Value::String(s) => parse_hex(s).map(Some),
        _ => Err("Farbe als \"#rrggbb\" angeben".into()),
    }
}

fn parse_kind(s: &str) -> Result<EffectKind, String> {
    let norm = |x: &str| x.to_lowercase().replace([' ', '_', '-'], "");
    EffectKind::ALL
        .into_iter()
        .find(|k| norm(kind_name(*k)) == norm(s) || norm(k.label()) == norm(s))
        .ok_or_else(|| format!("Unbekannter Effekttyp „{s}“"))
}

fn kind_name(k: EffectKind) -> &'static str {
    match k {
        EffectKind::Steps => "Steps",
        EffectKind::ColorFx => "ColorFx",
        EffectKind::ChaserFx => "ChaserFx",
        EffectKind::MoveFx => "MoveFx",
        EffectKind::ValueFx => "ValueFx",
        EffectKind::CurveFx => "CurveFx",
        EffectKind::Mappings => "Mappings",
        EffectKind::ColorMappings => "ColorMappings",
        EffectKind::SuperScene => "SuperScene",
    }
}

fn channel_kind(s: &str) -> Option<ChannelKind> {
    let n = s.to_lowercase().replace([' ', '_', '-'], "");
    let alias = match n.as_str() {
        "r" | "rot" => "red",
        "g" | "grün" | "gruen" => "green",
        "b" | "blau" => "blue",
        "w" | "weiß" | "weiss" => "white",
        "intensity" | "intensität" => "dimmer",
        "colour" | "farbrad" => "colorwheel",
        "nebel" | "smoke" | "haze" => "fog",
        "lüfter" | "luefter" => "fan",
        "fokus" => "focus",
        "programm" | "makro" | "macro" | "effekt" => "program",
        "steuerung" | "modus" | "mode" => "control",
        "keinefunktion" => "nofunction",
        other => other,
    };
    ChannelKind::ALL
        .into_iter()
        .find(|k| format!("{k:?}").to_lowercase() == alias)
}

/// Kanalindex eines Geräts aus Index, Kanalart oder Kanalname.
fn resolve_channel(show: &Show, fixture: FixtureId, ch: &Value) -> Result<u16, String> {
    let f = show
        .fixture(fixture)
        .ok_or(format!("Gerät {} nicht gefunden", fixture.0))?;
    let ty = show.type_of(f).ok_or("Gerätetyp fehlt")?;
    if let Some(i) = ch.as_u64() {
        return (i < ty.channels.len() as u64)
            .then_some(i as u16)
            .ok_or(format!("{} hat keinen Kanal {i}", f.name));
    }
    let s = ch.as_str().ok_or("`channel` als Index oder Name angeben")?;
    if let Some(k) = channel_kind(s) {
        if let Some(i) = ty.channels.iter().position(|c| c.kind == k) {
            return Ok(i as u16);
        }
    }
    ty.channels
        .iter()
        .position(|c| c.name.eq_ignore_ascii_case(s))
        .map(|i| i as u16)
        .ok_or(format!("{} hat keinen Kanal „{s}“", f.name))
}

/// Werte aus `values` sowie `fixtures` + `set` (je Kanalart).
fn collect_values(show: &Show, a: &Value) -> Result<Vec<ChannelValue>, String> {
    let mut out: Vec<ChannelValue> = Vec::new();
    let mut put = |v: ChannelValue| {
        out.retain(|o| o.channel_ref() != v.channel_ref());
        out.push(v);
    };
    if let Some(vals) = a.get("values").and_then(Value::as_array) {
        for v in vals {
            let fixture = FixtureId(req_u32(v, "fixture")?);
            let channel = resolve_channel(show, fixture, v.get("channel").unwrap_or(&Value::Null))?;
            let value = v
                .get("value")
                .and_then(Value::as_u64)
                .ok_or("`value` fehlt")?
                .min(255) as u8;
            put(ChannelValue {
                fixture,
                channel,
                value,
            });
        }
    }
    if let Some(set) = a.get("set").and_then(Value::as_object) {
        let fixtures = existing_fixtures(
            show,
            &id_list(a, "fixtures").map_err(|_| "`set` braucht `fixtures`")?,
        )?;
        // „color“ als Abkürzung für Rot/Grün/Blau
        let mut pairs: Vec<(String, u8)> = Vec::new();
        for (k, v) in set {
            if k.eq_ignore_ascii_case("color") {
                let rgb = parse_hex(v.as_str().ok_or("color als \"#rrggbb\"")?)?;
                pairs.extend([
                    ("Red".into(), rgb[0]),
                    ("Green".into(), rgb[1]),
                    ("Blue".into(), rgb[2]),
                ]);
            } else {
                let n = v.as_u64().ok_or(format!("Wert für {k} fehlt"))?.min(255) as u8;
                pairs.push((k.clone(), n));
            }
        }
        for f in fixtures {
            let ty = show
                .fixture(f)
                .and_then(|x| show.type_of(x))
                .ok_or("Gerätetyp fehlt")?;
            for (k, value) in &pairs {
                let kind = channel_kind(k).ok_or(format!("Unbekannte Kanalart „{k}“"))?;
                // Geräte ohne diese Kanalart werden übersprungen
                for (i, c) in ty.channels.iter().enumerate() {
                    if c.kind == kind {
                        put(ChannelValue {
                            fixture: f,
                            channel: i as u16,
                            value: *value,
                        });
                    }
                }
            }
        }
    }
    Ok(out)
}

fn build_steps(show: &Show, a: &Value) -> Result<Vec<Step>, String> {
    let list = a
        .get("steps")
        .and_then(Value::as_array)
        .ok_or("`steps` fehlt")?;
    if list.is_empty() {
        return Err("mindestens ein Schritt nötig".into());
    }
    let single = list.len() == 1;
    list.iter()
        .map(|s| {
            let mut st = Step::new(collect_values(show, s)?);
            st.hold_ms =
                opt_u64(s, "hold_ms").map_or(if single { 5000 } else { 1000 }, |v| v as u32);
            st.fade_ms = opt_u64(s, "fade_ms").unwrap_or(0) as u32;
            Ok(st)
        })
        .collect()
}

/// `patch` rekursiv in `base` übernehmen.
fn merge(base: &mut Value, patch: &Value) {
    match (base, patch) {
        (Value::Object(b), Value::Object(p)) => {
            for (k, v) in p {
                merge(b.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (b, p) => *b = p.clone(),
    }
}

fn merged<T: serde::Serialize + serde::de::DeserializeOwned>(
    v: &T,
    patch: &Value,
) -> Result<T, String> {
    let mut j = serde_json::to_value(v).map_err(|e| e.to_string())?;
    merge(&mut j, patch);
    serde_json::from_value(j).map_err(|e| format!("Ungültige Einstellungen: {e}"))
}

// ---------- Ausgaben ----------

fn fixture_json(show: &Show, id: FixtureId) -> Value {
    let Some(f) = show.fixture(id) else {
        return Value::Null;
    };
    let ty = show.type_of(f);
    json!({
        "id": f.id.0,
        "name": f.name,
        "type_key": f.type_key,
        "model": ty.map(|t| format!("{} {} ({})", t.manufacturer, t.model, t.mode)),
        "universe": f.universe + 1,
        "address": f.address,
        "channels": ty.map(|t| t.channels.iter().enumerate().map(|(i, c)| json!({ "index": i, "name": c.name, "kind": format!("{:?}", c.kind) })).collect::<Vec<_>>()),
        "plan": show.plan_position(f.id),
    })
}

fn scene_kind(s: &dmxcraft_core::Scene) -> &'static str {
    if s.timeline().is_some() {
        "SUPER SCENE"
    } else if !s.effects.is_empty() {
        "FX"
    } else if s.steps.len() > 1 {
        "ABLAUF"
    } else {
        "STATISCH"
    }
}

fn scene_brief(e: &Engine, s: &dmxcraft_core::Scene) -> Value {
    json!({
        "id": s.id.0,
        "name": s.name,
        "bank": s.zone.map(|z| z.0),
        "kind": scene_kind(s),
        "duration_ms": s.duration_ms(),
        "steps": s.steps.len(),
        "effects": s.effects.iter().map(|f| kind_name(f.kind)).collect::<Vec<_>>(),
        "running": e.is_active(s.id),
        "hidden": s.hidden,
    })
}

fn overview(e: &Engine) -> R {
    let show = e.show();
    Ok(json!({
        "show": show.name,
        "universes": show.universes,
        "fixtures": show.fixtures.len(),
        "banks": show.zones.iter().map(|z| json!({
            "id": z.id.0, "name": z.name, "hidden": z.hidden,
            "scenes": show.scenes.iter().filter(|s| s.zone == Some(z.id)).map(|s| json!({ "id": s.id.0, "name": s.name, "kind": scene_kind(s) })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "groups": show.groups.iter().map(|g| json!({ "id": g.id.0, "name": g.name, "fixtures": g.fixtures.iter().map(|f| f.0).collect::<Vec<_>>() })).collect::<Vec<_>>(),
        "running": e.active_scenes().iter().map(|s| s.0).collect::<Vec<_>>(),
        "master": e.master(),
        "blackout": e.blackout(),
        "speed": e.speed(),
    }))
}

fn list_scenes(e: &Engine, a: &Value) -> R {
    let bank = opt_u64(a, "bank").map(|b| ZoneId(b as u32));
    Ok(json!({
        "scenes": e.show().scenes.iter()
            .filter(|s| bank.is_none_or(|b| s.zone == Some(b)))
            .map(|s| scene_brief(e, s))
            .collect::<Vec<_>>()
    }))
}

fn get_scene(e: &Engine, id: SceneId) -> R {
    let show = e.show();
    let s = show.scene(id).ok_or("Szene nicht gefunden")?;
    let ch_name = |v: &ChannelValue| {
        show.channel_def(v.channel_ref())
            .map(|c| (c.name.clone(), format!("{:?}", c.kind)))
            .unwrap_or_default()
    };
    let mut out = scene_brief(e, s);
    out["color"] = s
        .color
        .map(|c| json!(format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])))
        .unwrap_or(Value::Null);
    out["loop"] = json!(s.loop_mode == LoopMode::Loop);
    out["settings"] = serde_json::to_value(&s.settings).unwrap_or_default();
    out["step_list"] = json!(s.steps.iter().map(|st| json!({
        "hold_ms": st.hold_ms,
        "fade_ms": st.fade_ms,
        "values": st.values.iter().map(|v| {
            let (n, k) = ch_name(v);
            json!({ "fixture": v.fixture.0, "channel": v.channel, "name": n, "kind": k, "value": v.value })
        }).collect::<Vec<_>>(),
    })).collect::<Vec<_>>());
    out["effect_list"] = json!(s
        .effects
        .iter()
        .filter(|f| f.kind != EffectKind::SuperScene && f.kind != EffectKind::Steps)
        .map(|f| {
            let mut j = serde_json::to_value(f).unwrap_or_default();
            j["kind"] = json!(kind_name(f.kind));
            j
        })
        .collect::<Vec<_>>());
    if let Some(tl) = s.timeline() {
        out["timeline"] = timeline_json(show, tl);
    }
    Ok(out)
}

fn timeline_json(show: &Show, tl: &Timeline) -> Value {
    json!({
        "length_ms": tl.length_ms(),
        "bpm": tl.bpm,
        "tracks": tl.tracks.iter().enumerate().map(|(i, t)| json!({
            "index": i, "name": t.name, "muted": t.muted,
            "clips": t.clips.iter().map(|c| {
                let content = match &c.content {
                    ClipContent::Scene(id) => json!({ "scene": id.0, "scene_name": show.scene(*id).map(|s| s.name.clone()) }),
                    ClipContent::Audio { path, .. } => json!({ "audio_path": path }),
                };
                json!({
                    "id": c.id, "start_ms": c.start_ms, "duration_ms": c.duration_ms,
                    "fade_in_ms": c.fade_in_ms, "fade_out_ms": c.fade_out_ms,
                    "level": c.level, "speed": c.speed, "content": content,
                })
            }).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

fn dmx_output(e: &Engine, a: &Value) -> R {
    let frame = e.render_frame();
    if let Ok(list) = id_list(a, "fixtures") {
        let show = e.show();
        return Ok(
            json!({ "fixtures": existing_fixtures(show, &list)?.into_iter().map(|id| {
            let f = show.fixture(id);
            let n = show.fixture_channels(id);
            json!({
                "id": id.0,
                "name": f.map(|f| f.name.clone()),
                "values": (0..n).map(|c| e.output_value(&frame, ChannelRef { fixture: id, channel: c })).collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>() }),
        );
    }
    let u = opt_u64(a, "universe").unwrap_or(1).max(1) as usize - 1;
    let start = opt_u64(a, "start")
        .unwrap_or(1)
        .clamp(1, UNIVERSE_SIZE as u64) as usize;
    let count = opt_u64(a, "count").unwrap_or(UNIVERSE_SIZE as u64) as usize;
    let data = frame.universe(u).ok_or("Universum nicht vorhanden")?;
    let end = (start - 1 + count).min(UNIVERSE_SIZE);
    Ok(json!({ "universe": u + 1, "start": start, "values": data[start - 1..end].to_vec() }))
}

// ---------- Geräte ----------

/// Gerätetyp aus Show, eingebauten Typen oder Bibliothek.
fn find_type(show: &Show, key: &str, user: &[FixtureType]) -> Option<FixtureType> {
    if let Some(t) = show.fixture_type(key) {
        return Some(t.clone());
    }
    if let Some(t) = user.iter().find(|t| t.key == key) {
        return Some(t.clone());
    }
    if let Some(t) = dmxcraft_core::library::builtin_fixture_types()
        .into_iter()
        .find(|t| t.key == key)
    {
        return Some(t);
    }
    dmxcraft_ofl::Library::embedded()
        .fixtures
        .iter()
        .flat_map(|f| f.modes.iter())
        .find(|m| m.key == key)
        .cloned()
}

fn search_library(a: &Value, user: &[FixtureType]) -> R {
    let q = req_str(a, "query")?;
    let limit = opt_u64(a, "limit").unwrap_or(15).clamp(1, 50) as usize;
    let words: Vec<String> = q.split_whitespace().map(str::to_lowercase).collect();
    let mode_json = |m: &FixtureType| json!({ "type_key": m.key, "mode": m.mode, "channels": m.channels.len(), "kinds": m.channels.iter().map(|c| format!("{:?}", c.kind)).collect::<Vec<_>>() });
    let mut hits: Vec<Value> = dmxcraft_core::library::builtin_fixture_types()
        .iter()
        .filter(|t| {
            let hay = format!("{} {} {} generic generisch", t.manufacturer, t.model, t.mode).to_lowercase();
            words.iter().all(|w| hay.contains(w))
        })
        .map(|t| json!({ "manufacturer": t.manufacturer, "model": t.model, "modes": [mode_json(t)] }))
        .collect();
    // eigene Profile (Profile Builder) zuerst
    let mut own: Vec<Value> = Vec::new();
    for t in user {
        let hay = format!("{} {} {} eigene own", t.manufacturer, t.model, t.mode).to_lowercase();
        if words.iter().all(|w| hay.contains(w)) {
            own.push(json!({ "manufacturer": t.manufacturer, "model": t.model, "own_profile": true, "modes": [mode_json(t)] }));
        }
    }
    own.append(&mut hits);
    let mut hits = own;
    for f in dmxcraft_ofl::Library::embedded().search(&q, limit) {
        hits.push(json!({
            "manufacturer": f.manufacturer,
            "model": f.name,
            "categories": f.categories,
            "modes": f.modes.iter().map(mode_json).collect::<Vec<_>>(),
        }));
    }
    hits.truncate(limit);
    Ok(json!({ "results": hits }))
}

fn add_fixtures(e: &mut Engine, a: &Value, user: &[FixtureType]) -> R {
    let key = req_str(a, "type_key")?;
    let ty = find_type(e.show(), &key, user).ok_or(format!(
        "Gerätetyp „{key}“ nicht gefunden – erst search_fixture_library nutzen"
    ))?;
    let count = opt_u64(a, "count").unwrap_or(1).clamp(1, 512) as usize;
    let base = opt_str(a, "name").unwrap_or_else(|| ty.model.clone());
    let universe = opt_u64(a, "universe").unwrap_or(1).max(1) as u8 - 1;
    let address = opt_u64(a, "address").unwrap_or(1).clamp(1, 512) as u16;
    e.show_mut().upsert_fixture_type(ty.clone());
    // Vorlage: Namen „Basis 1“ … – paste_fixtures zählt weiter und sucht freie Adressen
    let n0 = (1..)
        .find(|n| {
            !e.show()
                .fixtures
                .iter()
                .any(|f| f.name == format!("{base} {n}"))
        })
        .unwrap_or(1);
    let templates: Vec<dmxcraft_core::Fixture> = (0..count)
        .map(|i| dmxcraft_core::Fixture {
            id: FixtureId(0),
            name: format!("{base} {}", n0 + i),
            type_key: key.clone(),
            universe,
            address,
            plan_pos: None,
            plan_shape: None,
            plan_size: None,
            plan_rotation: None,
            limits: Default::default(),
            place3d: None,
        })
        .collect();
    let ids = e.show_mut().paste_fixtures(&templates, universe, address);
    if ids.is_empty() {
        return Err("Kein Platz im Patch".into());
    }
    let show = e.show();
    Ok(json!({
        "added": ids.iter().map(|id| fixture_json(show, *id)).collect::<Vec<_>>(),
        "missing": count - ids.len(),
    }))
}

fn update_fixture(e: &mut Engine, a: &Value) -> R {
    let id = FixtureId(req_u32(a, "id")?);
    let f = e.show().fixture(id).ok_or("Gerät nicht gefunden")?.clone();
    if a.get("universe").is_some() || a.get("address").is_some() {
        let u = opt_u64(a, "universe").map_or(f.universe, |u| u.max(1) as u8 - 1);
        let ad = opt_u64(a, "address").map_or(f.address, |v| v as u16);
        e.show_mut()
            .repatch_fixture(id, u, ad)
            .map_err(|err| err.to_string())?;
    }
    let fm = e.show_mut().fixture_mut(id).ok_or("Gerät nicht gefunden")?;
    if let Some(n) = opt_str(a, "name") {
        fm.name = n;
    }
    let x = a.get("plan_x").and_then(Value::as_f64);
    let y = a.get("plan_y").and_then(Value::as_f64);
    if x.is_some() || y.is_some() {
        let cur = fm.plan_pos.unwrap_or([0.5, 0.5]);
        fm.plan_pos = Some([
            x.map_or(cur[0], |v| v.clamp(0.0, 1.0) as f32),
            y.map_or(cur[1], |v| v.clamp(0.0, 1.0) as f32),
        ]);
    }
    Ok(fixture_json(e.show(), id))
}

// ---------- Szenen ----------

fn create_scene(host: &Host, e: &mut Engine, a: &Value) -> R {
    let bank = ZoneId(req_u32(a, "bank")?);
    e.show().zone(bank).ok_or("Bank nicht gefunden")?;
    let steps = build_steps(e.show(), a)?;
    let multi = steps.len() > 1;
    let id = e
        .show_mut()
        .add_scene(req_str(a, "name")?, Some(bank), steps);
    apply_scene_fields(e, id, a)?;
    if multi {
        // Ablauf mit mehreren Schritten = STEPS-Effekt (wie in der Oberfläche)
        let s = e.show_mut().scene_mut(id).ok_or("Szene fehlt")?;
        if !s.has_steps() {
            s.add_effect(EffectKind::Steps);
        }
    }
    if a.get("select").and_then(Value::as_bool).unwrap_or(true) {
        (host.notify)(AppEvent::SelectScene(id));
    }
    get_scene(e, id)
}

fn apply_scene_fields(e: &mut Engine, id: SceneId, a: &Value) -> Result<(), String> {
    let s = e.show_mut().scene_mut(id).ok_or("Szene nicht gefunden")?;
    if let Some(n) = opt_str(a, "name") {
        s.name = n;
    }
    if let Some(c) = a.get("color") {
        s.color = parse_color_opt(c)?;
    }
    if let Some(h) = a.get("hidden").and_then(Value::as_bool) {
        s.hidden = h;
    }
    if let Some(l) = a.get("loop").and_then(Value::as_bool) {
        s.loop_mode = if l { LoopMode::Loop } else { LoopMode::Once };
    }
    if let Some(p) = a.get("settings").filter(|v| v.is_object()) {
        s.settings = merged(&s.settings, p)?;
    }
    Ok(())
}

fn update_scene(e: &mut Engine, a: &Value) -> R {
    let id = SceneId(req_u32(a, "id")?);
    e.show().scene(id).ok_or("Szene nicht gefunden")?;
    if a.get("steps").is_some() {
        if e.show().scene(id).is_some_and(|s| s.timeline().is_some()) {
            return Err(
                "Eine SUPER SCENE hat keine Schritte – Blöcke über add_timeline_clips ändern"
                    .into(),
            );
        }
        let steps = build_steps(e.show(), a)?;
        let multi = steps.len() > 1;
        let s = e.show_mut().scene_mut(id).ok_or("Szene fehlt")?;
        s.steps = steps;
        if multi && !s.has_steps() && s.effects.is_empty() {
            s.add_effect(EffectKind::Steps);
        }
        if !multi {
            s.effects.retain(|f| f.kind != EffectKind::Steps);
        }
    }
    if let Some(b) = opt_u64(a, "bank") {
        let z = ZoneId(b as u32);
        e.show().zone(z).ok_or("Bank nicht gefunden")?;
        e.show_mut().move_scenes(&[id], z, None);
    }
    apply_scene_fields(e, id, a)?;
    e.sync();
    get_scene(e, id)
}

fn add_effect(e: &mut Engine, a: &Value) -> R {
    let sid = SceneId(req_u32(a, "scene")?);
    let kind = parse_kind(&req_str(a, "kind")?)?;
    if kind.exclusive() {
        return Err("Steps bzw. SuperScene über create_scene/create_super_scene anlegen".into());
    }
    let fixtures = existing_fixtures(e.show(), &id_list(a, "fixtures")?)?;
    let s = e.show_mut().scene_mut(sid).ok_or("Szene nicht gefunden")?;
    if s.effects.iter().any(|f| f.kind.exclusive()) {
        return Err("Diese Szene hat STEPS bzw. ist eine SUPER SCENE – dort sind keine weiteren Effekte möglich".into());
    }
    if s.steps.is_empty() {
        s.steps.push(Step::new(Vec::new()));
    }
    let fid = s.add_effect(kind);
    let fx = s
        .effects
        .iter_mut()
        .find(|f| f.id == fid)
        .ok_or("Effekt fehlt")?;
    fx.fixtures = fixtures;
    if let Some(p) = a.get("params").filter(|v| v.is_object()) {
        let mut p = p.clone();
        if let Some(o) = p.as_object_mut() {
            o.remove("id");
            o.remove("kind");
        }
        *fx = merged(fx, &p)?;
    }
    get_scene(e, sid)
}

fn update_effect(e: &mut Engine, a: &Value) -> R {
    let sid = SceneId(req_u32(a, "scene")?);
    let fid = req_u32(a, "effect")?;
    let fixtures = match a.get("fixtures") {
        Some(_) => Some(existing_fixtures(e.show(), &id_list(a, "fixtures")?)?),
        None => None,
    };
    let s = e.show_mut().scene_mut(sid).ok_or("Szene nicht gefunden")?;
    let fx = s
        .effects
        .iter_mut()
        .find(|f| f.id == fid)
        .ok_or("Effekt nicht gefunden")?;
    if let Some(f) = fixtures {
        fx.fixtures = f;
    }
    if let Some(p) = a.get("params").filter(|v| v.is_object()) {
        let mut p = p.clone();
        if let Some(o) = p.as_object_mut() {
            o.remove("id");
            o.remove("kind");
        }
        *fx = merged(fx, &p)?;
    }
    get_scene(e, sid)
}

// ---------- SUPER SCENE ----------

fn create_super_scene(host: &Host, e: &mut Engine, a: &Value) -> R {
    let bank = ZoneId(req_u32(a, "bank")?);
    e.show().zone(bank).ok_or("Bank nicht gefunden")?;
    let id = e
        .show_mut()
        .add_scene(req_str(a, "name")?, Some(bank), vec![Step::new(Vec::new())]);
    {
        let s = e.show_mut().scene_mut(id).ok_or("Szene fehlt")?;
        s.add_effect(EffectKind::SuperScene);
        let tl = s.timeline_mut().ok_or("Zeitleiste fehlt")?;
        if let Some(b) = a.get("bpm").and_then(Value::as_f64) {
            tl.bpm = b.clamp(20.0, 400.0) as f32;
        }
        if let Some(names) = a.get("track_names").and_then(Value::as_array) {
            for (i, n) in names.iter().enumerate() {
                while tl.tracks.len() <= i {
                    tl.add_track(None);
                }
                tl.tracks[i].name = n.as_str().unwrap_or("").to_string();
            }
        }
        s.sync_timeline_length();
    }
    if let Some(clips) = a.get("clips").and_then(Value::as_array) {
        add_clips(host, e, id, &clips.clone())?;
    }
    (host.notify)(AppEvent::SelectScene(id));
    get_scene(e, id)
}

fn add_clips(
    host: &Host,
    e: &mut Engine,
    sid: SceneId,
    clips: &[Value],
) -> Result<Vec<u32>, String> {
    // erst alles prüfen, dann einfügen
    let mut parts = Vec::new();
    for c in clips {
        let track = opt_u64(c, "track").unwrap_or(0) as usize;
        let start = opt_u64(c, "start_ms").ok_or("`start_ms` fehlt")?;
        if let Some(scene) = opt_u64(c, "scene") {
            let sc = e
                .show()
                .scene(SceneId(scene as u32))
                .ok_or(format!("Szene {scene} nicht gefunden"))?;
            if sc.id == sid {
                return Err("Eine SUPER SCENE kann sich nicht selbst enthalten".into());
            }
            parts.push((track, start, Some(sc.clone()), None::<String>, c));
        } else if let Some(p) = opt_str(c, "audio_path") {
            if !std::path::Path::new(&p).exists() {
                return Err(format!("Audiodatei nicht gefunden: {p}"));
            }
            parts.push((track, start, None, Some(p), c));
        } else {
            return Err("Block braucht `scene` oder `audio_path`".into());
        }
    }
    let durations: Vec<Option<u64>> = parts
        .iter()
        .map(|(_, _, _, audio, _)| audio.as_deref().and_then(|p| (host.audio_duration_ms)(p)))
        .collect();
    let s = e.show_mut().scene_mut(sid).ok_or("Szene nicht gefunden")?;
    let tl = s.timeline_mut().ok_or("Keine SUPER SCENE")?;
    let mut ids = Vec::new();
    for ((track, start, scene, audio, c), len) in parts.into_iter().zip(durations) {
        let id = match (scene, audio) {
            (Some(sc), _) => tl.add_scene_clip(track, &sc, start),
            (None, Some(path)) => {
                let length = len.unwrap_or(0);
                let dur = opt_u64(c, "duration_ms").or(len).unwrap_or(10_000);
                tl.add_clip(
                    track,
                    ClipContent::Audio {
                        path,
                        offset_ms: 0,
                        length_ms: length,
                        grid: None,
                        segment_grids: Vec::new(),
                        triggers: Vec::new(),
                    },
                    start,
                    dur,
                )
            }
            _ => continue,
        };
        if let Some(cl) = tl.clip_mut(id) {
            if let Some(d) = opt_u64(c, "duration_ms") {
                cl.duration_ms = d.max(40);
            }
            if let Some(v) = opt_u64(c, "fade_in_ms") {
                cl.fade_in_ms = v as u32;
            }
            if let Some(v) = opt_u64(c, "fade_out_ms") {
                cl.fade_out_ms = v as u32;
            }
            if let Some(v) = c.get("level").and_then(Value::as_f64) {
                cl.level = v.clamp(0.0, 1.0) as f32;
            }
            if let Some(v) = c.get("speed").and_then(Value::as_f64) {
                cl.speed = v.clamp(0.05, 20.0) as f32;
            }
        }
        ids.push(id);
    }
    s.sync_timeline_length();
    Ok(ids)
}

/// Base64 (Standard-Alphabet, mit Auffüllung) für Bilder im MCP-Ergebnis.
pub(crate) fn base64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for c in data.chunks(3) {
        let b = [c[0], *c.get(1).unwrap_or(&0), *c.get(2).unwrap_or(&0)];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for i in 0..4 {
            if i <= c.len() {
                out.push(T[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {

    #[test]
    fn base64_matches() {
        assert_eq!(super::base64(b""), "");
        assert_eq!(super::base64(b"f"), "Zg==");
        assert_eq!(super::base64(b"fo"), "Zm8=");
        assert_eq!(super::base64(b"foo"), "Zm9v");
        assert_eq!(super::base64(b"foobar"), "Zm9vYmFy");
    }

    use super::*;

    fn host() -> Host {
        let mut show = Show::new("Test", 1);
        for t in dmxcraft_core::library::builtin_fixture_types() {
            show.upsert_fixture_type(t);
        }
        Host::headless(Engine::new(show))
    }

    fn ok(h: &Host, name: &str, a: Value) -> Value {
        call(h, name, &a).unwrap_or_else(|e| panic!("{name}: {e}"))
    }

    #[test]
    fn full_workflow() {
        let h = host();
        let added = ok(
            &h,
            "add_fixtures",
            json!({"type_key":"generic/rgbw-par-6ch","count":4,"name":"PAR"}),
        );
        let list = added["added"].as_array().unwrap();
        assert_eq!(list.len(), 4);
        assert_eq!(list[0]["name"], "PAR 1");
        assert_eq!(list[1]["address"], 7);
        let ids: Vec<u64> = list.iter().map(|f| f["id"].as_u64().unwrap()).collect();

        let bank = ok(&h, "create_bank", json!({"name":"Farben"}))["bank"]
            .as_u64()
            .unwrap();
        let red = ok(
            &h,
            "create_scene",
            json!({
                "name":"Rot","bank":bank,
                "steps":[{"fixtures":ids,"set":{"Dimmer":255,"color":"#ff0000"}}],
                "settings":{"fade_in_ms":500}
            }),
        );
        assert_eq!(red["kind"], "STATISCH");
        assert_eq!(red["settings"]["fade_in_ms"], 500);
        let red_id = red["id"].as_u64().unwrap();
        let vals = red["step_list"][0]["values"].as_array().unwrap();
        assert!(vals.iter().any(|v| v["kind"] == "Red" && v["value"] == 255));
        assert!(vals.iter().any(|v| v["kind"] == "Green" && v["value"] == 0));

        // Ablauf mit zwei Schritten
        let chase = ok(
            &h,
            "create_scene",
            json!({
                "name":"Wechsel","bank":bank,
                "steps":[
                    {"fixtures":ids,"set":{"Dimmer":255,"color":"#0000ff"},"hold_ms":500},
                    {"values":[{"fixture":ids[0],"channel":"Dimmer","value":0}],"fade_ms":200}
                ]
            }),
        );
        assert_eq!(chase["steps"], 2);
        assert_eq!(chase["effects"][0], "Steps");

        // Effekt mit Parametern
        let tpl = ok(&h, "describe_effect", json!({"kind":"ColorFx"}));
        assert!(tpl["template"]["color"].is_object());
        let fx = ok(
            &h,
            "add_effect",
            json!({"scene":red_id,"kind":"color fx","fixtures":ids}),
        );
        assert_eq!(fx["kind"], "FX");
        let eid = fx["effect_list"][0]["id"].as_u64().unwrap();
        ok(
            &h,
            "update_effect",
            json!({"scene":red_id,"effect":eid,"fixtures":[ids[0], ids[1]]}),
        );

        // SUPER SCENE mit zwei Blöcken
        let sup = ok(
            &h,
            "create_super_scene",
            json!({
                "name":"Show","bank":bank,
                "clips":[{"scene":red_id,"start_ms":0,"duration_ms":2000},{"track":1,"scene":chase["id"],"start_ms":1000}]
            }),
        );
        assert_eq!(sup["kind"], "SUPER SCENE");
        assert_eq!(sup["timeline"]["tracks"][1]["clips"][0]["start_ms"], 1000);
        assert_eq!(sup["timeline"]["tracks"][0]["clips"][0]["fade_in_ms"], 500);
        let sup_id = sup["id"].as_u64().unwrap();
        assert!(call(
            &h,
            "add_timeline_clips",
            &json!({"scene":sup_id,"clips":[{"scene":sup_id,"start_ms":0}]})
        )
        .is_err());

        // Wiedergabe und Ausgang
        ok(&h, "play_scene", json!({"id":red_id}));
        h.engine
            .lock()
            .unwrap()
            .tick(std::time::Duration::from_millis(600));
        let out = ok(&h, "get_dmx_output", json!({"fixtures":[ids[0]]}));
        assert!(out["fixtures"][0]["values"].as_array().unwrap().len() == 6);
        ok(&h, "stop_scene", json!({}));
        ok(
            &h,
            "set_live_values",
            json!({"fixtures":[ids[0]],"set":{"Dimmer":128}}),
        );
        assert_eq!(h.engine.lock().unwrap().live().len(), 1);

        // Überblick und Aufräumen
        let ov = ok(&h, "get_overview", json!({}));
        assert_eq!(ov["fixtures"], 4);
        ok(&h, "delete_scenes", json!({"ids":[chase["id"]]}));
        let s = ok(&h, "get_scene", json!({"id":sup_id}));
        assert_eq!(
            s["timeline"]["tracks"][1]["clips"]
                .as_array()
                .unwrap()
                .len(),
            0
        );
        ok(&h, "delete_fixtures", json!({"ids":[ids[3]]}));
        assert_eq!(
            ok(&h, "list_fixtures", json!({}))["fixtures"]
                .as_array()
                .unwrap()
                .len(),
            3
        );
    }

    #[test]
    fn helpful_errors() {
        let h = host();
        assert!(call(
            &h,
            "create_scene",
            &json!({"name":"x","bank":99,"steps":[{}]})
        )
        .unwrap_err()
        .contains("Bank"));
        assert!(
            call(&h, "add_fixtures", &json!({"type_key":"gibt/es/nicht"}))
                .unwrap_err()
                .contains("search_fixture_library")
        );
        let r = call(&h, "search_fixture_library", &json!({"query":"rgbw"})).unwrap();
        assert!(!r["results"].as_array().unwrap().is_empty());
    }
}
