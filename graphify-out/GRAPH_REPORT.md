# Graph Report - dmxcraft-src  (2026-11-10)

## Corpus Check
- 148 files · ~225,507 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4125 nodes · 10927 edges · 258 communities (151 shown, 107 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 531 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bd39df23`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- Theme
- programmer.rs
- Clip
- steps
- fx.rs
- OutputError
- outline.rs
- midi_device.rs
- src/mapping.rs
- viz/library.rs
- jobs
- steps
- README.md
- titlebar.rs
- Capability
- textures.rs
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- Profile
- .push
- dialog.rs
- ofl/src/lib.rs
- fx_ui.rs
- env
- dmxcraft
- app/src/audio.rs
- t
- AudioGrid
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- range_slider.rs
- Deserialize
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- settings_window.rs
- Snapshot
- remote/mod.rs
- TouchLayout
- Dialog<'a>
- MenuAction
- window.rs
- stage.rs
- mcp.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- fixture_settings.rs
- egui
- OpenDmx
- views/sound.rs
- fire
- profile.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- OrbitCamera
- MapPreset
- audio_triggers.rs
- View
- show.rs
- FixtureType
- mcp/src/profiles.rs
- ShutterEffect
- arc
- Control
- Timeline
- app.rs
- server.rs
- Shape
- Venue
- BankScope
- .row
- Box<dyn serialport::SerialPort>
- convert.rs
- Timeline
- Host
- widgets/mod.rs
- protocol.rs
- app/src/obs.rs
- SlotKind
- CurveShape
- ColorPreset
- UpdateBadge
- control/timeline.rs
- i18n/mod.rs
- bridge.rs
- theme.rs
- stage_canvas.rs
- Tab
- missing_audio.rs
- DeviceModel
- fader.rs
- Mapper
- announce
- item
- mapping/mod.rs
- text_dim
- time_field.rs
- icon_library.rs
- build-local-artifacts
- Tab
- apply_pick
- .add_scene
- core/src/lib.rs
- dip.rs
- Transform
- Problem
- views/touch.rs
- AppState
- Category
- Clip
- Fullscreen
- TriggerBand
- midi.rs
- qlc-bundle.rs
- tools.rs
- PlanShape
- DmxFrame
- ClipAction
- with
- qxf.rs
- Fader
- scene_props.rs
- TouchKind
- Engine
- .fade_state
- plan.rs
- Settings
- GridMode
- WaveState
- de.rs
- ShowError
- ImageTarget
- Purpose
- MenuItem<'a>
- HeaderAction
- IconButton
- Drag
- Scene
- Quality
- FooterButton<'a>
- T
- state_json
- Grip
- release.ps1
- desktop.rs
- release-install.md
- AudioPlayer
- t

## God Nodes (most connected - your core abstractions)
1. `Engine` - 228 edges
2. `t()` - 209 edges
3. `Show` - 147 edges
4. `Icon` - 115 edges
5. `AppState` - 102 edges
6. `ViewCtx` - 88 edges
7. `text_dim()` - 87 edges
8. `ChannelKind` - 75 edges
9. `border()` - 65 edges
10. `text()` - 65 edges

## Surprising Connections (you probably didn't know these)
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (258 total, 107 thin omitted)

### Community 0 - "SoundPlayer"
Cohesion: 0.06
Nodes (20): DeckState, Empty, Ended, Fading, Paused, Playing, Ready, SoundPlayer (+12 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 5 - "programmer.rs"
Cohesion: 0.16
Nodes (24): kind_color(), color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup (+16 more)

### Community 6 - "Clip"
Cohesion: 0.38
Nodes (5): paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers()

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.16
Nodes (31): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+23 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "outline.rs"
Cohesion: 0.06
Nodes (32): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, DragFixtures, FixtureSort (+24 more)

### Community 11 - "midi_device.rs"
Cohesion: 0.24
Nodes (7): default_feedback(), APC_MK2_PALETTE, color32(), detect_and_match(), matching_output(), nearest(), palette_lookup()

### Community 12 - "src/mapping.rs"
Cohesion: 0.12
Nodes (16): add_find_and_json(), channels_key(), Feedback, FeedbackState, Input, Key, Midi, Mapping (+8 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.11
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

### Community 20 - "Capability"
Cohesion: 0.19
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 21 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.11
Nodes (31): analyze(), bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), BPM_MAX, BPM_MIN, detect_grid(), detect_segments(), energy_curve() (+23 more)

### Community 24 - "rig.rs"
Cohesion: 0.08
Nodes (33): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+25 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.20
Nodes (10): if, needs, outputs, runs-on, host, publishing, tag, tag-flag (+2 more)

### Community 27 - "Profile"
Cohesion: 0.10
Nodes (14): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+6 more)

### Community 28 - ".push"
Cohesion: 0.18
Nodes (10): Analyzer, Biquad, Envelope, envelope_filters_band(), FilterKind, Band, High, Low (+2 more)

### Community 29 - "dialog.rs"
Cohesion: 0.09
Nodes (13): Button, Dialog, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.13
Nodes (26): canvas_dim(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore (+18 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.14
Nodes (13): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+5 more)

### Community 36 - "t"
Cohesion: 0.08
Nodes (37): capability_fields(), check(), FileJob, Export, Import, FOOT_H, full_combo(), hex_field() (+29 more)

### Community 37 - "AudioGrid"
Cohesion: 0.21
Nodes (4): AudioAnalysis, AudioGrid, AudioSegment, grid_helpers()

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 43 - "Deserialize"
Cohesion: 0.09
Nodes (17): builtin(), load_user(), named(), NamedPalette, path(), save_user(), bx(), find_box() (+9 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (24): ai(), audio(), customize(), full_combo(), general(), HEIGHT, hint(), midi() (+16 more)

### Community 50 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (27): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+19 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "MenuAction"
Cohesion: 0.07
Nodes (34): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+26 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (23): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+15 more)

### Community 70 - "stage.rs"
Cohesion: 0.18
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.17
Nodes (9): .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType (+1 more)

### Community 73 - "feature.rs"
Cohesion: 0.09
Nodes (25): FxPreview, apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.20
Nodes (35): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+27 more)

### Community 78 - "fixture_settings.rs"
Cohesion: 0.10
Nodes (13): ProgrammerMode, Blind, Edit, Live, items_mode(), Layer, Live, Programmer (+5 more)

### Community 79 - "egui"
Cohesion: 0.05
Nodes (28): DipSwitch, Knob, Knob<'a>, SIZE, SWEEP, LimitPad, CARD_H, clip() (+20 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "views/sound.rs"
Cohesion: 0.13
Nodes (25): hovering(), os_cursor(), pointer(), handle(), rgb(), surface(), AUDIO_EXT, audio_files() (+17 more)

### Community 87 - "fire"
Cohesion: 0.23
Nodes (13): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+5 more)

### Community 89 - "profile.rs"
Cohesion: 0.10
Nodes (19): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS, KEY_PREFIX (+11 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (38): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles (+30 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.05
Nodes (65): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+57 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.12
Nodes (10): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, OrbitCamera (+2 more)

### Community 99 - "MapPreset"
Cohesion: 0.07
Nodes (31): default_palette(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, MapPreset, .ALL, Bounce (+23 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.13
Nodes (13): env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey, Events (+5 more)

### Community 101 - "View"
Cohesion: 0.20
Nodes (5): apply(), View, SetupView, StageEditorView, tool_sep()

### Community 102 - "show.rs"
Cohesion: 0.28
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 104 - "FixtureType"
Cohesion: 0.09
Nodes (17): ChannelDef, ChannelRange, FixtureType, Physics, RangeEffect, Closed, Color, Gobo (+9 more)

### Community 105 - "mcp/src/profiles.rs"
Cohesion: 0.37
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 106 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Control"
Cohesion: 0.09
Nodes (22): Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout, Channel, Channels (+14 more)

### Community 109 - "Timeline"
Cohesion: 0.15
Nodes (5): add_video(), has_audio(), mark_clipboard(), TimelineView, Timeline

### Community 110 - "app.rs"
Cohesion: 0.22
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView

### Community 112 - "server.rs"
Cohesion: 0.18
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 113 - "Shape"
Cohesion: 0.21
Nodes (13): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+5 more)

### Community 117 - "Venue"
Cohesion: 0.08
Nodes (19): renumber(), aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss() (+11 more)

### Community 118 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 119 - ".row"
Cohesion: 0.19
Nodes (15): DragObjects, Renaming, Object, Tree, TreeAction, Delete, Drop, Duplicate (+7 more)

### Community 121 - "convert.rs"
Cohesion: 0.11
Nodes (26): names(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+18 more)

### Community 123 - "Timeline"
Cohesion: 0.12
Nodes (12): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Obs, Scene, Video, fades(), group_moves_together_without_covering_others() (+4 more)

### Community 127 - "Host"
Cohesion: 0.21
Nodes (9): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+1 more)

### Community 128 - "widgets/mod.rs"
Cohesion: 0.12
Nodes (15): CanvasMode, Edit, Select, StageCanvas, Underlay, Arrange, AlignH, AlignV (+7 more)

### Community 129 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.07
Nodes (36): digest(), hash_and_verify(), hash_password(), hex(), verify(), auth_matches_obs_websocket_example(), auth_response(), b64() (+28 more)

### Community 131 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 136 - "control/timeline.rs"
Cohesion: 0.11
Nodes (20): add_audio(), add_audio_dialog(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, is_video() (+12 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "bridge.rs"
Cohesion: 0.29
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 140 - "theme.rs"
Cohesion: 0.06
Nodes (50): Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas(), canvas_faint() (+42 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 142 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.17
Nodes (11): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+3 more)

### Community 144 - "DeviceModel"
Cohesion: 0.20
Nodes (5): DeviceModel, .ALL, ApcMiniMk2, Generic, MidiDevice

### Community 145 - "fader.rs"
Cohesion: 0.12
Nodes (13): DOT_H, FADER_W, FaderEvent, Changed, None, Release, ToggleFx, fine_mark() (+5 more)

### Community 147 - "announce"
Cohesion: 0.22
Nodes (10): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+2 more)

### Community 149 - "item"
Cohesion: 0.09
Nodes (27): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+19 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.14
Nodes (29): alt_click(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id(), Pick (+21 more)

### Community 151 - "text_dim"
Cohesion: 0.15
Nodes (41): table(), window(), accent(), accent_dim(), accent_hover(), apply(), bg(), body() (+33 more)

### Community 152 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 155 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 157 - "Tab"
Cohesion: 0.25
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 158 - "apply_pick"
Cohesion: 0.38
Nodes (7): channel_name(), describe(), describe_mapping(), t_deleted(), apply_pick(), pick_adds_scene_button_and_assigns_to_selection(), programmer_fader_becomes_fixed_channels()

### Community 159 - ".add_scene"
Cohesion: 0.10
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 160 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (10): FixtureLimits, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static (+2 more)

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.11
Nodes (24): COLORS, content_rect(), display_label(), Drag, Move, Resize, fader_parts(), GAP (+16 more)

### Community 167 - "AppState"
Cohesion: 0.08
Nodes (17): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+9 more)

### Community 168 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 171 - "Fullscreen"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 172 - "TriggerBand"
Cohesion: 0.29
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 173 - "midi.rs"
Cohesion: 0.29
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 174 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 176 - "tools.rs"
Cohesion: 0.07
Nodes (84): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+76 more)

### Community 177 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 178 - "DmxFrame"
Cohesion: 0.16
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 179 - "ClipAction"
Cohesion: 0.11
Nodes (17): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+9 more)

### Community 180 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 181 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 182 - "Fader"
Cohesion: 0.19
Nodes (3): Fader, FaderRange, fader_ranges()

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (38): button(), zone_color(), advanced(), apply_to_others(), banks_of(), block_props(), fit_mapping(), FX_EXT (+30 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Engine"
Cohesion: 0.07
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 187 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 194 - "Settings"
Cohesion: 0.13
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 196 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

### Community 197 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ImageTarget"
Cohesion: 0.50
Nodes (4): ImageTarget, Item, Page, apply_texture()

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 207 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 211 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 213 - "Scene"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 226 - "Quality"
Cohesion: 0.17
Nodes (7): Quality, .ALL, High, Low, Medium, Ultra, vec3_row()

### Community 232 - "state_json"
Cohesion: 0.21
Nodes (12): apply(), Cmd, Color, Press, Value, Xy, state_json(), channels_of() (+4 more)

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 251 - "AudioPlayer"
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 278 - "t"
Cohesion: 0.06
Nodes (28): t(), heading_text(), text_field(), RangeSlider, fold_section(), section_header(), ViewCtx, category_label() (+20 more)

## Knowledge Gaps
- **546 isolated node(s):** `General`, `Customize`, `Startup`, `Outputs`, `Midi` (+541 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1321 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **107 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `app/src/obs.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `Mapper`, `item`, `mapping/mod.rs`, `text_dim`, `t`, `rig.rs`, `Profile`, `apply_pick`, `core/src/lib.rs`, `app/src/audio.rs`, `t`, `views/touch.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `Snapshot`, `remote/mod.rs`, `scene_props.rs`, `.fade_state`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `engine.rs`, `fixture_settings.rs`, `fire`, `scene_grid.rs`, `audio_triggers.rs`, `View`, `state_json`, `mcp/src/profiles.rs`, `Timeline`, `app.rs`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.136) - this node is a cross-community bridge._
- **Are the 177 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 177 INFERRED edges - model-reasoned connections that need verification._
- **What connects `General`, `Customize`, `Startup` to the rest of the system?**
  _546 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.05687645687645688 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `widgets/mod.rs`, `SoundPlayer`, `app/src/obs.rs`, `fx.rs`, `theme.rs`, `stage_canvas.rs`, `src/mapping.rs`, `missing_audio.rs`, `mapping/mod.rs`, `rig.rs`, `apply_pick`, `.add_scene`, `views/touch.rs`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `Snapshot`, `TouchLayout`, `scene_props.rs`, `Engine`, `feature.rs`, `ShowError`, `motion.rs`, `engine.rs`, `Scene`, `fire`, `viz/mod.rs`, `scene_grid.rs`, `show.rs`, `state_json`, `FixtureType`, `app.rs`, `Venue`, `BankScope`?**
  _High betweenness centrality (0.115) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11764705882352941 - nodes in this community are weakly interconnected._