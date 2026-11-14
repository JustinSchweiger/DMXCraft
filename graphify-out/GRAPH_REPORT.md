# Graph Report - dmxcraft-src  (2026-11-14)

## Corpus Check
- 165 files · ~271,224 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4662 nodes · 11939 edges · 325 communities (164 shown, 161 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 641 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `66743c7e`
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
- fx_color.rs
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
- Settings
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
- Action
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- toolbar.rs
- theme.rs
- OpenDmx
- trigger_cx
- ShutterEffect
- profile.rs
- Box<dyn serialport::SerialPort>
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- Dialog<'a>
- MapPreset
- audio_triggers.rs
- snap.rs
- Scene
- dialog.rs
- apply
- app/src/profiles.rs
- arc
- Placement
- src/tracking.rs
- OrbitCamera
- duration
- Shape
- Object
- TouchKind
- outline.rs
- app/src/obs.rs
- convert.rs
- Scene
- stage_editor.rs
- autosave.rs
- ChannelDef
- core/src/venue.rs
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
- panels.rs
- show.rs
- time_field.rs
- ZoneId
- Drag
- text_dim
- ProfileBuilder
- fixture_menu.rs
- UpdateState
- text_field
- updater.rs
- mcp/src/profiles.rs
- settings.rs
- fixture_settings.rs
- bank_menu
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
- TouchItem
- ClipAction
- control/tracking.rs
- fader.rs
- scene_props.rs
- protocol.rs
- Engine
- plan
- DmxFrame
- file_dialog.rs
- scene_group.rs
- core/src/lib.rs
- bridge.rs
- env
- de.rs
- Knob<'a>
- Calibrate
- Step
- build-local-artifacts
- ClipContent
- AppState
- DeckState
- HeaderAction
- Easing
- plan.rs
- Mapper
- FooterButton<'a>
- views/sound.rs
- announce
- Camera
- .check_placement
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- Tab
- with
- T
- super
- Tab
- Kind
- windows.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 257 edges
2. `t()` - 236 edges
3. `Show` - 181 edges
4. `Icon` - 119 edges
5. `AppState` - 113 edges
6. `ViewCtx` - 97 edges
7. `ChannelKind` - 79 edges
8. `ChannelRef` - 65 edges
9. `Timeline` - 58 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `angle_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `grid_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (325 total, 161 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.10
Nodes (22): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+14 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Venue"
Cohesion: 0.12
Nodes (8): Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object, room_and_tree(), Venue

### Community 5 - "programmer.rs"
Cohesion: 0.06
Nodes (46): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+38 more)

### Community 6 - "SoundBoard"
Cohesion: 0.12
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.15
Nodes (33): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+25 more)

### Community 9 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "props.rs"
Cohesion: 0.12
Nodes (19): fold_section(), ViewCtx, renumber(), targets(), VizView, aim_fixtures(), button(), color_field() (+11 more)

### Community 11 - "fx_color.rs"
Cohesion: 0.14
Nodes (17): builtin(), load_user(), named(), NamedPalette, path(), save_user(), rgb(), zone_color() (+9 more)

### Community 12 - "target_shape"
Cohesion: 0.10
Nodes (44): alt_click(), bank_step_wraps(), button_rect(), channel_mapping_sets_live_value(), fader(), fader_channels(), fire(), fires_id() (+36 more)

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
Cohesion: 0.11
Nodes (19): apply_pick(), COLORS, Drag, Move, Resize, GAP, GRID_STEPS, item_title() (+11 more)

### Community 21 - "Deserialize"
Cohesion: 0.09
Nodes (14): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media, B64 (+6 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.16
Nodes (23): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+15 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "Control"
Cohesion: 0.07
Nodes (36): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+28 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.20
Nodes (18): BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid(), mapping_ui(), mode_row() (+10 more)

### Community 29 - "Host"
Cohesion: 0.21
Nodes (9): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+1 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.08
Nodes (29): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+21 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.16
Nodes (11): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+3 more)

### Community 32 - "TouchView"
Cohesion: 0.20
Nodes (10): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+2 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.14
Nodes (15): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+7 more)

### Community 36 - "t"
Cohesion: 0.08
Nodes (45): t(), INPUT_H, kind_color(), capability_fields(), card_row(), category_label(), check(), color_label() (+37 more)

### Community 37 - "menu.rs"
Cohesion: 0.12
Nodes (21): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+13 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.15
Nodes (17): programmer_values(), Bank, FocusBank, RenameBank, apply(), apply_bank(), arrow(), BankDrag (+9 more)

### Community 43 - "Settings"
Cohesion: 0.11
Nodes (7): MidiDevice, cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

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
Cohesion: 0.13
Nodes (9): DmxCraftApp, edit_preview(), fill_types(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input() (+1 more)

### Community 48 - "Ui"
Cohesion: 0.20
Nodes (26): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+18 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (27): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+19 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Timeline"
Cohesion: 0.16
Nodes (7): add_audio(), add_audio_dialog(), has_audio(), media_track(), ungroup_clip(), linked_blocks_stay_together(), Timeline

### Community 66 - "control/timeline.rs"
Cohesion: 0.08
Nodes (29): add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, has_sound_cached() (+21 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (24): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+16 more)

### Community 71 - "Action"
Cohesion: 0.18
Nodes (20): Action, Compact, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump (+12 more)

### Community 72 - "Show"
Cohesion: 0.16
Nodes (4): .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.08
Nodes (28): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+20 more)

### Community 77 - "engine.rs"
Cohesion: 0.14
Nodes (51): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below() (+43 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.29
Nodes (6): chip(), divider(), level(), point_chip(), point_width(), VizView

### Community 79 - "theme.rs"
Cohesion: 0.05
Nodes (64): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+56 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "trigger_cx"
Cohesion: 0.47
Nodes (5): channel_name(), describe(), describe_mapping(), t_deleted(), trigger_cx()

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (29): add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text() (+21 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (61): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+53 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (36): apply_placement(), decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles, handles(), hit_y() (+28 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (27): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+19 more)

### Community 95 - "textures.rs"
Cohesion: 0.12
Nodes (29): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+21 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Dialog<'a>"
Cohesion: 0.13
Nodes (6): Button, Dialog<'a>, Kind, Cancel, Primary, Secondary

### Community 99 - "MapPreset"
Cohesion: 0.07
Nodes (31): default_palette(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, MapPreset, .ALL, Bounce (+23 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.11
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 102 - "Scene"
Cohesion: 0.12
Nodes (13): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+5 more)

### Community 104 - "dialog.rs"
Cohesion: 0.08
Nodes (7): DialogResult, FOOTER_H, FooterButton, Form, HEADER_H, LABEL_W, text_field()

### Community 105 - "apply"
Cohesion: 0.17
Nodes (16): apply(), Cmd, Color, Press, Track, TrackToggle, Value, Xy (+8 more)

### Community 106 - "app/src/profiles.rs"
Cohesion: 0.19
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "src/tracking.rs"
Cohesion: 0.16
Nodes (4): all_cells(), DEFAULT_HEIGHT, Show, white()

### Community 110 - "OrbitCamera"
Cohesion: 0.11
Nodes (10): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, OrbitCamera (+2 more)

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Object"
Cohesion: 0.19
Nodes (11): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+3 more)

### Community 118 - "TouchKind"
Cohesion: 0.18
Nodes (8): TouchKind, Button, Color, Fader, Knob, Label, Tracking, Xy

### Community 119 - "outline.rs"
Cohesion: 0.07
Nodes (35): Quality, .ALL, High, Low, Medium, Ultra, drag_row(), DragFixtures (+27 more)

### Community 120 - "app/src/obs.rs"
Cohesion: 0.06
Nodes (40): digest(), hash_and_verify(), hash_password(), hex(), verify(), auth_matches_obs_websocket_example(), auth_response(), b64() (+32 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "Scene"
Cohesion: 0.18
Nodes (13): Scene, audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), GridMode, Beat, Time, group_moves_together_without_covering_others() (+5 more)

### Community 127 - "stage_editor.rs"
Cohesion: 0.15
Nodes (13): CanvasMode, Edit, Select, StageCanvas, Underlay, Arrange, AlignH, AlignV (+5 more)

### Community 128 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 129 - "ChannelDef"
Cohesion: 0.09
Nodes (29): ChannelDef, ChannelRange, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color, Gobo (+21 more)

### Community 130 - "core/src/venue.rs"
Cohesion: 0.12
Nodes (12): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, moving_head_aim(), moving_head_direction(), mul(), rotation() (+4 more)

### Community 131 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.14
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 136 - "panels.rs"
Cohesion: 0.11
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, apply_texture(), OutlineTab (+6 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.06
Nodes (33): SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY, LimitPad (+25 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (18): AMBER, BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_effects() (+10 more)

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
Nodes (14): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead() (+6 more)

### Community 146 - "panels.rs"
Cohesion: 0.21
Nodes (6): End, Measure, snap_points(), SNAP_PX, AXIS_COLORS, project()

### Community 147 - "show.rs"
Cohesion: 0.16
Nodes (15): address_map_marks_ranges(), aim_moving_head_respects_limits(), FixtureLimits, groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names() (+7 more)

### Community 148 - "time_field.rs"
Cohesion: 0.17
Nodes (14): drag_number(), endless_delta(), raw_available(), COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP (+6 more)

### Community 149 - "ZoneId"
Cohesion: 0.12
Nodes (7): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), steps_insert_remove_and_normalize(), super_scene_has_no_values_of_its_own(), Zone

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.14
Nodes (29): window(), accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border() (+21 more)

### Community 152 - "ProfileBuilder"
Cohesion: 0.16
Nodes (8): FileJob, Export, Import, import(), new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 153 - "fixture_menu.rs"
Cohesion: 0.15
Nodes (6): apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 154 - "UpdateState"
Cohesion: 0.15
Nodes (11): run_updater(), Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (16): CANVAS_COLORS, canvas_swatches(), color_picker(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS (+8 more)

### Community 156 - "updater.rs"
Cohesion: 0.16
Nodes (17): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+9 more)

### Community 158 - "mcp/src/profiles.rs"
Cohesion: 0.19
Nodes (12): MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text() (+4 more)

### Community 159 - "settings.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "fixture_settings.rs"
Cohesion: 0.42
Nodes (4): body(), caption(), hanging_of(), range_row()

### Community 163 - "bank_menu"
Cohesion: 0.28
Nodes (13): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, Move, New (+5 more)

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "server.rs"
Cohesion: 0.23
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 167 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 169 - "icon_library.rs"
Cohesion: 0.29
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

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
Nodes (62): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_clip_fields(), apply_scene_fields(), base64(), boolean() (+54 more)

### Community 177 - "TouchItem"
Cohesion: 0.38
Nodes (9): content_rect(), display_label(), fader_parts(), item_target(), mapping_of(), paint_cover(), paint_item(), target_entry() (+1 more)

### Community 179 - "ClipAction"
Cohesion: 0.10
Nodes (18): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+10 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.15
Nodes (13): room_json(), trackers_json(), color_swatches(), Show, bar_row(), color(), edit(), LABEL_W (+5 more)

### Community 182 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.06
Nodes (55): ControlViews, Timeline, ProgrammerView, advanced(), apply_to_others(), banks_of(), bar_slider(), block_props() (+47 more)

### Community 184 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 185 - "Engine"
Cohesion: 0.06
Nodes (13): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), Engine, FadeState, In, Out, lerp(), move_hold_does_not_stop_a_playing_scene() (+5 more)

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 189 - "DmxFrame"
Cohesion: 0.15
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 190 - "file_dialog.rs"
Cohesion: 0.20
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (13): BankScope, All, Bank, Banks, Off, OutsideBank, PLAN_SIZE_RANGE, SceneKind (+5 more)

### Community 195 - "bridge.rs"
Cohesion: 0.29
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 203 - "Step"
Cohesion: 0.27
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "ClipContent"
Cohesion: 0.13
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 218 - "AppState"
Cohesion: 0.09
Nodes (14): AppState, Focus, Bank, Effect, Scenes, Mode, Builder, Control (+6 more)

### Community 219 - "DeckState"
Cohesion: 0.29
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 220 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 221 - "Easing"
Cohesion: 0.22
Nodes (8): Easing, .ALL, Custom, EaseIn, EaseInOut, EaseOut, EaseOutIn, Linear

### Community 225 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 226 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (20): button(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H (+12 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 234 - ".check_placement"
Cohesion: 0.48
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

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
Cohesion: 0.15
Nodes (12): SettingsWindow, Tab, Ai, .ALL, Audio, Customize, General, Midi (+4 more)

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 292 - "super"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 303 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 309 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

## Knowledge Gaps
- **579 isolated node(s):** `Off`, `Running`, `Error`, `Press`, `Color` (+574 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1500 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **161 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `Venue`, `programmer.rs`, `props.rs`, `mcp/src/venue.rs`, `target_shape`, `egui`, `.ui`, `views/touch.rs`, `text_dim`, `ProfileBuilder`, `fixture_menu.rs`, `rig.rs`, `Host`, `mcp/src/profiles.rs`, `app/src/audio.rs`, `t`, `menu.rs`, `mcp.rs`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `TouchItem`, `tools.rs`, `remote/mod.rs`, `control/tracking.rs`, `scene_props.rs`, `DmxFrame`, `core/src/lib.rs`, `control/timeline.rs`, `window.rs`, `stage.rs`, `Action`, `Show`, `Calibrate`, `engine.rs`, `trigger_cx`, `Mapper`, `audio_triggers.rs`, `apply`, `app/src/obs.rs`?**
  _High betweenness centrality (0.128) - this node is a cross-community bridge._
- **Are the 198 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 198 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Off`, `Running`, `Error` to the rest of the system?**
  _579 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.12299465240641712 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `programmer.rs`, `panels.rs`, `i18n/mod.rs`, `props.rs`, `fx_color.rs`, `egui`, `viz/library.rs`, `.ui`, `panels.rs`, `titlebar.rs`, `views/touch.rs`, `text_dim`, `ProfileBuilder`, `fixture_menu.rs`, `text_field`, `fx_ui.rs`, `missing_audio.rs`, `fixture_settings.rs`, `TouchView`, `bank_menu`, `menu.rs`, `FixtureAction`, `scene_grid.rs`, `Settings`, `DmxCraftApp`, `Ui`, `TouchItem`, `control/tracking.rs`, `fader.rs`, `scene_props.rs`, `windows.rs`, `file_dialog.rs`, `control/timeline.rs`, `window.rs`, `stage.rs`, `Action`, `Calibrate`, `toolbar.rs`, `trigger_cx`, `viz/mod.rs`, `textures.rs`, `snap.rs`, `views/sound.rs`, `dialog.rs`, `app/src/profiles.rs`, `outline.rs`, `stage_editor.rs`?**
  _High betweenness centrality (0.102) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10241820768136557 - nodes in this community are weakly interconnected._