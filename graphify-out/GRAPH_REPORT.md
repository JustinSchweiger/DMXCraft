# Graph Report - dmxcraft-src  (2026-11-13)

## Corpus Check
- 165 files · ~269,437 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4640 nodes · 11897 edges · 326 communities (170 shown, 156 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 636 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e231be0d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- Venue
- programmer.rs
- SoundBoard
- steps
- fx.rs
- DmxOutput
- props.rs
- assist.rs
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
- look.rs
- control/timeline.rs
- steps
- window.rs
- stage.rs
- ProfileChannel
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- toolbar.rs
- theme.rs
- OpenDmx
- fire
- ShutterEffect
- profile.rs
- Box<dyn serialport::SerialPort>
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- fx/mapping.rs
- MapPreset
- audio_triggers.rs
- snap.rs
- Scene
- dialog.rs
- call
- Profile
- arc
- Placement
- SceneId
- OrbitCamera
- duration
- Shape
- Object
- TouchKind
- outline.rs
- app/src/obs.rs
- convert.rs
- Timeline
- stage_editor.rs
- autosave.rs
- ChannelDef
- core/src/venue.rs
- IconButton
- CurveShape
- ColorPreset
- gltf_import.rs
- VizView
- i18n/mod.rs
- mcp/src/venue.rs
- egui
- stage_canvas.rs
- Theme
- Clip
- DeviceModel
- .ui
- panels.rs
- show.rs
- time_field.rs
- BankScope
- Drag
- text_dim
- ProfileBuilder
- .objects_tab
- UpdateState
- text_field
- updater.rs
- mcp/src/profiles.rs
- settings.rs
- fixture_settings.rs
- Obs
- Transform
- Problem
- server.rs
- mcp.rs
- ShowError
- icon_library.rs
- Snapshot
- PlanShape
- midi.rs
- tools.rs
- super
- ClipAction
- text_field
- fader.rs
- scene_props.rs
- range_slider.rs
- Engine
- plan
- DmxFrame
- file_dialog.rs
- scene_group.rs
- core/src/lib.rs
- Quality
- env
- de.rs
- Knob<'a>
- Calibrate
- Step
- build-local-artifacts
- ClipContent
- AppState
- app/src/sound.rs
- HeaderAction
- plan.rs
- Mapper
- FooterButton<'a>
- views/sound.rs
- announce
- run
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- Tab
- View
- .update
- with
- T
- Category
- lock.rs
- ProgrammerMode
- super
- TriggerBand
- Tab
- Kind
- windows.rs
- ok
- Grip
- programmer_overrides_and_master_scales
- ObsAction

## God Nodes (most connected - your core abstractions)
1. `Engine` - 257 edges
2. `t()` - 234 edges
3. `Show` - 181 edges
4. `Icon` - 119 edges
5. `AppState` - 113 edges
6. `ViewCtx` - 97 edges
7. `ChannelKind` - 79 edges
8. `ChannelRef` - 65 edges
9. `Timeline` - 58 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `angle_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `grid_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (326 total, 156 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Venue"
Cohesion: 0.13
Nodes (8): Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object, room_and_tree(), Venue

### Community 5 - "programmer.rs"
Cohesion: 0.16
Nodes (24): color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block() (+16 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+18 more)

### Community 9 - "DmxOutput"
Cohesion: 0.12
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "props.rs"
Cohesion: 0.10
Nodes (22): renumber(), apply_texture(), OutlineTab, Fixtures, Objects, TexAction, Builtin, Load (+14 more)

### Community 11 - "assist.rs"
Cohesion: 0.16
Nodes (11): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, decode_texture() (+3 more)

### Community 12 - "target_shape"
Cohesion: 0.13
Nodes (28): alt_click(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id(), Pick (+20 more)

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
Cohesion: 0.05
Nodes (62): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+54 more)

### Community 20 - "views/touch.rs"
Cohesion: 0.10
Nodes (34): apply(), find_item(), state_json(), apply_pick(), channels_of(), COLORS, content_rect(), current_color() (+26 more)

### Community 21 - "Deserialize"
Cohesion: 0.11
Nodes (9): load_file(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json() (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (44): analyze(), Analyzer, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad, BPM_MAX (+36 more)

### Community 24 - "rig.rs"
Cohesion: 0.17
Nodes (23): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+15 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "Control"
Cohesion: 0.06
Nodes (37): Overlay, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade (+29 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.09
Nodes (36): builtin(), load_user(), named(), NamedPalette, path(), save_user(), rgb(), color_fx_ui() (+28 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 32 - "TouchView"
Cohesion: 0.17
Nodes (13): section_header(), ViewCtx, add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page (+5 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.17
Nodes (14): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+6 more)

### Community 36 - "t"
Cohesion: 0.09
Nodes (45): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+37 more)

### Community 37 - "menu.rs"
Cohesion: 0.11
Nodes (20): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+12 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 40 - "FixtureAction"
Cohesion: 0.06
Nodes (33): separator(), apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete (+25 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (50): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+42 more)

### Community 43 - "settings.rs"
Cohesion: 0.18
Nodes (5): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX

### Community 44 - "AudioPlayer"
Cohesion: 0.15
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.13
Nodes (8): DmxCraftApp, edit_preview(), fill_types(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input()

### Community 48 - "Ui"
Cohesion: 0.11
Nodes (29): MidiDevice, Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general() (+21 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "look.rs"
Cohesion: 0.25
Nodes (16): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, nearest_slot(), position_in_degrees() (+8 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.08
Nodes (29): add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, has_sound_cached() (+21 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.15
Nodes (25): color(), Mapper, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.11
Nodes (26): select_all(), copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New (+18 more)

### Community 71 - "ProfileChannel"
Cohesion: 0.21
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 72 - "Show"
Cohesion: 0.16
Nodes (10): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap (+2 more)

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
Cohesion: 0.14
Nodes (50): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), defaults_are_rendered(), end_scene_follows_jump_to() (+42 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.26
Nodes (8): chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width(), VizView

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (53): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+45 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "fire"
Cohesion: 0.14
Nodes (21): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+13 more)

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (22): beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys() (+14 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.12
Nodes (29): apply_placement(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag, Handles, handles() (+21 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "fx/mapping.rs"
Cohesion: 0.14
Nodes (15): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+7 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.15
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 101 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 102 - "Scene"
Cohesion: 0.10
Nodes (13): Effect, EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings (+5 more)

### Community 104 - "dialog.rs"
Cohesion: 0.06
Nodes (14): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+6 more)

### Community 105 - "call"
Cohesion: 0.21
Nodes (26): add_clips(), add_effect(), add_fixtures(), apply_clip_fields(), apply_scene_fields(), build_steps(), call(), changed_scene() (+18 more)

### Community 106 - "Profile"
Cohesion: 0.09
Nodes (14): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+6 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "SceneId"
Cohesion: 0.11
Nodes (5): FadeState, In, Out, layered_start_keeps_other_scenes(), SceneProgress

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Object"
Cohesion: 0.24
Nodes (9): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+1 more)

### Community 118 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "app/src/obs.rs"
Cohesion: 0.18
Nodes (12): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+4 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "Timeline"
Cohesion: 0.10
Nodes (20): Show, add_audio(), add_audio_dialog(), has_audio(), media_track(), ungroup_clip(), audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow() (+12 more)

### Community 127 - "stage_editor.rs"
Cohesion: 0.13
Nodes (14): CanvasMode, Edit, Select, StageCanvas, Underlay, XyPad, Arrange, AlignH (+6 more)

### Community 128 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 129 - "ChannelDef"
Cohesion: 0.10
Nodes (14): ChannelDef, ChannelRange, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color, Gobo (+6 more)

### Community 130 - "core/src/venue.rs"
Cohesion: 0.13
Nodes (12): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, moving_head_aim(), moving_head_direction(), mul(), rotation() (+4 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.14
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 136 - "VizView"
Cohesion: 0.11
Nodes (13): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+5 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.05
Nodes (27): ICON_FAMILY, ICON_FILL_FAMILY, SIZE, SWEEP, LimitPad, CARD_H, clip(), COMPACT_H (+19 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 142 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 143 - "Clip"
Cohesion: 0.08
Nodes (12): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth, Layer (+4 more)

### Community 144 - "DeviceModel"
Cohesion: 0.15
Nodes (9): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), nearest() (+1 more)

### Community 145 - ".ui"
Cohesion: 0.15
Nodes (14): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead() (+6 more)

### Community 146 - "panels.rs"
Cohesion: 0.21
Nodes (5): End, Measure, snap_points(), SNAP_PX, sel_frame()

### Community 147 - "show.rs"
Cohesion: 0.16
Nodes (18): address_map_marks_ranges(), free_scenes_get_a_bank(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, move_scenes_between_banks(), move_zone_reorders_banks(), next_free_address_skips_used() (+10 more)

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 149 - "BankScope"
Cohesion: 0.12
Nodes (9): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), SceneSettings (+1 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.15
Nodes (28): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+20 more)

### Community 152 - "ProfileBuilder"
Cohesion: 0.15
Nodes (9): FileJob, Export, Import, import(), new_profile(), pick_export(), pick_import(), ProfileBuilder (+1 more)

### Community 153 - ".objects_tab"
Cohesion: 0.35
Nodes (3): fold_section(), VizView, button()

### Community 154 - "UpdateState"
Cohesion: 0.15
Nodes (11): run_updater(), Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 156 - "updater.rs"
Cohesion: 0.16
Nodes (17): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+9 more)

### Community 158 - "mcp/src/profiles.rs"
Cohesion: 0.37
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 159 - "settings.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "fixture_settings.rs"
Cohesion: 0.17
Nodes (10): RangeSlider, Quick, quick_action(), Center, Off, On, body(), caption() (+2 more)

### Community 163 - "Obs"
Cohesion: 0.14
Nodes (7): Obs, Status, Connected, Connecting, Failed, Off, VideoPlay

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

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
Cohesion: 0.33
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 170 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (33): any_obj(), base64(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values(), dmx_output() (+25 more)

### Community 177 - "super"
Cohesion: 0.29
Nodes (7): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "text_field"
Cohesion: 0.12
Nodes (13): bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT, TrackingView, AXIS_COLORS (+5 more)

### Community 182 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.08
Nodes (45): Timeline, advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header() (+37 more)

### Community 184 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 185 - "Engine"
Cohesion: 0.08
Nodes (7): Engine, lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, Track, TrackLight, ChannelRef

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 190 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (11): UNIVERSE_SIZE, FixtureLimits, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 195 - "Quality"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 203 - "Step"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "ClipContent"
Cohesion: 0.13
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 218 - "AppState"
Cohesion: 0.08
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Mode, Builder, Control (+7 more)

### Community 219 - "app/src/sound.rs"
Cohesion: 0.14
Nodes (10): Player, DeckRt, DeckState, Empty, Ended, Fading, Paused, Playing (+2 more)

### Community 220 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 225 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 226 - "Mapper"
Cohesion: 0.19
Nodes (3): default_feedback(), is_value_control(), Sent

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (20): button(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H (+12 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "run"
Cohesion: 0.26
Nodes (5): Link, run(), session(), Shared, ObsSettings

### Community 234 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 241 - "video_out.rs"
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 244 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 252 - "View"
Cohesion: 0.21
Nodes (6): ControlViews, ProgrammerView, View, FixtureSettings, SetupView, StageEditorView

### Community 256 - ".update"
Cohesion: 0.33
Nodes (8): fire(), media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 276 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 285 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 289 - "ProgrammerMode"
Cohesion: 0.25
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 292 - "super"
Cohesion: 0.46
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 301 - "TriggerBand"
Cohesion: 0.25
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 303 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 309 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 312 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 314 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 323 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

## Knowledge Gaps
- **576 isolated node(s):** `Edit`, `New`, `Open`, `Template`, `Builtin` (+571 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1491 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **156 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `autosave.rs`, `library_panel.rs`, `Venue`, `SoundBoard`, `fx.rs`, `assist.rs`, `target_shape`, `stage_canvas.rs`, `panels.rs`, `show.rs`, `views/touch.rs`, `BankScope`, `rig.rs`, `Control`, `missing_audio.rs`, `TouchView`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `ShowError`, `DmxCraftApp`, `tools.rs`, `TouchLayout`, `text_field`, `scene_props.rs`, `Engine`, `scene_group.rs`, `core/src/lib.rs`, `stage.rs`, `feature.rs`, `Step`, `motion.rs`, `engine.rs`, `theme.rs`, `fire`, `viz/mod.rs`, `snap.rs`, `Scene`, `run`, `call`, `Placement`, `SceneId`, `Object`, `stage_editor.rs`?**
  _High betweenness centrality (0.113) - this node is a cross-community bridge._
- **Are the 198 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 198 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Edit`, `New`, `Open` to the rest of the system?**
  _576 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `.update`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `mcp/src/venue.rs`, `egui`, `.ui`, `views/touch.rs`, `text_dim`, `ProfileBuilder`, `rig.rs`, `Host`, `mcp/src/profiles.rs`, `TouchView`, `fixture_settings.rs`, `app/src/audio.rs`, `Obs`, `t`, `menu.rs`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `text_field`, `scene_props.rs`, `programmer_overrides_and_master_scales`, `DmxFrame`, `core/src/lib.rs`, `control/timeline.rs`, `window.rs`, `stage.rs`, `Show`, `Calibrate`, `engine.rs`, `fire`, `ChannelKind`, `Mapper`, `audio_triggers.rs`, `call`, `SceneId`, `app/src/obs.rs`, `View`?**
  _High betweenness centrality (0.108) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12310606060606061 - nodes in this community are weakly interconnected._