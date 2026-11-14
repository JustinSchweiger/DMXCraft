# Graph Report - dmxcraft-src  (2026-11-14)

## Corpus Check
- 178 files · ~281,017 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4847 nodes · 12100 edges · 339 communities (172 shown, 167 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 730 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b9d2b2a0`
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
- Obs
- ViewCtx
- ChannelRef
- mapping/mod.rs
- viz/library.rs
- jobs
- with
- README.md
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
- Host
- ofl/src/lib.rs
- missing_audio.rs
- views/touch.rs
- dmxcraft
- FixtureType
- t
- call
- DMXCraft – UI guidelines
- system_fonts.rs
- FixtureAction
- scene_grid.rs
- file_dialog.rs
- AudioPlayer
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- Ui
- fx/mapping.rs
- remote/mod.rs
- TouchLayout
- Timeline
- control/timeline.rs
- steps
- window.rs
- stage.rs
- app/src/audio.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- src/timeline.rs
- theme.rs
- PaletteStore
- range_slider.rs
- ShutterEffect
- profile.rs
- runtime.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- fire
- MapPreset
- audio_triggers.rs
- qxf.rs
- show.rs
- dialog.rs
- paint_item
- Response
- arc
- Placement
- FixtureLimits
- EffectKind
- duration
- snap.rs
- Object
- TouchKind
- outline.rs
- ProfileBuilder
- convert.rs
- gltf_import.rs
- stage_editor.rs
- View
- app/src/obs.rs
- global.rs
- IconButton
- CurveShape
- ColorPreset
- qlc-bundle.rs
- measure.rs
- i18n/mod.rs
- mcp/src/venue.rs
- egui
- stage_canvas.rs
- Deserialize
- Clip
- DeviceModel
- .ui
- Theme
- assist.rs
- time_field.rs
- Settings
- OrbitCamera
- text_dim
- media.rs
- instant
- UpdateState
- text_field
- T
- Profile
- FxHasher
- ZoneId
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
- ChaserPreset
- paint_clip
- src/tracking.rs
- run
- Show
- scene_group.rs
- ChannelRef
- super
- env
- de.rs
- fx_color.rs
- ProfileChannel
- Dialog<'a>
- Step
- build-local-artifacts
- send_frame
- lock.rs
- AppState
- DeckState
- live.rs
- .update
- Drag
- FooterButton<'a>
- views/sound.rs
- fixture_settings.rs
- GH_TOKEN
- HeaderAction
- plan.rs
- release.ps1
- desktop.rs
- Camera
- pointer
- video_out.rs
- release-install.md
- super
- Category
- Knob<'a>
- LookPart
- with
- TriggerBand
- collect_values
- Tab
- WaveState
- make
- builtin_fixture_types
- ObsAction

## God Nodes (most connected - your core abstractions)
1. `Engine` - 251 edges
2. `t()` - 245 edges
3. `Show` - 182 edges
4. `AppState` - 124 edges
5. `Icon` - 119 edges
6. `ViewCtx` - 100 edges
7. `ChannelKind` - 78 edges
8. `ChannelRef` - 63 edges
9. `FixtureType` - 57 edges
10. `DmxCraftApp` - 54 edges

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
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (339 total, 167 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "DmxOutput"
Cohesion: 0.07
Nodes (10): DmxFrame, Engine, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.18
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "Venue"
Cohesion: 0.06
Nodes (29): FaceDrag, renumber(), apply_texture(), duplicate_object(), floor_height(), RESERVE, set_locked(), transfer() (+21 more)

### Community 5 - "programmer.rs"
Cohesion: 0.09
Nodes (39): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+31 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.19
Nodes (24): all_presets_render_every_beam(), apply(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels() (+16 more)

### Community 9 - "Obs"
Cohesion: 0.19
Nodes (8): browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), engine(), fake_obs(), Obs, VideoPlay, wait(), wrong_password_is_reported()

### Community 10 - "ViewCtx"
Cohesion: 0.20
Nodes (9): fold_section(), VizView, button(), color_field(), number(), percent(), selected_objects(), vec3_field() (+1 more)

### Community 12 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (34): alt_click(), button_rect(), channel_name(), describe(), describe_mapping(), fader(), fader_channels(), fires_id() (+26 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.14
Nodes (13): env, if, needs, runs-on, jobs, announce, name, on (+5 more)

### Community 15 - "with"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (41): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+33 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.05
Nodes (62): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+54 more)

### Community 20 - "Control"
Cohesion: 0.06
Nodes (38): hidden_tracking_follow_works(), Overlay, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control (+30 more)

### Community 21 - "Serialize"
Cohesion: 0.10
Nodes (10): load_file(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, Model, ModelPart (+2 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.07
Nodes (44): analyze(), Analyzer, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad, BPM_MAX (+36 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (30): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+22 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "tests.rs"
Cohesion: 0.13
Nodes (39): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), end_scene_follows_jump_to(), fades_between_steps_and_loops() (+31 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.13
Nodes (29): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+21 more)

### Community 29 - "Host"
Cohesion: 0.19
Nodes (14): AudioDuration, DEFAULT_PORT, Host, VizFn, call_tool(), error(), handle(), handle_one() (+6 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 32 - "views/touch.rs"
Cohesion: 0.07
Nodes (41): section_header(), add_page(), add_tile(), COLORS, display_label(), Drag, Move, Resize (+33 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "FixtureType"
Cohesion: 0.08
Nodes (30): ChannelDef, ChannelRange, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color, Gobo (+22 more)

### Community 36 - "t"
Cohesion: 0.08
Nodes (46): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+38 more)

### Community 37 - "call"
Cohesion: 0.19
Nodes (29): add_clips(), add_effect(), add_fixtures(), apply_clip_fields(), apply_scene_fields(), build_steps(), call(), changed_scene() (+21 more)

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
Cohesion: 0.08
Nodes (48): select_all(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+40 more)

### Community 43 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 44 - "AudioPlayer"
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.10
Nodes (13): block_input(), ControlViews, DmxCraftApp, edit_preview(), fill_types(), load_step(), LockDialog, moved_out() (+5 more)

### Community 48 - "Ui"
Cohesion: 0.11
Nodes (38): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+30 more)

### Community 50 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (37): addresses(), APP_JS, apply(), CLIENT_TIMEOUT, Cmd, Color, Press, TrackToggle (+29 more)

### Community 52 - "TouchLayout"
Cohesion: 0.16
Nodes (8): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), LONG_PRESS_MS, pages_can_be_reordered(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Timeline"
Cohesion: 0.10
Nodes (16): has_audio(), Scene, audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), GridMode, Beat, Time (+8 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.09
Nodes (20): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, LaneDrag, LaneGrab (+12 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (25): color(), Mapper, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.18
Nodes (17): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+9 more)

### Community 71 - "app/src/audio.rs"
Cohesion: 0.13
Nodes (17): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+9 more)

### Community 72 - "Show"
Cohesion: 0.14
Nodes (10): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap (+2 more)

### Community 73 - "feature.rs"
Cohesion: 0.14
Nodes (20): apply_effect(), apply_effect_hold(), apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams() (+12 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.16
Nodes (13): FadeState, In, Out, MAX_STEP_ADVANCES_PER_TICK, PAN_DEG_PER_S, RELEASE_DARK_MS, RELEASE_LIGHT_MS, SETTLE_MS (+5 more)

### Community 78 - "src/timeline.rs"
Cohesion: 0.13
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (53): Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas(), canvas_dim() (+45 more)

### Community 80 - "PaletteStore"
Cohesion: 0.36
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 86 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (20): beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+12 more)

### Community 90 - "runtime.rs"
Cohesion: 0.15
Nodes (11): engine_keeps_running_without_device(), output_keeps_running_after_a_panic_elsewhere(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (68): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+60 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.07
Nodes (43): ViewCtx, apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape (+35 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.16
Nodes (22): asphalt(), brick(), carpet(), checker(), cobblestone(), concrete(), fbm(), light_wood() (+14 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (32): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+24 more)

### Community 97 - "fire"
Cohesion: 0.13
Nodes (17): bank_step_wraps(), channel_mapping_sets_live_value(), default_feedback(), fire(), fixed_channels_work_without_selection(), index_targets(), is_on(), is_value_control() (+9 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.15
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 101 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 102 - "show.rs"
Cohesion: 0.13
Nodes (19): address_map_marks_ranges(), duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, move_scenes_between_banks(), move_zone_reorders_banks() (+11 more)

### Community 104 - "dialog.rs"
Cohesion: 0.10
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 105 - "paint_item"
Cohesion: 0.17
Nodes (17): content_rect(), fader_parts(), H_VALUE_W, icons_only(), label_galley(), label_with_glyphs(), led_color(), paint_cover() (+9 more)

### Community 106 - "Response"
Cohesion: 0.15
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, chip(), divider() (+6 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 110 - "EffectKind"
Cohesion: 0.12
Nodes (12): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+4 more)

### Community 112 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 117 - "Object"
Cohesion: 0.15
Nodes (14): axes(), Face, faces(), hit(), MIN_LEN, paint(), SIZE, Shape (+6 more)

### Community 118 - "TouchKind"
Cohesion: 0.18
Nodes (8): TouchKind, Button, Color, Fader, Knob, Label, Tracking, Xy

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "ProfileBuilder"
Cohesion: 0.17
Nodes (8): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder, drag_ghost()

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (28): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+20 more)

### Community 123 - "gltf_import.rs"
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 127 - "stage_editor.rs"
Cohesion: 0.14
Nodes (13): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+5 more)

### Community 128 - "View"
Cohesion: 0.16
Nodes (7): text_field(), View, FixtureSettings, SetupView, StageEditorView, ZonesView, Windows

### Community 129 - "app/src/obs.rs"
Cohesion: 0.15
Nodes (7): auth_matches_obs_websocket_example(), auth_response(), b64(), DRIFT_MS, Link, PRELOAD_MS, SETTLE

### Community 130 - "global.rs"
Cohesion: 0.26
Nodes (14): apply(), button(), cells_toggle_but_never_all_off(), fader(), Input, Press, Value, MAX_HEIGHT (+6 more)

### Community 131 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (17): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+9 more)

### Community 134 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 136 - "measure.rs"
Cohesion: 0.08
Nodes (14): Calibrate, VizView, End, Measure, snap_points(), SNAP_PX, project(), OutlineTab (+6 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (15): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.05
Nodes (33): DipSwitch, SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY (+25 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (20): AMBER, BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_effects() (+12 more)

### Community 142 - "Deserialize"
Cohesion: 0.29
Nodes (7): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media

### Community 143 - "Clip"
Cohesion: 0.09
Nodes (12): audio_segment(), Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth (+4 more)

### Community 144 - "DeviceModel"
Cohesion: 0.15
Nodes (9): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), nearest() (+1 more)

### Community 145 - ".ui"
Cohesion: 0.22
Nodes (4): fmt_ms(), grid_options(), mark_clipboard(), TimelineView

### Community 146 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 147 - "assist.rs"
Cohesion: 0.12
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 149 - "Settings"
Cohesion: 0.11
Nodes (7): MidiDevice, cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 151 - "text_dim"
Cohesion: 0.16
Nodes (25): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+17 more)

### Community 152 - "media.rs"
Cohesion: 0.14
Nodes (16): Timeline, add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), drop_hint(), has_sound_cached() (+8 more)

### Community 154 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 155 - "text_field"
Cohesion: 0.16
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "Profile"
Cohesion: 0.08
Nodes (24): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+16 more)

### Community 159 - "FxHasher"
Cohesion: 0.20
Nodes (3): FastBuild, FxHasher, SEED

### Community 160 - "ZoneId"
Cohesion: 0.12
Nodes (11): bank_multi_selection(), PageFilter, All, Zones, BankScope, All, Bank, Banks (+3 more)

### Community 163 - "crash.rs"
Cohesion: 0.14
Nodes (13): arm(), Armed, ARMED, civil(), emergency_save(), install(), log_file(), marker_file() (+5 more)

### Community 164 - "ev2.rs"
Cohesion: 0.18
Nodes (15): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+7 more)

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "server.rs"
Cohesion: 0.23
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 167 - "mcp.rs"
Cohesion: 0.14
Nodes (14): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+6 more)

### Community 168 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 169 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 170 - "Snapshot"
Cohesion: 0.29
Nodes (6): following_is_no_undo_step(), History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 173 - "midi.rs"
Cohesion: 0.13
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (30): Prepare, any_obj(), base64(), boolean(), camera_schema(), clip_schema(), failed_super_scene_leaves_nothing(), find_type() (+22 more)

### Community 177 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.27
Nodes (8): bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT, TrackingView, Tracker

### Community 181 - "bridge.rs"
Cohesion: 0.33
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 182 - "fader.rs"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.05
Nodes (53): Effect, advanced(), banks_of(), pick_tile(), PickColumn, Picker, picker_window(), PickTarget (+45 more)

### Community 184 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 185 - "paint_clip"
Cohesion: 0.23
Nodes (9): Track, paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), track_header(), row_h() (+1 more)

### Community 188 - "src/tracking.rs"
Cohesion: 0.15
Nodes (5): fading_flash_returns_to_the_color_below(), all_cells(), DEFAULT_HEIGHT, Show, white()

### Community 189 - "run"
Cohesion: 0.20
Nodes (9): run(), session(), Shared, Status, Connected, Connecting, Failed, Off (+1 more)

### Community 190 - "Show"
Cohesion: 0.24
Nodes (4): room_json(), trackers_json(), Show, RoomMap

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "ChannelRef"
Cohesion: 0.12
Nodes (5): lerp(), PlayingScene, FastMap, FastSet, ChannelRef

### Community 195 - "super"
Cohesion: 0.20
Nodes (9): Control, short_name(), target_menu(), known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES (+1 more)

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 200 - "fx_color.rs"
Cohesion: 0.08
Nodes (21): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+13 more)

### Community 201 - "ProfileChannel"
Cohesion: 0.27
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 203 - "Step"
Cohesion: 0.12
Nodes (14): cv(), demo_show(), empty_show(), ensure_builtin_types(), addresses_are_one_based(), UNIVERSE_SIZE, ChannelValue, PLAN_SIZE_RANGE (+6 more)

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 212 - "lock.rs"
Cohesion: 0.33
Nodes (7): digest(), hash_and_verify(), hash_password(), hex(), MASTER, unlocks(), verify()

### Community 218 - "AppState"
Cohesion: 0.08
Nodes (12): AppState, Focus, Bank, Scenes, Mode, Builder, Setup, Sound (+4 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 221 - "live.rs"
Cohesion: 0.35
Nodes (6): channels_of(), current_color(), current_position(), item_value(), set_color(), set_position()

### Community 225 - ".update"
Cohesion: 0.33
Nodes (8): fire(), media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws

### Community 226 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (20): button(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H (+12 more)

### Community 231 - "fixture_settings.rs"
Cohesion: 0.36
Nodes (4): body(), caption(), hanging_of(), range_row()

### Community 232 - "GH_TOKEN"
Cohesion: 0.29
Nodes (8): env, needs, runs-on, env, BUILD_MANIFEST_NAME, GH_TOKEN, build-global-artifacts, env

### Community 233 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 234 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

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
Cohesion: 0.33
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 244 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 251 - "LookPart"
Cohesion: 0.25
Nodes (8): LookAction, Copy, Paste, LookPart, All, Color, Intensity, Position

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 258 - "TriggerBand"
Cohesion: 0.25
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 272 - "collect_values"
Cohesion: 0.36
Nodes (7): channel_kind(), collect_values(), existing_fixtures(), fixture_json(), parse_color_opt(), parse_hex(), resolve_channel()

### Community 275 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 276 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 285 - "make"
Cohesion: 0.40
Nodes (5): builtin(), builtins_are_seamless_and_decode(), make(), Pattern, render()

### Community 286 - "builtin_fixture_types"
Cohesion: 0.50
Nodes (4): defaults_are_rendered(), limits_cap_dimmer_and_position(), tracking_aims_heads_at_the_point_and_releases_softly(), builtin_fixture_types()

### Community 289 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

## Knowledge Gaps
- **583 isolated node(s):** `Properties`, `Effects`, `Third`, `Jump`, `Release` (+578 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1570 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **167 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `ChannelRef` to `app/src/obs.rs`, `global.rs`, `library_panel.rs`, `DmxOutput`, `programmer.rs`, `measure.rs`, `Obs`, `mcp/src/venue.rs`, `mapping/mod.rs`, `egui`, `.ui`, `text_dim`, `rig.rs`, `Host`, `Profile`, `crash.rs`, `t`, `call`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `control/tracking.rs`, `scene_props.rs`, `Show`, `ChannelRef`, `control/timeline.rs`, `window.rs`, `stage.rs`, `app/src/audio.rs`, `fx_color.rs`, `Show`, `Step`, `engine.rs`, `runtime.rs`, `viz/mod.rs`, `fire`, `.update`, `audio_triggers.rs`, `ProfileBuilder`?**
  _High betweenness centrality (0.112) - this node is a cross-community bridge._
- **Are the 206 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 206 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Properties`, `Effects`, `Third` to the rest of the system?**
  _583 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.14022988505747128 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `View`, `library_panel.rs`, `Venue`, `programmer.rs`, `measure.rs`, `i18n/mod.rs`, `ViewCtx`, `mapping/mod.rs`, `egui`, `viz/library.rs`, `.ui`, `titlebar.rs`, `assist.rs`, `Settings`, `Serialize`, `text_dim`, `media.rs`, `text_field`, `fx_ui.rs`, `make`, `Profile`, `missing_audio.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `file_dialog.rs`, `DmxCraftApp`, `Ui`, `control/tracking.rs`, `fader.rs`, `scene_props.rs`, `paint_clip`, `Show`, `control/timeline.rs`, `super`, `window.rs`, `stage.rs`, `fx_color.rs`, `PaletteStore`, `viz/mod.rs`, `textures.rs`, `views/sound.rs`, `fixture_settings.rs`, `dialog.rs`, `Response`, `snap.rs`, `outline.rs`, `ProfileBuilder`, `stage_editor.rs`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Are the 8 inferred relationships involving `Show` (e.g. with `.apply_tracking()` and `Mappings`) actually correct?**
  _`Show` has 8 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12310606060606061 - nodes in this community are weakly interconnected._