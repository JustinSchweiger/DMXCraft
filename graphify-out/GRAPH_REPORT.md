# Graph Report - DmxCraft  (2026-11-21)

## Corpus Check
- 175 files · ~336,104 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4710 nodes · 11774 edges · 200 communities (177 shown, 23 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 493 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `258ed3cf`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- DmxOutput
- open_dmx.rs
- core/src/venue.rs
- programmer.rs
- SoundBoard
- steps
- fx.rs
- app/src/obs.rs
- props.rs
- ChannelRef
- target_shape
- viz/library.rs
- jobs
- with
- vm116.rs
- README.md
- ChannelRef
- titlebar.rs
- Control
- Serialize
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- tests.rs
- fx_ui.rs
- MenuAction
- ofl/src/lib.rs
- missing_audio.rs
- views/touch.rs
- dmxcraft
- ChannelDef
- profile_builder.rs
- fixture_menu.rs
- DMXCraft – UI guidelines
- system_fonts.rs
- FixtureAction
- scene_grid.rs
- Color32
- file_dialog.rs
- AudioPlayer
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- settings_window.rs
- updater.rs
- fx/mapping.rs
- remote/mod.rs
- TouchLayout
- send_frame
- audio_trigger_ui.rs
- patch_grid.rs
- app/src/profiles.rs
- SlotKind
- scene_card.rs
- drag.rs
- ButtonStyle
- Knob<'a>
- Grip
- Timeline
- control/timeline.rs
- steps
- Ui
- stage.rs
- app/src/audio.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- viz/mod.rs
- theme.rs
- PaletteStore
- range_slider.rs
- ShutterEffect
- profile.rs
- runtime.rs
- Toggle<'a>
- app.js
- VizView
- ChannelKind
- textures.rs
- CapabilityKind
- Mapper
- MapPreset
- audio_triggers.rs
- qxf.rs
- show.rs
- dialog.rs
- paint_item
- Response
- arc
- Placement
- Venue
- EffectKind
- duration
- snap.rs
- Object
- TouchKind
- .row
- convert.rs
- gltf_import.rs
- plan
- ViewCtx
- global.rs
- IconButton
- CurveShape
- ColorPreset
- qlc-bundle.rs
- Calibrate
- i18n/mod.rs
- mcp/src/venue.rs
- Stroke
- .show
- Deserialize
- Clip
- DeviceModel
- .ui
- Theme
- assist.rs
- time_field.rs
- resolve
- OrbitCamera
- pal
- media.rs
- builtin_fixture_types
- UpdateState
- text_field
- T
- Profile
- FxHasher
- crash.rs
- ev2.rs
- Problem
- server.rs
- mcp.rs
- ShowError
- autosave.rs
- Snapshot
- PlanShape
- midi.rs
- tools.rs
- OpenDmx
- ClipAction
- control/tracking.rs
- bridge.rs
- fader.rs
- scene_props.rs
- paint_clip
- scene_group.rs
- DmxFrame
- super
- env
- de.rs
- menu.rs
- ProfileChannel
- Dialog<'a>
- demo.rs
- build-local-artifacts
- advanced.rs
- app.rs
- AppState
- DeckState
- .play_item
- Drag
- FooterButton<'a>
- views/sound.rs
- look.rs
- announce
- plan.rs
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- super
- Category
- text
- with
- measure.rs
- mcp/src/profiles.rs
- object_box
- Layer
- Cmd
- ProgrammerMode
- MenuItem<'a>
- Quality
- Transform
- mapping/mod.rs

## God Nodes (most connected - your core abstractions)
1. `Ui` - 298 edges
2. `Engine` - 259 edges
3. `t()` - 245 edges
4. `Show` - 199 edges
5. `AppState` - 124 edges
6. `Icon` - 119 edges
7. `ViewCtx` - 100 edges
8. `ChannelKind` - 82 edges
9. `Theme` - 72 edges
10. `ChannelRef` - 67 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `modifier()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `angle_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `grid_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `alt_click()` --calls--> `fill()`  [INFERRED]
  crates/app/src/mapping/mod.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (200 total, 23 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (19): UserProfile, LibraryDrag, BuilderRequest, Edit, New, Open, Template, template() (+11 more)

### Community 2 - "DmxOutput"
Cohesion: 0.10
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), list_serial_ports(), MAB, resolve() (+9 more)

### Community 4 - "core/src/venue.rs"
Cohesion: 0.14
Nodes (9): aim_at_points_beam_to_target(), aiming_a_moving_head_inverts_its_direction(), IDENTITY, moving_head_aim(), moving_head_direction(), rotation(), rotation_order_x_then_y_then_z(), TrussHang (+1 more)

### Community 5 - "programmer.rs"
Cohesion: 0.20
Nodes (20): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup (+12 more)

### Community 6 - "SoundBoard"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (29): apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span(), fbm() (+21 more)

### Community 9 - "app/src/obs.rs"
Cohesion: 0.06
Nodes (43): digest(), hash_and_verify(), hash_password(), hex(), MASTER, unlocks(), verify(), auth_matches_obs_websocket_example() (+35 more)

### Community 10 - "props.rs"
Cohesion: 0.09
Nodes (21): text_field(), DragFixtures, move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order(), apply_texture(), TexAction (+13 more)

### Community 12 - "target_shape"
Cohesion: 0.14
Nodes (25): alt_click(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id(), Pick (+17 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.10
Nodes (19): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+11 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "with"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 16 - "vm116.rs"
Cohesion: 0.09
Nodes (25): all_zero_frame_is_compressed(), COMPRESSED_SLOTS, DATA_SLOTS, devices(), encode_frame(), full_frame_uses_data_reports(), key_of(), KEY_PREFIX (+17 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (41): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+33 more)

### Community 18 - "ChannelRef"
Cohesion: 0.14
Nodes (5): lerp(), PlayingScene, apply_levels(), apply(), ChannelRef

### Community 19 - "titlebar.rs"
Cohesion: 0.15
Nodes (23): AudioInfo, clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu() (+15 more)

### Community 20 - "Control"
Cohesion: 0.06
Nodes (37): default_feedback(), Overlay, MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value (+29 more)

### Community 21 - "Serialize"
Cohesion: 0.08
Nodes (8): APC_MK2_PALETTE, palette_lookup(), B64, b64_decode(), b64_encode(), Blob, ModelPart, part_roundtrip_through_json()

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (44): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+36 more)

### Community 24 - "rig.rs"
Cohesion: 0.14
Nodes (26): Beam, bracket(), build(), cell_light(), color_wheel(), cube(), cylinder(), gobo() (+18 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "tests.rs"
Cohesion: 0.09
Nodes (41): rgb(), a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), end_scene_follows_jump_to() (+33 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.09
Nodes (32): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+24 more)

### Community 29 - "MenuAction"
Cohesion: 0.07
Nodes (27): MenuAction, About, AudioFade, AudioMaster, AudioMute, CheckUpdates, DmxLevels, InstallUpdate (+19 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (16): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+8 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.18
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 32 - "views/touch.rs"
Cohesion: 0.09
Nodes (24): add_page(), COLORS, Drag, Move, Resize, GAP, GRID_STEPS, ImageTarget (+16 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "ChannelDef"
Cohesion: 0.13
Nodes (11): ChannelRange, open_value_takes_the_highest_open_range(), RangeEffect, Closed, Color, Gobo, Open, Other (+3 more)

### Community 36 - "profile_builder.rs"
Cohesion: 0.05
Nodes (58): kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FileJob, Export (+50 more)

### Community 37 - "fixture_menu.rs"
Cohesion: 0.09
Nodes (19): CanvasMode, Edit, Select, fit(), fixture_pos(), apply(), MenuPlace, Patch (+11 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 40 - "FixtureAction"
Cohesion: 0.10
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.07
Nodes (54): CARD_H, COMPACT_H, programmer_values(), Action, Bank, Compact, CreateSuper, Delete (+46 more)

### Community 42 - "Color32"
Cohesion: 0.15
Nodes (23): BAND_HIGH, BAND_LOW, BAND_MID, border(), c(), canvas(), canvas_dim(), canvas_faint() (+15 more)

### Community 43 - "file_dialog.rs"
Cohesion: 0.21
Nodes (5): Pending, Purpose, OpenShow, SaveShowAs, SHOW_EXT

### Community 44 - "AudioPlayer"
Cohesion: 0.14
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "settings_window.rs"
Cohesion: 0.06
Nodes (44): MidiDevice, cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings, ai() (+36 more)

### Community 49 - "updater.rs"
Cohesion: 0.16
Nodes (14): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+6 more)

### Community 50 - "fx/mapping.rs"
Cohesion: 0.18
Nodes (10): every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level(), transform() (+2 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (28): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+20 more)

### Community 52 - "TouchLayout"
Cohesion: 0.16
Nodes (8): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), LONG_PRESS_MS, pages_can_be_reordered(), TouchImage, TouchLayout, TouchPage

### Community 53 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 54 - "audio_trigger_ui.rs"
Cohesion: 0.25
Nodes (9): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+1 more)

### Community 55 - "patch_grid.rs"
Cohesion: 0.22
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 57 - "app/src/profiles.rs"
Cohesion: 0.26
Nodes (9): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+1 more)

### Community 58 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 59 - "scene_card.rs"
Cohesion: 0.29
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 60 - "drag.rs"
Cohesion: 0.46
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 61 - "ButtonStyle"
Cohesion: 0.25
Nodes (8): style_name(), ButtonStyle, .ALL, Led, Normal, Outline, Round, Switch

### Community 63 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 65 - "Timeline"
Cohesion: 0.08
Nodes (25): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Group, Obs, Scene, Video, fade_curve_fast_then_slow() (+17 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.08
Nodes (22): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, has_audio(), HEADER_W, LANE_H, LaneDrag (+14 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "Ui"
Cohesion: 0.06
Nodes (40): moved_out(), programmer_rail(), t(), channel_label(), combo(), hint(), LIST_H, MAP_EXT (+32 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (26): select_all(), copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New (+18 more)

### Community 71 - "app/src/audio.rs"
Cohesion: 0.12
Nodes (16): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+8 more)

### Community 72 - "Show"
Cohesion: 0.16
Nodes (10): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap (+2 more)

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (18): block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward, Outward (+10 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (34): along(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg, Easing (+26 more)

### Community 77 - "engine.rs"
Cohesion: 0.09
Nodes (17): FadeState, In, Out, MAX_STEP_ADVANCES_PER_TICK, PAN_DEG_PER_S, RELEASE_DARK_MS, RELEASE_LIGHT_MS, SETTLE_MS (+9 more)

### Community 78 - "viz/mod.rs"
Cohesion: 0.12
Nodes (27): AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, hit_y(), model_tex_id(), move_all() (+19 more)

### Community 79 - "theme.rs"
Cohesion: 0.10
Nodes (25): CONTRAST, DAYLIGHT, EMBER, FS_BODY, FS_CAPTION, FS_HEADING, FS_TITLE, HEADING_FAMILY (+17 more)

### Community 80 - "PaletteStore"
Cohesion: 0.57
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 86 - "range_slider.rs"
Cohesion: 0.29
Nodes (5): from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 87 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (19): beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+11 more)

### Community 90 - "runtime.rs"
Cohesion: 0.13
Nodes (12): engine_keeps_running_without_device(), Failing, output_keeps_running_after_a_panic_elsewhere(), OutputStatus, Connecting, Error, Running, Stopped (+4 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (68): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+60 more)

### Community 93 - "VizView"
Cohesion: 0.12
Nodes (11): Missing, Mode, Build, Live, VizView, FixtureSort, Address, Name (+3 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (26): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+18 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Mapper"
Cohesion: 0.30
Nodes (3): is_value_control(), Mapper, Sent

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.09
Nodes (17): compute(), env_key(), EnvState, Failed, Loading, Ready, fire(), is_fader() (+9 more)

### Community 101 - "qxf.rs"
Cohesion: 0.15
Nodes (12): Physics, channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical() (+4 more)

### Community 102 - "show.rs"
Cohesion: 0.08
Nodes (28): banks_of(), address_map_marks_ranges(), BankScope, All, Bank, Banks, Off, OutsideBank (+20 more)

### Community 104 - "dialog.rs"
Cohesion: 0.12
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 105 - "paint_item"
Cohesion: 0.20
Nodes (17): content_rect(), fader_parts(), H_VALUE_W, icons_only(), label_galley(), label_with_glyphs(), led_color(), paint_cover() (+9 more)

### Community 106 - "Response"
Cohesion: 0.12
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, chip(), divider() (+6 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.23
Nodes (14): core_direction_matches_visualizer(), hanging_moving_head_directions(), angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head (+6 more)

### Community 109 - "Venue"
Cohesion: 0.10
Nodes (12): renumber(), duplicate_object(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, mul() (+4 more)

### Community 110 - "EffectKind"
Cohesion: 0.10
Nodes (9): ChannelValue, Scene, SceneKind, Fx, Static, Super, Step, super_scene_has_no_values_of_its_own() (+1 more)

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "snap.rs"
Cohesion: 0.17
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 117 - "Object"
Cohesion: 0.23
Nodes (8): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE

### Community 118 - "TouchKind"
Cohesion: 0.13
Nodes (16): display_label(), Item, item_target(), mapping_of(), target_entry(), short_name(), target_menu(), TouchItem (+8 more)

### Community 119 - ".row"
Cohesion: 0.18
Nodes (16): DragObjects, Renaming, Object, Tree, TreeAction, Delete, Drop, Duplicate (+8 more)

### Community 121 - "convert.rs"
Cohesion: 0.14
Nodes (26): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), Lookup (+18 more)

### Community 123 - "gltf_import.rs"
Cohesion: 0.17
Nodes (8): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), RawPart, smooth_normals()

### Community 127 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 128 - "ViewCtx"
Cohesion: 0.09
Nodes (18): fold_section(), section_header(), View, ViewCtx, body(), caption(), FixtureSettings, hanging_of() (+10 more)

### Community 130 - "global.rs"
Cohesion: 0.21
Nodes (16): apply(), button(), fader(), hidden_tracking_follow_works(), Input, Press, Value, MAX_HEIGHT (+8 more)

### Community 131 - "IconButton"
Cohesion: 0.15
Nodes (3): ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.15
Nodes (12): all_presets_render_every_beam(), ColorFx, ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma (+4 more)

### Community 134 - "qlc-bundle.rs"
Cohesion: 0.21
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 137 - "i18n/mod.rs"
Cohesion: 0.13
Nodes (10): all_texts_translated(), DE, fill(), Lang, .ALL, De, En, tr! (+2 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.38
Nodes (11): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+3 more)

### Community 140 - "Stroke"
Cohesion: 0.09
Nodes (14): color32(), DipSwitch, Knob, SIZE, SWEEP, LimitPad, RangeSlider, Segmented (+6 more)

### Community 141 - ".show"
Cohesion: 0.22
Nodes (6): Camera, fixture_rect(), live_colors(), StageCanvas, StageInteraction, strobe_hz()

### Community 142 - "Deserialize"
Cohesion: 0.31
Nodes (7): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media

### Community 143 - "Clip"
Cohesion: 0.10
Nodes (11): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth, Layer (+3 more)

### Community 144 - "DeviceModel"
Cohesion: 0.20
Nodes (5): DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output()

### Community 145 - ".ui"
Cohesion: 0.25
Nodes (3): fmt_ms(), mark_clipboard(), TimelineView

### Community 146 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 147 - "assist.rs"
Cohesion: 0.26
Nodes (8): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, CameraPreset

### Community 148 - "time_field.rs"
Cohesion: 0.18
Nodes (12): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), STEPS_MS, time_field() (+4 more)

### Community 149 - "resolve"
Cohesion: 0.25
Nodes (6): Active, Overrides, overrides_apply(), resolve(), set_overrides(), set_theme()

### Community 151 - "pal"
Cohesion: 0.26
Nodes (11): accent(), accent_dim(), accent_hover(), apply(), bg(), fixture_bar(), install_fonts(), pal() (+3 more)

### Community 152 - "media.rs"
Cohesion: 0.14
Nodes (14): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), drop_hint(), has_sound_cached(), is_video() (+6 more)

### Community 153 - "builtin_fixture_types"
Cohesion: 0.16
Nodes (7): defaults_are_rendered(), dimmer_effects_dim_rgb_only_fixtures_lit_by_another_scene(), limits_cap_dimmer_and_position(), tracking_aims_heads_at_the_point_and_releases_softly(), builtin_fixture_types(), big_show(), large_show_renders_within_a_tick()

### Community 154 - "UpdateState"
Cohesion: 0.14
Nodes (12): is_newer(), run_updater(), Updater, UpdateState, Available, Checking, Error, Idle (+4 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (16): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS (+8 more)

### Community 158 - "Profile"
Cohesion: 0.13
Nodes (5): AppProfiles, import(), fine_names(), Profile, rename_and_resolution_update_modes()

### Community 163 - "crash.rs"
Cohesion: 0.16
Nodes (11): arm(), Armed, civil(), emergency_save(), install(), log_file(), marker_file(), MAX_LOG (+3 more)

### Community 164 - "ev2.rs"
Cohesion: 0.12
Nodes (21): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+13 more)

### Community 165 - "Problem"
Cohesion: 0.14
Nodes (20): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+12 more)

### Community 166 - "server.rs"
Cohesion: 0.18
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 167 - "mcp.rs"
Cohesion: 0.12
Nodes (13): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+5 more)

### Community 168 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 169 - "autosave.rs"
Cohesion: 0.18
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 170 - "Snapshot"
Cohesion: 0.29
Nodes (6): following_is_no_undo_step(), History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "PlanShape"
Cohesion: 0.12
Nodes (19): AMBER, BOX, paint_effects(), paint_fixture(), paint_live_corner(), paint_shape_icon(), UV, WHITE (+11 more)

### Community 173 - "midi.rs"
Cohesion: 0.12
Nodes (11): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+3 more)

### Community 176 - "tools.rs"
Cohesion: 0.06
Nodes (86): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+78 more)

### Community 177 - "OpenDmx"
Cohesion: 0.28
Nodes (4): missing_cable_is_reported_not_fatal(), missing_vm116_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.09
Nodes (14): bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT, RoomMap, TrackingView (+6 more)

### Community 181 - "bridge.rs"
Cohesion: 0.43
Nodes (5): dechunk(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 182 - "fader.rs"
Cohesion: 0.10
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.07
Nodes (35): apply_to_others(), BAR_H, bar_slider(), block_props(), child(), chip(), effect_header(), effect_tiles() (+27 more)

### Community 185 - "paint_clip"
Cohesion: 0.22
Nodes (12): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle, paint_bands(), paint_beats() (+4 more)

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "DmxFrame"
Cohesion: 0.12
Nodes (4): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE, Engine

### Community 195 - "super"
Cohesion: 0.32
Nodes (4): on_switches(), roundtrip(), switch_value(), SWITCHES

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 200 - "menu.rs"
Cohesion: 0.15
Nodes (9): begin(), ICON_SLOT, item(), ITEM_H, MenuItem, MIN_W, separator(), sub() (+1 more)

### Community 201 - "ProfileChannel"
Cohesion: 0.18
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 203 - "demo.rs"
Cohesion: 0.36
Nodes (5): cv(), demo_is_valid(), demo_show(), empty_show(), ensure_builtin_types()

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "advanced.rs"
Cohesion: 0.20
Nodes (12): advanced(), pick_tile(), Picker, picker_window(), PickTarget, Jump, Protect, Release (+4 more)

### Community 210 - "app.rs"
Cohesion: 0.11
Nodes (16): block_input(), ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, RAIL_W, viewport_input() (+8 more)

### Community 218 - "AppState"
Cohesion: 0.08
Nodes (16): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+8 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 221 - ".play_item"
Cohesion: 0.24
Nodes (10): apply(), find_item(), state_json(), tracking_moves_and_switches_a_target(), channels_of(), current_color(), current_position(), item_value() (+2 more)

### Community 226 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 231 - "look.rs"
Cohesion: 0.14
Nodes (21): fader_ranges(), ChannelDef, FixtureType, ft(), color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget (+13 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 234 - "plan.rs"
Cohesion: 0.27
Nodes (7): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread()

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 241 - "video_out.rs"
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 243 - "super"
Cohesion: 0.24
Nodes (4): glyph(), LIBRARY, paint(), ICON_FAMILY

### Community 244 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 249 - "text"
Cohesion: 0.29
Nodes (5): body(), caption(), heading_font(), heading_text(), text()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 272 - "measure.rs"
Cohesion: 0.24
Nodes (5): End, Measure, snap_points(), SNAP_PX, project()

### Community 276 - "mcp/src/profiles.rs"
Cohesion: 0.16
Nodes (11): DEFAULT_PORT, MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format(), get(), list() (+3 more)

### Community 309 - "object_box"
Cohesion: 0.22
Nodes (7): floor_plan(), FloorShape, Handles, object_box(), paint_box(), seg_dist(), shape_mesh()

### Community 340 - "Layer"
Cohesion: 0.15
Nodes (8): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On

### Community 341 - "Cmd"
Cohesion: 0.29
Nodes (7): Cmd, Color, Press, Track, TrackToggle, Value, Xy

### Community 342 - "ProgrammerMode"
Cohesion: 0.25
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 345 - "Quality"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 346 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 348 - "mapping/mod.rs"
Cohesion: 0.15
Nodes (23): bank_step_wraps(), button(), channel_mapping_sets_live_value(), channel_name(), color(), describe(), describe_mapping(), fire() (+15 more)

## Knowledge Gaps
- **591 isolated node(s):** `Loading`, `Loading`, `Failed`, `OpenShow`, `SaveShowAs` (+586 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1486 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **23 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `ChannelRef` to `ViewCtx`, `library_panel.rs`, `global.rs`, `programmer.rs`, `Calibrate`, `app/src/obs.rs`, `mcp/src/venue.rs`, `Stroke`, `.ui`, `ChannelRef`, `time_field.rs`, `mcp/src/profiles.rs`, `rig.rs`, `views/touch.rs`, `crash.rs`, `profile_builder.rs`, `fixture_menu.rs`, `mcp.rs`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `control/tracking.rs`, `audio_trigger_ui.rs`, `scene_props.rs`, `DmxFrame`, `control/timeline.rs`, `Ui`, `stage.rs`, `app/src/audio.rs`, `Show`, `engine.rs`, `advanced.rs`, `app.rs`, `Layer`, `runtime.rs`, `mapping/mod.rs`, `.play_item`, `ChannelKind`, `Mapper`, `audio_triggers.rs`, `show.rs`, `paint_item`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **Are the 201 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 201 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `Loading`, `Failed` to the rest of the system?**
  _591 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.12941176470588237 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `ViewCtx`, `SoundPlayer`, `core/src/lib.rs`, `fx.rs`, `app/src/obs.rs`, `ChannelRef`, `target_shape`, `.show`, `measure.rs`, `ChannelRef`, `Control`, `media.rs`, `rig.rs`, `builtin_fixture_types`, `tests.rs`, `ofl/src/lib.rs`, `missing_audio.rs`, `views/touch.rs`, `fixture_menu.rs`, `FixtureAction`, `autosave.rs`, `Snapshot`, `PlanShape`, `scene_grid.rs`, `ShowError`, `DmxCraftApp`, `tools.rs`, `remote/mod.rs`, `control/tracking.rs`, `object_box`, `TouchLayout`, `scene_props.rs`, `scene_group.rs`, `Ui`, `feature.rs`, `demo.rs`, `motion.rs`, `engine.rs`, `viz/mod.rs`, `theme.rs`, `app.rs`, `AppState`, `runtime.rs`, `mapping/mod.rs`, `.play_item`, `show.rs`, `look.rs`, `Placement`, `Venue`, `EffectKind`, `snap.rs`, `Object`, `TouchKind`?**
  _High betweenness centrality (0.114) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11363636363636363 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `Ui` to `ViewCtx`, `library_panel.rs`, `programmer.rs`, `Calibrate`, `i18n/mod.rs`, `props.rs`, `Stroke`, `viz/library.rs`, `measure.rs`, `.ui`, `titlebar.rs`, `time_field.rs`, `media.rs`, `text_field`, `fx_ui.rs`, `missing_audio.rs`, `views/touch.rs`, `profile_builder.rs`, `fixture_menu.rs`, `FixtureAction`, `scene_grid.rs`, `file_dialog.rs`, `DmxCraftApp`, `settings_window.rs`, `control/tracking.rs`, `fader.rs`, `audio_trigger_ui.rs`, `scene_props.rs`, `app/src/profiles.rs`, `paint_clip`, `patch_grid.rs`, `ButtonStyle`, `control/timeline.rs`, `stage.rs`, `viz/mod.rs`, `PaletteStore`, `advanced.rs`, `app.rs`, `Quality`, `mapping/mod.rs`, `textures.rs`, `views/sound.rs`, `look.rs`, `dialog.rs`, `Response`, `snap.rs`, `TouchKind`, `.row`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._