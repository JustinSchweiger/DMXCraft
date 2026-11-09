# DMXCraft

Eine in Rust geschriebene DMX-Lichtsteuerung für Linux, Windows und macOS.
Arbeitsweise und Aufbau sind zum großen Teil von Daslight 5 inspiriert: Bänke
mit Szenen, Programmer, Effekte und SUPER SCENES auf einer Zeitleiste. DMX geht
über günstige USB-DMX-Kabel (Open DMX / FTDI) raus, ein Kabel je Universum.

> Unabhängiges Projekt, nicht mit Nicolaudie/Daslight verbunden. Es wird kein
> Code, keine Gerätebibliothek und kein Grafik-Asset von Daslight verwendet.

## Funktionen

* **Patch** mit 8 Universen, Gerätebibliothek aus Open Fixture Library und QLC+
  (über 2.400 Geräte), eigene Gerätetypen im Profile Builder
* **Szenen** in Bänken und Zonen, mit Schritten, Fades und Programmer
* **Effekte**: Farb-, Bewegungs- und Kurveneffekte über beliebige Gerätegruppen
* **SUPER SCENES**: Zeitleiste mit Szenen und Musik, Audioanalyse mit
  Beat-Raster und Audio-Triggern
* **Bedienung** per Tastatur, MIDI und frei gestaltbarer Touch-Oberfläche
* **3D-Visualisierer** für den Veranstaltungsort
* **KI-Assistent** über MCP
* Deutsch und Englisch, Installer und Auto-Update für alle drei Systeme

Mehr in `docs/`: [Mapping](docs/MAPPING.md), [Touch](docs/TOUCH.md),
[3D-Visualisierer](docs/VISUALIZER.md), [MCP](docs/MCP.md).

## Installieren

**Linux / macOS** (Terminal):

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/JustinSchweiger/DMXCraft/releases/latest/download/dmxcraft-installer.sh | sh
```

**Windows**: Unter [Releases](https://github.com/JustinSchweiger/DMXCraft/releases/latest) die
`dmxcraft-x86_64-pc-windows-msvc.msi` herunterladen und ausführen. Alternativ in PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/JustinSchweiger/DMXCraft/releases/latest/download/dmxcraft-installer.ps1 | iex"
```

Gebaut wird für Linux (x86_64, ARM64), macOS (Apple Silicon und Intel) und Windows (x86_64).
DMXCraft prüft beim Start auf Updates und bietet sie oben in der Titelleiste an.

**USB-DMX-Kabel**: einstecken und unter *Bearbeiten → Einstellungen → Ausgänge* einem
Universum zuordnen. Ohne Zuordnung läuft DMXCraft als Simulation. Unter Linux braucht der
Benutzer Zugriff auf `/dev/ttyUSB*` (`sudo usermod -aG dialout $USER`, danach neu anmelden).

## Bauen und starten

Voraussetzung: Rust ≥ 1.85.

```sh
cargo run --release -p dmxcraft               # startet mit Demo-Show
cargo run --release -p dmxcraft -- show.dmxcraft
cargo test --workspace
```

## Entwicklung

```
crates/
  core/    Datenmodell, Engine, Effekte, Zeitleiste, Audioanalyse (ohne I/O)
  output/  Ausgabe-Thread und Backends (Simulation, USB-DMX-Kabel)
  ofl/     Gerätebibliothek (Open Fixture Library, QLC+)
  mcp/     MCP-Server
  app/     Oberfläche (egui), 3D-Visualisierer, Updater
docs/      Richtlinien und Anleitungen
```

**Release**: Version in `Cargo.toml` (`[workspace.package]`) erhöhen, committen, dann
`git tag vX.Y.Z && git push origin vX.Y.Z`. GitHub Actions baut alle Plattformen.

**Gerätebibliothek aktualisieren**:

```sh
git clone --depth 1 https://github.com/OpenLightingProject/open-fixture-library /tmp/ofl
cargo run --release -p dmxcraft-ofl --bin ofl-bundle -- /tmp/ofl crates/ofl/data/ofl-library.json.gz

git clone --depth 1 --filter=blob:none --sparse https://github.com/mcallegari/qlcplus /tmp/qlcplus
(cd /tmp/qlcplus && git sparse-checkout set resources/fixtures)
cargo run --release -p dmxcraft-ofl --bin qlc-bundle -- /tmp/qlcplus crates/ofl/data/qlc-library.json.gz
```

## Lizenz

DMXCraft steht unter der **GNU General Public License v3** mit einem Zusatz nach
Abschnitt 7(b): Weitergegebene und veränderte Versionen müssen im Über-Dialog
„Based on DMXCraft by Justin Schweiger“ nennen. Details in [LICENSE](LICENSE).
Frühere, unter MIT veröffentlichte Versionen bleiben unter MIT.

Für eine Nutzung außerhalb dieser Bedingungen (z. B. in geschlossener Software)
ist eine gesonderte Lizenz möglich, Anfrage über GitHub.

Mitgelieferte Gerätedaten haben eigene Lizenzen: QLC+ Apache-2.0
(`crates/ofl/data/QLC-LICENSE`, `QLC-NOTICE`), Open Fixture Library MIT
(`crates/ofl/data/OFL-LICENSE`).

„Daslight“ ist eine Marke ihres Inhabers und wird hier nur zur Beschreibung genannt.
