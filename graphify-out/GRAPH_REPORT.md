# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 80 files · ~64,415 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1609 nodes · 3625 edges · 152 communities (78 shown, 74 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 143 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3ef9c5eb`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- text_dim
- steps
- DmxCraftApp
- OutputError
- t
- send_frame
- file_dialog.rs
- AppState
- jobs
- steps
- DmxCraft
- menu.rs
- announce
- build-local-artifacts
- Icon
- super
- stage.rs
- scene_card.rs
- host
- plan
- with
- ensure_linux_menu_entry
- convert.rs
- swatch.rs
- env
- dmxcraft
- egui
- BankScope
- settings_window.rs
- DmxCraft – UI-Richtlinien
- limit_pad.rs
- FixtureAction
- accent
- fader.rs
- i18n/mod.rs
- DmxCraft – Hinweise für KI-Assistenten
- t
- general
- arc
- dip.rs
- core/src/lib.rs
- programmer.rs
- Settings
- titlebar.rs
- Show
- knob.rs
- OutputKind
- .check_placement
- Fullscreen
- .fade_state
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- ChannelValue
- Toggle<'a>
- MenuAction
- FixtureLimits
- ViewCtx
- patch_grid.rs
- Snapshot
- ShowError
- ZoneId
- EffectKind
- en.rs
- Dialog<'a>
- show
- Category
- DvcGoldUsb
- sidebar
- time_field.rs
- Theme
- scene_props.rs
- app.rs
- stage_canvas.rs
- quick.rs
- DmxFrame
- Vec2
- IconButton
- Ui
- body
- PickTarget
- Tab
- resize_handles

## God Nodes (most connected - your core abstractions)
1. `Engine` - 116 edges
2. `Icon` - 87 edges
3. `Show` - 83 edges
4. `t()` - 74 edges
5. `ChannelKind` - 42 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 38 edges
8. `AppState` - 35 edges
9. `ViewCtx` - 33 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `paint_fixture()` --calls--> `fill()`  [INFERRED]
  crates/app/src/ui/widgets/stage_canvas.rs → crates/app/src/i18n/mod.rs
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (152 total, 74 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.08
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 1 - "AppState"
Cohesion: 0.14
Nodes (8): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag(), FixtureType, ft()

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (33): accent_dim(), accent_hover(), ACTIVE, apply(), border(), c(), DANGER, DIP_BODY (+25 more)

### Community 5 - "Engine"
Cohesion: 0.24
Nodes (27): blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position(), live_overrides_scenes_and_resets() (+19 more)

### Community 6 - "text_dim"
Cohesion: 0.21
Nodes (9): load_meter(), show(), heading_font(), heading_text(), text(), text_dim(), text_faint(), title() (+1 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.06
Nodes (58): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+50 more)

### Community 11 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "AppState"
Cohesion: 0.14
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Mode, Control, Setup (+2 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "menu.rs"
Cohesion: 0.21
Nodes (10): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+2 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (65): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronDown (+57 more)

### Community 23 - "super"
Cohesion: 0.24
Nodes (6): digest(), hash_and_verify(), hash_password(), hex(), verify(), addresses_are_one_based()

### Community 24 - "stage.rs"
Cohesion: 0.23
Nodes (10): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+2 more)

### Community 25 - "scene_card.rs"
Cohesion: 0.20
Nodes (6): raised_hover(), CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "egui"
Cohesion: 0.27
Nodes (5): MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 36 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 37 - "settings_window.rs"
Cohesion: 0.22
Nodes (7): HEIGHT, SCALES, SettingsCx, SIDEBAR_W, TAB_H, WIDTH, UsbDmxPort

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "limit_pad.rs"
Cohesion: 0.13
Nodes (8): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T>

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "accent"
Cohesion: 0.46
Nodes (3): accent(), bg(), DipSwitch

### Community 44 - "fader.rs"
Cohesion: 0.05
Nodes (31): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+23 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "general"
Cohesion: 0.36
Nodes (8): full_combo(), general(), hint(), midi(), outputs(), project(), section(), SettingsChanges

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 65 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 66 - "core/src/lib.rs"
Cohesion: 0.15
Nodes (11): UNIVERSE_SIZE, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind, Fx (+3 more)

### Community 69 - "programmer.rs"
Cohesion: 0.13
Nodes (27): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), fader() (+19 more)

### Community 70 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 71 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 72 - "Show"
Cohesion: 0.26
Nodes (3): Fixture, Group, Show

### Community 73 - "knob.rs"
Cohesion: 0.13
Nodes (6): ICON_FAMILY, ICON_FILL_FAMILY, Knob, Knob<'a>, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 77 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 78 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.19
Nodes (12): ChannelDef, address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.13
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 90 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 95 - "ViewCtx"
Cohesion: 0.16
Nodes (6): t(), section_header(), ViewCtx, clip(), ZonesView, Windows

### Community 96 - "patch_grid.rs"
Cohesion: 0.11
Nodes (15): View, caption(), FixtureSettings, range_row(), SetupView, CELL_H_MAX, CELL_H_MIN, COLUMNS (+7 more)

### Community 97 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.20
Nodes (3): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks()

### Community 104 - "EffectKind"
Cohesion: 0.15
Nodes (11): effect_tiles(), EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings (+3 more)

### Community 109 - "show"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 118 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 128 - "sidebar"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 133 - "time_field.rs"
Cohesion: 0.29
Nodes (8): FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS, time_field()

### Community 136 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 137 - "scene_props.rs"
Cohesion: 0.16
Nodes (20): advanced(), banks_of(), bar_slider(), effect_header(), FX_EXT, FxDialog, header(), pick_tile() (+12 more)

### Community 138 - "app.rs"
Cohesion: 0.33
Nodes (4): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView

### Community 141 - "stage_canvas.rs"
Cohesion: 0.06
Nodes (44): BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos(), fixture_rect() (+36 more)

### Community 142 - "quick.rs"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 144 - "Vec2"
Cohesion: 0.22
Nodes (4): raised(), HANDLE, Toggle, XyPad

### Community 151 - "body"
Cohesion: 0.25
Nodes (4): body(), caption(), DialogResult, elide()

### Community 152 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 153 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

## Knowledge Gaps
- **228 isolated node(s):** `Play`, `Pause`, `Prev`, `Next`, `Plus` (+223 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 611 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **74 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `AppState`, `runtime.rs`, `Engine`, `scene_props.rs`, `app.rs`, `t`, `AppState`, `quick.rs`, `DmxFrame`, `stage.rs`, `egui`, `settings_window.rs`, `t`, `general`, `core/src/lib.rs`, `programmer.rs`, `Show`, `.fade_state`, `ViewCtx`, `Snapshot`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._
- **Are the 56 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 56 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Play`, `Pause`, `Prev` to the rest of the system?**
  _228 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.07770582793709528 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `AppState`, `runtime.rs`, `theme.rs`, `Engine`, `app.rs`, `t`, `stage_canvas.rs`, `egui`, `BankScope`, `FixtureAction`, `t`, `core/src/lib.rs`, `.check_placement`, `show.rs`, `ChannelValue`, `Snapshot`, `ShowError`, `ZoneId`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.13675213675213677 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `sidebar`, `patch_grid.rs`, `egui`, `.paint`, `settings_window.rs`, `programmer.rs`, `titlebar.rs`, `knob.rs`, `t`, `scene_props.rs`, `stage_canvas.rs`, `IconButton`, `menu.rs`, `dialog.rs`, `MenuItem<'a>`, `scene_card.rs`?**
  _High betweenness centrality (0.106) - this node is a cross-community bridge._