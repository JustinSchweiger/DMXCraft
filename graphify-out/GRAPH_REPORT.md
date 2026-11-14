# Graph Report - dmxcraft-src  (2026-11-14)

## Corpus Check
- 166 files · ~278,435 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4744 nodes · 12165 edges · 325 communities (170 shown, 155 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 670 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b0d2b42f`
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
- autosave.rs
- props.rs
- ChannelRef
- mapping/mod.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- ProgrammerMode
- Serialize
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
- call
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
- PaletteStore
- range_slider.rs
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
- Category
- Scene
- dialog.rs
- .update
- drag_number
- arc
- Placement
- qxf.rs
- OrbitCamera
- duration
- fixture_settings.rs
- Object
- TouchKind
- outline.rs
- app/src/obs.rs
- convert.rs
- Layer
- stage_editor.rs
- BankScope
- ChannelDef
- mcp/src/profiles.rs
- IconButton
- CurveShape
- ColorPreset
- gltf_import.rs
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
- OrbitCamera
- FadeCurve
- show.rs
- Drag
- text_dim
- ProfileBuilder
- EffectKind
- UpdateState
- text_field
- Obs
- Profile
- MenuItem<'a>
- text_field
- Transform
- hashmap
- Problem
- server.rs
- mcp.rs
- ShowError
- icon_library.rs
- Snapshot
- PlanShape
- midi.rs
- tools.rs
- OpenDmx
- ClipAction
- control/tracking.rs
- bridge.rs
- Fader
- scene_props.rs
- lock.rs
- SceneId
- plan
- plan.rs
- ButtonStyle
- scene_group.rs
- DmxFrame
- ChaserPreset
- core/src/lib.rs
- env
- de.rs
- Knob<'a>
- label_tokens
- PatchGrid
- Step
- build-local-artifacts
- Timeline
- FaderEvent
- AppState
- DeckState
- dip.rs
- Grip
- .fade_state
- Mapper
- FooterButton<'a>
- views/sound.rs
- announce
- Box<dyn serialport::SerialPort>
- .check_placement
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- Tab
- move_hold_does_not_stop_a_playing_scene
- ok
- with
- T
- ObsAction
- views/touch.rs
- mapping/mod.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 263 edges
2. `t()` - 244 edges
3. `Show` - 182 edges
4. `AppState` - 124 edges
5. `Icon` - 119 edges
6. `ViewCtx` - 100 edges
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

## Communities (325 total, 155 thin omitted)

### Community 0 - "SoundPlayer"
Cohesion: 0.12
Nodes (3): DeckRt, SoundPlayer, SoundBoard

### Community 1 - "library_panel.rs"
Cohesion: 0.13
Nodes (18): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+10 more)

### Community 2 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Venue"
Cohesion: 0.07
Nodes (19): aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), IDENTITY (+11 more)

### Community 5 - "programmer.rs"
Cohesion: 0.28
Nodes (13): elide(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock (+5 more)

### Community 6 - "SoundBoard"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+20 more)

### Community 9 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 10 - "props.rs"
Cohesion: 0.06
Nodes (36): fold_section(), section_header(), ViewCtx, VizView, apply_placement(), DialogKind, ImportEv2, ImportModel (+28 more)

### Community 11 - "ChannelRef"
Cohesion: 0.08
Nodes (6): Engine, lerp(), PlayingScene, Track, TrackLight, ChannelRef

### Community 12 - "mapping/mod.rs"
Cohesion: 0.23
Nodes (14): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), index_targets(), is_on(), mapping(), resolve() (+6 more)

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

### Community 20 - "ProgrammerMode"
Cohesion: 0.21
Nodes (9): ProgrammerMode, Blind, Edit, Live, hide_button(), items_mode(), mode_switch(), tool_separator() (+1 more)

### Community 21 - "Serialize"
Cohesion: 0.10
Nodes (10): load_file(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, Model, ModelPart (+2 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (44): analyze(), Analyzer, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad, BPM_MAX (+36 more)

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
Cohesion: 0.06
Nodes (35): Overlay, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade (+27 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.17
Nodes (23): color_fx_ui(), edit_colors(), label_row(), slider_row(), toggle_row(), BodyCtx, combo(), curve_graph() (+15 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.11
Nodes (20): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+12 more)

### Community 32 - "TouchView"
Cohesion: 0.12
Nodes (17): add_page(), add_tile(), Drag, Move, Resize, icon_picker(), ImageTarget, Item (+9 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.13
Nodes (16): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+8 more)

### Community 36 - "t"
Cohesion: 0.07
Nodes (52): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+44 more)

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
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.07
Nodes (51): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+43 more)

### Community 43 - "Settings"
Cohesion: 0.09
Nodes (11): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension(), cable_belongs_to_one_universe() (+3 more)

### Community 44 - "AudioPlayer"
Cohesion: 0.14
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

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
Cohesion: 0.14
Nodes (29): hidden_tabs_from_old_setting(), Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general() (+21 more)

### Community 50 - "call"
Cohesion: 0.26
Nodes (23): add_clips(), add_effect(), add_fixtures(), apply_clip_fields(), build_steps(), call(), changed_scene(), create_scene() (+15 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.06
Nodes (42): addresses(), APP_JS, apply(), CLIENT_TIMEOUT, Cmd, Color, Press, TrackToggle (+34 more)

### Community 52 - "TouchLayout"
Cohesion: 0.16
Nodes (8): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), LONG_PRESS_MS, pages_can_be_reordered(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Timeline"
Cohesion: 0.14
Nodes (17): Scene, audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), GridMode, Beat, Time, group_moves_together_without_covering_others() (+9 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.06
Nodes (39): Track, add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE (+31 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (25): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, MapFile (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.13
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "runtime.rs"
Cohesion: 0.13
Nodes (12): engine_keeps_running_without_device(), Failing, output_keeps_running_after_a_panic_elsewhere(), OutputStatus, Connecting, Error, Running, Stopped (+4 more)

### Community 72 - "Show"
Cohesion: 0.20
Nodes (4): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Show

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (16): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+8 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.14
Nodes (53): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), defaults_are_rendered(), end_scene_follows_jump_to() (+45 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.29
Nodes (6): chip(), divider(), level(), point_chip(), point_width(), VizView

### Community 79 - "theme.rs"
Cohesion: 0.06
Nodes (55): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+47 more)

### Community 80 - "PaletteStore"
Cohesion: 0.36
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 86 - "range_slider.rs"
Cohesion: 0.24
Nodes (6): from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 89 - "profile.rs"
Cohesion: 0.05
Nodes (31): MemoryProfiles, add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity() (+23 more)

### Community 90 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (12): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+4 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (68): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+60 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (39): decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag, Handles, handles() (+31 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "global.rs"
Cohesion: 0.25
Nodes (15): apply(), button(), cells_toggle_but_never_all_off(), fader(), hidden_tracking_follow_works(), Input, Press, Value (+7 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.11
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "Category"
Cohesion: 0.21
Nodes (11): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+3 more)

### Community 102 - "Scene"
Cohesion: 0.13
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings, steps_insert_remove_and_normalize(), super_scene_has_no_values_of_its_own()

### Community 104 - "dialog.rs"
Cohesion: 0.07
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 105 - ".update"
Cohesion: 0.33
Nodes (8): fire(), media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws

### Community 106 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.15
Nodes (13): Calibrate, angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm() (+5 more)

### Community 109 - "qxf.rs"
Cohesion: 0.19
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 112 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "fixture_settings.rs"
Cohesion: 0.36
Nodes (4): body(), caption(), hanging_of(), range_row()

### Community 117 - "Object"
Cohesion: 0.08
Nodes (31): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+23 more)

### Community 118 - "TouchKind"
Cohesion: 0.12
Nodes (13): apply_pick(), item_title(), kind_name(), pick_adds_scene_button_and_assigns_to_selection(), programmer_fader_becomes_fixed_channels(), TouchKind, Button, Color (+5 more)

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "app/src/obs.rs"
Cohesion: 0.18
Nodes (12): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+4 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "Layer"
Cohesion: 0.31
Nodes (8): color_bar(), current(), fixtures_of(), set_group(), xy_pad(), Layer, Live, Programmer

### Community 127 - "stage_editor.rs"
Cohesion: 0.15
Nodes (12): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+4 more)

### Community 128 - "BankScope"
Cohesion: 0.16
Nodes (8): BankScope, All, Bank, Banks, Off, OutsideBank, move_zone_reorders_banks(), Zone

### Community 129 - "ChannelDef"
Cohesion: 0.08
Nodes (32): ChannelDef, ChannelRange, FixtureType, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color (+24 more)

### Community 130 - "mcp/src/profiles.rs"
Cohesion: 0.27
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

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
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 136 - "measure.rs"
Cohesion: 0.22
Nodes (5): End, Measure, snap_points(), SNAP_PX, AXIS_COLORS

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.06
Nodes (32): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, Knob (+24 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.14
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 142 - "Deserialize"
Cohesion: 0.29
Nodes (7): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media

### Community 143 - "Clip"
Cohesion: 0.09
Nodes (13): Clip, ClipContent, Audio, Group, Obs, Scene, Video, Layer (+5 more)

### Community 144 - "DeviceModel"
Cohesion: 0.12
Nodes (12): default_feedback(), APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic (+4 more)

### Community 145 - ".ui"
Cohesion: 0.15
Nodes (11): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead() (+3 more)

### Community 146 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 147 - "OrbitCamera"
Cohesion: 0.10
Nodes (15): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, OrbitCamera (+7 more)

### Community 148 - "FadeCurve"
Cohesion: 0.20
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 149 - "show.rs"
Cohesion: 0.28
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.12
Nodes (29): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+21 more)

### Community 152 - "ProfileBuilder"
Cohesion: 0.15
Nodes (9): FileJob, Export, Import, import(), new_profile(), pick_export(), pick_import(), ProfileBuilder (+1 more)

### Community 153 - "EffectKind"
Cohesion: 0.14
Nodes (13): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+5 more)

### Community 154 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 156 - "Obs"
Cohesion: 0.11
Nodes (12): Link, Obs, run(), session(), Shared, Status, Connected, Connecting (+4 more)

### Community 158 - "Profile"
Cohesion: 0.19
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 160 - "text_field"
Cohesion: 0.15
Nodes (6): text_field(), View, FixtureSettings, SetupView, ZonesView, Windows

### Community 163 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 164 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

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
Nodes (33): any_obj(), apply_scene_fields(), base64(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values() (+25 more)

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

### Community 182 - "Fader"
Cohesion: 0.21
Nodes (3): Fader, FaderRange, fader_ranges()

### Community 183 - "scene_props.rs"
Cohesion: 0.05
Nodes (64): Timeline, COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over() (+56 more)

### Community 184 - "lock.rs"
Cohesion: 0.33
Nodes (7): digest(), hash_and_verify(), hash_password(), hex(), MASTER, unlocks(), verify()

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 189 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 190 - "ButtonStyle"
Cohesion: 0.25
Nodes (8): style_name(), ButtonStyle, .ALL, Led, Normal, Outline, Round, Switch

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "DmxFrame"
Cohesion: 0.16
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 193 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 195 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (8): FixtureLimits, Group, PLAN_SIZE_RANGE, SceneKind, Fx, Static, Super, UNIVERSES

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 201 - "label_tokens"
Cohesion: 0.38
Nodes (6): icons_only(), label_with_glyphs(), label_tokens(), LabelPart, Icon, Text

### Community 202 - "PatchGrid"
Cohesion: 0.40
Nodes (5): Drag, GridMode, Output, Patch, PatchGrid

### Community 203 - "Step"
Cohesion: 0.23
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "Timeline"
Cohesion: 0.15
Nodes (7): add_audio(), add_audio_dialog(), has_audio(), media_track(), ungroup_clip(), linked_blocks_stay_together(), Timeline

### Community 210 - "FaderEvent"
Cohesion: 0.40
Nodes (5): FaderEvent, Changed, None, Release, ToggleFx

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Mode, Builder, Control (+7 more)

### Community 219 - "DeckState"
Cohesion: 0.29
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 220 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 221 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 225 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 226 - "Mapper"
Cohesion: 0.30
Nodes (3): is_value_control(), Mapper, Sent

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

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
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 244 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 256 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 309 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

### Community 317 - "views/touch.rs"
Cohesion: 0.17
Nodes (24): COLORS, content_rect(), display_label(), fader_parts(), GAP, GRID_STEPS, H_VALUE_W, item_target() (+16 more)

### Community 321 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (34): alt_click(), button(), button_rect(), channel_name(), describe(), describe_mapping(), fader(), fader_channels() (+26 more)

## Knowledge Gaps
- **585 isolated node(s):** `Off`, `Running`, `Error`, `Press`, `Value` (+580 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1516 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **155 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `ChannelRef` to `library_panel.rs`, `mcp/src/profiles.rs`, `programmer.rs`, `props.rs`, `mcp/src/venue.rs`, `mapping/mod.rs`, `egui`, `.ui`, `ProgrammerMode`, `text_dim`, `ProfileBuilder`, `rig.rs`, `Obs`, `Host`, `text_field`, `app/src/audio.rs`, `t`, `menu.rs`, `hashmap`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `call`, `remote/mod.rs`, `control/tracking.rs`, `scene_props.rs`, `SceneId`, `views/touch.rs`, `DmxFrame`, `mapping/mod.rs`, `control/timeline.rs`, `core/src/lib.rs`, `window.rs`, `stage.rs`, `runtime.rs`, `Show`, `engine.rs`, `ChannelKind`, `global.rs`, `Mapper`, `.fade_state`, `audio_triggers.rs`, `Category`, `.update`, `Placement`, `TouchKind`, `move_hold_does_not_stop_a_playing_scene`, `app/src/obs.rs`, `Layer`?**
  _High betweenness centrality (0.117) - this node is a cross-community bridge._
- **Are the 206 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 206 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Off`, `Running`, `Error` to the rest of the system?**
  _585 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.11522048364153627 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `BankScope`, `ChannelDef`, `SoundPlayer`, `Venue`, `measure.rs`, `autosave.rs`, `fx.rs`, `ChannelRef`, `mapping/mod.rs`, `stage_canvas.rs`, `show.rs`, `rig.rs`, `Obs`, `missing_audio.rs`, `TouchView`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `ShowError`, `DmxCraftApp`, `tools.rs`, `call`, `remote/mod.rs`, `TouchLayout`, `control/tracking.rs`, `scene_props.rs`, `SceneId`, `views/touch.rs`, `scene_group.rs`, `mapping/mod.rs`, `Timeline`, `core/src/lib.rs`, `window.rs`, `stage.rs`, `feature.rs`, `Step`, `motion.rs`, `engine.rs`, `theme.rs`, `viz/mod.rs`, `Scene`, `.check_placement`, `Placement`, `Object`, `move_hold_does_not_stop_a_playing_scene`?**
  _High betweenness centrality (0.116) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1349206349206349 - nodes in this community are weakly interconnected._