# Graph Report - dmxcraft-src  (2026-11-13)

## Corpus Check
- 165 files · ~267,868 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4630 nodes · 11858 edges · 316 communities (168 shown, 148 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 636 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `52c9305f`
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
- control/timeline.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- Step
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
- src/timeline.rs
- ViewCtx
- dmxcraft
- app/src/audio.rs
- profile_builder.rs
- ShowError
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- scene_grid.rs
- missing_audio.rs
- AudioPlayer
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- Ui
- fx_color.rs
- remote/mod.rs
- TouchLayout
- ProfileBuilder
- snap.rs
- steps
- window.rs
- stage.rs
- core/src/venue.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- toolbar.rs
- theme.rs
- OpenDmx
- show.rs
- ShutterEffect
- profile.rs
- app/src/obs.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- fx/mapping.rs
- MapPreset
- audio_triggers.rs
- Settings
- system_fonts.rs
- dialog.rs
- mapping/mod.rs
- .objects_tab
- arc
- Placement
- titlebar.rs
- OrbitCamera
- duration
- Shape
- Object
- TouchKind
- outline.rs
- Tab
- convert.rs
- scene_card.rs
- egui
- autosave.rs
- FixtureType
- t
- strokekind
- CurveShape
- ColorPreset
- gltf_import.rs
- VizView
- i18n/mod.rs
- mcp/src/venue.rs
- IconButton
- stage_canvas.rs
- ChannelRef
- Clip
- DeviceModel
- .ui
- panels.rs
- Scene
- time_field.rs
- views/touch.rs
- Drag
- text_dim
- .modes_tab
- TouchItem
- ClipContent
- text_field
- Theme
- mcp/src/profiles.rs
- settings.rs
- patch_grid.rs
- send_frame
- Transform
- Problem
- server.rs
- mcp.rs
- fixture_settings.rs
- icon_library.rs
- Snapshot
- Effect
- midi.rs
- tools.rs
- super
- ClipAction
- control/tracking.rs
- paint_symbol
- scene_props.rs
- range_slider.rs
- Engine
- plan
- DmxFrame
- file_dialog.rs
- scene_group.rs
- protocol.rs
- bridge.rs
- app.rs
- env
- UpdateBadge
- de.rs
- Knob<'a>
- Calibrate
- FixtureLimits
- HeaderAction
- build-local-artifacts
- SceneKind
- AppState
- DeckState
- OutputStatus
- plan.rs
- Mapper
- FooterButton<'a>
- .check_placement
- views/sound.rs
- announce
- programmer_overrides_and_master_scales
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- Tab
- Easing
- with
- core/src/lib.rs
- Profile
- .fade_state

## God Nodes (most connected - your core abstractions)
1. `Engine` - 255 edges
2. `t()` - 233 edges
3. `Show` - 179 edges
4. `Icon` - 119 edges
5. `AppState` - 113 edges
6. `ViewCtx` - 97 edges
7. `ChannelKind` - 78 edges
8. `ChannelRef` - 64 edges
9. `Timeline` - 58 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (316 total, 148 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (18): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+10 more)

### Community 2 - "runtime.rs"
Cohesion: 0.20
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.18
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "Venue"
Cohesion: 0.11
Nodes (10): Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, mul(), Object, room_and_tree() (+2 more)

### Community 5 - "programmer.rs"
Cohesion: 0.09
Nodes (46): button(), ProgrammerMode, Blind, Edit, Live, kind_color(), Category, All (+38 more)

### Community 6 - "SoundBoard"
Cohesion: 0.12
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+20 more)

### Community 9 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "props.rs"
Cohesion: 0.11
Nodes (20): fold_section(), renumber(), apply_texture(), TexAction, Builtin, Load, Set, aim_fixtures() (+12 more)

### Community 11 - "assist.rs"
Cohesion: 0.13
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

### Community 12 - "control/timeline.rs"
Cohesion: 0.08
Nodes (31): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS (+23 more)

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
Cohesion: 0.07
Nodes (34): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+26 more)

### Community 20 - "Step"
Cohesion: 0.23
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

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
Nodes (42): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+34 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.17
Nodes (18): BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid(), mode_row(), move_editor() (+10 more)

### Community 29 - "Host"
Cohesion: 0.21
Nodes (9): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+1 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.15
Nodes (13): best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, fill_details(), fill_physical(), Library (+5 more)

### Community 31 - "src/timeline.rs"
Cohesion: 0.11
Nodes (18): has_audio(), media_track(), Scene, audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), GridMode, Beat (+10 more)

### Community 32 - "ViewCtx"
Cohesion: 0.15
Nodes (12): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+4 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.13
Nodes (15): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+7 more)

### Community 36 - "profile_builder.rs"
Cohesion: 0.09
Nodes (28): text_field(), capability_fields(), category_label(), check(), color_label(), FOOT_H, full_combo(), general() (+20 more)

### Community 37 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.06
Nodes (35): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+27 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (51): select_all(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+43 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.18
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 44 - "AudioPlayer"
Cohesion: 0.16
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

### Community 50 - "fx_color.rs"
Cohesion: 0.09
Nodes (18): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+10 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "ProfileBuilder"
Cohesion: 0.11
Nodes (13): card_row(), FileJob, Export, Import, ListAction, Add, Duplicate, Remove (+5 more)

### Community 66 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "core/src/venue.rs"
Cohesion: 0.15
Nodes (10): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, moving_head_aim(), moving_head_direction(), rotation(), rotation_order_x_then_y_then_z() (+2 more)

### Community 72 - "Show"
Cohesion: 0.17
Nodes (5): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.08
Nodes (28): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+20 more)

### Community 77 - "engine.rs"
Cohesion: 0.15
Nodes (45): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below(), finished_scene_holds_effects_at_their_end() (+37 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.18
Nodes (9): Segmented, chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width() (+1 more)

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (53): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+45 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.28
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (20): slot_label(), ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse (+12 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (27): add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text() (+19 more)

### Community 90 - "app/src/obs.rs"
Cohesion: 0.06
Nodes (40): digest(), hash_and_verify(), hash_password(), hex(), verify(), auth_matches_obs_websocket_example(), auth_response(), b64() (+32 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.13
Nodes (24): decode_texture(), floor_plan(), FloorShape, Handles, handles(), hit_y(), model_tex_id(), object_box() (+16 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.12
Nodes (29): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+21 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.11
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "Settings"
Cohesion: 0.11
Nodes (7): MidiDevice, cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 102 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 104 - "dialog.rs"
Cohesion: 0.07
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 105 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (33): alt_click(), button_rect(), channel_name(), describe(), describe_mapping(), fader(), fader_channels(), fires_id() (+25 more)

### Community 106 - ".objects_tab"
Cohesion: 0.33
Nodes (5): ViewCtx, Folder, VizView, VizView, button()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "titlebar.rs"
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

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
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (27): deg_quat(), euler_matches_core_rotation(), quat_deg(), drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects() (+19 more)

### Community 120 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.11
Nodes (33): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+25 more)

### Community 123 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 127 - "egui"
Cohesion: 0.09
Nodes (15): apply(), View, FixtureSettings, SetupView, Arrange, AlignH, AlignV, Grid (+7 more)

### Community 128 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 129 - "FixtureType"
Cohesion: 0.08
Nodes (31): ChannelDef, ChannelRange, FixtureType, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color (+23 more)

### Community 130 - "t"
Cohesion: 0.13
Nodes (22): t(), builtin(), load_user(), named(), NamedPalette, path(), save_user(), apply_trigger_pick() (+14 more)

### Community 131 - "strokekind"
Cohesion: 0.13
Nodes (8): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Swatch, XyPad

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
Cohesion: 0.08
Nodes (17): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+9 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.12
Nodes (24): AMBER, BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos() (+16 more)

### Community 142 - "ChannelRef"
Cohesion: 0.10
Nodes (5): blending_does_not_depend_on_the_order_of_values(), lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, ChannelRef

### Community 143 - "Clip"
Cohesion: 0.09
Nodes (12): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth, Layer (+4 more)

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 145 - ".ui"
Cohesion: 0.14
Nodes (14): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead() (+6 more)

### Community 146 - "panels.rs"
Cohesion: 0.15
Nodes (13): End, Measure, snap_points(), SNAP_PX, apply_placement(), GizmoDrag, move_all(), Pick (+5 more)

### Community 147 - "Scene"
Cohesion: 0.15
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), steps_insert_remove_and_normalize(), Zone

### Community 148 - "time_field.rs"
Cohesion: 0.16
Nodes (14): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+6 more)

### Community 149 - "views/touch.rs"
Cohesion: 0.13
Nodes (22): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+14 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.15
Nodes (26): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+18 more)

### Community 152 - ".modes_tab"
Cohesion: 0.12
Nodes (18): list_header(), list_row(), ModeOp, Add, Down, Remove, Up, separator_v() (+10 more)

### Community 153 - "TouchItem"
Cohesion: 0.23
Nodes (13): is_on(), sound_volume(), state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value() (+5 more)

### Community 154 - "ClipContent"
Cohesion: 0.12
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 155 - "text_field"
Cohesion: 0.14
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 156 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 158 - "mcp/src/profiles.rs"
Cohesion: 0.30
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 159 - "settings.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "patch_grid.rs"
Cohesion: 0.16
Nodes (14): MenuPlace, Patch, Stage, StageEditor, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS (+6 more)

### Community 163 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.14
Nodes (20): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+12 more)

### Community 166 - "server.rs"
Cohesion: 0.23
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 167 - "mcp.rs"
Cohesion: 0.17
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "fixture_settings.rs"
Cohesion: 0.15
Nodes (9): Toggle, Quick, Center, Off, On, body(), caption(), hanging_of() (+1 more)

### Community 169 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 170 - "Snapshot"
Cohesion: 0.26
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "Effect"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 173 - "midi.rs"
Cohesion: 0.21
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 176 - "tools.rs"
Cohesion: 0.08
Nodes (65): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_clip_fields(), apply_scene_fields(), base64(), boolean() (+57 more)

### Community 177 - "super"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.09
Nodes (14): Segmented<'a, T>, bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT, TrackingView (+6 more)

### Community 182 - "paint_symbol"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (43): Timeline, time_field(), advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip() (+35 more)

### Community 184 - "range_slider.rs"
Cohesion: 0.16
Nodes (9): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+1 more)

### Community 185 - "Engine"
Cohesion: 0.09
Nodes (3): Engine, Track, TrackLight

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 189 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 190 - "file_dialog.rs"
Cohesion: 0.24
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 193 - "bridge.rs"
Cohesion: 0.29
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 195 - "app.rs"
Cohesion: 0.25
Nodes (7): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input()

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 201 - "Knob<'a>"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "SceneKind"
Cohesion: 0.29
Nodes (4): SceneKind, Fx, Static, Super

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (17): AppState, Focus, Bank, Effect, Scenes, Mode, Builder, Control (+9 more)

### Community 219 - "DeckState"
Cohesion: 0.29
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 220 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 225 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 226 - "Mapper"
Cohesion: 0.13
Nodes (17): bank_step_wraps(), channel_mapping_sets_live_value(), default_feedback(), fire(), fixed_channels_work_without_selection(), index_targets(), is_value_control(), Mapper (+9 more)

### Community 229 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

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
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 252 - "Easing"
Cohesion: 0.22
Nodes (8): Easing, .ALL, Custom, EaseIn, EaseInOut, EaseOut, EaseOutIn, Linear

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 266 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (13): SceneProgress, BankScope, All, Bank, Banks, Off, OutsideBank, LoopMode (+5 more)

### Community 292 - "Profile"
Cohesion: 0.11
Nodes (13): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+5 more)

### Community 309 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

## Knowledge Gaps
- **576 isolated node(s):** `Properties`, `Effects`, `Third`, `Jump`, `Release` (+571 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1484 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **148 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `t()` connect `t` to `library_panel.rs`, `strokekind`, `programmer.rs`, `VizView`, `i18n/mod.rs`, `props.rs`, `assist.rs`, `control/timeline.rs`, `viz/library.rs`, `.ui`, `panels.rs`, `titlebar.rs`, `time_field.rs`, `views/touch.rs`, `text_dim`, `.modes_tab`, `TouchItem`, `text_field`, `fx_ui.rs`, `patch_grid.rs`, `ViewCtx`, `Profile`, `profile_builder.rs`, `FixtureAction`, `scene_grid.rs`, `fixture_settings.rs`, `missing_audio.rs`, `DmxCraftApp`, `Ui`, `fx_color.rs`, `control/tracking.rs`, `paint_symbol`, `scene_props.rs`, `file_dialog.rs`, `ProfileBuilder`, `snap.rs`, `app.rs`, `window.rs`, `stage.rs`, `Calibrate`, `toolbar.rs`, `ShutterEffect`, `viz/mod.rs`, `textures.rs`, `Settings`, `views/sound.rs`, `dialog.rs`, `mapping/mod.rs`, `.objects_tab`, `titlebar.rs`, `outline.rs`, `egui`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Are the 198 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 198 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Properties`, `Effects`, `Third` to the rest of the system?**
  _576 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.12299465240641712 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `t`, `runtime.rs`, `programmer.rs`, `core/src/lib.rs`, `mcp/src/venue.rs`, `control/timeline.rs`, `ChannelRef`, `.ui`, `time_field.rs`, `views/touch.rs`, `text_dim`, `rig.rs`, `TouchItem`, `Host`, `mcp/src/profiles.rs`, `app/src/audio.rs`, `profile_builder.rs`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `midi.rs`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `control/tracking.rs`, `.fade_state`, `scene_props.rs`, `ProfileBuilder`, `app.rs`, `window.rs`, `stage.rs`, `Show`, `Calibrate`, `engine.rs`, `app/src/obs.rs`, `Mapper`, `audio_triggers.rs`, `mapping/mod.rs`, `.objects_tab`, `programmer_overrides_and_master_scales`, `egui`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12413793103448276 - nodes in this community are weakly interconnected._