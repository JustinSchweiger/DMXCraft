# Graph Report - dmxcraft-src  (2026-11-14)

## Corpus Check
- 166 files · ~273,045 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4692 nodes · 12032 edges · 331 communities (170 shown, 161 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 661 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f1670bb8`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- DmxFrame
- open_dmx.rs
- Venue
- programmer.rs
- SoundBoard
- steps
- fx.rs
- call
- props.rs
- PaletteStore
- target_shape
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- views/touch.rs
- Deserialize
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- Control
- fx_ui.rs
- Host
- ofl/src/lib.rs
- missing_audio.rs
- TouchView
- dmxcraft
- app/src/audio.rs
- t
- menu.rs
- DMXCraft – UI guidelines
- system_fonts.rs
- FixtureAction
- scene_grid.rs
- settings.rs
- AudioPlayer
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- Ui
- MenuItem<'a>
- remote/mod.rs
- TouchLayout
- Timeline
- control/timeline.rs
- steps
- window.rs
- stage.rs
- MenuAction
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- toolbar.rs
- theme.rs
- text_field
- mapping/mod.rs
- ShutterEffect
- profile.rs
- fx/mapping.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- global.rs
- MapPreset
- audio_triggers.rs
- snap.rs
- Scene
- dialog.rs
- Cmd
- range_slider.rs
- arc
- Placement
- qxf.rs
- OrbitCamera
- duration
- Shape
- Object
- TouchKind
- outline.rs
- app/src/obs.rs
- convert.rs
- Obs
- stage_editor.rs
- autosave.rs
- ChannelDef
- AudioGrid
- IconButton
- CurveShape
- ColorPreset
- gltf_import.rs
- panels.rs
- i18n/mod.rs
- mcp/src/venue.rs
- egui
- stage_canvas.rs
- scene_card.rs
- Clip
- DeviceModel
- .ui
- Theme
- show.rs
- time_field.rs
- ZoneId
- Drag
- text_dim
- ProfileBuilder
- VideoOut
- UpdateState
- text_field
- run
- Profile
- settings.rs
- View
- app.rs
- RemoteLink
- Problem
- server.rs
- mcp.rs
- ShowError
- icon_library.rs
- Snapshot
- PlanShape
- midi.rs
- tools.rs
- Serialize
- ClipAction
- control/tracking.rs
- fader.rs
- scene_props.rs
- lock.rs
- Engine
- plan
- UpdateBadge
- file_dialog.rs
- scene_group.rs
- core/src/lib.rs
- Effect
- FixtureLimits
- env
- de.rs
- Knob<'a>
- Calibrate
- Step
- build-local-artifacts
- ClipContent
- ObsSettings
- AppState
- DeckState
- SceneKind
- ok
- plan.rs
- Mapper
- FooterButton<'a>
- views/sound.rs
- announce
- Grip
- .check_placement
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- Tab
- with
- T
- send_json
- RemoteStatus
- super
- .fade_state
- Rect
- ObsAction
- Drag

## God Nodes (most connected - your core abstractions)
1. `Engine` - 263 edges
2. `t()` - 241 edges
3. `Show` - 181 edges
4. `AppState` - 122 edges
5. `Icon` - 119 edges
6. `ViewCtx` - 99 edges
7. `ChannelKind` - 79 edges
8. `ChannelRef` - 65 edges
9. `Timeline` - 58 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `angle_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `grid_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (331 total, 161 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (18): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+10 more)

### Community 2 - "DmxFrame"
Cohesion: 0.05
Nodes (20): addresses_are_one_based(), DmxFrame, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound (+12 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.08
Nodes (21): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+13 more)

### Community 4 - "Venue"
Cohesion: 0.07
Nodes (19): aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), IDENTITY (+11 more)

### Community 5 - "programmer.rs"
Cohesion: 0.07
Nodes (48): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+40 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.21
Nodes (22): all_presets_render_every_beam(), apply(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels() (+14 more)

### Community 9 - "call"
Cohesion: 0.24
Nodes (25): add_clips(), add_effect(), add_fixtures(), apply_clip_fields(), apply_scene_fields(), build_steps(), call(), changed_scene() (+17 more)

### Community 10 - "props.rs"
Cohesion: 0.16
Nodes (16): renumber(), apply_texture(), aim_fixtures(), color_field(), duplicate_object(), floor_height(), number(), percent() (+8 more)

### Community 11 - "PaletteStore"
Cohesion: 0.36
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 12 - "target_shape"
Cohesion: 0.15
Nodes (23): alt_click(), button_rect(), fader(), fader_channels(), in_rotated(), Pick, pick(), pick_full() (+15 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "with"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (41): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+33 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.12
Nodes (31): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+23 more)

### Community 20 - "views/touch.rs"
Cohesion: 0.10
Nodes (34): apply(), find_item(), state_json(), tracking_moves_and_switches_a_target(), apply_pick(), channels_of(), COLORS, content_rect() (+26 more)

### Community 21 - "Deserialize"
Cohesion: 0.12
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.08
Nodes (41): analyze(), Analyzer, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad, BPM_MAX, BPM_MIN, detect_grid() (+33 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (30): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+22 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "Control"
Cohesion: 0.06
Nodes (36): add_find_and_json(), ButtonMode, Toggle, Value, Control, AudioFade, AudioMaster, AudioMute (+28 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.14
Nodes (24): color_fx_ui(), edit_colors(), label_row(), paint_strip(), slider_row(), toggle_row(), BodyCtx, combo() (+16 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 32 - "TouchView"
Cohesion: 0.19
Nodes (12): t(), ViewCtx, add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page (+4 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.14
Nodes (16): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+8 more)

### Community 36 - "t"
Cohesion: 0.08
Nodes (35): kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H, full_combo() (+27 more)

### Community 37 - "menu.rs"
Cohesion: 0.10
Nodes (22): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+14 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (50): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+42 more)

### Community 43 - "settings.rs"
Cohesion: 0.20
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), last_tab_stays(), RECENT_MAX

### Community 44 - "AudioPlayer"
Cohesion: 0.16
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.15
Nodes (5): DmxCraftApp, fill_types(), moved_out(), programmer_rail(), viewport_input()

### Community 48 - "Ui"
Cohesion: 0.13
Nodes (30): MidiDevice, hidden_tabs_from_old_setting(), Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo() (+22 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.12
Nodes (20): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+12 more)

### Community 52 - "TouchLayout"
Cohesion: 0.23
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Timeline"
Cohesion: 0.10
Nodes (20): add_audio(), add_audio_dialog(), has_audio(), media_track(), ungroup_clip(), Scene, audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow() (+12 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.07
Nodes (39): Track, add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE (+31 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "MenuAction"
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 72 - "Show"
Cohesion: 0.20
Nodes (4): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.13
Nodes (54): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), defaults_are_rendered(), end_scene_follows_jump_to() (+46 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.21
Nodes (9): select_all(), chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width() (+1 more)

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (54): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+46 more)

### Community 80 - "text_field"
Cohesion: 0.16
Nodes (6): text_field(), drop_marker(), Folder, VizView, VizView, button()

### Community 86 - "mapping/mod.rs"
Cohesion: 0.21
Nodes (19): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+11 more)

### Community 87 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.08
Nodes (27): add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text() (+19 more)

### Community 90 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (61): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+53 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (41): apply_placement(), decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag, Handles (+33 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.12
Nodes (29): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+21 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "global.rs"
Cohesion: 0.20
Nodes (18): apply(), button(), cells_toggle_but_never_all_off(), fader(), hidden_tracking_follow_works(), Input, Press, Value (+10 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.11
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 102 - "Scene"
Cohesion: 0.15
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 104 - "dialog.rs"
Cohesion: 0.07
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 105 - "Cmd"
Cohesion: 0.33
Nodes (6): Cmd, Color, Press, TrackToggle, Value, Xy

### Community 106 - "range_slider.rs"
Cohesion: 0.16
Nodes (9): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+1 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 110 - "OrbitCamera"
Cohesion: 0.08
Nodes (16): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, OrbitCamera (+8 more)

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Object"
Cohesion: 0.24
Nodes (8): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE

### Community 118 - "TouchKind"
Cohesion: 0.18
Nodes (8): TouchKind, Button, Color, Fader, Knob, Label, Tracking, Xy

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (26): drag_row(), DragFixtures, DragObjects, FixtureSort, Address, Name, move_objects(), moving_into_rotated_group_keeps_world_place() (+18 more)

### Community 120 - "app/src/obs.rs"
Cohesion: 0.17
Nodes (12): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+4 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (28): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+20 more)

### Community 123 - "Obs"
Cohesion: 0.19
Nodes (6): fire(), media_action(), media_settings(), Obs, seek(), VideoPlay

### Community 127 - "stage_editor.rs"
Cohesion: 0.13
Nodes (15): CanvasMode, Edit, Select, fit(), fixture_pos(), apply(), Arrange, AlignH (+7 more)

### Community 128 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 129 - "ChannelDef"
Cohesion: 0.08
Nodes (31): ChannelDef, ChannelRange, FixtureType, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color (+23 more)

### Community 130 - "AudioGrid"
Cohesion: 0.19
Nodes (4): AudioAnalysis, AudioGrid, AudioSegment, grid_helpers()

### Community 131 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (17): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+9 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 136 - "panels.rs"
Cohesion: 0.07
Nodes (19): Segmented, End, Measure, snap_points(), SNAP_PX, AXIS_COLORS, DialogKind, ImportEv2 (+11 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (15): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.39
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.06
Nodes (32): heading_text(), DipSwitch, Knob, SIZE, SWEEP, LimitPad, fold_section(), section_header() (+24 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.14
Nodes (19): AMBER, BOX, Camera, fixture_rect(), live_colors(), MenuFn, paint_effects(), paint_fixture() (+11 more)

### Community 142 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 143 - "Clip"
Cohesion: 0.09
Nodes (12): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth, Layer (+4 more)

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 145 - ".ui"
Cohesion: 0.15
Nodes (11): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead() (+3 more)

### Community 146 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 147 - "show.rs"
Cohesion: 0.28
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (10): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+2 more)

### Community 149 - "ZoneId"
Cohesion: 0.12
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), steps_insert_remove_and_normalize(), super_scene_has_no_values_of_its_own()

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.16
Nodes (23): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+15 more)

### Community 152 - "ProfileBuilder"
Cohesion: 0.17
Nodes (8): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder, drag_ghost()

### Community 153 - "VideoOut"
Cohesion: 0.19
Nodes (6): get(), lock(), media_id(), OutClip, serves_state_and_byte_ranges(), VideoOut

### Community 154 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 155 - "text_field"
Cohesion: 0.16
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 156 - "run"
Cohesion: 0.26
Nodes (8): run(), session(), Shared, Status, Connected, Connecting, Failed, Off

### Community 158 - "Profile"
Cohesion: 0.08
Nodes (25): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+17 more)

### Community 159 - "settings.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "View"
Cohesion: 0.14
Nodes (9): View, body(), caption(), FixtureSettings, hanging_of(), range_row(), SetupView, ZonesView (+1 more)

### Community 163 - "app.rs"
Cohesion: 0.23
Nodes (7): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, RAIL_W, ProgrammerView

### Community 164 - "RemoteLink"
Cohesion: 0.24
Nodes (3): layout_json(), RemoteLink, Running

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "server.rs"
Cohesion: 0.13
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 167 - "mcp.rs"
Cohesion: 0.18
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 169 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 170 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 173 - "midi.rs"
Cohesion: 0.22
Nodes (7): MidiEvent, MidiHub, parse(), parses_note_and_cc(), MidiMessage, Cc, Note

### Community 176 - "tools.rs"
Cohesion: 0.08
Nodes (34): any_obj(), base64(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values(), existing_fixtures() (+26 more)

### Community 177 - "Serialize"
Cohesion: 0.29
Nodes (7): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.09
Nodes (13): room_json(), trackers_json(), Show, bar_row(), color(), edit(), RoomMap, TrackingView (+5 more)

### Community 182 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.07
Nodes (51): button(), Timeline, time_field(), advanced(), apply_to_others(), banks_of(), BAR_H, bar_slider() (+43 more)

### Community 184 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 185 - "Engine"
Cohesion: 0.07
Nodes (7): Engine, lerp(), PlayingScene, SceneProgress, Track, TrackLight, ChannelRef

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 189 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 190 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (14): BankScope, All, Bank, Banks, Off, OutsideBank, Group, LoopMode (+6 more)

### Community 193 - "Effect"
Cohesion: 0.43
Nodes (7): apply_effect(), apply_effect_hold(), feature_levels(), luma(), to_rgb(), write_colors(), Effect

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 203 - "Step"
Cohesion: 0.23
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "ClipContent"
Cohesion: 0.13
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Mode, Builder, Control (+7 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 220 - "SceneKind"
Cohesion: 0.33
Nodes (4): SceneKind, Fx, Static, Super

### Community 221 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 225 - "plan.rs"
Cohesion: 0.25
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 226 - "Mapper"
Cohesion: 0.16
Nodes (9): default_feedback(), fires_id(), is_value_control(), Mapper, overlay_id(), pick_id(), Sent, states_id() (+1 more)

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 234 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 241 - "video_out.rs"
Cohesion: 0.20
Nodes (12): CHUNK, DEFAULT_PORT, header(), host_ok(), mime(), PAGE, range(), SEEN (+4 more)

### Community 244 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 275 - "send_json"
Cohesion: 0.83
Nodes (4): open(), read_json(), send_json(), Ws

### Community 283 - "RemoteStatus"
Cohesion: 0.50
Nodes (4): RemoteStatus, Error, Off, Running

### Community 292 - "super"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 296 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 303 - "Rect"
Cohesion: 0.10
Nodes (23): list_header(), list_row(), ListAction, Add, Duplicate, Remove, ModeOp, Add (+15 more)

### Community 309 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

### Community 317 - "Drag"
Cohesion: 0.67
Nodes (3): Drag, Move, Resize

## Knowledge Gaps
- **580 isolated node(s):** `Press`, `Setup`, `Control`, `Touch`, `Visualizer` (+575 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1501 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **161 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `DmxFrame`, `programmer.rs`, `call`, `mcp/src/venue.rs`, `egui`, `.ui`, `views/touch.rs`, `text_dim`, `ProfileBuilder`, `rig.rs`, `Host`, `Profile`, `View`, `TouchView`, `app/src/audio.rs`, `app.rs`, `RemoteLink`, `menu.rs`, `t`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `.fade_state`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `control/tracking.rs`, `scene_props.rs`, `core/src/lib.rs`, `control/timeline.rs`, `window.rs`, `stage.rs`, `Show`, `Calibrate`, `engine.rs`, `mapping/mod.rs`, `global.rs`, `Mapper`, `audio_triggers.rs`, `app/src/obs.rs`, `Obs`, `stage_editor.rs`?**
  _High betweenness centrality (0.126) - this node is a cross-community bridge._
- **Are the 202 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 202 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Press`, `Setup`, `Control` to the rest of the system?**
  _580 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `autosave.rs`, `ChannelDef`, `Venue`, `SoundBoard`, `panels.rs`, `fx.rs`, `call`, `stage_canvas.rs`, `show.rs`, `views/touch.rs`, `ZoneId`, `rig.rs`, `Control`, `missing_audio.rs`, `TouchView`, `app.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `ShowError`, `DmxCraftApp`, `tools.rs`, `TouchLayout`, `control/tracking.rs`, `scene_props.rs`, `Engine`, `scene_group.rs`, `core/src/lib.rs`, `Effect`, `Timeline`, `stage.rs`, `feature.rs`, `Step`, `motion.rs`, `engine.rs`, `theme.rs`, `ObsSettings`, `mapping/mod.rs`, `viz/mod.rs`, `Mapper`, `snap.rs`, `.check_placement`, `Placement`, `Object`, `stage_editor.rs`?**
  _High betweenness centrality (0.099) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11827956989247312 - nodes in this community are weakly interconnected._