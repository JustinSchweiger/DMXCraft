# Graph Report - dmxcraft-src  (2026-11-13)

## Corpus Check
- 165 files · ~268,491 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4637 nodes · 11880 edges · 327 communities (166 shown, 161 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 636 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a4bd4da3`
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
- core/src/lib.rs
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
- Scene
- ViewCtx
- dmxcraft
- app/src/audio.rs
- t
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
- show.rs
- ShutterEffect
- profile.rs
- Obs
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
- Profile
- arc
- Placement
- call
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
- ChannelRef
- Clip
- DeviceModel
- .ui
- panels.rs
- ZoneId
- time_field.rs
- views/touch.rs
- Drag
- text_dim
- ProfileBuilder
- ViewCtx
- run
- text_field
- TouchItem
- mcp/src/profiles.rs
- settings.rs
- SlotKind
- Box<dyn serialport::SerialPort>
- Transform
- Problem
- server.rs
- mcp.rs
- View
- icon_library.rs
- Snapshot
- Easing
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
- ClipContent
- apply_effect_hold
- MoveShape
- env
- T
- de.rs
- Knob<'a>
- Calibrate
- FixtureLimits
- widgets/mod.rs
- build-local-artifacts
- apply
- AppState
- DeckState
- HeaderAction
- plan.rs
- Mapper
- FooterButton<'a>
- .check_placement
- views/sound.rs
- announce
- lock.rs
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- ChaserPreset
- .update
- with
- BankScope
- Show
- ok
- Profile
- Ui
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
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `angle_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs

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

## Communities (327 total, 161 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.16
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Venue"
Cohesion: 0.13
Nodes (7): Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, room_and_tree(), Venue

### Community 5 - "programmer.rs"
Cohesion: 0.07
Nodes (45): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+37 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.25
Nodes (18): all_presets_render_every_beam(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels(), hard_edges_use_palette_colors_only(), knight_rider(), knight_rider_lights_one_bar() (+10 more)

### Community 9 - "DmxOutput"
Cohesion: 0.11
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer, Failing

### Community 10 - "props.rs"
Cohesion: 0.11
Nodes (21): fold_section(), renumber(), apply_texture(), TexAction, Builtin, Load, Set, aim_fixtures() (+13 more)

### Community 11 - "assist.rs"
Cohesion: 0.13
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

### Community 12 - "target_shape"
Cohesion: 0.11
Nodes (29): alt_click(), button(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id() (+21 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.10
Nodes (19): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+11 more)

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

### Community 20 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (13): cv(), demo_show(), empty_show(), ensure_builtin_types(), UNIVERSE_SIZE, ChannelValue, PLAN_SIZE_RANGE, SceneKind (+5 more)

### Community 21 - "Deserialize"
Cohesion: 0.08
Nodes (10): all_cells(), DEFAULT_HEIGHT, white(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob (+2 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.07
Nodes (40): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+32 more)

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
Cohesion: 0.09
Nodes (39): t(), builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui() (+31 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "Scene"
Cohesion: 0.16
Nodes (10): Scene, audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), group_moves_together_without_covering_others(), level_points_shape_the_course(), media_and_light_tracks_stay_apart(), paste_finds_free_place_and_keeps_gaps() (+2 more)

### Community 32 - "ViewCtx"
Cohesion: 0.15
Nodes (13): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+5 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.14
Nodes (16): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+8 more)

### Community 36 - "t"
Cohesion: 0.07
Nodes (48): kind_color(), drag_ghost(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+40 more)

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
Nodes (44): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+36 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (49): ControlViews, programmer_values(), ProgrammerView, Action, Bank, Compact, CreateSuper, Delete (+41 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

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
Cohesion: 0.14
Nodes (6): DmxCraftApp, fill_types(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input()

### Community 48 - "Ui"
Cohesion: 0.11
Nodes (38): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+30 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "look.rs"
Cohesion: 0.25
Nodes (16): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, nearest_slot(), position_in_degrees() (+8 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.08
Nodes (31): add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, has_sound_cached() (+23 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (25): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.13
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "ProfileChannel"
Cohesion: 0.40
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 72 - "Show"
Cohesion: 0.18
Nodes (5): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.18
Nodes (14): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), custom_value(), fill_ends_full() (+6 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.16
Nodes (13): along(), catmull(), dist(), insert_and_remove_points(), lerp(), points_jump(), points_travel_the_closed_path(), proj() (+5 more)

### Community 77 - "engine.rs"
Cohesion: 0.15
Nodes (48): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below(), finished_scene_holds_effects_at_their_end() (+40 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.23
Nodes (7): select_all(), chip(), divider(), level(), point_chip(), point_width(), VizView

### Community 79 - "theme.rs"
Cohesion: 0.05
Nodes (66): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+58 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.26
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 87 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (20): beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+12 more)

### Community 90 - "Obs"
Cohesion: 0.14
Nodes (7): Obs, Status, Connected, Connecting, Failed, Off, VideoPlay

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.11
Nodes (34): apply_placement(), decode_texture(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag, Handles, handles() (+26 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (32): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+24 more)

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
Cohesion: 0.12
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 102 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 104 - "dialog.rs"
Cohesion: 0.07
Nodes (12): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind (+4 more)

### Community 105 - "mapping/mod.rs"
Cohesion: 0.20
Nodes (20): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+12 more)

### Community 106 - "Profile"
Cohesion: 0.17
Nodes (5): import(), MemoryProfiles, fine_names(), Profile, rename_and_resolution_update_modes()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "call"
Cohesion: 0.20
Nodes (26): add_clips(), add_effect(), add_fixtures(), base64(), build_steps(), call(), changed_scene(), create_scene() (+18 more)

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.18
Nodes (15): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+7 more)

### Community 117 - "Object"
Cohesion: 0.15
Nodes (15): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+7 more)

### Community 118 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 119 - "outline.rs"
Cohesion: 0.10
Nodes (24): deg_quat(), drag_row(), DragFixtures, DragObjects, move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+16 more)

### Community 120 - "app/src/obs.rs"
Cohesion: 0.18
Nodes (12): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+4 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "Timeline"
Cohesion: 0.14
Nodes (9): add_audio(), add_audio_dialog(), has_audio(), media_track(), GridMode, Beat, Time, linked_blocks_stay_together() (+1 more)

### Community 127 - "stage_editor.rs"
Cohesion: 0.14
Nodes (12): fit(), fixture_pos(), apply(), Arrange, AlignH, AlignV, Grid, Reset (+4 more)

### Community 128 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 129 - "ChannelDef"
Cohesion: 0.10
Nodes (14): ChannelDef, ChannelRange, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color, Gobo (+6 more)

### Community 130 - "core/src/venue.rs"
Cohesion: 0.13
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
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 136 - "VizView"
Cohesion: 0.08
Nodes (17): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+9 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.04
Nodes (36): DipSwitch, SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY (+28 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.10
Nodes (25): AMBER, BOX, Camera, fixture_rect(), live_colors(), MenuFn, paint_effects(), paint_fixture() (+17 more)

### Community 142 - "ChannelRef"
Cohesion: 0.12
Nodes (6): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, ChannelRef

### Community 143 - "Clip"
Cohesion: 0.10
Nodes (11): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth, Layer (+3 more)

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), MidiDevice (+2 more)

### Community 145 - ".ui"
Cohesion: 0.15
Nodes (13): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_segments() (+5 more)

### Community 146 - "panels.rs"
Cohesion: 0.22
Nodes (5): End, Measure, snap_points(), SNAP_PX, project()

### Community 147 - "ZoneId"
Cohesion: 0.14
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), steps_insert_remove_and_normalize(), Zone

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 149 - "views/touch.rs"
Cohesion: 0.14
Nodes (21): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+13 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.12
Nodes (35): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+27 more)

### Community 152 - "ProfileBuilder"
Cohesion: 0.13
Nodes (12): FileJob, Export, Import, new_profile(), ProfileBuilder, Tab, .ALL, Channels (+4 more)

### Community 153 - "ViewCtx"
Cohesion: 0.27
Nodes (6): ViewCtx, drop_marker(), Folder, VizView, VizView, button()

### Community 154 - "run"
Cohesion: 0.26
Nodes (5): Link, run(), session(), Shared, ObsSettings

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 156 - "TouchItem"
Cohesion: 0.26
Nodes (11): state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of(), paint_cover() (+3 more)

### Community 158 - "mcp/src/profiles.rs"
Cohesion: 0.27
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 159 - "settings.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 164 - "Transform"
Cohesion: 0.22
Nodes (8): normalized(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "server.rs"
Cohesion: 0.13
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 167 - "mcp.rs"
Cohesion: 0.17
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "View"
Cohesion: 0.21
Nodes (7): View, body(), caption(), FixtureSettings, hanging_of(), range_row(), SetupView

### Community 169 - "icon_library.rs"
Cohesion: 0.29
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 170 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "Easing"
Cohesion: 0.12
Nodes (12): EaseScope, Cycle, Leg, Easing, .ALL, Custom, EaseIn, EaseInOut (+4 more)

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (33): any_obj(), apply_clip_fields(), apply_scene_fields(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values() (+25 more)

### Community 177 - "super"
Cohesion: 0.19
Nodes (13): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches(), bx(), find_box() (+5 more)

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "text_field"
Cohesion: 0.16
Nodes (11): text_field(), bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT, TrackingView (+3 more)

### Community 182 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.05
Nodes (57): Timeline, advanced(), apply_to_others(), bar_slider(), block_props(), chip(), effect_tiles(), fit_mapping() (+49 more)

### Community 184 - "range_slider.rs"
Cohesion: 0.16
Nodes (9): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+1 more)

### Community 185 - "Engine"
Cohesion: 0.08
Nodes (9): edit_preview(), load_step(), Engine, FadeState, In, Out, SceneProgress, Track (+1 more)

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 190 - "file_dialog.rs"
Cohesion: 0.24
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "ClipContent"
Cohesion: 0.13
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 193 - "apply_effect_hold"
Cohesion: 0.31
Nodes (9): apply(), apply_effect(), apply_effect_hold(), Beam, beams(), feature_levels(), luma(), to_rgb() (+1 more)

### Community 195 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 205 - "widgets/mod.rs"
Cohesion: 0.25
Nodes (7): LimitPad, CanvasMode, Edit, Select, StageCanvas, Underlay, XyPad

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "apply"
Cohesion: 0.29
Nodes (3): apply(), circle_goes_round(), local_roundtrip()

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (17): AppState, Focus, Bank, Effect, Scenes, Mode, Builder, Control (+9 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 220 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 225 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 226 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 229 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

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

### Community 244 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 252 - ".update"
Cohesion: 0.33
Nodes (8): fire(), media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 266 - "BankScope"
Cohesion: 0.20
Nodes (8): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 289 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 292 - "Profile"
Cohesion: 0.19
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 309 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

## Knowledge Gaps
- **576 isolated node(s):** `Dimmer`, `Red`, `Green`, `Blue`, `White` (+571 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1494 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **161 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `t()` connect `fx_ui.rs` to `library_panel.rs`, `programmer.rs`, `VizView`, `i18n/mod.rs`, `props.rs`, `assist.rs`, `egui`, `viz/library.rs`, `.ui`, `panels.rs`, `titlebar.rs`, `views/touch.rs`, `text_dim`, `ProfileBuilder`, `ViewCtx`, `rig.rs`, `text_field`, `TouchItem`, `ViewCtx`, `Profile`, `t`, `FixtureAction`, `scene_grid.rs`, `View`, `missing_audio.rs`, `DmxCraftApp`, `Ui`, `text_field`, `fader.rs`, `scene_props.rs`, `file_dialog.rs`, `control/timeline.rs`, `window.rs`, `stage.rs`, `Calibrate`, `toolbar.rs`, `viz/mod.rs`, `textures.rs`, `Settings`, `views/sound.rs`, `dialog.rs`, `mapping/mod.rs`, `outline.rs`, `stage_editor.rs`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._
- **Are the 198 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 198 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Dimmer`, `Red`, `Green` to the rest of the system?**
  _576 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `mcp/src/venue.rs`, `egui`, `ChannelRef`, `.ui`, `core/src/lib.rs`, `views/touch.rs`, `text_dim`, `ProfileBuilder`, `ViewCtx`, `rig.rs`, `TouchItem`, `Host`, `mcp/src/profiles.rs`, `app/src/audio.rs`, `t`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `View`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `text_field`, `scene_props.rs`, `DmxFrame`, `control/timeline.rs`, `window.rs`, `stage.rs`, `Show`, `Calibrate`, `engine.rs`, `Obs`, `Mapper`, `audio_triggers.rs`, `mapping/mod.rs`, `call`, `app/src/obs.rs`, `.update`, `stage_editor.rs`?**
  _High betweenness centrality (0.089) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11586452762923351 - nodes in this community are weakly interconnected._