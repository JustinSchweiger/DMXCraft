# DMXCraft – notes for AI assistants

DMX lighting control in Rust (egui). Goal: the Daslight 5 workflow on Linux,
Windows and macOS, output via USB DMX cables (Open DMX / FTDI).

## Orientation (save tokens)

* Use the code graph first: `graphify query "<question>"`,
  `graphify explain "<symbol>"`, overview in `graphify-out/GRAPH_REPORT.md`.
* Run `graphify update .` after code changes (costs no tokens).
* Only open files when they need to be changed.

## Structure

```
crates/core    data model (show, patch, scenes, zones, groups) + engine – no I/O, with tests
crates/output  DmxOutput trait, output thread, backends (open_dmx.rs = USB DMX cable)
crates/ofl     Open Fixture Library / QLC+ converters + embedded library (data/)
crates/mcp     MCP server (AI assistant): tools.rs, protocol.rs, server.rs, bridge.rs – docs/MCP.md
crates/app     user interface
  src/ui/        design system: theme.rs (tokens), widgets/ (Fader, SceneCard, …)
  src/views/     views, one type with `impl View` each (control/, setup/, windows.rs)
  src/titlebar.rs  custom title bar + main menu; src/settings.rs program settings
  src/state.rs   shared UI state (AppState)
  src/i18n/      translation: English source, German in de.rs
  src/viz/       3D visualizer (wgpu): render.rs, shader.wgsl, rig.rs, panels.rs, gltf_import.rs – docs/VISUALIZER.md
  src/sound.rs   playback of the SOUND tab (players, pads), view in views/sound.rs – docs/SOUND.md
  src/obs.rs     OBS integration (obs-websocket 5): video blocks, OBS actions – docs/OBS.md
  src/app.rs     frame: header, mode, layout, runtime
docs/UI-GUIDELINES.md  binding UI rules – read before UI work
```

## Rules

* The repository language is English (code, comments, docs, commit messages).
* UI changes follow `docs/UI-GUIDELINES.md` (tokens, components, no special
  glyphs, fill the space).
* **UI texts** always go through `t("…")` or `tr!("… {name}", name = x)`
  (English source); add the German translation to `crates/app/src/i18n/de.rs`
  – the test `all_texts_translated` checks this.
* **Colors** only via `theme::bg()`, `theme::text()` … (theme-dependent);
  fixed signal colors `theme::DANGER/OK/WARN`.
* Do not copy graphics, icons or fixture libraries from Daslight.
* Keep the show format backwards compatible (`#[serde(default)]` for new
  fields).
* Before every commit: `cargo fmt --all`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo test --workspace`; push every state to `main`.
* Releases: bump the version in `Cargo.toml`, push a `vX.Y.Z` tag
  (release.yml; macos-app.yml adds the macOS DMG and the install notes
  from .github/release-install.md to the release page).
