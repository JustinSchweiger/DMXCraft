# Graph Report - dmxcraft-src  (2026-11-10)

## Corpus Check
- 145 files · ~221,257 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4068 nodes · 10882 edges · 249 communities (145 shown, 104 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 531 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `27146209`
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
- patch_grid.rs
- app/src/obs.rs
- viz/library.rs
- jobs
- steps
- README.md
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
- Dialog<'a>
- ofl/src/lib.rs
- fx_ui.rs
- env
- dmxcraft
- app/src/audio.rs
- t
- fx/mapping.rs
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- scene_card.rs
- textures.rs
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- settings_window.rs
- toolbar.rs
- remote/mod.rs
- TouchLayout
- assist.rs
- MenuAction
- window.rs
- stage.rs
- mcp.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- View
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
- Knob<'a>
- ModelPart
- Deserialize
- .push
- core/src/obs.rs
- arc
- Control
- TimelineView
- app.rs
- server.rs
- Shape
- Venue
- dialog.rs
- outline.rs
- DmxLine
- convert.rs
- Timeline
- Host
- widgets/mod.rs
- ShutterEffect
- protocol.rs
- apply_pick
- CurveShape
- ColorPreset
- UpdateBadge
- control/timeline.rs
- i18n/mod.rs
- lock.rs
- theme.rs
- stage_canvas.rs
- swatch.rs
- missing_audio.rs
- pointer
- bridge.rs
- Mapper
- EffectKind
- item
- mapping/mod.rs
- text_dim
- Physics
- icon_library.rs
- audio_trigger_ui.rs
- core/src/sound.rs
- OutputStatus
- .add_scene
- show.rs
- color_picker.rs
- widgets/mod.rs
- Transform
- Problem
- views/touch.rs
- Profile
- gltf_import.rs
- BankScope
- Fullscreen
- midi.rs
- time_field.rs
- tools.rs
- PlanShape
- DmxFrame
- ClipAction
- Layer
- qxf.rs
- Fader
- scene_props.rs
- TouchKind
- Engine
- super
- plan.rs
- DeckState
- qlc-bundle.rs
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
- fx_color.rs
- FooterButton<'a>
- AudioPlayer
- .fade_state
- FixtureLimits
- ViewCtx
- Scene

## God Nodes (most connected - your core abstractions)
1. `Engine` - 227 edges
2. `t()` - 211 edges
3. `Show` - 147 edges
4. `Icon` - 115 edges
5. `AppState` - 102 edges
6. `text_dim()` - 94 edges
7. `ViewCtx` - 88 edges
8. `ChannelKind` - 75 edges
9. `text()` - 67 edges
10. `border()` - 64 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (249 total, 104 thin omitted)

### Community 0 - "SoundPlayer"
Cohesion: 0.12
Nodes (3): SoundPlayer, poll(), SoundBoard

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (15): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), LibAction (+7 more)

### Community 2 - "runtime.rs"
Cohesion: 0.17
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.14
Nodes (16): BREAK, DMX_BAUD, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB (+8 more)

### Community 4 - "Theme"
Cohesion: 0.09
Nodes (17): ensure_linux_menu_entry(), register_mime(), window_icon(), main(), active(), set_theme(), Theme, .ALL (+9 more)

### Community 5 - "programmer.rs"
Cohesion: 0.12
Nodes (33): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+25 more)

### Community 6 - "AppState"
Cohesion: 0.10
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (29): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+21 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "Deserialize"
Cohesion: 0.13
Nodes (6): RECENT_MAX, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob

### Community 11 - "patch_grid.rs"
Cohesion: 0.13
Nodes (13): DipSwitch, Segmented, Segmented<'a, T>, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag (+5 more)

### Community 12 - "app/src/obs.rs"
Cohesion: 0.09
Nodes (32): auth_matches_obs_websocket_example(), auth_response(), b64(), connects_and_plays_video_block(), engine(), fake_obs(), fire(), Link (+24 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.12
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.14
Nodes (13): env, if, needs, runs-on, jobs, announce, name, on (+5 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (38): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+30 more)

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
Cohesion: 0.08
Nodes (35): analyze(), AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), BPM_MAX, BPM_MIN (+27 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (24): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+16 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "mcp/src/profiles.rs"
Cohesion: 0.37
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 29 - "Dialog<'a>"
Cohesion: 0.14
Nodes (5): Dialog<'a>, Kind, Cancel, Primary, Secondary

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.18
Nodes (19): canvas(), color_fx_ui(), edit_colors(), label_row(), slider_row(), toggle_row(), BodyCtx, combo() (+11 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.16
Nodes (12): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+4 more)

### Community 36 - "t"
Cohesion: 0.06
Nodes (58): moved_out(), t(), heading_text(), text_field(), kind_color(), capability_fields(), category_label(), check() (+50 more)

### Community 37 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (12): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+4 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.08
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 43 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "settings_window.rs"
Cohesion: 0.05
Nodes (52): default_feedback(), APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+44 more)

### Community 50 - "toolbar.rs"
Cohesion: 0.28
Nodes (5): load_meter(), missing_alert(), show(), FLY_SECS, H

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "assist.rs"
Cohesion: 0.14
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (26): MenuAction, About, AudioFade, AudioMaster, AudioMute, CheckUpdates, DmxLevels, InstallUpdate (+18 more)

### Community 69 - "window.rs"
Cohesion: 0.14
Nodes (23): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, MapFile (+15 more)

### Community 70 - "stage.rs"
Cohesion: 0.14
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "mcp.rs"
Cohesion: 0.14
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.17
Nodes (9): .VIRTUAL_DIMMER, Fixture, paste_fixtures_finds_free_addresses_and_names(), PatchError, BadUniverse, OutOfRange, Overlap, UnknownType (+1 more)

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.20
Nodes (35): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+27 more)

### Community 78 - "View"
Cohesion: 0.14
Nodes (8): View, body(), caption(), FixtureSettings, range_row(), SetupView, ZonesView, Windows

### Community 79 - "egui"
Cohesion: 0.10
Nodes (12): SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY, LimitPad (+4 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "views/sound.rs"
Cohesion: 0.17
Nodes (21): button(), rgb(), color_swatches(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar() (+13 more)

### Community 87 - "fire"
Cohesion: 0.21
Nodes (14): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+6 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (27): add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), file_slug() (+19 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (38): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles (+30 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (27): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+19 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.07
Nodes (48): ControlViews, programmer_values(), ProgrammerView, Action, Bank, Compact, CreateSuper, Delete (+40 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.10
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 104 - "Deserialize"
Cohesion: 0.13
Nodes (12): ChannelDef, ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other (+4 more)

### Community 105 - ".push"
Cohesion: 0.18
Nodes (10): Analyzer, Biquad, Envelope, envelope_filters_band(), FilterKind, Band, High, Low (+2 more)

### Community 106 - "core/src/obs.rs"
Cohesion: 0.60
Nodes (4): bx(), find_box(), mp4_duration_ms(), reads_mp4_length()

### Community 108 - "Control"
Cohesion: 0.07
Nodes (36): Overlay, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade (+28 more)

### Community 109 - "TimelineView"
Cohesion: 0.24
Nodes (6): paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), TimelineView

### Community 110 - "app.rs"
Cohesion: 0.27
Nodes (6): edit_preview(), fill_types(), load_step(), LockDialog, OUTPUT_HZ, viewport_input()

### Community 112 - "server.rs"
Cohesion: 0.18
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 113 - "Shape"
Cohesion: 0.09
Nodes (24): aim_at_points_beam_to_target(), argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node() (+16 more)

### Community 117 - "Venue"
Cohesion: 0.09
Nodes (17): apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), IDENTITY, Material (+9 more)

### Community 118 - "dialog.rs"
Cohesion: 0.12
Nodes (8): Button, Dialog, DialogResult, FOOTER_H, FooterButton, HEADER_H, INPUT_H, LABEL_W

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (28): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+20 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (24): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+16 more)

### Community 123 - "Timeline"
Cohesion: 0.09
Nodes (20): add_video(), has_audio(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Obs, Scene (+12 more)

### Community 127 - "Host"
Cohesion: 0.21
Nodes (9): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+1 more)

### Community 128 - "widgets/mod.rs"
Cohesion: 0.16
Nodes (11): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), StageEditorView (+3 more)

### Community 129 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 130 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 131 - "apply_pick"
Cohesion: 0.38
Nodes (7): channel_name(), describe(), describe_mapping(), t_deleted(), apply_pick(), pick_adds_scene_button_and_assigns_to_selection(), programmer_fader_becomes_fixed_channels()

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
Cohesion: 0.09
Nodes (19): add_audio(), add_audio_dialog(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, fmt_ms(), HEADER_W (+11 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (15): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 138 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (44): accent_hover(), apply(), BAND_HIGH, BAND_LOW, BAND_MID, BLACK, c(), canvas_dim() (+36 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 144 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 145 - "bridge.rs"
Cohesion: 0.29
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 146 - "Mapper"
Cohesion: 0.19
Nodes (7): fires_id(), Mapper, overlay_id(), pick_id(), Sent, states_id(), ui_states()

### Community 147 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 149 - "item"
Cohesion: 0.10
Nodes (28): audio_control(), select_all(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item() (+20 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.15
Nodes (23): alt_click(), button_rect(), fader(), fader_channels(), in_rotated(), Pick, pick(), pick_full() (+15 more)

### Community 151 - "text_dim"
Cohesion: 0.13
Nodes (38): table(), accent(), accent_dim(), bg(), body(), border(), caption(), heading_font() (+30 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.25
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 155 - "audio_trigger_ui.rs"
Cohesion: 0.39
Nodes (3): apply_trigger_pick(), trigger_cx(), TriggerCx

### Community 157 - "core/src/sound.rs"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 158 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 159 - ".add_scene"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings, steps_insert_remove_and_normalize()

### Community 160 - "show.rs"
Cohesion: 0.12
Nodes (22): UNIVERSE_SIZE, address_map_marks_ranges(), Group, groups_follow_fixture_removal(), json_roundtrip(), LoopMode, Loop, Once (+14 more)

### Community 162 - "color_picker.rs"
Cohesion: 0.15
Nodes (12): color_picker(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H, Swatch (+4 more)

### Community 163 - "widgets/mod.rs"
Cohesion: 0.18
Nodes (10): FaderEvent, Changed, None, Release, ToggleFx, RangeSlider, CanvasMode, Edit (+2 more)

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.12
Nodes (29): apply(), state_json(), channels_of(), COLORS, content_rect(), current_color(), current_position(), display_label() (+21 more)

### Community 167 - "Profile"
Cohesion: 0.09
Nodes (14): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+6 more)

### Community 168 - "gltf_import.rs"
Cohesion: 0.16
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 169 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 171 - "Fullscreen"
Cohesion: 0.15
Nodes (8): Fullscreen, SETTLE, Step, Enter, Maximize, drag_number(), endless_delta(), raw_available()

### Community 173 - "midi.rs"
Cohesion: 0.14
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 174 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 176 - "tools.rs"
Cohesion: 0.08
Nodes (73): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), base64(), boolean(), build_steps() (+65 more)

### Community 177 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

### Community 180 - "Layer"
Cohesion: 0.11
Nodes (13): ProgrammerMode, Blind, Edit, Live, items_mode(), Layer, Live, Programmer (+5 more)

### Community 181 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 182 - "Fader"
Cohesion: 0.11
Nodes (10): DOT_H, Fader, FADER_W, FaderRange, HANDLE, NUM_H, SYMBOL_H, VALUE_H (+2 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.08
Nodes (40): zone_color(), drag_ghost(), advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), fit_mapping() (+32 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Engine"
Cohesion: 0.07
Nodes (6): super_scene(), Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 187 - "super"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 196 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.22
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 200 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ViewCtx"
Cohesion: 0.17
Nodes (11): section_header(), add_page(), ImageTarget, Item, Page, pick_image(), prepare_image(), select_only() (+3 more)

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
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 223 - "fx_color.rs"
Cohesion: 0.18
Nodes (11): builtin(), load_user(), named(), NamedPalette, path(), save_user(), LABEL_W, paint_strip() (+3 more)

### Community 251 - "AudioPlayer"
Cohesion: 0.13
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 273 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 278 - "ViewCtx"
Cohesion: 0.10
Nodes (19): fold_section(), ViewCtx, DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue (+11 more)

## Knowledge Gaps
- **546 isolated node(s):** `Loading`, `De`, `En`, `Off`, `Connecting` (+541 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1299 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **104 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `apply_pick`, `programmer.rs`, `control/timeline.rs`, `app/src/obs.rs`, `theme.rs`, `.fade_state`, `Mapper`, `item`, `ViewCtx`, `text_dim`, `rig.rs`, `audio_trigger_ui.rs`, `mcp/src/profiles.rs`, `show.rs`, `app/src/audio.rs`, `t`, `views/touch.rs`, `Profile`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `Layer`, `scene_props.rs`, `super`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `engine.rs`, `View`, `fire`, `scene_grid.rs`, `audio_triggers.rs`, `Timeline`, `TimelineView`, `app.rs`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.148) - this node is a cross-community bridge._
- **Are the 175 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 175 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `De`, `En` to the rest of the system?**
  _546 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.11861861861861862 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `widgets/mod.rs`, `library_panel.rs`, `apply_pick`, `programmer.rs`, `control/timeline.rs`, `i18n/mod.rs`, `Deserialize`, `patch_grid.rs`, `app/src/obs.rs`, `theme.rs`, `viz/library.rs`, `missing_audio.rs`, `titlebar.rs`, `item`, `ViewCtx`, `text_dim`, `audio_trigger_ui.rs`, `fx_ui.rs`, `color_picker.rs`, `views/touch.rs`, `Profile`, `FixtureAction`, `textures.rs`, `DmxCraftApp`, `settings_window.rs`, `toolbar.rs`, `Fader`, `scene_props.rs`, `assist.rs`, `window.rs`, `stage.rs`, `ViewCtx`, `Purpose`, `View`, `egui`, `views/sound.rs`, `viz/mod.rs`, `fx_color.rs`, `scene_grid.rs`, `app.rs`, `dialog.rs`, `outline.rs`?**
  _High betweenness centrality (0.115) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12121212121212122 - nodes in this community are weakly interconnected._