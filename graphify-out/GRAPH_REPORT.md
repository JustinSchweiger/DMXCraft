# Graph Report - dmxcraft-src  (2026-11-14)

## Corpus Check
- 178 files · ~281,215 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4848 nodes · 12102 edges · 355 communities (191 shown, 164 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 731 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8b348c61`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- DmxOutput
- open_dmx.rs
- core/src/venue.rs
- programmer.rs
- SoundBoard
- steps
- fx.rs
- Obs
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
- tests.rs
- fx_ui.rs
- Host
- ofl/src/lib.rs
- missing_audio.rs
- views/touch.rs
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
- viz/mod.rs
- theme.rs
- PaletteStore
- range_slider.rs
- ShutterEffect
- profile.rs
- runtime.rs
- Toggle<'a>
- app.js
- VizView
- ChannelKind
- textures.rs
- CapabilityKind
- Mapper
- MapPreset
- audio_triggers.rs
- qxf.rs
- show.rs
- dialog.rs
- paint_item
- Response
- arc
- Placement
- Venue
- EffectKind
- duration
- snap.rs
- Object
- TouchKind
- outline.rs
- ProfileBuilder
- convert.rs
- gltf_import.rs
- plan
- ViewCtx
- app/src/obs.rs
- global.rs
- IconButton
- CurveShape
- ColorPreset
- qlc-bundle.rs
- panels.rs
- i18n/mod.rs
- mcp/src/venue.rs
- egui
- stage_canvas.rs
- Deserialize
- FadeCurve
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
- Rect
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
- DmxFrame
- super
- env
- de.rs
- fx_color.rs
- ProfileChannel
- Dialog<'a>
- Step
- build-local-artifacts
- advanced.rs
- lock.rs
- AppState
- DeckState
- live.rs
- .update
- Drag
- FooterButton<'a>
- views/sound.rs
- look.rs
- announce
- effects.rs
- plan.rs
- release.ps1
- desktop.rs
- Camera
- pointer
- video_out.rs
- release-install.md
- super
- Category
- RemoteLink
- LookPart
- with
- TriggerBand
- measure.rs
- Tab
- mcp/src/profiles.rs
- Fader
- FixtureType
- ObsAction
- Tab
- Layer
- Cmd
- ProgrammerMode
- Shape
- MenuItem<'a>
- Quality
- Transform
- Effect
- trigger_cx
- DialogKind
- Layer
- ok
- label_tokens
- Tab
- Mode

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
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `label_with_glyphs()` --calls--> `label_tokens()`  [INFERRED]
  crates/app/src/views/touch/paint.rs → crates/core/src/touch.rs
- `angle_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `grid_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (355 total, 164 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (18): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+10 more)

### Community 2 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 3 - "open_dmx.rs"
Cohesion: 0.10
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "core/src/venue.rs"
Cohesion: 0.11
Nodes (14): aim_at_points_beam_to_target(), apply(), close(), Environment, hang_below_nearest_truss(), IDENTITY, Material, moving_head_aim() (+6 more)

### Community 5 - "programmer.rs"
Cohesion: 0.22
Nodes (18): color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block() (+10 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels() (+15 more)

### Community 9 - "Obs"
Cohesion: 0.19
Nodes (8): browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), engine(), fake_obs(), Obs, VideoPlay, wait(), wrong_password_is_reported()

### Community 10 - "props.rs"
Cohesion: 0.12
Nodes (20): renumber(), apply_texture(), TexAction, Builtin, Load, Set, aim_fixtures(), color_field() (+12 more)

### Community 11 - "ChannelRef"
Cohesion: 0.05
Nodes (7): Engine, lerp(), PlayingScene, SceneProgress, FastSet, apply_levels(), ChannelRef

### Community 12 - "mapping/mod.rs"
Cohesion: 0.10
Nodes (42): alt_click(), bank_step_wraps(), button_rect(), channel_mapping_sets_live_value(), fader(), fader_channels(), fire(), fires_id() (+34 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.12
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
Cohesion: 0.07
Nodes (44): analyze(), Analyzer, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad, BPM_MAX (+36 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (30): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+22 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "tests.rs"
Cohesion: 0.12
Nodes (40): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), end_scene_follows_jump_to(), fades_between_steps_and_loops() (+32 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.12
Nodes (29): rgb(), zone_color(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), ROW_H (+21 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 32 - "views/touch.rs"
Cohesion: 0.08
Nodes (33): t(), section_header(), add_page(), add_tile(), COLORS, display_label(), Drag, Move (+25 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "ChannelDef"
Cohesion: 0.12
Nodes (14): ChannelDef, ChannelRange, open_value_takes_the_highest_open_range(), RangeEffect, Closed, Color, Gobo, Open (+6 more)

### Community 36 - "t"
Cohesion: 0.11
Nodes (25): capability_fields(), category_label(), check(), color_label(), FOOT_H, full_combo(), general(), hex_field() (+17 more)

### Community 37 - "call"
Cohesion: 0.24
Nodes (25): add_clips(), add_effect(), add_fixtures(), apply_clip_fields(), apply_scene_fields(), call(), changed_scene(), create_scene() (+17 more)

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
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.11
Nodes (13): block_input(), ControlViews, DmxCraftApp, edit_preview(), fill_types(), load_step(), LockDialog, moved_out() (+5 more)

### Community 48 - "Ui"
Cohesion: 0.19
Nodes (27): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+19 more)

### Community 50 - "fx/mapping.rs"
Cohesion: 0.18
Nodes (10): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+2 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.13
Nodes (20): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+12 more)

### Community 52 - "TouchLayout"
Cohesion: 0.16
Nodes (8): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), LONG_PRESS_MS, pages_can_be_reordered(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Timeline"
Cohesion: 0.07
Nodes (27): grid_options(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Group, Obs, Scene (+19 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.08
Nodes (22): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, fmt_ms(), has_audio(), HEADER_W, LANE_H (+14 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (25): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, MapFile (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.17
Nodes (16): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+8 more)

### Community 71 - "app/src/audio.rs"
Cohesion: 0.13
Nodes (18): analysis(), analyze_file(), cache_file(), DECLICK, duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder() (+10 more)

### Community 72 - "Show"
Cohesion: 0.15
Nodes (11): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap (+3 more)

### Community 73 - "feature.rs"
Cohesion: 0.16
Nodes (16): block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value(), fill_ends_full() (+8 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.09
Nodes (17): FadeState, In, Out, MAX_STEP_ADVANCES_PER_TICK, PAN_DEG_PER_S, RELEASE_DARK_MS, RELEASE_LIGHT_MS, SETTLE_MS (+9 more)

### Community 78 - "viz/mod.rs"
Cohesion: 0.16
Nodes (20): AXIS_COLORS, floor_plan(), FloorShape, Handles, handles(), hit_y(), model_tex_id(), object_box() (+12 more)

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (51): Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas(), canvas_dim() (+43 more)

### Community 80 - "PaletteStore"
Cohesion: 0.36
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 86 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (20): slot_label(), ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse (+12 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (20): beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+12 more)

### Community 90 - "runtime.rs"
Cohesion: 0.13
Nodes (12): engine_keeps_running_without_device(), Failing, output_keeps_running_after_a_panic_elsewhere(), OutputStatus, Connecting, Error, Running, Stopped (+4 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (68): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+60 more)

### Community 93 - "VizView"
Cohesion: 0.11
Nodes (17): apply_placement(), GizmoDrag, Missing, move_all(), Pick, Fixture, Object, proposals_for_easy_view_names() (+9 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (32): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+24 more)

### Community 97 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

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
Cohesion: 0.13
Nodes (7): FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, LABEL_W, text_field()

### Community 105 - "paint_item"
Cohesion: 0.23
Nodes (13): content_rect(), fader_parts(), H_VALUE_W, icons_only(), label_galley(), label_with_glyphs(), led_color(), paint_cover() (+5 more)

### Community 106 - "Response"
Cohesion: 0.23
Nodes (8): chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width(), VizView

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "Venue"
Cohesion: 0.14
Nodes (7): load_file(), fixture_folders_are_exclusive(), FixtureFolder, Texture, room_and_tree(), TrussHang, Venue

### Community 110 - "EffectKind"
Cohesion: 0.09
Nodes (12): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+4 more)

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
Cohesion: 0.11
Nodes (15): ButtonStyle, .ALL, Led, Normal, Outline, Round, Switch, TouchKind (+7 more)

### Community 119 - "outline.rs"
Cohesion: 0.08
Nodes (32): deg_quat(), euler_matches_core_rotation(), quat_deg(), drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort (+24 more)

### Community 120 - "ProfileBuilder"
Cohesion: 0.16
Nodes (9): FileJob, Export, Import, import(), new_profile(), pick_export(), pick_import(), ProfileBuilder (+1 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (28): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+20 more)

### Community 123 - "gltf_import.rs"
Cohesion: 0.14
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 127 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 128 - "ViewCtx"
Cohesion: 0.08
Nodes (21): View, ViewCtx, body(), caption(), FixtureSettings, hanging_of(), range_row(), SetupView (+13 more)

### Community 129 - "app/src/obs.rs"
Cohesion: 0.15
Nodes (7): auth_matches_obs_websocket_example(), auth_response(), b64(), DRIFT_MS, Link, PRELOAD_MS, SETTLE

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
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 136 - "panels.rs"
Cohesion: 0.13
Nodes (4): fold_section(), Toggle, Calibrate, VizView

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (15): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.05
Nodes (33): DipSwitch, Knob, SIZE, SWEEP, Grip, Body, Corner, EdgeX (+25 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (20): AMBER, BOX, CanvasMode, Edit, Select, fit(), fixture_pos(), fixture_rect() (+12 more)

### Community 142 - "Deserialize"
Cohesion: 0.33
Nodes (7): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media

### Community 143 - "FadeCurve"
Cohesion: 0.25
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 144 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 146 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 147 - "assist.rs"
Cohesion: 0.18
Nodes (10): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, decode_texture() (+2 more)

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (10): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+2 more)

### Community 149 - "Settings"
Cohesion: 0.12
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 151 - "text_dim"
Cohesion: 0.14
Nodes (25): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+17 more)

### Community 152 - "media.rs"
Cohesion: 0.13
Nodes (20): Track, Timeline, mark_clipboard(), add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound() (+12 more)

### Community 153 - "instant"
Cohesion: 0.18
Nodes (4): name_pick(), obs_props(), big_show(), large_show_renders_within_a_tick()

### Community 154 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 155 - "text_field"
Cohesion: 0.16
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "Profile"
Cohesion: 0.10
Nodes (14): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+6 more)

### Community 159 - "FxHasher"
Cohesion: 0.20
Nodes (3): FastBuild, FxHasher, SEED

### Community 160 - "Rect"
Cohesion: 0.14
Nodes (16): card_row(), list_header(), list_row(), ListAction, Add, Duplicate, Remove, ModeOp (+8 more)

### Community 163 - "crash.rs"
Cohesion: 0.15
Nodes (13): arm(), Armed, ARMED, civil(), emergency_save(), install(), log_file(), marker_file() (+5 more)

### Community 164 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

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

### Community 169 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 170 - "Snapshot"
Cohesion: 0.29
Nodes (6): following_is_no_undo_step(), History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "PlanShape"
Cohesion: 0.15
Nodes (11): paint_fixture(), paint_live_corner(), paint_shape_icon(), PlanShape, .ALL, BarHorizontal, BarVertical, Effect (+3 more)

### Community 173 - "midi.rs"
Cohesion: 0.12
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.08
Nodes (35): Prepare, any_obj(), base64(), boolean(), build_steps(), camera_schema(), channel_kind(), clip_schema() (+27 more)

### Community 177 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 179 - "ClipAction"
Cohesion: 0.10
Nodes (18): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+10 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.27
Nodes (8): bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT, TrackingView, Tracker

### Community 181 - "bridge.rs"
Cohesion: 0.29
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 182 - "fader.rs"
Cohesion: 0.12
Nodes (13): DOT_H, FADER_W, FaderEvent, Changed, None, Release, ToggleFx, fine_mark() (+5 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.13
Nodes (24): time_field(), apply_to_others(), BAR_H, bar_slider(), block_props(), child(), chip(), fit_mapping() (+16 more)

### Community 184 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 185 - "paint_clip"
Cohesion: 0.17
Nodes (12): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle, paint_bands(), paint_beats() (+4 more)

### Community 188 - "src/tracking.rs"
Cohesion: 0.16
Nodes (4): all_cells(), DEFAULT_HEIGHT, Show, white()

### Community 189 - "run"
Cohesion: 0.20
Nodes (9): run(), session(), Shared, Status, Connected, Connecting, Failed, Off (+1 more)

### Community 190 - "Show"
Cohesion: 0.24
Nodes (4): room_json(), trackers_json(), Show, RoomMap

### Community 191 - "scene_group.rs"
Cohesion: 0.20
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "DmxFrame"
Cohesion: 0.13
Nodes (4): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE, Engine

### Community 195 - "super"
Cohesion: 0.39
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 200 - "fx_color.rs"
Cohesion: 0.10
Nodes (22): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+14 more)

### Community 201 - "ProfileChannel"
Cohesion: 0.21
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 202 - "Dialog<'a>"
Cohesion: 0.11
Nodes (7): Button, Dialog, Dialog<'a>, Kind, Cancel, Primary, Secondary

### Community 203 - "Step"
Cohesion: 0.09
Nodes (20): cv(), demo_show(), empty_show(), ensure_builtin_types(), FastMap, BankScope, All, Bank (+12 more)

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "advanced.rs"
Cohesion: 0.18
Nodes (13): advanced(), banks_of(), pick_tile(), PickColumn, Picker, picker_window(), PickTarget, Jump (+5 more)

### Community 212 - "lock.rs"
Cohesion: 0.33
Nodes (7): digest(), hash_and_verify(), hash_password(), hex(), MASTER, unlocks(), verify()

### Community 218 - "AppState"
Cohesion: 0.08
Nodes (12): AppState, Focus, Bank, Scenes, Mode, Builder, Setup, Sound (+4 more)

### Community 219 - "DeckState"
Cohesion: 0.15
Nodes (10): Player, DeckRt, DeckState, Empty, Ended, Fading, Paused, Playing (+2 more)

### Community 221 - "live.rs"
Cohesion: 0.18
Nodes (11): Control, apply_pick(), channels_of(), current_color(), current_position(), set_color(), set_position(), pick_adds_scene_button_and_assigns_to_selection() (+3 more)

### Community 225 - ".update"
Cohesion: 0.33
Nodes (8): fire(), media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws

### Community 226 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (21): button(), target(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color() (+13 more)

### Community 231 - "look.rs"
Cohesion: 0.28
Nodes (14): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, position_in_degrees(), rgbw_to_rgb_and_back() (+6 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "effects.rs"
Cohesion: 0.16
Nodes (8): Effect, effect_header(), effect_tiles(), FxFile, FxPreview, import_fx(), remove_effect(), steps_table()

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
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 244 - "Category"
Cohesion: 0.21
Nodes (11): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+3 more)

### Community 246 - "RemoteLink"
Cohesion: 0.15
Nodes (7): layout_json(), RemoteLink, RemoteStatus, Error, Off, Running, Running

### Community 251 - "LookPart"
Cohesion: 0.22
Nodes (9): LookAction, Copy, Paste, part_label(), LookPart, All, Color, Intensity (+1 more)

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 258 - "TriggerBand"
Cohesion: 0.25
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 272 - "measure.rs"
Cohesion: 0.23
Nodes (4): End, Measure, snap_points(), SNAP_PX

### Community 275 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 276 - "mcp/src/profiles.rs"
Cohesion: 0.30
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 285 - "Fader"
Cohesion: 0.19
Nodes (4): Fader, FaderRange, kind_color(), fader_ranges()

### Community 286 - "FixtureType"
Cohesion: 0.17
Nodes (8): defaults_are_rendered(), dimmer_effects_dim_rgb_only_fixtures_lit_by_another_scene(), limits_cap_dimmer_and_position(), tracking_aims_heads_at_the_point_and_releases_softly(), FixtureType, Physics, builtin_fixture_types(), ft()

### Community 289 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

### Community 339 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 340 - "Layer"
Cohesion: 0.21
Nodes (8): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On

### Community 341 - "Cmd"
Cohesion: 0.20
Nodes (10): apply(), Cmd, Color, Press, TrackToggle, Value, Xy, find_item() (+2 more)

### Community 342 - "ProgrammerMode"
Cohesion: 0.29
Nodes (9): ProgrammerMode, Blind, Edit, Live, hide_button(), items_mode(), mode_switch(), tool_separator() (+1 more)

### Community 343 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 345 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 346 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 347 - "Effect"
Cohesion: 0.43
Nodes (7): apply_effect(), apply_effect_hold(), feature_levels(), luma(), to_rgb(), write_colors(), Effect

### Community 348 - "trigger_cx"
Cohesion: 0.47
Nodes (5): channel_name(), describe(), describe_mapping(), t_deleted(), trigger_cx()

### Community 349 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 350 - "Layer"
Cohesion: 0.33
Nodes (4): Layer, .ALL, Blend, Cover

### Community 351 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 352 - "label_tokens"
Cohesion: 0.60
Nodes (4): label_tokens(), LabelPart, Icon, Text

### Community 353 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 354 - "Mode"
Cohesion: 0.67
Nodes (3): Mode, Build, Live

## Knowledge Gaps
- **583 isolated node(s):** `Single`, `Fill`, `Inward`, `Outward`, `Random` (+578 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1570 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **164 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `ChannelRef` to `ViewCtx`, `app/src/obs.rs`, `global.rs`, `library_panel.rs`, `programmer.rs`, `panels.rs`, `Obs`, `mcp/src/venue.rs`, `mapping/mod.rs`, `egui`, `.ui`, `mcp/src/profiles.rs`, `text_dim`, `media.rs`, `rig.rs`, `Host`, `crash.rs`, `t`, `call`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `control/tracking.rs`, `scene_props.rs`, `Show`, `Timeline`, `control/timeline.rs`, `window.rs`, `stage.rs`, `app/src/audio.rs`, `fx_color.rs`, `Show`, `Step`, `engine.rs`, `Layer`, `Cmd`, `ProgrammerMode`, `runtime.rs`, `trigger_cx`, `ChannelKind`, `Mapper`, `.update`, `audio_triggers.rs`, `Category`, `RemoteLink`, `ProfileBuilder`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._
- **Are the 206 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 206 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Single`, `Fill`, `Inward` to the rest of the system?**
  _583 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `SoundBoard`, `fx.rs`, `ChannelRef`, `mapping/mod.rs`, `stage_canvas.rs`, `measure.rs`, `assist.rs`, `rig.rs`, `instant`, `tests.rs`, `fx_ui.rs`, `FixtureType`, `missing_audio.rs`, `views/touch.rs`, `call`, `FixtureAction`, `autosave.rs`, `Snapshot`, `scene_grid.rs`, `ShowError`, `DmxCraftApp`, `tools.rs`, `TouchLayout`, `control/tracking.rs`, `scene_props.rs`, `src/tracking.rs`, `run`, `scene_group.rs`, `window.rs`, `stage.rs`, `feature.rs`, `Step`, `motion.rs`, `engine.rs`, `viz/mod.rs`, `theme.rs`, `Effect`, `trigger_cx`, `VizView`, `show.rs`, `Placement`, `Venue`, `EffectKind`, `snap.rs`, `Object`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._
- **Are the 8 inferred relationships involving `Show` (e.g. with `.apply_tracking()` and `Mappings`) actually correct?**
  _`Show` has 8 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12413793103448276 - nodes in this community are weakly interconnected._