# Graph Report - dmxcraft-src  (2026-11-09)

## Corpus Check
- 136 files · ~201,731 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3788 nodes · 9475 edges · 261 communities (156 shown, 105 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 498 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6939c96e`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- paint_item
- library_panel.rs
- runtime.rs
- open_dmx.rs
- Theme
- programmer.rs
- AppState
- steps
- fx.rs
- OutputError
- Deserialize
- Engine
- Venue
- viz/library.rs
- jobs
- steps
- Tastatur- und MIDI-Zuordnung
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- Host
- rig.rs
- scene_props.rs
- host
- core/src/audio.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- fx_ui.rs
- env
- dmxcraft
- app/src/audio.rs
- t
- .push
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag.rs
- textures.rs
- Fader
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- MenuAction
- remote/mod.rs
- TouchLayout
- BankScope
- core/src/lib.rs
- ViewCtx
- window.rs
- stage.rs
- mcp.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- Profile
- engine.rs
- sense
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- toolbar.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- .modes_tab
- MapPreset
- audio_triggers.rs
- fx/mapping.rs
- qxf.rs
- file_dialog.rs
- en.rs
- DialogKind
- Rect
- super
- server.rs
- ev2.rs
- Capability
- mcp/src/profiles.rs
- outline.rs
- convert.rs
- VizView
- windows.rs
- CurveShape
- ColorPreset
- control/timeline.rs
- i18n/mod.rs
- ChannelDef
- theme.rs
- stage_canvas.rs
- AudioGrid
- Snapshot
- SetupView
- Mapper
- scene_card.rs
- menu.rs
- plan.rs
- midi.rs
- mapping/mod.rs
- icon_library.rs
- mcp/src/venue.rs
- views/touch.rs
- ShutterEffect
- .viewport
- Scene
- time_field.rs
- FixtureLimits
- color_picker.rs
- Problem
- paint_clip
- Profile
- gltf_import.rs
- fixture_settings.rs
- TexAction
- track_header
- src/mapping.rs
- TouchKind
- tools.rs
- EffectKind
- core/src/venue.rs
- ClipAction
- ChannelValue
- MenuItem<'a>
- knob.rs
- app.rs
- Input
- Drag
- lock.rs
- ProfileBuilder
- ProgrammerMode
- fire
- Ui
- Segmented<'a, T>
- widgets/mod.rs
- qlc-bundle.rs
- super
- ChaserPreset
- .check_placement
- ShowError
- views/touch.rs
- Cmd
- arc
- BuilderRequest
- IconButton
- FilterKind
- widgets/mod.rs
- PlanShape
- .fade_state
- OrbitCamera
- Category
- SlotKind
- fullscreen.rs
- UpdateBadge
- palettes.rs
- Timeline
- FooterButton<'a>
- Grip
- Box<dyn serialport::SerialPort>
- Mode
- plan
- AudioAnalysis
- AudioPlayer

## God Nodes (most connected - your core abstractions)
1. `Engine` - 218 edges
2. `t()` - 197 edges
3. `Show` - 144 edges
4. `Icon` - 113 edges
5. `AppState` - 100 edges
6. `ViewCtx` - 82 edges
7. `ChannelKind` - 72 edges
8. `ChannelRef` - 57 edges
9. `Profile` - 53 edges
10. `FixtureType` - 50 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (261 total, 105 thin omitted)

### Community 0 - "paint_item"
Cohesion: 0.16
Nodes (10): section_header(), add_page(), icon_picker(), ImageTarget, Item, Page, pick_image(), prepare_image() (+2 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.17
Nodes (12): UserProfile, LibraryDrag, template(), drag_ghost(), LibAction, Builder, Delete, LibraryPanel (+4 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Theme"
Cohesion: 0.13
Nodes (13): active(), set_theme(), Theme, .ALL, Black, Contrast, Daslight, Graphite (+5 more)

### Community 5 - "programmer.rs"
Cohesion: 0.17
Nodes (25): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup (+17 more)

### Community 6 - "AppState"
Cohesion: 0.09
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (24): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+16 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "Deserialize"
Cohesion: 0.12
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 11 - "Engine"
Cohesion: 0.06
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "Venue"
Cohesion: 0.13
Nodes (8): Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object, room_and_tree(), Venue

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.06
Nodes (28): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten, Aufbau, Einschalten (+20 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.13
Nodes (27): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+19 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (83): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+75 more)

### Community 23 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (24): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+16 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.08
Nodes (40): Step, Enter, Maximize, Show, advanced(), banks_of(), bar_slider(), block_props() (+32 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "core/src/audio.rs"
Cohesion: 0.11
Nodes (31): analyze(), bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), BPM_MAX, BPM_MIN, detect_grid(), detect_segments(), energy_curve() (+23 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.11
Nodes (33): c(), canvas(), canvas_dim(), canvas_faint(), canvas_line(), handle(), color_fx_ui(), edit_colors() (+25 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.18
Nodes (10): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+2 more)

### Community 36 - "t"
Cohesion: 0.09
Nodes (28): t(), text_field(), fine_mark(), kind_color(), paint_symbol(), capability_fields(), card_row(), category_label() (+20 more)

### Community 37 - ".push"
Cohesion: 0.25
Nodes (6): Analyzer, Biquad, Envelope, envelope_filters_band(), percentile(), speed_six_minutes()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "drag.rs"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 44 - "Fader"
Cohesion: 0.12
Nodes (8): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fader_ranges()

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.18
Nodes (4): DmxCraftApp, moved_out(), viewport_input(), Fullscreen

### Community 48 - "settings_window.rs"
Cohesion: 0.05
Nodes (50): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+42 more)

### Community 50 - "MenuAction"
Cohesion: 0.09
Nodes (23): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+15 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (29): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+21 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 66 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (12): UNIVERSE_SIZE, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind, Fx (+4 more)

### Community 68 - "ViewCtx"
Cohesion: 0.21
Nodes (8): fold_section(), ViewCtx, renumber(), vec3_row(), drop_marker(), Folder, VizView, VizView

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.17
Nodes (15): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+7 more)

### Community 71 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.17
Nodes (4): .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+7 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "Profile"
Cohesion: 0.26
Nodes (8): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today()

### Community 78 - "engine.rs"
Cohesion: 0.21
Nodes (33): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), layered_start_keeps_other_scenes() (+25 more)

### Community 79 - "sense"
Cohesion: 0.08
Nodes (23): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, Knob (+15 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.07
Nodes (15): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H (+7 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (19): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS, KEY_PREFIX (+11 more)

### Community 90 - "toolbar.rs"
Cohesion: 0.26
Nodes (8): chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width(), VizView

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.13
Nodes (23): AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles, handles(), hit_y(), move_all() (+15 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.08
Nodes (49): CARD_H, COMPACT_H, Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+41 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - ".modes_tab"
Cohesion: 0.19
Nodes (12): list_header(), ListAction, Add, Duplicate, Remove, ModeOp, Add, Down (+4 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.05
Nodes (40): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+32 more)

### Community 101 - "fx/mapping.rs"
Cohesion: 0.15
Nodes (15): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+7 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.21
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 109 - "Rect"
Cohesion: 0.26
Nodes (4): grid_options(), has_audio(), mark_clipboard(), TimelineView

### Community 112 - "server.rs"
Cohesion: 0.14
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 113 - "ev2.rs"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Capability"
Cohesion: 0.19
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 118 - "mcp/src/profiles.rs"
Cohesion: 0.19
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.08
Nodes (25): drag_row(), DragFixtures, DragObjects, FixtureSort, Address, Name, move_objects(), moving_into_rotated_group_keeps_world_place() (+17 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "VizView"
Cohesion: 0.12
Nodes (12): aim_selected(), apply_placement(), Missing, Pick, Fixture, Object, sel_frame(), targets() (+4 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (18): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+10 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.13
Nodes (14): add_audio(), add_audio_dialog(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H (+6 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "ChannelDef"
Cohesion: 0.11
Nodes (13): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+5 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (55): accent(), accent_dim(), accent_hover(), apply(), BAND_HIGH, BAND_LOW, BAND_MID, bg() (+47 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 143 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 144 - "SetupView"
Cohesion: 0.18
Nodes (8): apply(), SetupView, clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 146 - "Mapper"
Cohesion: 0.26
Nodes (3): default_feedback(), Mapper, Sent

### Community 147 - "scene_card.rs"
Cohesion: 0.24
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 149 - "menu.rs"
Cohesion: 0.19
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 150 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 151 - "midi.rs"
Cohesion: 0.27
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 152 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (33): alt_click(), button(), button_rect(), channel_name(), describe(), describe_mapping(), fader(), fader_channels() (+25 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.25
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 155 - "views/touch.rs"
Cohesion: 0.30
Nodes (10): find_item(), content_rect(), display_label(), fader_parts(), item_target(), mapping_of(), paint_cover(), paint_item() (+2 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.20
Nodes (7): Quality, .ALL, High, Low, Medium, Ultra, QualityParams

### Community 159 - "Scene"
Cohesion: 0.19
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize()

### Community 160 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 162 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "paint_clip"
Cohesion: 0.25
Nodes (6): fmt_ms(), paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers()

### Community 167 - "Profile"
Cohesion: 0.13
Nodes (5): AppProfiles, MemoryProfiles, fine_names(), Profile, rename_and_resolution_update_modes()

### Community 168 - "gltf_import.rs"
Cohesion: 0.16
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 169 - "fixture_settings.rs"
Cohesion: 0.20
Nodes (9): Quick, quick_action(), Center, Off, On, body(), caption(), FixtureSettings (+1 more)

### Community 170 - "TexAction"
Cohesion: 0.50
Nodes (4): TexAction, Builtin, Load, Set

### Community 171 - "track_header"
Cohesion: 0.24
Nodes (9): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle, row_h(), track_h() (+1 more)

### Community 173 - "src/mapping.rs"
Cohesion: 0.07
Nodes (35): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+27 more)

### Community 174 - "TouchKind"
Cohesion: 0.18
Nodes (8): add_tile(), TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (59): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), base64(), boolean(), build_steps() (+51 more)

### Community 177 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 178 - "core/src/venue.rs"
Cohesion: 0.14
Nodes (10): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, mul(), rotation(), rotation_order_x_then_y_then_z(), Transform (+2 more)

### Community 179 - "ClipAction"
Cohesion: 0.11
Nodes (17): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+9 more)

### Community 180 - "ChannelValue"
Cohesion: 0.20
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), programmer_values(), ChannelValue, Step

### Community 184 - "app.rs"
Cohesion: 0.23
Nodes (7): ControlViews, edit_preview(), fill_types(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 185 - "Input"
Cohesion: 0.28
Nodes (6): Input, Key, Midi, MidiMessage, Cc, Note

### Community 186 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 187 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 189 - "ProfileBuilder"
Cohesion: 0.23
Nodes (6): FileJob, Export, Import, import(), new_profile(), ProfileBuilder

### Community 190 - "ProgrammerMode"
Cohesion: 0.29
Nodes (6): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch()

### Community 192 - "fire"
Cohesion: 0.23
Nodes (13): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+5 more)

### Community 193 - "Ui"
Cohesion: 0.17
Nodes (17): check(), general(), hint(), manufacturer_picker(), paint_handle(), physical(), pill(), section() (+9 more)

### Community 196 - "widgets/mod.rs"
Cohesion: 0.29
Nodes (6): CanvasMode, Edit, Select, StageCanvas, Underlay, XyPad

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "super"
Cohesion: 0.46
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 199 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 200 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "views/touch.rs"
Cohesion: 0.14
Nodes (21): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+13 more)

### Community 203 - "Cmd"
Cohesion: 0.40
Nodes (5): Cmd, Color, Press, Value, Xy

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "BuilderRequest"
Cohesion: 0.40
Nodes (5): BuilderRequest, Edit, New, Open, Template

### Community 207 - "IconButton"
Cohesion: 0.12
Nodes (4): ICON_FILL_FAMILY, IconButton, paint_alert(), Toggle

### Community 209 - "FilterKind"
Cohesion: 0.50
Nodes (4): FilterKind, Band, High, Low

### Community 210 - "widgets/mod.rs"
Cohesion: 0.19
Nodes (9): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), StageEditorView (+1 more)

### Community 211 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 212 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 213 - "OrbitCamera"
Cohesion: 0.09
Nodes (13): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, OrbitCamera (+5 more)

### Community 215 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 216 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 221 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 223 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 224 - "Timeline"
Cohesion: 0.11
Nodes (15): audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat (+7 more)

### Community 230 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 234 - "Mode"
Cohesion: 0.67
Nodes (3): Mode, Build, Live

### Community 237 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 250 - "AudioAnalysis"
Cohesion: 0.21
Nodes (6): WaveState, Failed, Loading, Ready, AudioAnalysis, AudioSegment

### Community 251 - "AudioPlayer"
Cohesion: 0.18
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

## Knowledge Gaps
- **530 isolated node(s):** `New`, `NotFound`, `Transfer`, `Simulation`, `OpenDmx` (+525 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1224 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **105 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `AppState`, `control/timeline.rs`, `Snapshot`, `SetupView`, `Mapper`, `Host`, `mapping/mod.rs`, `scene_props.rs`, `rig.rs`, `views/touch.rs`, `mcp/src/venue.rs`, `app/src/audio.rs`, `t`, `FixtureAction`, `fixture_settings.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `ChannelValue`, `app.rs`, `ProfileBuilder`, `fire`, `Ui`, `core/src/lib.rs`, `ViewCtx`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `views/touch.rs`, `engine.rs`, `.fade_state`, `ChannelKind`, `scene_grid.rs`, `audio_triggers.rs`, `Rect`, `mcp/src/profiles.rs`, `AudioPlayer`?**
  _High betweenness centrality (0.151) - this node is a cross-community bridge._
- **Are the 170 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 170 INFERRED edges - model-reasoned connections that need verification._
- **What connects `New`, `NotFound`, `Transfer` to the rest of the system?**
  _530 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11904761904761904 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `paint_item`, `library_panel.rs`, `runtime.rs`, `fx.rs`, `Engine`, `theme.rs`, `stage_canvas.rs`, `Venue`, `Snapshot`, `rig.rs`, `mapping/mod.rs`, `views/touch.rs`, `Scene`, `FixtureAction`, `src/mapping.rs`, `DmxCraftApp`, `tools.rs`, `EffectKind`, `ChannelValue`, `TouchLayout`, `app.rs`, `fire`, `BankScope`, `core/src/lib.rs`, `widgets/mod.rs`, `.check_placement`, `feature.rs`, `views/touch.rs`, `ShowError`, `motion.rs`, `engine.rs`, `OrbitCamera`, `show.rs`, `viz/mod.rs`, `scene_grid.rs`, `audio_triggers.rs`, `VizView`?**
  _High betweenness centrality (0.104) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `Show` (e.g. with `Mappings` and `TouchLayout`) actually correct?**
  _`Show` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11330049261083744 - nodes in this community are weakly interconnected._