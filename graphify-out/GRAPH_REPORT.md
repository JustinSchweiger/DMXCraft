# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 111 files · ~131,968 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2724 nodes · 6568 edges · 224 communities (106 shown, 118 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 305 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b9d11bae`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- venue.rs
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- swatch.rs
- steps
- fx.rs
- DmxFrame
- scene_grid.rs
- Engine
- i18n/mod.rs
- pal
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- .add_scene
- rig.rs
- scene_props.rs
- host
- menu.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- audio.rs
- env
- dmxcraft
- mapping.rs
- Timeline
- color_picker.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag_number
- fader.rs
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- VizView
- ClipAction
- PlanShape
- demo.rs
- ViewCtx
- Scene
- stage.rs
- Transform
- Show
- feature.rs
- output/src/lib.rs
- motion.rs
- patch_grid.rs
- fx_ui.rs
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- plan
- Toggle<'a>
- t
- viz/mod.rs
- ChannelKind
- src/timeline.rs
- CapabilityKind
- Dialog<'a>
- MapPreset
- app.rs
- qxf.rs
- en.rs
- Profile
- Rect
- View
- Grip
- knob.rs
- mcp.rs
- EffectKind
- convert.rs
- bank_menu
- SlotKind
- CurveShape
- ColorPreset
- Tab
- stage_editor.rs
- Step
- stage_canvas.rs
- super
- control/timeline.rs
- scene_card.rs
- MenuItem<'a>
- server.rs
- bridge.rs
- IconButton
- Settings
- ShutterEffect
- Quality
- sense
- core/src/lib.rs
- Kind
- FixtureLimits
- Problem
- .check_placement
- OrbitCamera
- plan.rs
- .show
- tools.rs
- Ui
- lock.rs
- toggle.rs
- qlc-bundle.rs
- Pos2
- ShowError
- dip.rs
- arc
- deserialize
- Drag

## God Nodes (most connected - your core abstractions)
1. `Engine` - 172 edges
2. `t()` - 136 edges
3. `Show` - 117 edges
4. `Icon` - 103 edges
5. `ChannelKind` - 60 edges
6. `AppState` - 55 edges
7. `ViewCtx` - 46 edges
8. `FixtureType` - 43 edges
9. `DmxFrame` - 42 edges
10. `Profile` - 42 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `AppState` --references--> `Quality`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/viz/mod.rs
- `AppState` --references--> `Fixture`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/viz/mod.rs
- `DmxCraftApp` --references--> `VizView`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/viz/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (224 total, 118 thin omitted)

### Community 0 - "venue.rs"
Cohesion: 0.06
Nodes (38): renumber(), aim_at_points_beam_to_target(), apply(), CameraPreset, close(), Environment, argb(), DEFAULT_TRUSS_LEN (+30 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (14): LibraryDrag, BuilderRequest, Edit, New, Open, Template, drag_ghost(), LibAction (+6 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (12): AudioDuration, DEFAULT_PORT, engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped (+4 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (33): accent_hover(), active(), body(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+25 more)

### Community 5 - "programmer.rs"
Cohesion: 0.05
Nodes (57): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+49 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (25): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+17 more)

### Community 9 - "DmxFrame"
Cohesion: 0.06
Nodes (14): DmxFrame, NullOutput, RecordingOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput (+6 more)

### Community 10 - "scene_grid.rs"
Cohesion: 0.11
Nodes (34): programmer_values(), Action, Bank, CreateSuper, Delete, DuplicateScene, Flash, FocusBank (+26 more)

### Community 11 - "Engine"
Cohesion: 0.05
Nodes (43): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+35 more)

### Community 12 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 13 - "pal"
Cohesion: 0.25
Nodes (15): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+7 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.08
Nodes (22): Aufbau, Einschalten, KI-Assistent (MCP), Sicherheit, Verbinden, Werkzeuge, 3D-Visualisierer, Import aus Easy View 2 (+14 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.06
Nodes (48): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+40 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (77): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+69 more)

### Community 23 - ".add_scene"
Cohesion: 0.19
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize()

### Community 24 - "rig.rs"
Cohesion: 0.16
Nodes (13): Beam, build(), cell_light(), cube(), kind(), linear(), Part, placement() (+5 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.05
Nodes (52): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension(), COMPACT_W (+44 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.15
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (15): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+7 more)

### Community 31 - "audio.rs"
Cohesion: 0.14
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 36 - "Timeline"
Cohesion: 0.19
Nodes (6): grid_options(), Clip, fades(), paste_finds_free_place_and_keeps_gaps(), split_and_length(), Timeline

### Community 37 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (38): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+30 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.14
Nodes (26): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+18 more)

### Community 50 - "VizView"
Cohesion: 0.17
Nodes (9): DialogKind, ImportEv2, LoadVenue, SaveVenue, Missing, Mode, Build, Live (+1 more)

### Community 51 - "ClipAction"
Cohesion: 0.25
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 68 - "ViewCtx"
Cohesion: 0.26
Nodes (9): ViewCtx, aim_selected(), apply_placement(), object_tree(), Pick, Fixture, Object, targets() (+1 more)

### Community 69 - "Scene"
Cohesion: 0.16
Nodes (8): LoopMode, Loop, Once, Scene, SceneKind, Fx, Static, Super

### Community 70 - "stage.rs"
Cohesion: 0.20
Nodes (15): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+7 more)

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.21
Nodes (3): Fixture, Show, Placement

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "output/src/lib.rs"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.19
Nodes (11): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+3 more)

### Community 78 - "fx_ui.rs"
Cohesion: 0.11
Nodes (30): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+22 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.09
Nodes (12): Button, Dialog, FOOTER_H, HEADER_H, INPUT_H, Kind, Cancel, Primary (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (22): cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+14 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 92 - "t"
Cohesion: 0.10
Nodes (40): t(), text_field(), capability_card(), capability_fields(), category_label(), check(), color_label(), full_combo() (+32 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.25
Nodes (13): handles(), hit_y(), project(), proposals_for_easy_view_names(), propose_type(), QualityParams, ray_aabb_exit(), ray_box() (+5 more)

### Community 94 - "ChannelKind"
Cohesion: 0.07
Nodes (23): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+15 more)

### Community 95 - "src/timeline.rs"
Cohesion: 0.16
Nodes (9): ClipContent, Audio, Scene, GridMode, Beat, Time, one(), super_scene_from_scenes_keeps_fades() (+1 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "app.rs"
Cohesion: 0.27
Nodes (7): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView

### Community 104 - "qxf.rs"
Cohesion: 0.23
Nodes (11): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), rejects_files_without_modes() (+3 more)

### Community 108 - "Profile"
Cohesion: 0.12
Nodes (13): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+5 more)

### Community 109 - "Rect"
Cohesion: 0.30
Nodes (3): fmt_ms(), paint_clip(), TimelineView

### Community 110 - "View"
Cohesion: 0.19
Nodes (8): section_header(), View, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 113 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 117 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 118 - "mcp.rs"
Cohesion: 0.15
Nodes (9): McpLink, McpStatus, Error, Off, Running, AppEvent, Saved, SelectScene (+1 more)

### Community 119 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 123 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 128 - "SlotKind"
Cohesion: 0.19
Nodes (8): slot_label(), SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 137 - "stage_editor.rs"
Cohesion: 0.21
Nodes (8): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), tool_sep()

### Community 140 - "Step"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (21): SceneCard, BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos() (+13 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.11
Nodes (21): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete (+13 more)

### Community 147 - "scene_card.rs"
Cohesion: 0.31
Nodes (4): CARD_H, clip(), SceneCardResponse, STRIP_W

### Community 150 - "server.rs"
Cohesion: 0.24
Nodes (6): header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 152 - "bridge.rs"
Cohesion: 0.24
Nodes (7): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 153 - "IconButton"
Cohesion: 0.13
Nodes (4): FooterButton, ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - "sense"
Cohesion: 0.12
Nodes (10): LimitPad, from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H, Segmented (+2 more)

### Community 160 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (12): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank, Group (+4 more)

### Community 161 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 170 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 172 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 174 - ".show"
Cohesion: 0.31
Nodes (6): FileJob, Export, Import, import(), new_profile(), ProfileBuilder

### Community 176 - "tools.rs"
Cohesion: 0.07
Nodes (81): ChannelValue, Step, Host, call_tool(), error(), handle(), handle_one(), host() (+73 more)

### Community 195 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 201 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "deserialize"
Cohesion: 0.22
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 207 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

## Knowledge Gaps
- **424 isolated node(s):** `Low`, `Medium`, `High`, `Ultra`, `Live` (+419 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 974 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **118 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `scene_grid.rs`, `control/timeline.rs`, `rig.rs`, `scene_props.rs`, `audio.rs`, `core/src/lib.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `ViewCtx`, `stage.rs`, `Show`, `t`, `app.rs`, `Profile`, `Rect`, `mcp.rs`?**
  _High betweenness centrality (0.142) - this node is a cross-community bridge._
- **Are the 111 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 111 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Low`, `Medium`, `High` to the rest of the system?**
  _424 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `venue.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05952380952380952 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `venue.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `scene_grid.rs`, `Engine`, `stage_canvas.rs`, `.add_scene`, `rig.rs`, `core/src/lib.rs`, `FixtureAction`, `.check_placement`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `Scene`, `feature.rs`, `ShowError`, `motion.rs`, `show.rs`, `viz/mod.rs`, `ChannelKind`, `app.rs`, `EffectKind`, `bank_menu`?**
  _High betweenness centrality (0.131) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13793103448275862 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `SlotKind`, `library_panel.rs`, `programmer.rs`, `stage_editor.rs`, `scene_grid.rs`, `i18n/mod.rs`, `pal`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `Settings`, `Timeline`, `color_picker.rs`, `Problem`, `FixtureAction`, `fader.rs`, `DmxCraftApp`, `settings_window.rs`, `VizView`, `ViewCtx`, `stage.rs`, `patch_grid.rs`, `fx_ui.rs`, `dialog.rs`, `Dialog<'a>`, `app.rs`, `Profile`, `Rect`, `View`, `bank_menu`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._