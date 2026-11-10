# Graph Report - dmxcraft-src  (2026-11-10)

## Corpus Check
- 145 files · ~220,551 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4063 nodes · 10926 edges · 259 communities (160 shown, 99 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 530 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3a7f5181`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
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
- app/src/obs.rs
- viz/library.rs
- jobs
- steps
- Tastatur- und MIDI-Zuordnung
- titlebar.rs
- GH_TOKEN
- build-local-artifacts
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- mcp/src/profiles.rs
- with
- ProfileBuilder
- ofl/src/lib.rs
- fx_ui.rs
- env
- dmxcraft
- app/src/audio.rs
- t
- fx/mapping.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- range_slider.rs
- textures.rs
- Obs
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- form
- super
- remote/mod.rs
- TouchLayout
- assist.rs
- MenuAction
- ViewCtx
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
- VizView
- Quality
- Deserialize
- .push
- core/src/obs.rs
- arc
- Control
- Rect
- app.rs
- server.rs
- ev2.rs
- Venue
- dialog.rs
- outline.rs
- AudioGrid
- convert.rs
- Timeline
- Host
- widgets/mod.rs
- ShutterEffect
- protocol.rs
- midi_device.rs
- CurveShape
- ColorPreset
- Shape
- control/timeline.rs
- i18n/mod.rs
- lock.rs
- theme.rs
- stage_canvas.rs
- core/src/venue.rs
- missing_audio.rs
- Category
- bridge.rs
- Mapper
- EffectKind
- item
- mapping/mod.rs
- text_dim
- Tab
- icon_library.rs
- ChaserPreset
- audio_trigger_ui.rs
- SlotKind
- core/src/sound.rs
- OutputStatus
- .add_scene
- show.rs
- Tab
- color_picker.rs
- fader.rs
- Transform
- Problem
- views/touch.rs
- Profile
- gltf_import.rs
- BankScope
- super
- Fullscreen
- .update
- midi.rs
- time_field.rs
- tools.rs
- PlanShape
- DmxFrame
- ClipAction
- ProgrammerMode
- qxf.rs
- Fader
- scene_props.rs
- TouchKind
- Engine
- StageCanvas
- super
- windows.rs
- .check_placement
- plan.rs
- WaveState
- DeckState
- qlc-bundle.rs
- ObsSettings
- de.rs
- dip.rs
- ShowError
- ViewCtx
- Purpose
- MenuItem<'a>
- HeaderAction
- IconButton
- Drag
- Scene
- palettes.rs
- Grip
- FooterButton<'a>
- paint_clip
- AudioPlayer
- run
- .fade_state
- FixtureLimits
- panels.rs
- core/src/lib.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 227 edges
2. `t()` - 214 edges
3. `Show` - 147 edges
4. `Icon` - 115 edges
5. `AppState` - 102 edges
6. `text_dim()` - 101 edges
7. `ViewCtx` - 88 edges
8. `ChannelKind` - 75 edges
9. `text()` - 68 edges
10. `text_faint()` - 67 edges

## Surprising Connections (you probably didn't know these)
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (259 total, 99 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.09
Nodes (18): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), LibAction (+10 more)

### Community 2 - "runtime.rs"
Cohesion: 0.20
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.10
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "Theme"
Cohesion: 0.09
Nodes (17): ensure_linux_menu_entry(), register_mime(), window_icon(), main(), active(), set_theme(), Theme, .ALL (+9 more)

### Community 5 - "programmer.rs"
Cohesion: 0.17
Nodes (25): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup (+17 more)

### Community 6 - "AppState"
Cohesion: 0.11
Nodes (14): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+6 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+20 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "Deserialize"
Cohesion: 0.10
Nodes (8): Physics, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 12 - "app/src/obs.rs"
Cohesion: 0.22
Nodes (8): auth_matches_obs_websocket_example(), auth_response(), b64(), open(), read_json(), send_json(), VideoPlay, Ws

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.14
Nodes (13): env, if, needs, runs-on, jobs, announce, name, on (+5 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.04
Nodes (38): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten, Aufbau, Einschalten (+30 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.16
Nodes (21): AudioInfo, clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu() (+13 more)

### Community 20 - "GH_TOKEN"
Cohesion: 0.29
Nodes (8): env, needs, runs-on, env, BUILD_MANIFEST_NAME, GH_TOKEN, build-global-artifacts, env

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.11
Nodes (31): analyze(), bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), BPM_MAX, BPM_MIN, detect_grid(), detect_segments(), energy_curve() (+23 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (23): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+15 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "mcp/src/profiles.rs"
Cohesion: 0.24
Nodes (12): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+4 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 29 - "ProfileBuilder"
Cohesion: 0.18
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.16
Nodes (13): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+5 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.10
Nodes (33): c(), canvas(), canvas_dim(), canvas_faint(), canvas_line(), handle(), color_fx_ui(), edit_colors() (+25 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.12
Nodes (16): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+8 more)

### Community 36 - "t"
Cohesion: 0.10
Nodes (35): t(), text_field(), capability_fields(), category_label(), check(), color_label(), FOOT_H, full_combo() (+27 more)

### Community 37 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 43 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 44 - "Obs"
Cohesion: 0.29
Nodes (6): connects_and_plays_video_block(), engine(), fake_obs(), Obs, wait(), wrong_password_is_reported()

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "form"
Cohesion: 0.16
Nodes (26): Settings, ai(), audio(), customize(), full_combo(), general(), HEIGHT, hint() (+18 more)

### Community 50 - "super"
Cohesion: 0.20
Nodes (5): DeviceModel, .ALL, ApcMiniMk2, Generic, MidiDevice

### Community 51 - "remote/mod.rs"
Cohesion: 0.06
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "assist.rs"
Cohesion: 0.18
Nodes (10): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, decode_texture() (+2 more)

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (26): MenuAction, About, AudioFade, AudioMaster, AudioMute, CheckUpdates, DmxLevels, InstallUpdate (+18 more)

### Community 68 - "ViewCtx"
Cohesion: 0.23
Nodes (5): ViewCtx, targets(), vec3_row(), apply_texture(), VizView

### Community 69 - "window.rs"
Cohesion: 0.16
Nodes (17): channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button(), pick_export() (+9 more)

### Community 70 - "stage.rs"
Cohesion: 0.14
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "mcp.rs"
Cohesion: 0.18
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.20
Nodes (4): .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.18
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+7 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.17
Nodes (35): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+27 more)

### Community 78 - "fixture_settings.rs"
Cohesion: 0.19
Nodes (10): RangeSlider, Quick, quick_action(), Center, Off, On, body(), caption() (+2 more)

### Community 79 - "egui"
Cohesion: 0.04
Nodes (33): DipSwitch, ICON_FAMILY, ICON_FILL_FAMILY, Knob, Knob<'a>, SIZE, SWEEP, LimitPad (+25 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "views/sound.rs"
Cohesion: 0.12
Nodes (25): hovering(), os_cursor(), pointer(), rgb(), surface(), AUDIO_EXT, audio_files(), BAR_H (+17 more)

### Community 87 - "fire"
Cohesion: 0.23
Nodes (13): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+5 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (25): add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys() (+17 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.11
Nodes (31): aim_selected(), apply_placement(), AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles, handles() (+23 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.08
Nodes (49): zone_color(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+41 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (31): kind_label(), CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed (+23 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.10
Nodes (18): env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey, Events (+10 more)

### Community 101 - "VizView"
Cohesion: 0.16
Nodes (8): Missing, Mode, Build, Live, VizView, FixtureSort, Address, Name

### Community 102 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 104 - "Deserialize"
Cohesion: 0.19
Nodes (9): ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other, Strobe (+1 more)

### Community 105 - ".push"
Cohesion: 0.18
Nodes (10): Analyzer, Biquad, Envelope, envelope_filters_band(), FilterKind, Band, High, Low (+2 more)

### Community 106 - "core/src/obs.rs"
Cohesion: 0.31
Nodes (7): bx(), find_box(), mp4_duration_ms(), ObsAction, Scene, Source, reads_mp4_length()

### Community 108 - "Control"
Cohesion: 0.07
Nodes (38): Overlay, MapFile, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control (+30 more)

### Community 109 - "Rect"
Cohesion: 0.20
Nodes (8): add_audio(), add_audio_dialog(), add_video(), fmt_ms(), grid_options(), is_video(), mark_clipboard(), TimelineView

### Community 110 - "app.rs"
Cohesion: 0.13
Nodes (15): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), UpdateBadge (+7 more)

### Community 112 - "server.rs"
Cohesion: 0.19
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Venue"
Cohesion: 0.16
Nodes (6): renumber(), fixture_folders_are_exclusive(), FixtureFolder, Object, room_and_tree(), Venue

### Community 118 - "dialog.rs"
Cohesion: 0.07
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (25): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+17 more)

### Community 120 - "AudioGrid"
Cohesion: 0.19
Nodes (4): AudioAnalysis, AudioGrid, AudioSegment, grid_helpers()

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (24): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+16 more)

### Community 123 - "Timeline"
Cohesion: 0.09
Nodes (19): has_audio(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Obs, Scene, Video (+11 more)

### Community 127 - "Host"
Cohesion: 0.21
Nodes (9): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+1 more)

### Community 128 - "widgets/mod.rs"
Cohesion: 0.11
Nodes (14): apply(), View, SetupView, Arrange, AlignH, AlignV, Grid, Reset (+6 more)

### Community 129 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 130 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 131 - "midi_device.rs"
Cohesion: 0.24
Nodes (7): default_feedback(), APC_MK2_PALETTE, color32(), detect_and_match(), matching_output(), nearest(), palette_lookup()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.10
Nodes (18): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, Mark, OBS_COLOR (+10 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (15): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 138 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (38): accent_hover(), ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, BLACK, CONTRAST, DANGER (+30 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 142 - "core/src/venue.rs"
Cohesion: 0.11
Nodes (14): aim_at_points_beam_to_target(), apply(), close(), Environment, hang_below_nearest_truss(), IDENTITY, Material, mul() (+6 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 144 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 145 - "bridge.rs"
Cohesion: 0.46
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 147 - "EffectKind"
Cohesion: 0.13
Nodes (12): Effect, EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings (+4 more)

### Community 149 - "item"
Cohesion: 0.10
Nodes (25): audio_control(), select_all(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item() (+17 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.13
Nodes (30): alt_click(), button_rect(), color(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id() (+22 more)

### Community 151 - "text_dim"
Cohesion: 0.13
Nodes (44): table(), sidebar(), load_meter(), missing_alert(), show(), accent(), accent_dim(), apply() (+36 more)

### Community 152 - "Tab"
Cohesion: 0.25
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 154 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 155 - "audio_trigger_ui.rs"
Cohesion: 0.23
Nodes (7): channel_name(), describe(), describe_mapping(), t_deleted(), apply_trigger_pick(), trigger_cx(), TriggerCx

### Community 156 - "SlotKind"
Cohesion: 0.19
Nodes (8): slot_label(), SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 157 - "core/src/sound.rs"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 158 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 159 - ".add_scene"
Cohesion: 0.19
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize()

### Community 160 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 161 - "Tab"
Cohesion: 0.15
Nodes (12): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+4 more)

### Community 162 - "color_picker.rs"
Cohesion: 0.16
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 163 - "fader.rs"
Cohesion: 0.12
Nodes (13): DOT_H, FADER_W, FaderEvent, Changed, None, Release, ToggleFx, fine_mark() (+5 more)

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.09
Nodes (27): apply(), state_json(), add_page(), apply_pick(), channels_of(), COLORS, current_color(), current_position() (+19 more)

### Community 167 - "Profile"
Cohesion: 0.10
Nodes (15): fill_types(), AppProfiles, delete(), file(), load_all(), load_user(), save(), sort() (+7 more)

### Community 168 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 169 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 170 - "super"
Cohesion: 0.20
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 171 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 172 - ".update"
Cohesion: 0.48
Nodes (4): fire(), media_action(), media_settings(), seek()

### Community 173 - "midi.rs"
Cohesion: 0.13
Nodes (12): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+4 more)

### Community 174 - "time_field.rs"
Cohesion: 0.27
Nodes (10): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.08
Nodes (73): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), base64(), boolean(), build_steps() (+65 more)

### Community 177 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 179 - "ClipAction"
Cohesion: 0.11
Nodes (17): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+9 more)

### Community 180 - "ProgrammerMode"
Cohesion: 0.29
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 181 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 182 - "Fader"
Cohesion: 0.19
Nodes (3): Fader, FaderRange, fader_ranges()

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (38): button(), time_field(), advanced(), apply_to_others(), banks_of(), block_props(), fit_mapping(), FX_EXT (+30 more)

### Community 184 - "TouchKind"
Cohesion: 0.17
Nodes (12): display_label(), item_target(), mapping_of(), target_entry(), TouchItem, TouchKind, Button, Color (+4 more)

### Community 185 - "Engine"
Cohesion: 0.10
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 186 - "StageCanvas"
Cohesion: 0.33
Nodes (5): CanvasMode, Edit, Select, StageCanvas, Underlay

### Community 187 - "super"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 191 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 193 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 196 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 200 - "dip.rs"
Cohesion: 0.46
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ViewCtx"
Cohesion: 0.17
Nodes (9): content_rect(), fader_parts(), ImageTarget, Item, Page, paint_cover(), pick_image(), swatch_row() (+1 more)

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 211 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 213 - "Scene"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 223 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 224 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 249 - "paint_clip"
Cohesion: 0.38
Nodes (5): paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers()

### Community 251 - "AudioPlayer"
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 268 - "run"
Cohesion: 0.24
Nodes (8): run(), session(), Shared, Status, Connected, Connecting, Failed, Off

### Community 273 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 278 - "panels.rs"
Cohesion: 0.11
Nodes (13): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, OutlineTab, Fixtures (+5 more)

### Community 283 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (12): UNIVERSE_SIZE, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind, Fx (+4 more)

## Knowledge Gaps
- **546 isolated node(s):** `Loading`, `Loading`, `Failed`, `OpenShow`, `SaveShowAs` (+541 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1293 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **99 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `widgets/mod.rs`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `Engine`, `app/src/obs.rs`, `.fade_state`, `Mapper`, `item`, `mapping/mod.rs`, `text_dim`, `rig.rs`, `audio_trigger_ui.rs`, `core/src/lib.rs`, `ProfileBuilder`, `mcp/src/profiles.rs`, `app/src/audio.rs`, `t`, `views/touch.rs`, `Profile`, `FixtureAction`, `.update`, `DmxCraftApp`, `form`, `tools.rs`, `DmxFrame`, `remote/mod.rs`, `scene_props.rs`, `super`, `ViewCtx`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `engine.rs`, `fixture_settings.rs`, `fire`, `ChannelKind`, `scene_grid.rs`, `audio_triggers.rs`, `Timeline`, `Rect`, `app.rs`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._
- **Are the 175 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 175 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `Loading`, `Failed` to the rest of the system?**
  _546 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.1310483870967742 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `widgets/mod.rs`, `library_panel.rs`, `programmer.rs`, `control/timeline.rs`, `i18n/mod.rs`, `viz/library.rs`, `missing_audio.rs`, `titlebar.rs`, `item`, `panels.rs`, `text_dim`, `audio_trigger_ui.rs`, `SlotKind`, `ProfileBuilder`, `fx_ui.rs`, `color_picker.rs`, `fader.rs`, `Problem`, `views/touch.rs`, `Profile`, `FixtureAction`, `super`, `textures.rs`, `DmxCraftApp`, `form`, `tools.rs`, `Fader`, `scene_props.rs`, `TouchKind`, `windows.rs`, `ViewCtx`, `window.rs`, `stage.rs`, `ViewCtx`, `Purpose`, `fixture_settings.rs`, `egui`, `views/sound.rs`, `viz/mod.rs`, `palettes.rs`, `scene_grid.rs`, `CapabilityKind`, `Quality`, `Rect`, `app.rs`, `dialog.rs`, `outline.rs`?**
  _High betweenness centrality (0.100) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08771929824561403 - nodes in this community are weakly interconnected._