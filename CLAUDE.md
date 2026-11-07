# DMXCraft – Hinweise für KI-Assistenten

DMX-Lichtsteuerung in Rust (egui), Ziel: Daslight-5-Arbeitsablauf auf Linux,
Windows und macOS, Ausgabe später über USB an eine Daslight DVC Gold.

## Orientierung (Tokens sparen)

* Zuerst den Code-Graphen nutzen: `graphify query "<Frage>"`,
  `graphify explain "<Symbol>"`, Überblick in `graphify-out/GRAPH_REPORT.md`.
* Nach Codeänderungen `graphify update .` ausführen (kostet keine Tokens).
* Dateien erst öffnen, wenn sie geändert werden müssen.

## Struktur

```
crates/core    Datenmodell (Show, Patch, Szenen, Zonen, Gruppen) + Engine – ohne I/O, mit Tests
crates/output  DmxOutput-Trait, Ausgabe-Thread, Backends (dvc_gold.rs = USB, noch Platzhalter)
crates/ofl     Open-Fixture-Library-Konverter + eingebettete Bibliothek (data/)
crates/mcp     MCP-Server (KI-Assistent): tools.rs, protocol.rs, server.rs, bridge.rs – docs/MCP.md
crates/app     Oberfläche
  src/ui/        Design-System: theme.rs (Tokens), widgets/ (Fader, SceneCard, …)
  src/views/     Ansichten, je ein Typ mit `impl View` (control/, setup/, windows.rs)
  src/titlebar.rs  eigene Titelleiste + Hauptmenü; src/settings.rs Programmeinstellungen
  src/state.rs   gemeinsamer UI-Zustand (AppState)
  src/app.rs     Rahmen: Kopfzeile, Modus, Layout, Runtime
docs/UI-GUIDELINES.md  verbindliche UI-Regeln – vor UI-Arbeit lesen
```

## Regeln

* UI-Änderungen folgen `docs/UI-GUIDELINES.md` (Tokens, Komponenten, keine
  Sonderzeichen-Glyphen, Flächen füllen).
* **Texte** der Oberfläche immer über `t("…")` bzw. `tr!("… {name}", name = x)`
  (deutsche Quelle) und die englische Übersetzung in `crates/app/src/i18n/en.rs`
  ergänzen – der Test `all_texts_translated` prüft das.
* **Farben** nur über `theme::bg()`, `theme::text()` … (abhängig vom Theme);
  feste Signalfarben `theme::DANGER/OK/WARN`.
* Keine Grafiken, Icons oder Gerätebibliotheken aus Daslight übernehmen.
* Show-Format abwärtskompatibel halten (`#[serde(default)]` für neue Felder).
* Vor jedem Commit: `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`; jeden Stand auf `main` pushen.
* Releases: Version in `Cargo.toml` erhöhen, Tag `vX.Y.Z` pushen (release.yml).
