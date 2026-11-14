# Graph Report - dmxcraft-src  (2026-11-14)

## Corpus Check
- 166 files · ~274,892 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4710 nodes · 12062 edges · 320 communities (165 shown, 155 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 662 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `417a8fb5`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- DmxOutput
- open_dmx.rs
- Venue
- programmer.rs
- SoundBoard
- steps
- fx.rs
- Venue
- props.rs
- PaletteStore
- target_shape
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- TouchItem
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
- runtime.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- toolbar.rs
- theme.rs
- text_field
- trigger_cx
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
- Action
- dialog.rs
- apply
- drag_number
- arc
- Placement
- .push
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
- measure.rs
- i18n/mod.rs
- mcp/src/venue.rs
- egui
- stage_canvas.rs
- scene_card.rs
- FadeCurve
- DeviceModel
- Clip
- Theme
- assist.rs
- send_frame
- show.rs
- Drag
- text_dim
- ProfileBuilder
- bank_menu
- UpdateState
- text_field
- .update
- Profile
- settings.rs
- View
- app.rs
- protocol.rs
- Problem
- server.rs
- mcp.rs
- ShowError
- icon_library.rs
- super
- PlanShape
- midi.rs
- tools.rs
- OpenDmx
- ClipAction
- control/tracking.rs
- bridge.rs
- fader.rs
- scene_props.rs
- lock.rs
- Engine
- plan
- Shape
- file_dialog.rs
- scene_group.rs
- DmxFrame
- Quality
- FixtureLimits
- env
- de.rs
- fold_section
- Knob<'a>
- Calibrate
- Step
- build-local-artifacts
- Timeline
- Transform
- AppState
- DeckState
- TriggerBand
- Layer
- Mapper
- FooterButton<'a>
- views/sound.rs
- announce
- Grip
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- Tab
- with
- T
- Rect
- ObsAction
- views/touch.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 263 edges
2. `t()` - 242 edges
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
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs

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

## Communities (320 total, 155 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (20): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+12 more)

### Community 2 - "DmxOutput"
Cohesion: 0.12
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, Transfer, Failing

### Community 3 - "open_dmx.rs"
Cohesion: 0.18
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "Venue"
Cohesion: 0.11
Nodes (14): aim_at_points_beam_to_target(), apply(), close(), Environment, hang_below_nearest_truss(), IDENTITY, Material, moving_head_aim() (+6 more)

### Community 5 - "programmer.rs"
Cohesion: 0.06
Nodes (53): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+45 more)

### Community 6 - "SoundBoard"
Cohesion: 0.12
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+18 more)

### Community 9 - "Venue"
Cohesion: 0.14
Nodes (7): load_file(), fixture_folders_are_exclusive(), FixtureFolder, Texture, room_and_tree(), TrussHang, Venue

### Community 10 - "props.rs"
Cohesion: 0.08
Nodes (28): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, renumber(), apply_texture() (+20 more)

### Community 11 - "PaletteStore"
Cohesion: 0.36
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 12 - "target_shape"
Cohesion: 0.10
Nodes (43): alt_click(), bank_step_wraps(), button_rect(), channel_mapping_sets_live_value(), fader(), fader_channels(), fire(), fires_id() (+35 more)

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

### Community 20 - "TouchItem"
Cohesion: 0.22
Nodes (14): content_rect(), display_label(), fader_parts(), item_target(), item_value(), led_color(), mapping_of(), paint_cover() (+6 more)

### Community 21 - "Deserialize"
Cohesion: 0.09
Nodes (14): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media, B64 (+6 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.11
Nodes (31): analyze(), bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), BPM_MAX, BPM_MIN, detect_grid(), detect_segments(), energy_curve() (+23 more)

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
Cohesion: 0.07
Nodes (33): add_find_and_json(), ButtonMode, Toggle, Value, Control, AudioFade, AudioMaster, AudioMute (+25 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.11
Nodes (31): rgb(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore (+23 more)

### Community 29 - "Host"
Cohesion: 0.21
Nodes (9): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+1 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.08
Nodes (29): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+21 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.18
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 32 - "TouchView"
Cohesion: 0.18
Nodes (13): ViewCtx, add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+5 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.14
Nodes (16): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+8 more)

### Community 36 - "t"
Cohesion: 0.11
Nodes (30): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+22 more)

### Community 37 - "menu.rs"
Cohesion: 0.11
Nodes (21): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+13 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.13
Nodes (17): ControlViews, ProgrammerView, Bank, FocusBank, RenameBank, apply(), apply_bank(), arrow() (+9 more)

### Community 43 - "Settings"
Cohesion: 0.12
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 44 - "AudioPlayer"
Cohesion: 0.15
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "Ui"
Cohesion: 0.19
Nodes (27): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+19 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (27): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+19 more)

### Community 52 - "TouchLayout"
Cohesion: 0.18
Nodes (8): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), LONG_PRESS_MS, pages_can_be_reordered(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Timeline"
Cohesion: 0.10
Nodes (21): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Group, Obs, Scene, Video, fade_curve_fast_then_slow() (+13 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.07
Nodes (41): Track, add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H (+33 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "runtime.rs"
Cohesion: 0.15
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 72 - "Show"
Cohesion: 0.14
Nodes (10): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap (+2 more)

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.22
Nodes (5): OutputConfig, OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (35): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+27 more)

### Community 77 - "engine.rs"
Cohesion: 0.14
Nodes (53): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), defaults_are_rendered(), end_scene_follows_jump_to() (+45 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.23
Nodes (8): chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width(), VizView

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (53): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+45 more)

### Community 80 - "text_field"
Cohesion: 0.22
Nodes (3): text_field(), VizView, button()

### Community 86 - "trigger_cx"
Cohesion: 0.47
Nodes (5): channel_name(), describe(), describe_mapping(), t_deleted(), trigger_cx()

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (27): add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text() (+19 more)

### Community 90 - "fx/mapping.rs"
Cohesion: 0.14
Nodes (15): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+7 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (63): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+55 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.09
Nodes (39): apply_placement(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag, Handles, handles() (+31 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

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
Cohesion: 0.16
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 101 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 102 - "Action"
Cohesion: 0.18
Nodes (21): Action, Compact, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump (+13 more)

### Community 104 - "dialog.rs"
Cohesion: 0.07
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 105 - "apply"
Cohesion: 0.18
Nodes (15): apply(), Cmd, Color, Press, TrackToggle, Value, Xy, find_item() (+7 more)

### Community 106 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - ".push"
Cohesion: 0.18
Nodes (10): Analyzer, Biquad, Envelope, envelope_filters_band(), FilterKind, Band, High, Low (+2 more)

### Community 112 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Object"
Cohesion: 0.22
Nodes (9): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+1 more)

### Community 118 - "TouchKind"
Cohesion: 0.18
Nodes (8): TouchKind, Button, Color, Fader, Knob, Label, Tracking, Xy

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "app/src/obs.rs"
Cohesion: 0.16
Nodes (13): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+5 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (28): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+20 more)

### Community 123 - "Obs"
Cohesion: 0.16
Nodes (7): Obs, Shared, Status, Connected, Connecting, Failed, Off

### Community 127 - "stage_editor.rs"
Cohesion: 0.16
Nodes (11): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+3 more)

### Community 128 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 129 - "ChannelDef"
Cohesion: 0.08
Nodes (30): ChannelDef, ChannelRange, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color, Gobo (+22 more)

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
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.14
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 136 - "measure.rs"
Cohesion: 0.19
Nodes (5): End, Measure, snap_points(), SNAP_PX, AXIS_COLORS

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.06
Nodes (34): SIZE, SWEEP, LimitPad, from_pct(), HANDLE, pct(), RangeSlider, VALUE_H (+26 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 142 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 143 - "FadeCurve"
Cohesion: 0.15
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 144 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 145 - "Clip"
Cohesion: 0.14
Nodes (15): grid_options(), LaneDrag, LaneGrab, Level, Point, Segment, paint_bands(), paint_beats() (+7 more)

### Community 146 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 147 - "assist.rs"
Cohesion: 0.21
Nodes (10): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, decode_texture() (+2 more)

### Community 148 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 149 - "show.rs"
Cohesion: 0.07
Nodes (28): address_map_marks_ranges(), BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original() (+20 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.15
Nodes (25): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+17 more)

### Community 152 - "ProfileBuilder"
Cohesion: 0.18
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 153 - "bank_menu"
Cohesion: 0.28
Nodes (13): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, Move, New (+5 more)

### Community 154 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 156 - ".update"
Cohesion: 0.17
Nodes (12): Link, media_action(), media_settings(), names(), open(), read_json(), run(), seek() (+4 more)

### Community 158 - "Profile"
Cohesion: 0.08
Nodes (24): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+16 more)

### Community 159 - "settings.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "View"
Cohesion: 0.19
Nodes (5): View, SetupView, StageEditorView, ZonesView, Windows

### Community 163 - "app.rs"
Cohesion: 0.22
Nodes (9): edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, programmer_rail(), RAIL_W, viewport_input() (+1 more)

### Community 164 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "server.rs"
Cohesion: 0.23
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 167 - "mcp.rs"
Cohesion: 0.18
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 169 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 170 - "super"
Cohesion: 0.11
Nodes (20): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), known_addresses(), on_switches(), roundtrip() (+12 more)

### Community 171 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 173 - "midi.rs"
Cohesion: 0.14
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.07
Nodes (76): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+68 more)

### Community 177 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.09
Nodes (16): room_json(), trackers_json(), Show, bar_row(), color(), edit(), LABEL_W, LIST_W (+8 more)

### Community 181 - "bridge.rs"
Cohesion: 0.29
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 182 - "fader.rs"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.07
Nodes (52): Timeline, time_field(), advanced(), apply_to_others(), banks_of(), BAR_H, bar_slider(), block_props() (+44 more)

### Community 184 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 185 - "Engine"
Cohesion: 0.06
Nodes (8): Engine, lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, SceneProgress, Track, TrackLight, ChannelRef

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 189 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 190 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "DmxFrame"
Cohesion: 0.08
Nodes (15): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE, FadeState, In, Out, LoopMode, Loop (+7 more)

### Community 193 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 200 - "fold_section"
Cohesion: 0.36
Nodes (5): heading_text(), fold_section(), section_header(), GroupDrag, strip()

### Community 203 - "Step"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "Timeline"
Cohesion: 0.13
Nodes (5): TimelineView, GridMode, Beat, Time, Timeline

### Community 210 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 218 - "AppState"
Cohesion: 0.08
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Mode, Builder, Control (+7 more)

### Community 219 - "DeckState"
Cohesion: 0.29
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 220 - "TriggerBand"
Cohesion: 0.25
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 221 - "Layer"
Cohesion: 0.33
Nodes (4): Layer, .ALL, Blend, Cover

### Community 226 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (20): button(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H (+12 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

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

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 303 - "Rect"
Cohesion: 0.11
Nodes (23): list_header(), list_row(), ListAction, Add, Duplicate, Remove, ModeOp, Add (+15 more)

### Community 309 - "ObsAction"
Cohesion: 0.50
Nodes (4): fire(), ObsAction, Scene, Source

### Community 317 - "views/touch.rs"
Cohesion: 0.10
Nodes (23): apply_pick(), COLORS, Drag, Move, Resize, GAP, GRID_STEPS, item_title() (+15 more)

## Knowledge Gaps
- **585 isolated node(s):** `Press`, `Off`, `Running`, `Error`, `Press` (+580 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1506 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **155 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `programmer.rs`, `mcp/src/venue.rs`, `target_shape`, `egui`, `TouchItem`, `text_dim`, `ProfileBuilder`, `rig.rs`, `.update`, `Host`, `Profile`, `View`, `TouchView`, `app/src/audio.rs`, `app.rs`, `t`, `menu.rs`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `super`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `control/tracking.rs`, `scene_props.rs`, `views/touch.rs`, `DmxFrame`, `control/timeline.rs`, `window.rs`, `stage.rs`, `runtime.rs`, `fold_section`, `Show`, `Calibrate`, `engine.rs`, `Timeline`, `trigger_cx`, `global.rs`, `Mapper`, `audio_triggers.rs`, `Action`, `apply`, `app/src/obs.rs`, `Obs`?**
  _High betweenness centrality (0.138) - this node is a cross-community bridge._
- **Are the 203 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 203 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Press`, `Off`, `Running` to the rest of the system?**
  _585 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.1268939393939394 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `autosave.rs`, `library_panel.rs`, `SoundBoard`, `measure.rs`, `fx.rs`, `Venue`, `target_shape`, `stage_canvas.rs`, `assist.rs`, `TouchItem`, `show.rs`, `rig.rs`, `bank_menu`, `Control`, `.update`, `missing_audio.rs`, `TouchView`, `app.rs`, `FixtureAction`, `scene_grid.rs`, `super`, `ShowError`, `DmxCraftApp`, `tools.rs`, `TouchLayout`, `control/tracking.rs`, `scene_props.rs`, `Engine`, `views/touch.rs`, `scene_group.rs`, `DmxFrame`, `stage.rs`, `feature.rs`, `Step`, `motion.rs`, `engine.rs`, `theme.rs`, `trigger_cx`, `viz/mod.rs`, `snap.rs`, `apply`, `Placement`, `Object`?**
  _High betweenness centrality (0.097) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11428571428571428 - nodes in this community are weakly interconnected._