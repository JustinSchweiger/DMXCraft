# Graph Report - dmxcraft-src  (2026-11-14)

## Corpus Check
- 168 files · ~280,182 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4791 nodes · 12250 edges · 337 communities (171 shown, 166 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 665 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2006763a`
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
- autosave.rs
- props.rs
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
- MidiMessage
- fx_ui.rs
- Host
- ofl/src/lib.rs
- missing_audio.rs
- TouchView
- dmxcraft
- ChannelDef
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
- crash.rs
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
- Timeline
- theme.rs
- PaletteStore
- range_slider.rs
- ShutterEffect
- profile.rs
- Capability
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- .end_frame
- MapPreset
- audio_triggers.rs
- qxf.rs
- show.rs
- dialog.rs
- mcp/src/profiles.rs
- Profile
- arc
- Placement
- show.rs
- EffectKind
- duration
- snap.rs
- Object
- TouchKind
- outline.rs
- View
- convert.rs
- SlotKind
- stage_editor.rs
- fixture_settings.rs
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
- Quality
- FixtureType
- pathbuf
- toolbar.rs
- text_dim
- ProfileBuilder
- Category
- UpdateState
- text_field
- T
- Profile
- FxHasher
- patch_grid.rs
- text_field
- ev2.rs
- Problem
- server.rs
- mcp.rs
- ShowError
- scene_card.rs
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
- Lang
- Tab
- LookPart
- Transform
- ButtonStyle
- scene_group.rs
- .ui
- ProgrammerMode
- core/src/lib.rs
- env
- de.rs
- menu.rs
- Knob<'a>
- viz/calibrate.rs
- Step
- build-local-artifacts
- DmxLine
- lock.rs
- AppState
- DeckState
- plan.rs
- send_frame
- Drag
- FooterButton<'a>
- views/sound.rs
- label_tokens
- GH_TOKEN
- HeaderAction
- FixtureLimits
- release.ps1
- desktop.rs
- Camera
- pointer
- video_out.rs
- release-install.md
- super
- app.rs
- .check_placement
- ok
- with
- Grip
- .fade_state
- Physics
- views/touch.rs
- ObsAction
- Drag

## God Nodes (most connected - your core abstractions)
1. `Engine` - 267 edges
2. `t()` - 244 edges
3. `Show` - 183 edges
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
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (337 total, 166 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.13
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "DmxFrame"
Cohesion: 0.05
Nodes (21): addresses_are_one_based(), DmxFrame, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound (+13 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.18
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "Venue"
Cohesion: 0.07
Nodes (19): aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), IDENTITY (+11 more)

### Community 5 - "programmer.rs"
Cohesion: 0.19
Nodes (21): color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block() (+13 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.19
Nodes (25): all_presets_render_every_beam(), apply(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels() (+17 more)

### Community 9 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 10 - "props.rs"
Cohesion: 0.07
Nodes (29): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, renumber(), apply_texture() (+21 more)

### Community 11 - "ChannelRef"
Cohesion: 0.05
Nodes (11): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), Engine, lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, SceneProgress, Track (+3 more)

### Community 12 - "mapping/mod.rs"
Cohesion: 0.10
Nodes (46): alt_click(), bank_step_wraps(), button_rect(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fader() (+38 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.10
Nodes (19): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+11 more)

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
Nodes (35): Overlay, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade (+27 more)

### Community 21 - "Serialize"
Cohesion: 0.12
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

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
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "MidiMessage"
Cohesion: 0.28
Nodes (6): Input, Key, Midi, MidiMessage, Cc, Note

### Community 28 - "fx_ui.rs"
Cohesion: 0.12
Nodes (29): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+21 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (16): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+8 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 32 - "TouchView"
Cohesion: 0.14
Nodes (18): section_header(), ViewCtx, add_page(), add_tile(), display_label(), icon_picker(), ImageTarget, Item (+10 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "ChannelDef"
Cohesion: 0.11
Nodes (15): ChannelDef, ChannelRange, open_value_takes_the_highest_open_range(), RangeEffect, Closed, Color, Gobo, Open (+7 more)

### Community 36 - "t"
Cohesion: 0.08
Nodes (44): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+36 more)

### Community 37 - "call"
Cohesion: 0.21
Nodes (27): add_clips(), add_effect(), add_fixtures(), apply_clip_fields(), apply_scene_fields(), call(), changed_scene(), create_scene() (+19 more)

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
Nodes (52): select_all(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+44 more)

### Community 43 - "file_dialog.rs"
Cohesion: 0.24
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 44 - "AudioPlayer"
Cohesion: 0.13
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "Ui"
Cohesion: 0.13
Nodes (28): MidiDevice, Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general() (+20 more)

### Community 50 - "crash.rs"
Cohesion: 0.16
Nodes (13): arm(), Armed, ARMED, civil(), emergency_save(), install(), log_file(), marker_file() (+5 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.06
Nodes (35): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, TrackToggle, Value (+27 more)

### Community 52 - "TouchLayout"
Cohesion: 0.16
Nodes (8): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), LONG_PRESS_MS, pages_can_be_reordered(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Timeline"
Cohesion: 0.13
Nodes (16): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Group, Obs, Scene, Video, fade_curve_fast_then_slow() (+8 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.07
Nodes (33): Track, add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE (+25 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (26): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, MapFile (+18 more)

### Community 70 - "stage.rs"
Cohesion: 0.18
Nodes (17): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+9 more)

### Community 71 - "app/src/audio.rs"
Cohesion: 0.14
Nodes (16): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+8 more)

### Community 72 - "Show"
Cohesion: 0.19
Nodes (4): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Show

### Community 73 - "feature.rs"
Cohesion: 0.10
Nodes (28): apply_effect(), apply_effect_hold(), apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL (+20 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.14
Nodes (51): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below() (+43 more)

### Community 78 - "Timeline"
Cohesion: 0.10
Nodes (13): add_audio(), add_audio_dialog(), has_audio(), media_track(), GridMode, Beat, Time, is_light() (+5 more)

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
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (20): beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+12 more)

### Community 90 - "Capability"
Cohesion: 0.19
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (68): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+60 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.07
Nodes (41): apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag (+33 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.08
Nodes (36): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+28 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - ".end_frame"
Cohesion: 0.22
Nodes (5): default_feedback(), fires_id(), is_value_control(), Mapper, Sent

### Community 99 - "MapPreset"
Cohesion: 0.07
Nodes (33): fbm(), default_palette(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, MapPreset, .ALL (+25 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.09
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "qxf.rs"
Cohesion: 0.19
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 102 - "show.rs"
Cohesion: 0.10
Nodes (8): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize(), super_scene_has_no_values_of_its_own()

### Community 104 - "dialog.rs"
Cohesion: 0.06
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 105 - "mcp/src/profiles.rs"
Cohesion: 0.24
Nodes (12): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+4 more)

### Community 106 - "Profile"
Cohesion: 0.17
Nodes (5): import(), MemoryProfiles, fine_names(), Profile, rename_and_resolution_update_modes()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "show.rs"
Cohesion: 0.28
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 110 - "EffectKind"
Cohesion: 0.15
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 112 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 117 - "Object"
Cohesion: 0.24
Nodes (8): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE

### Community 118 - "TouchKind"
Cohesion: 0.18
Nodes (8): TouchKind, Button, Color, Fader, Knob, Label, Tracking, Xy

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "View"
Cohesion: 0.23
Nodes (6): ControlViews, ProgrammerView, View, FixtureSettings, SetupView, StageEditorView

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 127 - "stage_editor.rs"
Cohesion: 0.14
Nodes (13): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+5 more)

### Community 128 - "fixture_settings.rs"
Cohesion: 0.20
Nodes (9): Quick, quick_action(), Center, Off, On, body(), caption(), hanging_of() (+1 more)

### Community 129 - "app/src/obs.rs"
Cohesion: 0.08
Nodes (32): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+24 more)

### Community 130 - "global.rs"
Cohesion: 0.25
Nodes (15): apply(), button(), cells_toggle_but_never_all_off(), fader(), hidden_tracking_follow_works(), Input, Press, Value (+7 more)

### Community 131 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 136 - "measure.rs"
Cohesion: 0.26
Nodes (5): End, Measure, snap_points(), SNAP_PX, project()

### Community 137 - "i18n/mod.rs"
Cohesion: 0.19
Nodes (8): all_texts_translated(), DE, fill(), fill_replaces_names(), LANG, tr!, placeholders_match(), texts_in_source()

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.07
Nodes (23): heading_text(), SIZE, SWEEP, LimitPad, RangeSlider, fold_section(), Swatch, COMPACT_W (+15 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (20): AMBER, BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_effects() (+12 more)

### Community 142 - "Deserialize"
Cohesion: 0.29
Nodes (7): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media

### Community 143 - "Clip"
Cohesion: 0.09
Nodes (12): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth, Layer (+4 more)

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 145 - ".ui"
Cohesion: 0.16
Nodes (10): drop_hint(), fmt_ms(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead(), paint_segments() (+2 more)

### Community 146 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 147 - "Quality"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 148 - "FixtureType"
Cohesion: 0.28
Nodes (14): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, position_in_degrees(), rgbw_to_rgb_and_back() (+6 more)

### Community 149 - "pathbuf"
Cohesion: 0.18
Nodes (5): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX

### Community 150 - "toolbar.rs"
Cohesion: 0.08
Nodes (18): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, OrbitCamera (+10 more)

### Community 151 - "text_dim"
Cohesion: 0.13
Nodes (31): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+23 more)

### Community 152 - "ProfileBuilder"
Cohesion: 0.19
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 153 - "Category"
Cohesion: 0.21
Nodes (11): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+3 more)

### Community 154 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "Profile"
Cohesion: 0.19
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 159 - "FxHasher"
Cohesion: 0.18
Nodes (4): FastBuild, FastMap, FxHasher, SEED

### Community 160 - "patch_grid.rs"
Cohesion: 0.23
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 163 - "text_field"
Cohesion: 0.29
Nodes (4): text_field(), drag_ghost(), VizView, button()

### Community 164 - "ev2.rs"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

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

### Community 169 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 170 - "Snapshot"
Cohesion: 0.29
Nodes (6): following_is_no_undo_step(), History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 173 - "midi.rs"
Cohesion: 0.21
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (31): any_obj(), base64(), boolean(), build_steps(), camera_schema(), channel_kind(), clip_schema(), collect_values() (+23 more)

### Community 177 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 179 - "ClipAction"
Cohesion: 0.10
Nodes (18): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+10 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.09
Nodes (15): Show, ungroup_clip(), bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT (+7 more)

### Community 181 - "bridge.rs"
Cohesion: 0.26
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 182 - "fader.rs"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.06
Nodes (59): button(), Timeline, time_field(), advanced(), apply_to_others(), banks_of(), BAR_H, bar_slider() (+51 more)

### Community 184 - "Lang"
Cohesion: 0.25
Nodes (6): Lang, lang(), .ALL, De, En, set_lang()

### Community 185 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 188 - "LookPart"
Cohesion: 0.25
Nodes (8): LookAction, Copy, Paste, LookPart, All, Color, Intensity, Position

### Community 189 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 190 - "ButtonStyle"
Cohesion: 0.25
Nodes (8): style_name(), ButtonStyle, .ALL, Led, Normal, Outline, Round, Switch

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 193 - "ProgrammerMode"
Cohesion: 0.21
Nodes (9): ProgrammerMode, Blind, Edit, Live, hide_button(), items_mode(), mode_switch(), tool_separator() (+1 more)

### Community 195 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (12): UNIVERSE_SIZE, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+4 more)

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 200 - "menu.rs"
Cohesion: 0.11
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, ITEM_H, MenuItem, MenuItem<'a> (+3 more)

### Community 203 - "Step"
Cohesion: 0.27
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 212 - "lock.rs"
Cohesion: 0.33
Nodes (7): digest(), hash_and_verify(), hash_password(), hex(), MASTER, unlocks(), verify()

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (17): AppState, Focus, Bank, Effect, Scenes, Mode, Builder, Control (+9 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 221 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 225 - "send_frame"
Cohesion: 0.39
Nodes (5): frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 226 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 231 - "label_tokens"
Cohesion: 0.38
Nodes (6): icons_only(), label_with_glyphs(), label_tokens(), LabelPart, Icon, Text

### Community 232 - "GH_TOKEN"
Cohesion: 0.29
Nodes (8): env, needs, runs-on, env, BUILD_MANIFEST_NAME, GH_TOKEN, build-global-artifacts, env

### Community 233 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

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
Cohesion: 0.19
Nodes (10): glyph(), LIBRARY, names_are_unique(), paint(), known_addresses(), on_switches(), roundtrip(), switch_value() (+2 more)

### Community 244 - "app.rs"
Cohesion: 0.10
Nodes (21): block_input(), edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, programmer_rail(), RAIL_W (+13 more)

### Community 246 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 251 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 258 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 272 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 291 - "views/touch.rs"
Cohesion: 0.11
Nodes (36): apply(), find_item(), state_json(), tracking_moves_and_switches_a_target(), apply_pick(), channels_of(), COLORS, content_rect() (+28 more)

### Community 304 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

### Community 305 - "Drag"
Cohesion: 0.67
Nodes (3): Drag, Move, Resize

## Knowledge Gaps
- **585 isolated node(s):** `Low`, `Medium`, `High`, `Ultra`, `Live` (+580 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1541 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **166 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `ChannelRef` to `fixture_settings.rs`, `app/src/obs.rs`, `global.rs`, `library_panel.rs`, `DmxFrame`, `programmer.rs`, `mcp/src/venue.rs`, `mapping/mod.rs`, `egui`, `.fade_state`, `.ui`, `text_dim`, `ProfileBuilder`, `Category`, `rig.rs`, `fx_ui.rs`, `Host`, `TouchView`, `views/touch.rs`, `t`, `text_field`, `call`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `crash.rs`, `remote/mod.rs`, `control/tracking.rs`, `scene_props.rs`, `ProgrammerMode`, `control/timeline.rs`, `core/src/lib.rs`, `window.rs`, `stage.rs`, `app/src/audio.rs`, `Show`, `viz/calibrate.rs`, `engine.rs`, `AppState`, `ChannelKind`, `.end_frame`, `audio_triggers.rs`, `mcp/src/profiles.rs`, `app.rs`, `View`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **Are the 206 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 206 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Low`, `Medium`, `High` to the rest of the system?**
  _585 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `fixture_settings.rs`, `library_panel.rs`, `programmer.rs`, `measure.rs`, `i18n/mod.rs`, `props.rs`, `mapping/mod.rs`, `egui`, `viz/library.rs`, `.ui`, `titlebar.rs`, `Quality`, `pathbuf`, `toolbar.rs`, `text_dim`, `ProfileBuilder`, `Category`, `text_field`, `fx_ui.rs`, `Profile`, `missing_audio.rs`, `patch_grid.rs`, `TouchView`, `text_field`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `file_dialog.rs`, `DmxCraftApp`, `Ui`, `control/tracking.rs`, `fader.rs`, `scene_props.rs`, `Lang`, `ButtonStyle`, `.ui`, `ProgrammerMode`, `control/timeline.rs`, `window.rs`, `stage.rs`, `viz/calibrate.rs`, `PaletteStore`, `AppState`, `viz/mod.rs`, `textures.rs`, `views/sound.rs`, `dialog.rs`, `snap.rs`, `app.rs`, `outline.rs`, `stage_editor.rs`?**
  _High betweenness centrality (0.086) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1310344827586207 - nodes in this community are weakly interconnected._