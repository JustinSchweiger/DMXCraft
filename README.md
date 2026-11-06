# DMXCraft

Eine freie, in Rust geschriebene DMX-Lichtsteuerung nach dem Vorbild von
Daslight 5. Sie läuft auf Linux, Windows und macOS und gibt DMX über
günstige USB-DMX-Kabel (Open DMX / FTDI) aus – ein Kabel je Universum.

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
| **USB-DMX-Kabel (Open DMX / FTDI)**, ein Kabel je Universum | ✅ |
| Daslight DVC Gold | ❌ verschlüsselt, nicht ansprechbar (siehe `dvc_gold.rs`) |
| Gerätebibliothek: 650+ Geräte aus der Open Fixture Library, eigene OFL-Dateien importieren | ✅ |
| Installer für Linux/macOS/Windows, Auto-Update | ✅ |
| Effekt-Generator, 3D-Ansicht, MIDI | ❌ geplant |

## Installieren

**Linux / macOS** (Terminal):

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/JustinSchweiger/DMXCraft/releases/latest/download/dmxcraft-installer.sh | sh
```

**Windows**: Unter [Releases](https://github.com/JustinSchweiger/DMXCraft/releases/latest) die
`dmxcraft-x86_64-pc-windows-msvc.msi` herunterladen und ausführen. Dabei werden auch eine
Startmenü- und eine Desktop-Verknüpfung angelegt. Alternativ in PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/JustinSchweiger/DMXCraft/releases/latest/download/dmxcraft-installer.ps1 | iex"
```

Gebaut wird für Linux (x86_64, ARM64 z. B. Raspberry Pi), macOS (Apple Silicon und Intel)
und Windows (x86_64). Unter Linux legt DMXCraft beim ersten Start einen Eintrag im
Anwendungsmenü an.

**Updates**: DMXCraft prüft beim Start, ob es eine neue Version gibt. Ist eine da, erscheint
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

DMXCraft hat eine eigene Titelleiste: links das Menü und der Umschalter **Einrichten/Steuern**,
in der Mitte Programm- und Showname, rechts die Auslastungsleiste (Ausgangsstatus und Details im
Tooltip), das Schloss und die Fensterknöpfe. Ziehen an der Leiste verschiebt das Fenster,
Doppelklick maximiert.

| Menü | Einträge |
|---|---|
| Datei | Neu (Strg+N), Öffnen (Strg+O), Zuletzt geöffnet, Speichern (Strg+S), Speichern unter (Strg+Umschalt+S), Importieren von Daslight *(folgt)* |
| Bearbeiten | Rückgängig (Strg+Z), Wiederherstellen (Strg+Y), Sperren (Strg+L), Einstellungen (Strg+,) |
| Ansicht | Vollbild (F11), Größer (Strg++), Kleiner (Strg+-), Größe wiederherstellen (Strg+0) |
| Tools | 3D-Visualisierer *(folgt)*, DMX-Werte (Strg+D, Patch-Raster im Modus „Ausgang“), Profile Builder *(folgt)* |
| Mappings | Tastatur, MIDI, Touch *(folgen)* |
| Hilfe | Nach Updates suchen, Über DMXCraft |

Außerdem: B schaltet Blackout. Unter macOS steht Cmd statt Strg. Zonen und Gruppen bearbeitet man
über das Stift-Symbol neben den Zonen-Chips in STEUERN.

Das **Schloss** sperrt die Oberfläche mit einem Passwort (beim ersten Mal festlegen, mindestens
4 Zeichen; gespeichert wird nur ein gesalzener SHA-256-Hash). Gesperrt bleibt nur STEUERN:
Patch, Szenen und Einstellungen lassen sich nicht ändern. Die Sperre bleibt nach einem Neustart
bestehen; das Passwort lässt sich entsperrt unter Einstellungen → Sperre entfernen.

Eine Show hat fest **8 Universen** à 512 Kanäle.

Die Oberfläche hat zwei Bereiche:

* **STEUERN** – oben die Zonen als Spalten mit Szenenkarten (Klick = an/aus, Rechtsklick =
  bearbeiten/löschen). Jede Spalte hat Tasten für Pause sowie vorherigen/nächsten Schritt und „+“
  (neue Szene aus dem Programmer). Darüber filtert die Seitenleiste nach Zonen.
  Unten links liegt die **Bühne** mit Gruppen: Geräte per Klick, Strg/Shift+Klick oder
  Aufziehen auswählen. Unten rechts ist der **Programmer** mit den Kategorien Dimmer, Farbe
  (mit Farbfeldern), Position (mit XY-Feld), Gobo, Strahl und Sonstiges. Fader: ziehen,
  Mausrad (Shift = 10er-Schritte), Doppelklick = 0/voll, Rechtsklick = Freigeben.
* **EINRICHTEN** – eine Seite, alle Bereiche in der Größe veränderbar: links die **Bibliothek**
  (Generisch, Open Fixture Library nach Hersteller, eigene OFL-Dateien) mit Patch-Formular,
  in der Mitte oben das **Patch-Raster** (Ziehen = neue Startadresse, Doppelklick = umbenennen,
  Patch/Ausgang, DIP-Schalter oben rechts), darunter der **Bühnenplan** mit Werkzeugen links
  und rechts die **Geräteeinstellungen**: Schnelltest (Licht aus, Licht an, Moving Head
  zentrieren), Form und Größe, Dimmer-Bereich, Bewegungsgrenzen als XY-Feld,
  Pan/Tilt invertieren und tauschen. Szenen-Editor und Zonen & Gruppen öffnen sich als
  Fenster (Menü → Bearbeiten).
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
           open_dmx.rs  USB-DMX über FTDI-Kabel (Open DMX)
           dvc_gold.rs  Ergebnisse zur DVC Gold (verschlüsselt)
  ofl/     Open-Fixture-Library-Konverter + eingebettete Bibliothek (data/)
  app/     Oberfläche mit egui/eframe, Updater
           ui/      Design-System: theme.rs (Tokens) + widgets/ (Fader, SceneCard, StageCanvas, …)
           views/   Ansichten (je ein Typ mit `impl View`): control/ und setup/
           state.rs gemeinsamer UI-Zustand
docs/UI-GUIDELINES.md   UI-Richtlinien
graphify-out/           Code-Graph für KI-Assistenten (`graphify update .`)
```

Jeder Ausgang ist ein Backend, das `DmxOutput` implementiert (`open` =
verbinden, `send` = Frame übergeben). Engine und Oberfläche bleiben dabei
unverändert.

**USB-DMX-Kabel einrichten:** Kabel einstecken (unter Windows installiert sich
der FTDI-Treiber meist selbst, sonst von ftdichip.com; unter Linux und macOS
kein Treiber nötig). In DMXCraft unter Einstellungen → Ausgang „USB-DMX-Kabel“
wählen und jedem Universum ein Kabel zuordnen. Die Zuordnung läuft über die
Seriennummer des Kabels und bleibt nach Neustart und Umstecken erhalten.
Fehlt ein Kabel, zeigt der Statuspunkt in der Titelleiste Gelb, und DMXCraft
verbindet sich automatisch, sobald es wieder steckt. Unter Linux braucht der
Benutzer Zugriff auf `/dev/ttyUSB*` (meist Gruppe `dialout`:
`sudo usermod -aG dialout $USER`, danach neu anmelden).

## Show-Format

Dateiendung **`.dmxcraft`**, Inhalt JSON (Version 1); ältere `.json`-Shows lassen sich weiter
öffnen. Der Windows-Installer verknüpft `.dmxcraft` mit DMXCraft, unter Linux trägt DMXCraft
den Dateityp beim ersten Start ein (Doppelklick öffnet die Show). Die verwendeten Gerätetypen werden in jede Show kopiert,
damit die Datei auf jedem Rechner identisch lädt. Universen sind in der Datei
0-basiert, DMX-Adressen 1-basiert.
