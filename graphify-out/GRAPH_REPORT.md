# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~58,684 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1467 nodes · 3320 edges · 125 communities (54 shown, 71 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 134 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `fe99a50a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- app.rs
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- qxf.rs
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- widgets/mod.rs
- super
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- MenuAction
- stage.rs
- ofl/src/lib.rs
- host
- plan
- with
- ensure_linux_menu_entry
- color_picker.rs
- swatch.rs
- env
- dmxcraft
- menu.rs
- Settings
- Context
- DmxCraft – UI-Richtlinien
- T
- FixtureAction
- qlc-bundle.rs
- fader.rs
- form
- DmxCraft – Hinweise für KI-Assistenten
- t
- settings_window.rs
- DmxFrame
- IconButton
- show
- DvcGoldUsb
- programmer.rs
- UpdateBadge
- convert.rs
- OutputKind
- Pos2
- resize_handles
- OpenDmx
- Stroke
- MenuItem<'a>
- Toggle<'a>
- arc
- FixtureType
- t
- Show
- stage_canvas.rs
- en.rs
- Tab
- egui
- limit_pad.rs
- Dialog<'a>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 103 edges
2. `Show` - 80 edges
3. `Icon` - 73 edges
4. `t()` - 67 edges
5. `ChannelKind` - 44 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 37 edges
8. `AppState` - 33 edges
9. `ChannelRef` - 29 edges
10. `ViewCtx` - 28 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `paint_fixture()` --calls--> `fill()`  [INFERRED]
  crates/app/src/ui/widgets/stage_canvas.rs → crates/app/src/i18n/mod.rs
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (125 total, 71 thin omitted)

### Community 0 - "app.rs"
Cohesion: 0.15
Nodes (9): LockDialog, OUTPUT_HZ, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT (+1 more)

### Community 1 - "Engine"
Cohesion: 0.23
Nodes (4): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (52): sidebar(), accent(), accent_dim(), accent_hover(), active(), ACTIVE, apply(), bg() (+44 more)

### Community 5 - "Engine"
Cohesion: 0.06
Nodes (32): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered() (+24 more)

### Community 6 - "qxf.rs"
Cohesion: 0.26
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.09
Nodes (45): color_picker(), color_swatches(), select_all(), text_field(), programmer_values(), Action, Bank, Delete (+37 more)

### Community 11 - "dialog.rs"
Cohesion: 0.11
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 12 - "widgets/mod.rs"
Cohesion: 0.33
Nodes (6): RangeSlider, CanvasMode, Edit, Select, StageCanvas, XyPad

### Community 13 - "super"
Cohesion: 0.05
Nodes (35): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+27 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (52): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronLeft (+44 more)

### Community 23 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 24 - "stage.rs"
Cohesion: 0.19
Nodes (12): ControlViews, ProgrammerView, group_pill(), GroupAction, AddSelection, Delete, New, Rename (+4 more)

### Community 25 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "color_picker.rs"
Cohesion: 0.15
Nodes (11): hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H, Swatch, Auto (+3 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "menu.rs"
Cohesion: 0.16
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 36 - "Settings"
Cohesion: 0.19
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), Settings

### Community 37 - "Context"
Cohesion: 0.20
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.06
Nodes (35): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+27 more)

### Community 41 - "qlc-bundle.rs"
Cohesion: 0.27
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 44 - "fader.rs"
Cohesion: 0.06
Nodes (32): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+24 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.19
Nodes (16): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+8 more)

### Community 66 - "show"
Cohesion: 0.47
Nodes (3): load_meter(), show(), TitleInfo

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.07
Nodes (45): edit_preview(), AppState, Mode, Control, Setup, PatchForm, ProgrammerMode, Blind (+37 more)

### Community 72 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 73 - "convert.rs"
Cohesion: 0.11
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "FixtureType"
Cohesion: 0.15
Nodes (4): RECENT_MAX, ChannelDef, FixtureType, ft()

### Community 97 - "t"
Cohesion: 0.05
Nodes (31): section_header(), ZOOM_MAX, ZOOM_MIN, Toggle, View, ViewCtx, caption(), FixtureSettings (+23 more)

### Community 101 - "Show"
Cohesion: 0.05
Nodes (41): cv(), demo_show(), empty_show(), ensure_builtin_types(), address_map_marks_ranges(), ChannelValue, duplicate_zone_copies_scenes_behind_original(), Fixture (+33 more)

### Community 102 - "stage_canvas.rs"
Cohesion: 0.19
Nodes (12): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+4 more)

### Community 110 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 111 - "egui"
Cohesion: 0.15
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 112 - "limit_pad.rs"
Cohesion: 0.25
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

## Knowledge Gaps
- **191 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+186 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 543 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **71 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `app.rs`, `t`, `Engine`, `runtime.rs`, `programmer.rs`, `Show`, `FixtureAction`, `t`, `t`, `settings_window.rs`, `egui`, `stage.rs`?**
  _High betweenness centrality (0.134) - this node is a cross-community bridge._
- **Are the 51 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 51 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _191 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `app.rs`, `Engine`, `theme.rs`, `t`, `dialog.rs`, `super`, `titlebar.rs`, `stage.rs`, `color_picker.rs`, `Settings`, `FixtureAction`, `fader.rs`, `form`, `settings_window.rs`, `show`, `programmer.rs`, `t`, `stage_canvas.rs`, `egui`?**
  _High betweenness centrality (0.100) - this node is a cross-community bridge._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0946969696969697 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `app.rs`, `runtime.rs`, `theme.rs`, `Engine`, `stage_canvas.rs`, `FixtureAction`, `t`, `widgets/mod.rs`, `t`, `FixtureType`?**
  _High betweenness centrality (0.089) - this node is a cross-community bridge._