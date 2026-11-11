# Graph Report - dmxcraft-src  (2026-11-11)

## Corpus Check
- 154 files · ~244,545 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4332 nodes · 11118 edges · 296 communities (164 shown, 132 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 581 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `5cae5dd2`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- fx_color.rs
- programmer.rs
- SoundBoard
- steps
- fx.rs
- OutputError
- ViewCtx
- DialogKind
- server.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- fx/mapping.rs
- super
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- src/mapping.rs
- Settings
- profile_builder.rs
- ofl/src/lib.rs
- fx_ui.rs
- OrbitCamera
- dmxcraft
- app/src/audio.rs
- profile_builder.rs
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- scene_grid.rs
- duration
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- settings_window.rs
- Snapshot
- remote/mod.rs
- TouchLayout
- call
- MenuAction
- .end_frame
- window.rs
- stage.rs
- mcp.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- .ui
- egui
- OpenDmx
- range_slider.rs
- fire
- profile.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- OrbitCamera
- MapPreset
- audio_triggers.rs
- autosave.rs
- Obs
- look.rs
- Rect
- DmxFrame
- arc
- panels.rs
- BankScope
- Theme
- audio_trigger_ui.rs
- Shape
- Venue
- View
- .row
- Tab
- convert.rs
- Timeline
- Host
- ProfileBuilder
- .update
- app/src/obs.rs
- scene_card.rs
- CurveShape
- ColorPreset
- qlc-bundle.rs
- control/timeline.rs
- i18n/mod.rs
- mcp/src/venue.rs
- theme.rs
- stage_canvas.rs
- Capability
- missing_audio.rs
- DeviceModel
- ShowError
- RangeEffect
- Mode
- t
- target_shape
- text_dim
- time_field.rs
- icon_library.rs
- text_field
- app/src/profiles.rs
- BankScope
- EffectKind
- dip.rs
- mcp/src/profiles.rs
- Problem
- views/touch.rs
- lock.rs
- ShutterEffect
- menu.rs
- ObsAction
- Drag
- midi.rs
- tools.rs
- show.rs
- Easing
- ClipAction
- DeckState
- Transform
- fader.rs
- scene_props.rs
- Dialog<'a>
- Engine
- plan
- PlanShape
- scene_group.rs
- plan.rs
- hashmap
- T
- ProfileChannel
- env
- apply_effect_hold
- de.rs
- MoveShape
- MenuItem<'a>
- Purpose
- Tab
- HeaderAction
- build-local-artifacts
- IconButton
- Quality
- app.rs
- Scene
- AppState
- Knob<'a>
- TriggerBand
- FooterButton<'a>
- Camera
- views/sound.rs
- announce
- OutputStatus
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- release-install.md
- AudioPlayer
- with
- StageCanvas
- .feedback
- Grip
- BuilderRequest
- OflError
- WaveState
- .fade_state

## God Nodes (most connected - your core abstractions)
1. `Engine` - 235 edges
2. `t()` - 214 edges
3. `Show` - 161 edges
4. `Icon` - 118 edges
5. `AppState` - 106 edges
6. `ViewCtx` - 93 edges
7. `ChannelKind` - 76 edges
8. `ChannelRef` - 60 edges
9. `FixtureType` - 57 edges
10. `Profile` - 53 edges

## Surprising Connections (you probably didn't know these)
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs

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

## Communities (296 total, 132 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.16
Nodes (13): LibraryDrag, template(), builtin_types(), lib_index(), LibAction, Builder, Delete, LibIndex (+5 more)

### Community 2 - "runtime.rs"
Cohesion: 0.20
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (19): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+11 more)

### Community 4 - "fx_color.rs"
Cohesion: 0.16
Nodes (16): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+8 more)

### Community 5 - "programmer.rs"
Cohesion: 0.08
Nodes (36): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+28 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.25
Nodes (18): all_presets_render_every_beam(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels(), hard_edges_use_palette_colors_only(), knight_rider(), knight_rider_lights_one_bar() (+10 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "ViewCtx"
Cohesion: 0.16
Nodes (15): renumber(), aim_fixtures(), color_field(), duplicate_object(), floor_height(), number(), percent(), RESERVE (+7 more)

### Community 11 - "DialogKind"
Cohesion: 0.20
Nodes (7): drag_ghost(), DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 12 - "server.rs"
Cohesion: 0.14
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.12
Nodes (17): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+9 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "with"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.12
Nodes (31): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+23 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 21 - "super"
Cohesion: 0.07
Nodes (17): APC_MK2_PALETTE, detect_and_match(), matching_output(), nearest(), palette_lookup(), strobe_ranges(), bx(), find_box() (+9 more)

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

### Community 27 - "src/mapping.rs"
Cohesion: 0.06
Nodes (34): Overlay, ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+26 more)

### Community 28 - "Settings"
Cohesion: 0.12
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 29 - "profile_builder.rs"
Cohesion: 0.09
Nodes (13): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+5 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.14
Nodes (13): best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, fill_details(), fill_physical(), Library (+5 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.20
Nodes (19): toggle_row(), BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid(), mapping_ui() (+11 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.17
Nodes (12): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+4 more)

### Community 36 - "profile_builder.rs"
Cohesion: 0.10
Nodes (24): kind_color(), capability_fields(), card_row(), category_label(), color_label(), FOOT_H, full_combo(), general() (+16 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (48): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+40 more)

### Community 43 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.17
Nodes (3): DmxCraftApp, fill_types(), keep()

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (25): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+17 more)

### Community 50 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "call"
Cohesion: 0.19
Nodes (28): add_clips(), add_effect(), add_fixtures(), apply_clip_fields(), apply_scene_fields(), base64(), call(), changed_scene() (+20 more)

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 68 - ".end_frame"
Cohesion: 0.14
Nodes (16): default_feedback(), fires_id(), is_value_control(), overlay_id(), Pick, pick(), pick_full(), pick_id() (+8 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (28): color(), Mapper, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+20 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "mcp.rs"
Cohesion: 0.13
Nodes (14): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+6 more)

### Community 72 - "Show"
Cohesion: 0.15
Nodes (8): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.12
Nodes (17): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+9 more)

### Community 77 - "engine.rs"
Cohesion: 0.18
Nodes (40): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+32 more)

### Community 78 - ".ui"
Cohesion: 0.17
Nodes (11): add_audio(), add_audio_dialog(), fmt_ms(), grid_options(), is_video(), mark_clipboard(), obs_label(), row_h() (+3 more)

### Community 79 - "egui"
Cohesion: 0.06
Nodes (25): DipSwitch, SIZE, SWEEP, LimitPad, Segmented, Swatch, Toggle, XyPad (+17 more)

### Community 86 - "range_slider.rs"
Cohesion: 0.16
Nodes (9): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+1 more)

### Community 87 - "fire"
Cohesion: 0.16
Nodes (20): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+12 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (22): import(), beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity_text(), fine_names(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+14 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.10
Nodes (35): apply_placement(), decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, handles(), hit_y(), Missing (+27 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.08
Nodes (36): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+28 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.19
Nodes (9): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, CameraPreset (+1 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.13
Nodes (14): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+6 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "Obs"
Cohesion: 0.31
Nodes (6): connects_and_plays_video_block(), engine(), fake_obs(), Obs, wait(), wrong_password_is_reported()

### Community 104 - "look.rs"
Cohesion: 0.15
Nodes (20): ChannelDef, FixtureType, Physics, ft(), color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget (+12 more)

### Community 105 - "Rect"
Cohesion: 0.18
Nodes (14): list_header(), ListAction, Add, Duplicate, Remove, ModeOp, Add, Down (+6 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "panels.rs"
Cohesion: 0.12
Nodes (10): proposals_for_easy_view_names(), propose_type(), apply_texture(), OutlineTab, Fixtures, Objects, TexAction, Builtin (+2 more)

### Community 109 - "BankScope"
Cohesion: 0.22
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 110 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 112 - "audio_trigger_ui.rs"
Cohesion: 0.21
Nodes (8): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.07
Nodes (22): load_file(), aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss() (+14 more)

### Community 118 - "View"
Cohesion: 0.09
Nodes (18): Show, apply(), View, body(), caption(), FixtureSettings, hanging_of(), range_row() (+10 more)

### Community 119 - ".row"
Cohesion: 0.09
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (28): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+20 more)

### Community 123 - "Timeline"
Cohesion: 0.07
Nodes (27): add_video(), has_audio(), media_track(), ungroup_clip(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio (+19 more)

### Community 127 - "Host"
Cohesion: 0.36
Nodes (11): Host, call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list() (+3 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.19
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 129 - ".update"
Cohesion: 0.19
Nodes (13): media_action(), media_settings(), names(), run(), seek(), send_json(), session(), Shared (+5 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.15
Nodes (9): auth_matches_obs_websocket_example(), auth_response(), b64(), Link, open(), read_json(), VideoPlay, Ws (+1 more)

### Community 131 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 136 - "control/timeline.rs"
Cohesion: 0.10
Nodes (23): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, drop_hint(), EDGE, HEADER_W, LANE_H, Mark (+15 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.39
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (53): accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+45 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (18): AMBER, BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_effects() (+10 more)

### Community 142 - "Capability"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 143 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 144 - "DeviceModel"
Cohesion: 0.20
Nodes (5): DeviceModel, .ALL, ApcMiniMk2, Generic, MidiDevice

### Community 145 - "ShowError"
Cohesion: 0.32
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 147 - "RangeEffect"
Cohesion: 0.17
Nodes (9): ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other, Strobe (+1 more)

### Community 148 - "Mode"
Cohesion: 0.18
Nodes (8): Mode, Builder, Control, Setup, Sound, .TABS, Touch, Visualizer

### Community 149 - "t"
Cohesion: 0.10
Nodes (19): t(), fold_section(), section_header(), ViewCtx, ZonesView, add_page(), add_tile(), icon_picker() (+11 more)

### Community 150 - "target_shape"
Cohesion: 0.20
Nodes (15): alt_click(), button(), button_rect(), fader(), fader_channels(), in_rotated(), report(), rotated_quad() (+7 more)

### Community 151 - "text_dim"
Cohesion: 0.16
Nodes (28): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), fixture_bar() (+20 more)

### Community 152 - "time_field.rs"
Cohesion: 0.27
Nodes (10): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+2 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "app/src/profiles.rs"
Cohesion: 0.19
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 159 - "BankScope"
Cohesion: 0.14
Nodes (7): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Show, steps_insert_remove_and_normalize(), Zone

### Community 160 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "mcp/src/profiles.rs"
Cohesion: 0.13
Nodes (15): AudioDuration, DEFAULT_PORT, MemoryProfiles, ProfileStore, VizFn, create_list_update_delete(), delete(), format() (+7 more)

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.09
Nodes (36): apply(), state_json(), apply_pick(), channels_of(), COLORS, content_rect(), current_color(), current_position() (+28 more)

### Community 167 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 168 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 169 - "menu.rs"
Cohesion: 0.12
Nodes (16): select_all(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H (+8 more)

### Community 170 - "ObsAction"
Cohesion: 0.50
Nodes (4): fire(), ObsAction, Scene, Source

### Community 171 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (37): any_obj(), boolean(), build_steps(), camera_schema(), channel_kind(), clip_schema(), collect_values(), existing_fixtures() (+29 more)

### Community 177 - "show.rs"
Cohesion: 0.09
Nodes (25): SceneProgress, address_map_marks_ranges(), FixtureLimits, Group, groups_follow_fixture_removal(), json_roundtrip(), LoopMode, Loop (+17 more)

### Community 178 - "Easing"
Cohesion: 0.14
Nodes (12): EaseScope, Cycle, Leg, Easing, .ALL, Custom, EaseIn, EaseInOut (+4 more)

### Community 179 - "ClipAction"
Cohesion: 0.10
Nodes (18): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+10 more)

### Community 180 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 181 - "Transform"
Cohesion: 0.22
Nodes (8): normalized(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 182 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.08
Nodes (43): time_field(), advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header() (+35 more)

### Community 185 - "Engine"
Cohesion: 0.06
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 190 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 191 - "scene_group.rs"
Cohesion: 0.29
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 193 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 195 - "ProfileChannel"
Cohesion: 0.19
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "apply_effect_hold"
Cohesion: 0.31
Nodes (9): apply(), apply_effect(), apply_effect_hold(), Beam, beams(), feature_levels(), luma(), to_rgb() (+1 more)

### Community 201 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "Tab"
Cohesion: 0.19
Nodes (13): check(), hint(), physical(), problem_text(), section(), sidebar(), Tab, .ALL (+5 more)

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 207 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 208 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 211 - "app.rs"
Cohesion: 0.12
Nodes (15): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), UpdateBadge (+7 more)

### Community 213 - "Scene"
Cohesion: 0.31
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 218 - "AppState"
Cohesion: 0.08
Nodes (19): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm, ProgrammerMode (+11 more)

### Community 227 - "TriggerBand"
Cohesion: 0.25
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 234 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 251 - "AudioPlayer"
Cohesion: 0.16
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 272 - "StageCanvas"
Cohesion: 0.33
Nodes (5): CanvasMode, Edit, Select, StageCanvas, Underlay

### Community 276 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 281 - "BuilderRequest"
Cohesion: 0.40
Nodes (5): BuilderRequest, Edit, New, Open, Template

### Community 285 - "OflError"
Cohesion: 0.40
Nodes (4): OflError, Invalid, Json, Skip

### Community 286 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 292 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

## Knowledge Gaps
- **566 isolated node(s):** `Setup`, `Control`, `Touch`, `Visualizer`, `Builder` (+561 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1377 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **132 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `.update`, `app/src/obs.rs`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `mcp/src/venue.rs`, `.feedback`, `t`, `text_dim`, `rig.rs`, `BankScope`, `app/src/audio.rs`, `profile_builder.rs`, `.fade_state`, `views/touch.rs`, `mcp/src/profiles.rs`, `FixtureAction`, `scene_grid.rs`, `DmxCraftApp`, `settings_window.rs`, `show.rs`, `Snapshot`, `remote/mod.rs`, `tools.rs`, `scene_props.rs`, `hashmap`, `call`, `.end_frame`, `window.rs`, `stage.rs`, `mcp.rs`, `engine.rs`, `.ui`, `app.rs`, `fire`, `AppState`, `audio_triggers.rs`, `DmxFrame`, `Timeline`, `audio_trigger_ui.rs`, `View`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Are the 185 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 185 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Touch` to the rest of the system?**
  _566 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `ProfileBuilder`, `library_panel.rs`, `app/src/obs.rs`, `fx_color.rs`, `programmer.rs`, `i18n/mod.rs`, `ViewCtx`, `DialogKind`, `viz/library.rs`, `missing_audio.rs`, `titlebar.rs`, `text_dim`, `text_field`, `Settings`, `profile_builder.rs`, `app/src/profiles.rs`, `fx_ui.rs`, `profile_builder.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `menu.rs`, `DmxCraftApp`, `settings_window.rs`, `fader.rs`, `scene_props.rs`, `window.rs`, `stage.rs`, `Purpose`, `Tab`, `.ui`, `egui`, `Quality`, `app.rs`, `fire`, `AppState`, `viz/mod.rs`, `textures.rs`, `views/sound.rs`, `Rect`, `panels.rs`, `audio_trigger_ui.rs`, `Venue`, `View`, `.row`, `Timeline`?**
  _High betweenness centrality (0.097) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0946969696969697 - nodes in this community are weakly interconnected._