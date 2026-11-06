# DmxCraft

Eine freie, in Rust geschriebene DMX-Lichtsteuerung nach dem Vorbild von
Daslight 5. Sie läuft auf Linux, Windows und macOS und soll später die
vorhandene **Daslight DVC Gold** per USB ansteuern.

> Unabhängiges Projekt, nicht mit Nicolaudie/Daslight verbunden. Es wird
> kein Code, keine Gerätebibliothek und kein Grafik-Asset von Daslight
> verwendet.

## Stand (v0.1)

| Bereich | Status |
|---|---|
| Patch (Geräte, Universen, Adressen, Kollisionsprüfung) | ✅ |
| Eingebaute generische Gerätetypen (Dimmer, RGB, RGBW, Moving Head) | ✅ |
| Szenen mit Schritten, Überblend- und Haltezeit, Schleife/Einmal | ✅ |
| Zonen (Szenen einer Zone schließen sich aus) | ✅ |
| Merge: HTP für Dimmer, LTP für alles andere | ✅ |
| Programmer (Live-Fader, Farbwähler) → als Szene/Schritt speichern | ✅ |
| Master, Blackout, Speed | ✅ |
| DMX-Monitor (512 Kanäle pro Universum) | ✅ |
| Show-Datei (JSON) laden/speichern | ✅ |
| Ausgabe-Thread mit 40 Hz, automatischer Reconnect | ✅ |
| **USB-Ausgang DVC Gold** | ⏳ Platzhalter – Protokoll wird noch entschlüsselt |
| Gerätebibliothek: 650+ Geräte aus der Open Fixture Library, eigene OFL-Dateien importieren | ✅ |
| Installer für Linux/macOS/Windows, Auto-Update | ✅ |
| Effekt-Generator, 3D-Ansicht, MIDI | ❌ geplant |

## Installieren

**Linux / macOS** (Terminal):

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/JustinSchweiger/DmxCraft/releases/latest/download/dmxcraft-installer.sh | sh
```

**Windows**: Unter [Releases](https://github.com/JustinSchweiger/DmxCraft/releases/latest) die
`dmxcraft-x86_64-pc-windows-msvc.msi` herunterladen und ausführen. Dabei werden auch eine
Startmenü- und eine Desktop-Verknüpfung angelegt. Alternativ in PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/JustinSchweiger/DmxCraft/releases/latest/download/dmxcraft-installer.ps1 | iex"
```

Gebaut wird für Linux (x86_64, ARM64 z. B. Raspberry Pi), macOS (Apple Silicon und Intel)
und Windows (x86_64). Unter Linux legt DmxCraft beim ersten Start einen Eintrag im
Anwendungsmenü an.

**Updates**: DmxCraft prüft beim Start, ob es eine neue Version gibt. Ist eine da, erscheint
oben der Button *Update installieren*. Alternativ geht es im Terminal mit `dmxcraft-update`.

## Neue Version veröffentlichen

1. Version in `Cargo.toml` (`[workspace.package] version`) erhöhen und committen.
2. Tag setzen und pushen: `git tag v0.2.0 && git push origin v0.2.0`
3. GitHub Actions (`.github/workflows/release.yml`, erzeugt mit
   [dist](https://opensource.axo.dev/cargo-dist/)) baut alle Plattformen und legt das
   Release mit den Installern an. Installierte Programme bieten das Update dann automatisch an.

Die Gerätebibliothek aktualisieren (Open Fixture Library):

```sh
git clone --depth 1 https://github.com/OpenLightingProject/open-fixture-library /tmp/ofl
cargo run --release -p dmxcraft-ofl --bin ofl-bundle -- /tmp/ofl crates/ofl/data/ofl-library.json.gz
```

## Bauen und starten

Voraussetzung: Rust ≥ 1.85 (`rustup`). Unter Linux werden zur Laufzeit die
üblichen Grafikbibliotheken gebraucht (OpenGL/Mesa sowie X11 oder Wayland).
Die sind auf Desktop-Systemen normalerweise schon installiert.

```sh
cargo run --release -p dmxcraft            # startet mit Demo-Show
cargo run --release -p dmxcraft -- show.json  # lädt eine Show
cargo test --workspace                          # Tests
```

Ohne Gerät wählst du oben „Simulation (kein Gerät)“. Die Engine läuft dann
trotzdem, und im Tab **DMX-Monitor** siehst du, was an die Box gehen würde.

## Oberfläche

Die Oberfläche hat zwei Bereiche (oben links umschalten):

* **STEUERN** – oben die Zonen als Spalten mit Szenenkarten (Klick = an/aus, Rechtsklick =
  bearbeiten/löschen). Jede Spalte hat Tasten für Pause sowie vorherigen/nächsten Schritt und „+“
  (neue Szene aus dem Programmer). Darüber filtert die Seitenleiste nach Zonen.
  Unten links liegt die **Bühne** mit Gruppen: Geräte per Klick, Strg/Shift+Klick oder
  Aufziehen auswählen. Unten rechts ist der **Programmer** mit den Kategorien Dimmer, Farbe
  (mit Farbfeldern), Position (mit XY-Feld), Gobo, Strahl und Sonstiges. Fader: ziehen,
  Mausrad (Shift = 10er-Schritte), Doppelklick = 0/voll, Rechtsklick = Freigeben.
* **EINRICHTEN** – Patch und Gerätebibliothek, **Bühnenplan** (Geräte verschieben, Einrasten,
  in Reihe/Raster anordnen, ausrichten), Szenen-Editor, Zonen (Farbe, Reihenfolge) & Gruppen,
  **DMX-Adressen** (Modus *Patch*: Geräte als Balken über ihren Adressbereich, per Ziehen auf
  eine neue Startadresse verschieben, freie Adressen sichtbar; Modus *Ausgang*: gesendete
  Werte; Seitenleiste mit DIP-Schalter-Stellung des ausgewählten Geräts).
* **Rechtsklick auf ein Gerät** (Bühne, Bühnenplan, DMX-Adressen): Gruppen zuweisen/entfernen,
  neue Gruppe, gleichen Typ auswählen, Reihenfolge umkehren, Layout kopieren/einfügen,
  spiegeln, nächste freie Adresse, löschen (mit Rückfrage).

## Bedienung in Kürze

1. **Patch** (links): Gerätetyp wählen, Anzahl, Universum, Startadresse →
   *Hinzufügen*. Mit den Kästchen wählst du Geräte für den Programmer aus.
2. **Programmer**: Fader bewegen. Der Inhalt lässt sich als *neue Szene*
   speichern oder als *Schritt* an eine bestehende Szene anhängen.
3. **Szenen**: Name, Zone, Ablauf sowie die Zeiten jedes Schritts
   bearbeiten. Schritte kannst du in den Programmer laden und überschreiben.
4. **Live**: Szenen-Buttons nach Zone. Ein Klick schaltet die Szene ein
   oder aus.

## Aufbau

```
crates/
  core/    Datenmodell + Engine, ohne I/O, vollständig getestet
           show.rs    Patch, Zonen, Szenen, JSON
           engine.rs  Wiedergabe, Fades, HTP/LTP, Master, Programmer
  output/  DmxOutput-Trait, Ausgabe-Thread (runtime.rs), Backends
           dvc_gold.rs  ← hier kommt die USB-Anbindung hinein
  ofl/     Open-Fixture-Library-Konverter + eingebettete Bibliothek (data/)
  app/     Oberfläche mit egui/eframe, Updater
           ui/      Design-System: theme.rs (Tokens) + widgets/ (Fader, SceneCard, StageCanvas, …)
           views/   Ansichten (je ein Typ mit `impl View`): control/ und setup/
           state.rs gemeinsamer UI-Zustand
docs/UI-GUIDELINES.md   UI-Richtlinien
graphify-out/           Code-Graph für KI-Assistenten (`graphify update .`)
```

Die USB-Anbindung ist genau ein Backend: Man implementiert `DmxOutput`
(`open` = verbinden + Handshake, `send` = Frame senden) in
`crates/output/src/dvc_gold.rs`. Engine und Oberfläche bleiben dabei
unverändert. Die Schritte dafür stehen als Kommentar in der Datei.

## Show-Format

JSON, Version 1. Die verwendeten Gerätetypen werden in jede Show kopiert,
damit die Datei auf jedem Rechner identisch lädt. Universen sind in der Datei
0-basiert, DMX-Adressen 1-basiert.
