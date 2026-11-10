//! Tools of the MCP server. Every tool gets its arguments as JSON and
//! returns JSON (or an understandable error message).

use dmxcraft_core::fixture::ChannelKind;
use dmxcraft_core::obs::ObsAction;
use dmxcraft_core::timeline::{ClipContent, Timeline};
use dmxcraft_core::{
    ChannelRef, ChannelValue, Effect, EffectKind, Engine, FixtureId, FixtureType, GroupId,
    LoopMode, SceneId, Show, Step, ZoneId, UNIVERSE_SIZE,
};
use serde_json::{json, Map, Value};

use crate::{AppEvent, Host};

type R = Result<Value, String>;

/// Description of a tool for `tools/list`.
struct Tool {
    name: &'static str,
    title: &'static str,
    description: &'static str,
    schema: fn() -> Value,
    read_only: bool,
}

macro_rules! props {
    ($($k:literal : $v:expr),* $(,)?) => {{
        let mut m = Map::new();
        $( m.insert($k.to_string(), $v); )*
        Value::Object(m)
    }};
}

fn obj(properties: Value, required: &[&str]) -> Value {
    json!({ "type": "object", "properties": properties, "required": required })
}
fn int(d: &str) -> Value {
    json!({ "type": "integer", "description": d })
}
fn num(d: &str) -> Value {
    json!({ "type": "number", "description": d })
}
fn string(d: &str) -> Value {
    json!({ "type": "string", "description": d })
}
fn boolean(d: &str) -> Value {
    json!({ "type": "boolean", "description": d })
}
fn ids(d: &str) -> Value {
    json!({ "type": "array", "items": { "type": "integer" }, "description": d })
}
fn any_obj(d: &str) -> Value {
    json!({ "type": "object", "description": d })
}

/// Values for channels: either individually or per channel kind for several
/// fixtures.
fn values_schema() -> Value {
    json!({
        "type": "array",
        "description": "Channel values. Each entry: {fixture: ID, channel: index (from 0) OR channel kind (\"Dimmer\", \"Red\", \"Green\", \"Blue\", \"White\", \"Amber\", \"Uv\", \"Pan\", \"Tilt\", \"Strobe\", \"ColorWheel\", \"Gobo\", \"Speed\") OR channel name, value: 0–255} – or short as [fixture, channel, value] (saves space with many values).",
        "items": {
            "anyOf": [
                {
                    "type": "object",
                    "properties": {
                        "fixture": { "type": "integer" },
                        "channel": { "type": ["integer", "string"] },
                        "value": { "type": "integer", "minimum": 0, "maximum": 255 }
                    },
                    "required": ["fixture", "channel", "value"]
                },
                {
                    "type": "array",
                    "items": { "type": ["integer", "string"] },
                    "minItems": 3,
                    "maxItems": 3
                }
            ]
        }
    })
}

fn step_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "hold_ms": { "type": "integer", "description": "Hold time (default 5000 for a single step, otherwise 1000)" },
            "fade_ms": { "type": "integer", "description": "Fade time from the previous step (default 0)" },
            "values": values_schema(),
            "fixtures": { "type": "array", "items": { "type": "integer" }, "description": "Fixtures that `set` is applied to" },
            "set": { "type": "object", "description": "Channel kind → value for all `fixtures`, e.g. {\"Dimmer\":255,\"Red\":255,\"Green\":0,\"Blue\":0}. Instead of RGB you can also use \"color\":\"#ff8800\"." }
        }
    })
}

fn venue_object_schema() -> Value {
    let v3 = |d: &str| json!({ "type": "array", "items": { "type": "number" }, "description": d });
    obj(
        props! {
            "id": int("Object ID (only when changing)"),
            "name": string("Name"),
            "shape": any_obj("Shape, see description"),
            "parent": int("Group the new object goes into (optional)"),
            "position": v3("[x, y, z] in m"),
            "rotation": v3("Degrees, X then Y then Z"),
            "scale": v3("Scale"),
            "color": string("#rrggbb"),
            "opacity": num("Opacity 0–1"),
            "gloss": num("Gloss 0–1"),
            "emissive": num("Self-illumination 0–1"),
            "locked": boolean("lock against moving"),
            "hidden": boolean("hide"),
        },
        &[],
    )
}

fn camera_schema() -> Value {
    json!({
        "type": "object",
        "description": "Camera (all optional): point = saved camera point (name or number from 1, see get_visualizer) or view = \"front\" | \"back\" | \"left\" | \"right\" | \"top\" | \"overview\" (home position) as the starting point, then yaw (degrees around the vertical axis; 0 = from the front, i.e. from the audience side +z; 90 = from the right +x), pitch (degrees, positive = from above, -89–89), distance (m), target [x, y, z] (look-at point in m), fov (field of view in degrees).",
        "properties": {
            "point": { "type": ["string", "integer"] },
            "view": { "type": "string" },
            "yaw": { "type": "number" },
            "pitch": { "type": "number" },
            "distance": { "type": "number" },
            "target": { "type": "array", "items": { "type": "number" } },
            "fov": { "type": "number" }
        }
    })
}

fn clip_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "track": { "type": "integer", "description": "Track (from 0); missing tracks are created" },
            "scene": { "type": "integer", "description": "Scene ID for a scene block" },
            "audio_path": { "type": "string", "description": "Path of an audio file for an audio block" },
            "video_path": { "type": "string", "description": "Path of a video file (MP4/MOV) for a video block that OBS plays" },
            "obs_scene": { "type": "string", "description": "Video block: OBS scene to switch to at the start. On its own (without scene/audio_path/video_path): OBS block that switches to this scene" },
            "obs_source": { "type": "string", "description": "Video block: media source in OBS (default from the OBS settings). With obs_scene + obs_visible: OBS block that shows/hides this source" },
            "obs_visible": { "type": "boolean", "description": "OBS block: show (true) or hide (false) the source" },
            "start_ms": { "type": "integer", "description": "Start on the timeline" },
            "duration_ms": { "type": "integer", "description": "Length (default: length of the scene or the file)" },
            "fade_in_ms": { "type": "integer" },
            "fade_out_ms": { "type": "integer" },
            "level": { "type": "number", "description": "Level 0–1 (dimmer or volume)" },
            "speed": { "type": "number", "description": "Speed of the block (1 = normal)" }
        },
        "required": ["start_ms"]
    })
}

const TOOLS: &[Tool] = &[
    Tool {
        name: "get_overview",
        title: "Overview",
        description: "Overview of the open show: name, number of fixtures, banks with their scenes, groups, running scenes, master, blackout, tempo.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "list_fixtures",
        title: "List fixtures",
        description: "All patched fixtures with ID, name, type, universe (from 1), start address and their channels (index, name, kind).",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "search_fixture_library",
        title: "Search fixture library",
        description: "Searches the built-in fixture library (generic types, Open Fixture Library, QLC+) by manufacturer/model/category. Returns the `type_key` for add_fixtures per mode.",
        schema: || obj(props! { "query": string("Search terms, e.g. \"rgbw par\" or \"moving head\""), "limit": int("max. hits (default 15)") }, &["query"]),
        read_only: true,
    },
    Tool {
        name: "add_fixtures",
        title: "Patch fixtures",
        description: "Patches one or more fixtures of a type at the next free addresses (from `address` in `universe`). Names are numbered.",
        schema: || obj(props! {
            "type_key": string("Type from search_fixture_library or list_fixtures, e.g. generic/rgbw-par-6ch"),
            "count": int("Count (default 1)"),
            "name": string("Base name, e.g. \"PAR\" → PAR 1, PAR 2 … (default: model name)"),
            "universe": int("Universe from 1 (default 1)"),
            "address": int("first start address (default 1 = first free)"),
        }, &["type_key"]),
        read_only: false,
    },
    Tool {
        name: "update_fixture",
        title: "Update fixture",
        description: "Changes name, patch address or position on the stage plan (x/y 0–1) of a fixture.",
        schema: || obj(props! {
            "id": int("Fixture ID"), "name": string("new name"),
            "universe": int("Universe from 1"), "address": int("Start address 1–512"),
            "plan_x": num("Stage plan x 0–1 (left → right)"), "plan_y": num("Stage plan y 0–1 (back → front)"),
        }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "delete_fixtures",
        title: "Delete fixtures",
        description: "Removes fixtures from the patch (and from groups, scenes and effects).",
        schema: || obj(props! { "ids": ids("Fixture IDs") }, &["ids"]),
        read_only: false,
    },
    Tool {
        name: "create_group",
        title: "Create group",
        description: "Creates a fixture group.",
        schema: || obj(props! { "name": string("Name"), "fixtures": ids("Fixture IDs") }, &["name", "fixtures"]),
        read_only: false,
    },
    Tool {
        name: "delete_group",
        title: "Delete group",
        description: "Deletes a group (the fixtures stay).",
        schema: || obj(props! { "id": int("Group ID") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "create_bank",
        title: "Create bank",
        description: "Creates a new bank (column of scenes).",
        schema: || obj(props! { "name": string("Name"), "color": string("Color as #rrggbb (optional)") }, &["name"]),
        read_only: false,
    },
    Tool {
        name: "update_bank",
        title: "Update bank",
        description: "Renames a bank, changes its color or hides/shows it.",
        schema: || obj(props! { "id": int("Bank ID"), "name": string("new name"), "color": string("#rrggbb or \"auto\""), "hidden": boolean("hide") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "delete_bank",
        title: "Delete bank",
        description: "Deletes a bank together with its scenes.",
        schema: || obj(props! { "id": int("Bank ID") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "list_scenes",
        title: "List scenes",
        description: "Scenes (optionally only of one bank) with ID, name, bank, kind (STATIC/FX/SUPER SCENE), duration and whether they are running.",
        schema: || obj(props! { "bank": int("only this bank (optional)") }, &[]),
        read_only: true,
    },
    Tool {
        name: "get_scene",
        title: "Read scene",
        description: "All details of a scene: settings, steps with channel values, effects (as JSON) and for SUPER SCENES the timeline.",
        schema: || obj(props! { "id": int("Scene ID") }, &["id"]),
        read_only: true,
    },
    Tool {
        name: "create_scene",
        title: "Create scene",
        description: "Creates a scene in a bank. One step = static scene; several steps = sequence (Steps). Values via `values` or conveniently via `fixtures` + `set` per step.",
        schema: || obj(props! {
            "name": string("Name"), "bank": int("Bank ID"),
            "steps": json!({ "type": "array", "items": step_schema(), "description": "Steps (at least one)" }),
            "color": string("Color of the card #rrggbb (optional)"),
            "loop": boolean("Repeat the sequence endlessly (default true)"),
            "settings": any_obj("Scene settings, e.g. {\"fade_in_ms\":500,\"fade_out_ms\":500,\"dimmer\":255,\"speed_pct\":100,\"flash\":false}"),
            "select": boolean("select in the UI (default true)"),
            "verbose": boolean("return the full scene with all channel values (default: short summary; details via get_scene)"),
        }, &["name", "bank", "steps"]),
        read_only: false,
    },
    Tool {
        name: "update_scene",
        title: "Update scene",
        description: "Changes a scene: name, color, bank, hidden, endless loop, settings (merged) or replaces the steps.",
        schema: || obj(props! {
            "id": int("Scene ID"), "name": string("new name"), "color": string("#rrggbb or \"auto\""),
            "bank": int("move to this bank"), "hidden": boolean("hide"),
            "loop": boolean("repeat endlessly"), "settings": any_obj("Settings (only the given fields change)"),
            "steps": json!({ "type": "array", "items": step_schema(), "description": "new steps (replace all)" }),
            "verbose": boolean("return the full scene with all channel values (default: short summary; details via get_scene)"),
        }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "delete_scenes",
        title: "Delete scenes",
        description: "Deletes scenes.",
        schema: || obj(props! { "ids": ids("Scene IDs") }, &["ids"]),
        read_only: false,
    },
    Tool {
        name: "describe_effect",
        title: "Describe effect",
        description: "Returns the default settings of an effect type as JSON (template for add_effect/update_effect). Types: ColorFx, ChaserFx, MoveFx, ValueFx, CurveFx, Mappings, ColorMappings.",
        schema: || obj(props! { "kind": string("Effekttyp") }, &["kind"]),
        read_only: true,
    },
    Tool {
        name: "add_effect",
        title: "Add effect",
        description: "Adds an effect to a scene (makes it an FX scene). `fixtures` in the desired order (= run/color order). `params` overrides fields of the template from describe_effect.",
        schema: || obj(props! {
            "scene": int("Scene ID"), "kind": string("Effect type, e.g. ColorFx"),
            "fixtures": ids("Fixtures in effect order"), "params": any_obj("Fields as in describe_effect"),
            "verbose": boolean("return the full scene with all channel values (default: short summary; details via get_scene)"),
        }, &["scene", "kind", "fixtures"]),
        read_only: false,
    },
    Tool {
        name: "update_effect",
        title: "Update effect",
        description: "Changes an effect of a scene (fields are merged).",
        schema: || obj(props! {
            "scene": int("Scene ID"), "effect": int("Effect ID from get_scene"),
            "fixtures": ids("new fixtures (optional)"), "params": any_obj("changed fields"),
            "verbose": boolean("return the full scene with all channel values (default: short summary; details via get_scene)"),
        }, &["scene", "effect"]),
        read_only: false,
    },
    Tool {
        name: "remove_effect",
        title: "Remove effect",
        description: "Removes an effect from a scene.",
        schema: || obj(props! { "scene": int("Scene ID"), "effect": int("Effect ID") }, &["scene", "effect"]),
        read_only: false,
    },
    Tool {
        name: "create_super_scene",
        title: "Create SUPER SCENE",
        description: "Creates a SUPER SCENE: a timeline whose tracks hold scene and audio blocks (later tracks override earlier ones). Without `clips` an empty timeline is created.",
        schema: || obj(props! {
            "name": string("Name"), "bank": int("Bank ID"),
            "clips": json!({ "type": "array", "items": clip_schema() }),
            "track_names": json!({ "type": "array", "items": { "type": "string" }, "description": "Names of the tracks (optional)" }),
            "bpm": num("Tempo for the beat grid (optional)"),
            "verbose": boolean("return the full scene with all channel values (default: short summary; details via get_scene)"),
        }, &["name", "bank"]),
        read_only: false,
    },
    Tool {
        name: "add_timeline_clips",
        title: "Add blocks",
        description: "Adds scene or audio blocks to a SUPER SCENE.",
        schema: || obj(props! { "scene": int("ID of the SUPER SCENE"), "clips": json!({ "type": "array", "items": clip_schema() }) }, &["scene", "clips"]),
        read_only: false,
    },
    Tool {
        name: "remove_timeline_clips",
        title: "Remove blocks",
        description: "Removes blocks (IDs from get_scene) from a SUPER SCENE.",
        schema: || obj(props! { "scene": int("ID of the SUPER SCENE"), "clips": ids("Block-IDs") }, &["scene", "clips"]),
        read_only: false,
    },
    Tool {
        name: "play_scene",
        title: "Play scene",
        description: "Starts a scene (other scenes of the same bank stop). SUPER SCENES run from `position_ms`.",
        schema: || obj(props! { "id": int("Scene ID"), "position_ms": int("Start position of a SUPER SCENE (optional)") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "stop_scene",
        title: "Stop scene",
        description: "Stops a scene (with its fade-out time). Without `id`: all scenes.",
        schema: || obj(props! { "id": int("Scene ID (empty = all)") }, &[]),
        read_only: false,
    },
    Tool {
        name: "pause_scene",
        title: "Pause",
        description: "Pauses or resumes a running scene.",
        schema: || obj(props! { "id": int("Scene ID") }, &["id"]),
        read_only: false,
    },
    Tool {
        name: "set_output",
        title: "Master/Blackout/Tempo",
        description: "Sets master (0–1), blackout and global tempo (1 = normal).",
        schema: || obj(props! { "master": num("0–1"), "blackout": boolean("Blackout on/off"), "speed": num("Tempo, 1 = normal") }, &[]),
        read_only: false,
    },
    Tool {
        name: "set_live_values",
        title: "Set live values",
        description: "Sets channels directly in live mode (sits above all scenes, not saved). `clear: true` removes all live values first.",
        schema: || obj(props! {
            "values": values_schema(),
            "fixtures": ids("Fixtures for `set`"),
            "set": any_obj("Channel kind → value, e.g. {\"Dimmer\":255,\"color\":\"#00ffaa\"}"),
            "clear": boolean("clear all live values first"),
        }, &[]),
        read_only: false,
    },
    Tool {
        name: "clear_live_values",
        title: "Clear live values",
        description: "Removes all live values.",
        schema: || obj(json!({}), &[]),
        read_only: false,
    },
    Tool {
        name: "get_dmx_output",
        title: "Read DMX output",
        description: "Current DMX values of a universe (or per fixture: `fixtures`).",
        schema: || obj(props! {
            "universe": int("Universe from 1 (default 1)"),
            "start": int("first address (default 1)"), "count": int("Count (default 512)"),
            "fixtures": ids("instead of an address range: values of these fixtures per channel"),
        }, &[]),
        read_only: true,
    },
    Tool {
        name: "select_scene",
        title: "Show scene",
        description: "Selects a scene in the UI so the user can see it.",
        schema: || obj(props! { "id": int("Scene ID") }, &["id"]),
        read_only: true,
    },
    Tool {
        name: "get_venue",
        title: "Read venue",
        description: "3D venue: room size, environment (ambient light, beams, fog), all objects (boxes, planes, cylinders, trusses, groups) with placement and world position, and the 3D placement of every fixture. Units: meters, y up.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "set_venue",
        title: "Set up venue",
        description: "Changes name, room size [width, height, depth] and environment (ambient, beams, fog each 0–1).",
        schema: || obj(props! {
            "name": string("Name"), "size": json!({ "type": "array", "items": { "type": "number" }, "description": "[width x, height y, depth z] in m" }),
            "ambient": num("Ambient light 0–1"), "beams": num("Visibility of the beams 0–1"), "fog": num("Fog 0–1"),
        }, &[]),
        read_only: false,
    },
    Tool {
        name: "add_venue_object",
        title: "Add object to the venue",
        description: "Adds an object. `shape` e.g. {\"type\":\"box\",\"size\":[2,0.4,1]}, {\"type\":\"plane\",\"size\":[4,3]}, {\"type\":\"cylinder\",\"radius\":0.05,\"height\":3}, {\"type\":\"truss\",\"length\":3,\"profile\":\"square\"} (along x) or {\"type\":\"group\"}. The center of the object is at `position`.",
        schema: venue_object_schema,
        read_only: false,
    },
    Tool {
        name: "update_venue_object",
        title: "Update object",
        description: "Changes shape, placement, color, opacity … of an object (only the given fields).",
        schema: venue_object_schema,
        read_only: false,
    },
    Tool {
        name: "delete_venue_objects",
        title: "Remove objects",
        description: "Removes objects (groups together with their content).",
        schema: || obj(props! { "ids": ids("Object IDs") }, &["ids"]),
        read_only: false,
    },
    Tool {
        name: "place_fixtures",
        title: "Place fixtures in the room",
        description: "Sets the 3D placement of fixtures: `position` (for several, offset by `spacing` per fixture), optionally hang them on the nearest truss, `rotation` or aim them at a point `aim_at`. `automatic`: derive from the stage plan again.",
        schema: || obj(props! {
            "fixtures": ids("Fixture IDs"),
            "position": json!({ "type": "array", "items": { "type": "number" }, "description": "[x, y, z] in m for the first fixture" }),
            "spacing": json!({ "type": "array", "items": { "type": "number" }, "description": "Offset per further fixture [x, y, z]" }),
            "hang_on_truss": boolean("hang under the nearest truss"),
            "rotation": json!({ "type": "array", "items": { "type": "number" }, "description": "Euler angles in degrees; [0,0,0] points straight down" }),
            "aim_at": json!({ "type": "array", "items": { "type": "number" }, "description": "Target point [x, y, z] the fixture points at" }),
            "automatic": boolean("Placement automatic again"),
        }, &["fixtures"]),
        read_only: false,
    },
    Tool {
        name: "get_visualizer",
        title: "Query 3D visualizer",
        description: "State of the 3D visualizer: window open, quality, camera (yaw, pitch, distance, target, fov), saved camera points and room size. The venue itself (structures, fixture placement) comes from get_venue; ambient light, beams and fog are set with set_venue.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "set_visualizer",
        title: "Set up 3D visualizer",
        description: "Open/close the window, set the quality (\"low\", \"medium\", \"high\", \"ultra\") and the camera of the view (with a camera move when the window is open). Create or overwrite camera points (reachable in the visualizer bar by click or keys 1–9, saved in the show) with `save_point` (current camera or `camera`) and remove them with `delete_point`.",
        schema: || obj(props! {
            "open": boolean("Open (true) or close (false) the window"),
            "quality": string("low | medium | high | ultra"),
            "camera": camera_schema(),
            "save_point": string("Save a camera point with this name (an existing one with the same name is overwritten)"),
            "delete_point": string("Remove a camera point (name or number)"),
        }, &[]),
        read_only: false,
    },
    Tool {
        name: "visualizer_screenshot",
        title: "3D visualizer image",
        description: "Renders the 3D view with the current DMX output as a PNG image (light beams, light spots, structures) – e.g. to check positions, aiming or colors of a scene. Without `camera` from the point of view of the current camera. The window does not need to be open.",
        schema: || obj(props! {
            "camera": camera_schema(),
            "width": int("Width in pixels (default 1280, 320–2560)"),
            "height": int("Height in pixels (default 720, 240–1600)"),
        }, &[]),
        read_only: true,
    },
    Tool {
        name: "list_profiles",
        title: "Custom fixture profiles",
        description: "Custom fixture profiles from the Profile Builder with modes, fixture type keys (type_key for add_fixtures) and open problems.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "get_profile",
        title: "Read fixture profile",
        description: "A custom fixture profile in full (channels with ranges, modes, physical data) – to change it, read, adjust and save with save_profile.",
        schema: || obj(props! { "key": string("Key from list_profiles") }, &["key"]),
        read_only: true,
    },
    Tool {
        name: "describe_profile_format",
        title: "Fixture profile format",
        description: "Template (example profile), notes and all range kinds (capability kinds) with their values for save_profile.",
        schema: || obj(json!({}), &[]),
        read_only: true,
    },
    Tool {
        name: "save_profile",
        title: "Save fixture profile",
        description: "Creates a custom fixture profile (without key) or replaces one (with key) – like saving in the Profile Builder. Ranges are made gapless; on errors (missing name, mode, angle …) nothing is saved and the problems are listed. Already patched fixtures of the profile are updated.",
        schema: || obj(props! { "profile": any_obj("Profile as returned by get_profile or describe_profile_format") }, &["profile"]),
        read_only: false,
    },
    Tool {
        name: "delete_profile",
        title: "Remove fixture profile",
        description: "Removes a custom fixture profile from the library (the file goes to the “deleted” folder). Patched fixtures keep their fixture type.",
        schema: || obj(props! { "key": string("Key from list_profiles") }, &["key"]),
        read_only: false,
    },
    Tool {
        name: "save_show",
        title: "Save show",
        description: "Saves the show (without `path` under the current path).",
        schema: || obj(props! { "path": string("File path, extension .dmxcraft (optional)") }, &[]),
        read_only: false,
    },
];

/// Tool list for `tools/list`.
pub fn list() -> Vec<Value> {
    TOOLS
        .iter()
        .map(|t| {
            json!({
                "name": t.name,
                "title": t.title,
                "description": t.description,
                "inputSchema": (t.schema)(),
                "annotations": { "readOnlyHint": t.read_only, "destructiveHint": t.name.starts_with("delete") || t.name.starts_with("remove") },
            })
        })
        .collect()
}

/// Executes a tool.
pub fn call(host: &Host, name: &str, a: &Value) -> R {
    let read_only = TOOLS
        .iter()
        .find(|t| t.name == name)
        .is_some_and(|t| t.read_only);
    if !read_only && host.locked.load(std::sync::atomic::Ordering::Relaxed) {
        return Err(
            "DMXCraft is locked (lock in the title bar) – only read-only tools are allowed".into(),
        );
    }
    // 3D visualizer: runs in the UI, which needs the engine for it – so call
    // it without locking the engine
    match name {
        "get_visualizer" => return (host.viz)("get", a).map(|r| r.0),
        "set_visualizer" => return (host.viz)("set", a).map(|r| r.0),
        "visualizer_screenshot" => {
            let (mut info, png) = (host.viz)("screenshot", a)?;
            let png = png.ok_or("No image received")?;
            info["__image_png"] = Value::String(base64(&png));
            return Ok(info);
        }
        "list_profiles" => return crate::profiles::list(host.profiles.as_ref()),
        "get_profile" => return crate::profiles::get(host.profiles.as_ref(), &req_str(a, "key")?),
        "describe_profile_format" => return crate::profiles::format(),
        _ => {}
    }
    let user_types = if matches!(name, "search_fixture_library" | "add_fixtures") {
        crate::profiles::user_types(host.profiles.as_ref())
    } else {
        Vec::new()
    };
    let mut guard = host
        .engine
        .lock()
        .map_err(|_| "Engine not available".to_string())?;
    let e: &mut Engine = &mut guard;
    let out = match name {
        "get_overview" => overview(e),
        "list_fixtures" => Ok(
            json!({ "fixtures": e.show().fixtures.iter().map(|f| fixture_json(e.show(), f.id)).collect::<Vec<_>>() }),
        ),
        "search_fixture_library" => search_library(a, &user_types),
        "add_fixtures" => add_fixtures(e, a, &user_types),
        "save_profile" => {
            let r = crate::profiles::save(host.profiles.as_ref(), e, a);
            if r.is_ok() {
                (host.notify)(AppEvent::ProfilesChanged);
            }
            r
        }
        "delete_profile" => {
            let r = crate::profiles::delete(host.profiles.as_ref(), e, &req_str(a, "key")?);
            if r.is_ok() {
                (host.notify)(AppEvent::ProfilesChanged);
            }
            r
        }
        "update_fixture" => update_fixture(e, a),
        "delete_fixtures" => {
            let list = id_list(a, "ids")?;
            for id in &list {
                e.show_mut().remove_fixture(FixtureId(*id));
                for s in &mut e.show_mut().scenes {
                    for st in &mut s.steps {
                        st.values.retain(|v| v.fixture.0 != *id);
                    }
                    for fx in &mut s.effects {
                        fx.fixtures.retain(|f| f.0 != *id);
                    }
                }
            }
            e.sync();
            Ok(json!({ "deleted": list }))
        }
        "create_group" => {
            let name = req_str(a, "name")?;
            let fx = existing_fixtures(e.show(), &id_list(a, "fixtures")?)?;
            let id = e.show_mut().add_group(name, fx);
            Ok(json!({ "group": id.0 }))
        }
        "delete_group" => {
            let id = GroupId(req_u32(a, "id")?);
            e.show()
                .groups
                .iter()
                .find(|g| g.id == id)
                .ok_or("Group not found")?;
            e.show_mut().remove_group(id);
            Ok(json!({ "deleted": id.0 }))
        }
        "create_bank" => {
            let id = e.show_mut().add_zone(req_str(a, "name")?);
            if let Some(c) = a.get("color") {
                e.show_mut().zone_mut(id).ok_or("Bank missing")?.color = parse_color_opt(c)?;
            }
            Ok(json!({ "bank": id.0 }))
        }
        "update_bank" => {
            let id = ZoneId(req_u32(a, "id")?);
            let z = e.show_mut().zone_mut(id).ok_or("Bank not found")?;
            if let Some(n) = opt_str(a, "name") {
                z.name = n;
            }
            if let Some(c) = a.get("color") {
                z.color = parse_color_opt(c)?;
            }
            if let Some(h) = a.get("hidden").and_then(Value::as_bool) {
                z.hidden = h;
            }
            Ok(json!({ "bank": id.0 }))
        }
        "delete_bank" => {
            let id = ZoneId(req_u32(a, "id")?);
            e.show().zone(id).ok_or("Bank not found")?;
            e.show_mut().remove_zone_with_scenes(id);
            e.sync();
            Ok(json!({ "deleted": id.0 }))
        }
        "list_scenes" => list_scenes(e, a),
        "get_scene" => get_scene(e, SceneId(req_u32(a, "id")?)),
        "create_scene" => create_scene(host, e, a),
        "update_scene" => update_scene(e, a),
        "delete_scenes" => {
            let list = id_list(a, "ids")?;
            for id in &list {
                e.release(SceneId(*id));
                e.show_mut().remove_scene(SceneId(*id));
            }
            e.sync();
            Ok(json!({ "deleted": list }))
        }
        "describe_effect" => {
            let kind = parse_kind(&req_str(a, "kind")?)?;
            Ok(
                json!({ "kind": kind_name(kind), "template": serde_json::to_value(Effect::new(1, kind)).unwrap_or_default() }),
            )
        }
        "add_effect" => add_effect(e, a),
        "update_effect" => update_effect(e, a),
        "remove_effect" => {
            let sid = SceneId(req_u32(a, "scene")?);
            let fid = req_u32(a, "effect")?;
            let s = e.show_mut().scene_mut(sid).ok_or("Scene not found")?;
            let n = s.effects.len();
            s.effects.retain(|f| f.id != fid);
            if s.effects.len() == n {
                return Err("Effect not found".into());
            }
            Ok(json!({ "scene": sid.0, "removed": fid }))
        }
        "create_super_scene" => create_super_scene(host, e, a),
        "add_timeline_clips" => {
            let sid = SceneId(req_u32(a, "scene")?);
            let clips = a
                .get("clips")
                .and_then(Value::as_array)
                .ok_or("`clips` missing")?
                .clone();
            let ids = add_clips(host, e, sid, &clips)?;
            Ok(json!({ "scene": sid.0, "clips": ids }))
        }
        "remove_timeline_clips" => {
            let sid = SceneId(req_u32(a, "scene")?);
            let list = id_list(a, "clips")?;
            let s = e.show_mut().scene_mut(sid).ok_or("Scene not found")?;
            let tl = s.timeline_mut().ok_or("Not a SUPER SCENE")?;
            for c in &list {
                tl.remove_clip(*c);
            }
            s.sync_timeline_length();
            Ok(json!({ "scene": sid.0, "removed": list }))
        }
        "play_scene" => {
            let id = SceneId(req_u32(a, "id")?);
            let s = e.show().scene(id).ok_or("Scene not found")?;
            if s.steps.is_empty() {
                return Err("Scene is empty".into());
            }
            e.activate(id);
            if let Some(ms) = a.get("position_ms").and_then(Value::as_f64) {
                e.seek_timeline(id, ms);
            }
            Ok(json!({ "playing": id.0 }))
        }
        "stop_scene" => match a.get("id").and_then(Value::as_u64) {
            Some(id) => {
                e.release(SceneId(id as u32));
                Ok(json!({ "stopped": id }))
            }
            None => {
                e.release_all();
                Ok(json!({ "stopped": "all" }))
            }
        },
        "pause_scene" => {
            let id = SceneId(req_u32(a, "id")?);
            if !e.is_active(id) {
                return Err("Scene is not running".into());
            }
            e.toggle_pause(id);
            Ok(json!({ "scene": id.0, "paused": e.progress(id).is_some_and(|p| p.paused) }))
        }
        "set_output" => {
            if let Some(m) = a.get("master").and_then(Value::as_f64) {
                e.set_master(m.clamp(0.0, 1.0) as f32);
            }
            if let Some(b) = a.get("blackout").and_then(Value::as_bool) {
                e.set_blackout(b);
            }
            if let Some(s) = a.get("speed").and_then(Value::as_f64) {
                e.set_speed(s as f32);
            }
            Ok(json!({ "master": e.master(), "blackout": e.blackout(), "speed": e.speed() }))
        }
        "set_live_values" => {
            if a.get("clear").and_then(Value::as_bool).unwrap_or(false) {
                e.clear_live();
            }
            let vals = collect_values(e.show(), a)?;
            for v in &vals {
                e.set_live(v.channel_ref(), v.value);
            }
            Ok(json!({ "set": vals.len() }))
        }
        "clear_live_values" => {
            e.clear_live();
            Ok(json!({ "cleared": true }))
        }
        "get_dmx_output" => dmx_output(e, a),
        "select_scene" => {
            let id = SceneId(req_u32(a, "id")?);
            e.show().scene(id).ok_or("Scene not found")?;
            (host.notify)(AppEvent::SelectScene(id));
            Ok(json!({ "selected": id.0 }))
        }
        "save_show" => {
            let path = opt_str(a, "path")
                .or_else(|| {
                    host.show_path
                        .lock()
                        .ok()
                        .map(|p| p.clone())
                        .filter(|p| !p.is_empty())
                })
                .ok_or("The show has no file path yet – please provide `path`")?;
            if let Some(stem) = std::path::Path::new(&path).file_stem() {
                e.show_mut().name = stem.to_string_lossy().into_owned();
            }
            e.show().save(&path).map_err(|err| err.to_string())?;
            (host.notify)(AppEvent::Saved(path.clone()));
            Ok(json!({ "saved": path }))
        }
        "get_venue" => crate::venue::get_venue(e),
        "set_venue" => crate::venue::set_venue(e, a),
        "add_venue_object" => crate::venue::add_object(e, a),
        "update_venue_object" => crate::venue::update_object(e, a),
        "delete_venue_objects" => crate::venue::delete_objects(e, &id_list(a, "ids")?),
        "place_fixtures" => crate::venue::place_fixtures(e, a),
        _ => Err(format!("Unknown tool: {name}")),
    };
    drop(guard);
    out
}

// ---------- Helpers: arguments ----------

fn req_u32(a: &Value, k: &str) -> Result<u32, String> {
    a.get(k)
        .and_then(Value::as_u64)
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| format!("`{k}` missing or not a number"))
}
fn opt_u64(a: &Value, k: &str) -> Option<u64> {
    a.get(k).and_then(Value::as_u64)
}
fn req_str(a: &Value, k: &str) -> Result<String, String> {
    opt_str(a, k).ok_or_else(|| format!("`{k}` missing"))
}
fn opt_str(a: &Value, k: &str) -> Option<String> {
    a.get(k).and_then(Value::as_str).map(str::to_string)
}
fn id_list(a: &Value, k: &str) -> Result<Vec<u32>, String> {
    a.get(k)
        .and_then(Value::as_array)
        .map(|v| {
            v.iter()
                .filter_map(Value::as_u64)
                .filter_map(|n| u32::try_from(n).ok())
                .collect()
        })
        .ok_or_else(|| format!("`{k}` missing (list of IDs)"))
}

fn existing_fixtures(show: &Show, list: &[u32]) -> Result<Vec<FixtureId>, String> {
    list.iter()
        .map(|id| {
            let f = FixtureId(*id);
            show.fixture(f)
                .map(|_| f)
                .ok_or(format!("Fixture {id} not found"))
        })
        .collect()
}

/// “#rrggbb” → RGB.
pub(crate) fn parse_hex(s: &str) -> Result<[u8; 3], String> {
    let h = s.trim().trim_start_matches('#');
    if h.len() != 6 {
        return Err(format!("Color “{s}”: expected #rrggbb"));
    }
    let p =
        |i: usize| u8::from_str_radix(&h[i..i + 2], 16).map_err(|_| format!("Invalid color “{s}”"));
    Ok([p(0)?, p(2)?, p(4)?])
}

/// Color of a bank/scene: `"auto"`/null = no own color.
fn parse_color_opt(v: &Value) -> Result<Option<[u8; 3]>, String> {
    match v {
        Value::Null => Ok(None),
        Value::String(s) if s.eq_ignore_ascii_case("auto") || s.is_empty() => Ok(None),
        Value::String(s) => parse_hex(s).map(Some),
        _ => Err("Give the color as \"#rrggbb\"".into()),
    }
}

fn parse_kind(s: &str) -> Result<EffectKind, String> {
    let norm = |x: &str| x.to_lowercase().replace([' ', '_', '-'], "");
    EffectKind::ALL
        .into_iter()
        .find(|k| norm(kind_name(*k)) == norm(s) || norm(k.label()) == norm(s))
        .ok_or_else(|| format!("Unknown effect type “{s}”"))
}

fn kind_name(k: EffectKind) -> &'static str {
    match k {
        EffectKind::Steps => "Steps",
        EffectKind::ColorFx => "ColorFx",
        EffectKind::ChaserFx => "ChaserFx",
        EffectKind::MoveFx => "MoveFx",
        EffectKind::ValueFx => "ValueFx",
        EffectKind::CurveFx => "CurveFx",
        EffectKind::Mappings => "Mappings",
        EffectKind::ColorMappings => "ColorMappings",
        EffectKind::SuperScene => "SuperScene",
    }
}

fn channel_kind(s: &str) -> Option<ChannelKind> {
    let n = s.to_lowercase().replace([' ', '_', '-'], "");
    let alias = match n.as_str() {
        "r" | "rot" => "red",
        "g" | "grün" | "gruen" => "green",
        "b" | "blau" => "blue",
        "w" | "weiß" | "weiss" => "white",
        "intensity" | "intensität" => "dimmer",
        "colour" | "farbrad" => "colorwheel",
        "nebel" | "smoke" | "haze" => "fog",
        "lüfter" | "luefter" => "fan",
        "fokus" => "focus",
        "programm" | "makro" | "macro" | "effekt" => "program",
        "steuerung" | "modus" | "mode" => "control",
        "keinefunktion" => "nofunction",
        other => other,
    };
    ChannelKind::ALL
        .into_iter()
        .find(|k| format!("{k:?}").to_lowercase() == alias)
}

/// Channel index of a fixture from index, channel kind or channel name.
fn resolve_channel(show: &Show, fixture: FixtureId, ch: &Value) -> Result<u16, String> {
    let f = show
        .fixture(fixture)
        .ok_or(format!("Fixture {} not found", fixture.0))?;
    let ty = show.type_of(f).ok_or("Fixture type missing")?;
    if let Some(i) = ch.as_u64() {
        return (i < ty.channels.len() as u64)
            .then_some(i as u16)
            .ok_or(format!("{} has no channel {i}", f.name));
    }
    let s = ch.as_str().ok_or("Give `channel` as index or name")?;
    if let Some(k) = channel_kind(s) {
        if let Some(i) = ty.channels.iter().position(|c| c.kind == k) {
            return Ok(i as u16);
        }
    }
    ty.channels
        .iter()
        .position(|c| c.name.eq_ignore_ascii_case(s))
        .map(|i| i as u16)
        .ok_or(format!("{} has no channel “{s}”", f.name))
}

/// Values from `values` as well as `fixtures` + `set` (per channel kind).
fn collect_values(show: &Show, a: &Value) -> Result<Vec<ChannelValue>, String> {
    let mut out: Vec<ChannelValue> = Vec::new();
    let mut put = |v: ChannelValue| {
        out.retain(|o| o.channel_ref() != v.channel_ref());
        out.push(v);
    };
    if let Some(vals) = a.get("values").and_then(Value::as_array) {
        for v in vals {
            // short: [fixture, channel, value]
            let (f, c, val) = match v.as_array() {
                Some(t) if t.len() == 3 => (t[0].clone(), t[1].clone(), t[2].clone()),
                Some(_) => return Err("short form: [fixture, channel, value]".into()),
                None => (
                    v.get("fixture").cloned().unwrap_or(Value::Null),
                    v.get("channel").cloned().unwrap_or(Value::Null),
                    v.get("value").cloned().unwrap_or(Value::Null),
                ),
            };
            let fixture = FixtureId(
                f.as_u64()
                    .and_then(|n| u32::try_from(n).ok())
                    .ok_or("`fixture` missing")?,
            );
            let channel = resolve_channel(show, fixture, &c)?;
            let value = val.as_u64().ok_or("`value` missing")?.min(255) as u8;
            put(ChannelValue {
                fixture,
                channel,
                value,
            });
        }
    }
    if let Some(set) = a.get("set").and_then(Value::as_object) {
        let fixtures = existing_fixtures(
            show,
            &id_list(a, "fixtures").map_err(|_| "`set` needs `fixtures`")?,
        )?;
        // “color” as a shorthand for red/green/blue
        let mut pairs: Vec<(String, u8)> = Vec::new();
        for (k, v) in set {
            if k.eq_ignore_ascii_case("color") {
                let rgb = parse_hex(v.as_str().ok_or("color as \"#rrggbb\"")?)?;
                pairs.extend([
                    ("Red".into(), rgb[0]),
                    ("Green".into(), rgb[1]),
                    ("Blue".into(), rgb[2]),
                ]);
            } else {
                let n = v.as_u64().ok_or(format!("Value for {k} missing"))?.min(255) as u8;
                pairs.push((k.clone(), n));
            }
        }
        for f in fixtures {
            let ty = show
                .fixture(f)
                .and_then(|x| show.type_of(x))
                .ok_or("Fixture type missing")?;
            for (k, value) in &pairs {
                let kind = channel_kind(k).ok_or(format!("Unknown channel kind “{k}”"))?;
                // Fixtures without this channel kind are skipped
                for (i, c) in ty.channels.iter().enumerate() {
                    if c.kind == kind {
                        put(ChannelValue {
                            fixture: f,
                            channel: i as u16,
                            value: *value,
                        });
                    }
                }
            }
        }
    }
    Ok(out)
}

fn build_steps(show: &Show, a: &Value) -> Result<Vec<Step>, String> {
    let list = a
        .get("steps")
        .and_then(Value::as_array)
        .ok_or("`steps` missing")?;
    if list.is_empty() {
        return Err("at least one step required".into());
    }
    let single = list.len() == 1;
    list.iter()
        .map(|s| {
            let mut st = Step::new(collect_values(show, s)?);
            st.hold_ms =
                opt_u64(s, "hold_ms").map_or(if single { 5000 } else { 1000 }, |v| v as u32);
            st.fade_ms = opt_u64(s, "fade_ms").unwrap_or(0) as u32;
            Ok(st)
        })
        .collect()
}

/// Merge `patch` recursively into `base`.
fn merge(base: &mut Value, patch: &Value) {
    match (base, patch) {
        (Value::Object(b), Value::Object(p)) => {
            for (k, v) in p {
                merge(b.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (b, p) => *b = p.clone(),
    }
}

fn merged<T: serde::Serialize + serde::de::DeserializeOwned>(
    v: &T,
    patch: &Value,
) -> Result<T, String> {
    let mut j = serde_json::to_value(v).map_err(|e| e.to_string())?;
    merge(&mut j, patch);
    serde_json::from_value(j).map_err(|e| format!("Invalid settings: {e}"))
}

// ---------- Outputs ----------

fn fixture_json(show: &Show, id: FixtureId) -> Value {
    let Some(f) = show.fixture(id) else {
        return Value::Null;
    };
    let ty = show.type_of(f);
    json!({
        "id": f.id.0,
        "name": f.name,
        "type_key": f.type_key,
        "model": ty.map(|t| format!("{} {} ({})", t.manufacturer, t.model, t.mode)),
        "universe": f.universe + 1,
        "address": f.address,
        "channels": ty.map(|t| t.channels.iter().enumerate().map(|(i, c)| json!({ "index": i, "name": c.name, "kind": format!("{:?}", c.kind) })).collect::<Vec<_>>()),
        "plan": show.plan_position(f.id),
    })
}

fn scene_kind(s: &dmxcraft_core::Scene) -> &'static str {
    if s.timeline().is_some() {
        "SUPER SCENE"
    } else if !s.effects.is_empty() {
        "FX"
    } else if s.steps.len() > 1 {
        "SEQUENCE"
    } else {
        "STATIC"
    }
}

fn scene_brief(e: &Engine, s: &dmxcraft_core::Scene) -> Value {
    json!({
        "id": s.id.0,
        "name": s.name,
        "bank": s.zone.map(|z| z.0),
        "kind": scene_kind(s),
        "duration_ms": s.duration_ms(),
        "steps": s.steps.len(),
        "effects": s.effects.iter().map(|f| kind_name(f.kind)).collect::<Vec<_>>(),
        "running": e.is_active(s.id),
        "hidden": s.hidden,
    })
}

fn overview(e: &Engine) -> R {
    let show = e.show();
    Ok(json!({
        "show": show.name,
        "universes": show.universes,
        "fixtures": show.fixtures.len(),
        "banks": show.zones.iter().map(|z| json!({
            "id": z.id.0, "name": z.name, "hidden": z.hidden,
            "scenes": show.scenes.iter().filter(|s| s.zone == Some(z.id)).map(|s| json!({ "id": s.id.0, "name": s.name, "kind": scene_kind(s) })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "groups": show.groups.iter().map(|g| json!({ "id": g.id.0, "name": g.name, "fixtures": g.fixtures.iter().map(|f| f.0).collect::<Vec<_>>() })).collect::<Vec<_>>(),
        "running": e.active_scenes().iter().map(|s| s.0).collect::<Vec<_>>(),
        "master": e.master(),
        "blackout": e.blackout(),
        "speed": e.speed(),
    }))
}

fn list_scenes(e: &Engine, a: &Value) -> R {
    let bank = opt_u64(a, "bank").map(|b| ZoneId(b as u32));
    Ok(json!({
        "scenes": e.show().scenes.iter()
            .filter(|s| bank.is_none_or(|b| s.zone == Some(b)))
            .map(|s| scene_brief(e, s))
            .collect::<Vec<_>>()
    }))
}

/// Answer after a change: short (summary) or with `verbose` the whole scene
/// – large scenes would otherwise blow up the answer.
fn changed_scene(e: &Engine, id: SceneId, a: &Value) -> R {
    if a.get("verbose").and_then(Value::as_bool).unwrap_or(false) {
        return get_scene(e, id);
    }
    let s = e.show().scene(id).ok_or("Scene not found")?;
    let mut out = scene_brief(e, s);
    out["values_per_step"] = json!(s.steps.iter().map(|st| st.values.len()).collect::<Vec<_>>());
    out["settings"] = serde_json::to_value(&s.settings).unwrap_or_default();
    out["effect_list"] = json!(s
        .effects
        .iter()
        .filter(|f| f.kind != EffectKind::SuperScene && f.kind != EffectKind::Steps)
        .map(|f| json!({ "id": f.id, "kind": kind_name(f.kind), "fixtures": f.fixtures.len() }))
        .collect::<Vec<_>>());
    if let Some(tl) = s.timeline() {
        out["timeline"] = json!({
            "length_ms": tl.length_ms(),
            "clips": tl.tracks.iter().map(|t| t.clips.len()).sum::<usize>(),
        });
    }
    Ok(out)
}

fn get_scene(e: &Engine, id: SceneId) -> R {
    let show = e.show();
    let s = show.scene(id).ok_or("Scene not found")?;
    let ch_name = |v: &ChannelValue| {
        show.channel_def(v.channel_ref())
            .map(|c| (c.name.clone(), format!("{:?}", c.kind)))
            .unwrap_or_default()
    };
    let mut out = scene_brief(e, s);
    out["color"] = s
        .color
        .map(|c| json!(format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])))
        .unwrap_or(Value::Null);
    out["loop"] = json!(s.loop_mode == LoopMode::Loop);
    out["settings"] = serde_json::to_value(&s.settings).unwrap_or_default();
    out["step_list"] = json!(s.steps.iter().map(|st| json!({
        "hold_ms": st.hold_ms,
        "fade_ms": st.fade_ms,
        "values": st.values.iter().map(|v| {
            let (n, k) = ch_name(v);
            json!({ "fixture": v.fixture.0, "channel": v.channel, "name": n, "kind": k, "value": v.value })
        }).collect::<Vec<_>>(),
    })).collect::<Vec<_>>());
    out["effect_list"] = json!(s
        .effects
        .iter()
        .filter(|f| f.kind != EffectKind::SuperScene && f.kind != EffectKind::Steps)
        .map(|f| {
            let mut j = serde_json::to_value(f).unwrap_or_default();
            j["kind"] = json!(kind_name(f.kind));
            j
        })
        .collect::<Vec<_>>());
    if let Some(tl) = s.timeline() {
        out["timeline"] = timeline_json(show, tl);
    }
    Ok(out)
}

fn timeline_json(show: &Show, tl: &Timeline) -> Value {
    json!({
        "length_ms": tl.length_ms(),
        "bpm": tl.bpm,
        "tracks": tl.tracks.iter().enumerate().map(|(i, t)| json!({
            "index": i, "name": t.name, "muted": t.muted,
            "clips": t.clips.iter().map(|c| {
                let content = match &c.content {
                    ClipContent::Scene(id) => json!({ "scene": id.0, "scene_name": show.scene(*id).map(|s| s.name.clone()) }),
                    ClipContent::Audio { path, .. } => json!({ "audio_path": path }),
                    ClipContent::Video { path, obs_scene, source, .. } => json!({ "video_path": path, "obs_scene": obs_scene, "obs_source": source }),
                    ClipContent::Obs(ObsAction::Scene(scene)) => json!({ "obs_scene": scene }),
                    ClipContent::Obs(ObsAction::Source { scene, source, visible }) => json!({ "obs_scene": scene, "obs_source": source, "obs_visible": visible }),
                };
                json!({
                    "id": c.id, "start_ms": c.start_ms, "duration_ms": c.duration_ms,
                    "fade_in_ms": c.fade_in_ms, "fade_out_ms": c.fade_out_ms,
                    "level": c.level, "speed": c.speed, "content": content,
                })
            }).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
    })
}

fn dmx_output(e: &Engine, a: &Value) -> R {
    let frame = e.render_frame();
    if let Ok(list) = id_list(a, "fixtures") {
        let show = e.show();
        return Ok(
            json!({ "fixtures": existing_fixtures(show, &list)?.into_iter().map(|id| {
            let f = show.fixture(id);
            let n = show.fixture_channels(id);
            json!({
                "id": id.0,
                "name": f.map(|f| f.name.clone()),
                "values": (0..n).map(|c| e.output_value(&frame, ChannelRef { fixture: id, channel: c })).collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>() }),
        );
    }
    let u = opt_u64(a, "universe").unwrap_or(1).max(1) as usize - 1;
    let start = opt_u64(a, "start")
        .unwrap_or(1)
        .clamp(1, UNIVERSE_SIZE as u64) as usize;
    let count = opt_u64(a, "count").unwrap_or(UNIVERSE_SIZE as u64) as usize;
    let data = frame.universe(u).ok_or("Universe does not exist")?;
    let end = (start - 1 + count).min(UNIVERSE_SIZE);
    Ok(json!({ "universe": u + 1, "start": start, "values": data[start - 1..end].to_vec() }))
}

// ---------- Fixtures ----------

/// Fixture type from the show, built-in types or the library.
fn find_type(show: &Show, key: &str, user: &[FixtureType]) -> Option<FixtureType> {
    if let Some(t) = show.fixture_type(key) {
        return Some(t.clone());
    }
    if let Some(t) = user.iter().find(|t| t.key == key) {
        return Some(t.clone());
    }
    if let Some(t) = dmxcraft_core::library::builtin_fixture_types()
        .into_iter()
        .find(|t| t.key == key)
    {
        return Some(t);
    }
    dmxcraft_ofl::Library::embedded()
        .fixtures
        .iter()
        .flat_map(|f| f.modes.iter())
        .find(|m| m.key == key)
        .cloned()
}

fn search_library(a: &Value, user: &[FixtureType]) -> R {
    let q = req_str(a, "query")?;
    let limit = opt_u64(a, "limit").unwrap_or(15).clamp(1, 50) as usize;
    let words: Vec<String> = q.split_whitespace().map(str::to_lowercase).collect();
    let mode_json = |m: &FixtureType| json!({ "type_key": m.key, "mode": m.mode, "channels": m.channels.len(), "kinds": m.channels.iter().map(|c| format!("{:?}", c.kind)).collect::<Vec<_>>() });
    let mut hits: Vec<Value> = dmxcraft_core::library::builtin_fixture_types()
        .iter()
        .filter(|t| {
            let hay = format!("{} {} {} generic generisch", t.manufacturer, t.model, t.mode).to_lowercase();
            words.iter().all(|w| hay.contains(w))
        })
        .map(|t| json!({ "manufacturer": t.manufacturer, "model": t.model, "modes": [mode_json(t)] }))
        .collect();
    // custom profiles (Profile Builder) first
    let mut own: Vec<Value> = Vec::new();
    for t in user {
        let hay = format!("{} {} {} eigene own", t.manufacturer, t.model, t.mode).to_lowercase();
        if words.iter().all(|w| hay.contains(w)) {
            own.push(json!({ "manufacturer": t.manufacturer, "model": t.model, "own_profile": true, "modes": [mode_json(t)] }));
        }
    }
    own.append(&mut hits);
    let mut hits = own;
    for f in dmxcraft_ofl::Library::embedded().search(&q, limit) {
        hits.push(json!({
            "manufacturer": f.manufacturer,
            "model": f.name,
            "categories": f.categories,
            "modes": f.modes.iter().map(mode_json).collect::<Vec<_>>(),
        }));
    }
    hits.truncate(limit);
    Ok(json!({ "results": hits }))
}

fn add_fixtures(e: &mut Engine, a: &Value, user: &[FixtureType]) -> R {
    let key = req_str(a, "type_key")?;
    let ty = find_type(e.show(), &key, user).ok_or(format!(
        "Fixture type “{key}” not found – use search_fixture_library first"
    ))?;
    let count = opt_u64(a, "count").unwrap_or(1).clamp(1, 512) as usize;
    let base = opt_str(a, "name").unwrap_or_else(|| ty.model.clone());
    let universe = opt_u64(a, "universe").unwrap_or(1).max(1) as u8 - 1;
    let address = opt_u64(a, "address").unwrap_or(1).clamp(1, 512) as u16;
    e.show_mut().upsert_fixture_type(ty.clone());
    // Template: names “Base 1” … – paste_fixtures counts on and finds free
    // addresses
    let n0 = (1..)
        .find(|n| {
            !e.show()
                .fixtures
                .iter()
                .any(|f| f.name == format!("{base} {n}"))
        })
        .unwrap_or(1);
    let templates: Vec<dmxcraft_core::Fixture> = (0..count)
        .map(|i| dmxcraft_core::Fixture {
            id: FixtureId(0),
            name: format!("{base} {}", n0 + i),
            type_key: key.clone(),
            universe,
            address,
            plan_pos: None,
            plan_shape: None,
            plan_size: None,
            plan_rotation: None,
            limits: Default::default(),
            place3d: None,
        })
        .collect();
    let ids = e.show_mut().paste_fixtures(&templates, universe, address);
    if ids.is_empty() {
        return Err("No room in the patch".into());
    }
    let show = e.show();
    Ok(json!({
        "added": ids.iter().map(|id| fixture_json(show, *id)).collect::<Vec<_>>(),
        "missing": count - ids.len(),
    }))
}

fn update_fixture(e: &mut Engine, a: &Value) -> R {
    let id = FixtureId(req_u32(a, "id")?);
    let f = e.show().fixture(id).ok_or("Fixture not found")?.clone();
    if a.get("universe").is_some() || a.get("address").is_some() {
        let u = opt_u64(a, "universe").map_or(f.universe, |u| u.max(1) as u8 - 1);
        let ad = opt_u64(a, "address").map_or(f.address, |v| v as u16);
        e.show_mut()
            .repatch_fixture(id, u, ad)
            .map_err(|err| err.to_string())?;
    }
    let fm = e.show_mut().fixture_mut(id).ok_or("Fixture not found")?;
    if let Some(n) = opt_str(a, "name") {
        fm.name = n;
    }
    let x = a.get("plan_x").and_then(Value::as_f64);
    let y = a.get("plan_y").and_then(Value::as_f64);
    if x.is_some() || y.is_some() {
        let cur = fm.plan_pos.unwrap_or([0.5, 0.5]);
        fm.plan_pos = Some([
            x.map_or(cur[0], |v| v.clamp(0.0, 1.0) as f32),
            y.map_or(cur[1], |v| v.clamp(0.0, 1.0) as f32),
        ]);
    }
    Ok(fixture_json(e.show(), id))
}

// ---------- Scenes ----------

fn create_scene(host: &Host, e: &mut Engine, a: &Value) -> R {
    let bank = ZoneId(req_u32(a, "bank")?);
    e.show().zone(bank).ok_or("Bank not found")?;
    let steps = build_steps(e.show(), a)?;
    let multi = steps.len() > 1;
    let id = e
        .show_mut()
        .add_scene(req_str(a, "name")?, Some(bank), steps);
    apply_scene_fields(e, id, a)?;
    if multi {
        // Sequence with several steps = STEPS effect (as in the UI)
        let s = e.show_mut().scene_mut(id).ok_or("Scene missing")?;
        if !s.has_steps() {
            s.add_effect(EffectKind::Steps);
        }
    }
    if a.get("select").and_then(Value::as_bool).unwrap_or(true) {
        (host.notify)(AppEvent::SelectScene(id));
    }
    changed_scene(e, id, a)
}

fn apply_scene_fields(e: &mut Engine, id: SceneId, a: &Value) -> Result<(), String> {
    let s = e.show_mut().scene_mut(id).ok_or("Scene not found")?;
    if let Some(n) = opt_str(a, "name") {
        s.name = n;
    }
    if let Some(c) = a.get("color") {
        s.color = parse_color_opt(c)?;
    }
    if let Some(h) = a.get("hidden").and_then(Value::as_bool) {
        s.hidden = h;
    }
    if let Some(l) = a.get("loop").and_then(Value::as_bool) {
        s.loop_mode = if l { LoopMode::Loop } else { LoopMode::Once };
    }
    if let Some(p) = a.get("settings").filter(|v| v.is_object()) {
        s.settings = merged(&s.settings, p)?;
    }
    Ok(())
}

fn update_scene(e: &mut Engine, a: &Value) -> R {
    let id = SceneId(req_u32(a, "id")?);
    e.show().scene(id).ok_or("Scene not found")?;
    if a.get("steps").is_some() {
        if e.show().scene(id).is_some_and(|s| s.timeline().is_some()) {
            return Err("A SUPER SCENE has no steps – change blocks via add_timeline_clips".into());
        }
        let steps = build_steps(e.show(), a)?;
        let multi = steps.len() > 1;
        let s = e.show_mut().scene_mut(id).ok_or("Scene missing")?;
        s.steps = steps;
        if multi && !s.has_steps() && s.effects.is_empty() {
            s.add_effect(EffectKind::Steps);
        }
        if !multi {
            s.effects.retain(|f| f.kind != EffectKind::Steps);
        }
    }
    if let Some(b) = opt_u64(a, "bank") {
        let z = ZoneId(b as u32);
        e.show().zone(z).ok_or("Bank not found")?;
        e.show_mut().move_scenes(&[id], z, None);
    }
    apply_scene_fields(e, id, a)?;
    e.sync();
    changed_scene(e, id, a)
}

fn add_effect(e: &mut Engine, a: &Value) -> R {
    let sid = SceneId(req_u32(a, "scene")?);
    let kind = parse_kind(&req_str(a, "kind")?)?;
    if kind.exclusive() {
        return Err("Create Steps or a SuperScene via create_scene/create_super_scene".into());
    }
    let fixtures = existing_fixtures(e.show(), &id_list(a, "fixtures")?)?;
    let s = e.show_mut().scene_mut(sid).ok_or("Scene not found")?;
    if s.effects.iter().any(|f| f.kind.exclusive()) {
        return Err(
            "This scene has STEPS or is a SUPER SCENE – no further effects are possible there"
                .into(),
        );
    }
    if s.steps.is_empty() {
        s.steps.push(Step::new(Vec::new()));
    }
    let fid = s.add_effect(kind);
    let fx = s
        .effects
        .iter_mut()
        .find(|f| f.id == fid)
        .ok_or("Effect missing")?;
    fx.fixtures = fixtures;
    if let Some(p) = a.get("params").filter(|v| v.is_object()) {
        let mut p = p.clone();
        if let Some(o) = p.as_object_mut() {
            o.remove("id");
            o.remove("kind");
        }
        *fx = merged(fx, &p)?;
    }
    changed_scene(e, sid, a)
}

fn update_effect(e: &mut Engine, a: &Value) -> R {
    let sid = SceneId(req_u32(a, "scene")?);
    let fid = req_u32(a, "effect")?;
    let fixtures = match a.get("fixtures") {
        Some(_) => Some(existing_fixtures(e.show(), &id_list(a, "fixtures")?)?),
        None => None,
    };
    let s = e.show_mut().scene_mut(sid).ok_or("Scene not found")?;
    let fx = s
        .effects
        .iter_mut()
        .find(|f| f.id == fid)
        .ok_or("Effect not found")?;
    if let Some(f) = fixtures {
        fx.fixtures = f;
    }
    if let Some(p) = a.get("params").filter(|v| v.is_object()) {
        let mut p = p.clone();
        if let Some(o) = p.as_object_mut() {
            o.remove("id");
            o.remove("kind");
        }
        *fx = merged(fx, &p)?;
    }
    changed_scene(e, sid, a)
}

// ---------- SUPER SCENE ----------

fn create_super_scene(host: &Host, e: &mut Engine, a: &Value) -> R {
    let bank = ZoneId(req_u32(a, "bank")?);
    e.show().zone(bank).ok_or("Bank not found")?;
    let id = e
        .show_mut()
        .add_scene(req_str(a, "name")?, Some(bank), vec![Step::new(Vec::new())]);
    {
        let s = e.show_mut().scene_mut(id).ok_or("Scene missing")?;
        s.add_effect(EffectKind::SuperScene);
        let tl = s.timeline_mut().ok_or("Timeline missing")?;
        if let Some(b) = a.get("bpm").and_then(Value::as_f64) {
            tl.bpm = b.clamp(20.0, 400.0) as f32;
        }
        if let Some(names) = a.get("track_names").and_then(Value::as_array) {
            for (i, n) in names.iter().enumerate() {
                while tl.tracks.len() <= i {
                    tl.add_track(None);
                }
                tl.tracks[i].name = n.as_str().unwrap_or("").to_string();
            }
        }
        s.sync_timeline_length();
    }
    if let Some(clips) = a.get("clips").and_then(Value::as_array) {
        // invalid block: do not leave half a SUPER SCENE behind
        if let Err(err) = add_clips(host, e, id, &clips.clone()) {
            e.show_mut().remove_scene(id);
            return Err(err);
        }
    }
    (host.notify)(AppEvent::SelectScene(id));
    changed_scene(e, id, a)
}

fn add_clips(
    host: &Host,
    e: &mut Engine,
    sid: SceneId,
    clips: &[Value],
) -> Result<Vec<u32>, String> {
    // check everything first, then insert
    enum Part {
        Scene(dmxcraft_core::Scene),
        Audio(String),
        Video(String),
        Obs(ObsAction),
    }
    let mut parts = Vec::new();
    for c in clips {
        let track = opt_u64(c, "track").unwrap_or(0) as usize;
        let start = opt_u64(c, "start_ms").ok_or("`start_ms` missing")?;
        let part = if let Some(scene) = opt_u64(c, "scene") {
            let sc = e
                .show()
                .scene(SceneId(scene as u32))
                .ok_or(format!("Scene {scene} not found"))?;
            if sc.id == sid {
                return Err("A SUPER SCENE cannot contain itself".into());
            }
            Part::Scene(sc.clone())
        } else if let Some(p) = opt_str(c, "audio_path") {
            if !std::path::Path::new(&p).exists() {
                return Err(format!("Audio file not found: {p}"));
            }
            Part::Audio(p)
        } else if let Some(p) = opt_str(c, "video_path") {
            if !std::path::Path::new(&p).exists() {
                return Err(format!("Video file not found: {p}"));
            }
            Part::Video(p)
        } else if let Some(scene) = opt_str(c, "obs_scene") {
            match (
                opt_str(c, "obs_source"),
                c.get("obs_visible").and_then(Value::as_bool),
            ) {
                (Some(source), visible) => Part::Obs(ObsAction::Source {
                    scene,
                    source,
                    visible: visible.unwrap_or(true),
                }),
                (None, _) => Part::Obs(ObsAction::Scene(scene)),
            }
        } else {
            return Err("Block needs `scene`, `audio_path`, `video_path` or `obs_scene`".into());
        };
        parts.push((track, start, part, c));
    }
    let durations: Vec<Option<u64>> = parts
        .iter()
        .map(|(_, _, p, _)| match p {
            Part::Audio(path) => (host.audio_duration_ms)(path),
            Part::Video(path) => std::fs::File::open(path)
                .ok()
                .and_then(|mut f| dmxcraft_core::obs::mp4_duration_ms(&mut f)),
            _ => None,
        })
        .collect();
    let s = e.show_mut().scene_mut(sid).ok_or("Scene not found")?;
    let tl = s.timeline_mut().ok_or("Not a SUPER SCENE")?;
    let mut ids = Vec::new();
    for ((track, start, part, c), len) in parts.into_iter().zip(durations) {
        let dur = opt_u64(c, "duration_ms").or(len).unwrap_or(10_000);
        let id = match part {
            Part::Scene(sc) => tl.add_scene_clip(track, &sc, start),
            Part::Audio(path) => tl.add_clip(
                track,
                ClipContent::Audio {
                    path,
                    offset_ms: 0,
                    length_ms: len.unwrap_or(0),
                    grid: None,
                    segment_grids: Vec::new(),
                    triggers: Vec::new(),
                },
                start,
                dur,
            ),
            Part::Video(path) => tl.add_clip(
                track,
                ClipContent::Video {
                    path,
                    offset_ms: 0,
                    length_ms: len.unwrap_or(0),
                    obs_scene: opt_str(c, "obs_scene").unwrap_or_default(),
                    source: opt_str(c, "obs_source").unwrap_or_default(),
                },
                start,
                dur,
            ),
            Part::Obs(action) => tl.add_clip(
                track,
                ClipContent::Obs(action),
                start,
                opt_u64(c, "duration_ms").unwrap_or(1000),
            ),
        };
        if let Some(cl) = tl.clip_mut(id) {
            if let Some(d) = opt_u64(c, "duration_ms") {
                cl.duration_ms = d.max(40);
            }
            if let Some(v) = opt_u64(c, "fade_in_ms") {
                cl.fade_in_ms = v as u32;
            }
            if let Some(v) = opt_u64(c, "fade_out_ms") {
                cl.fade_out_ms = v as u32;
            }
            if let Some(v) = c.get("level").and_then(Value::as_f64) {
                cl.level = v.clamp(0.0, 1.0) as f32;
            }
            if let Some(v) = c.get("speed").and_then(Value::as_f64) {
                cl.speed = v.clamp(0.05, 20.0) as f32;
            }
        }
        ids.push(id);
    }
    s.sync_timeline_length();
    Ok(ids)
}

/// Base64 (standard alphabet, with padding) for images in the MCP result.
pub(crate) fn base64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for c in data.chunks(3) {
        let b = [c[0], *c.get(1).unwrap_or(&0), *c.get(2).unwrap_or(&0)];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for i in 0..4 {
            if i <= c.len() {
                out.push(T[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {

    #[test]
    fn base64_matches() {
        assert_eq!(super::base64(b""), "");
        assert_eq!(super::base64(b"f"), "Zg==");
        assert_eq!(super::base64(b"fo"), "Zm8=");
        assert_eq!(super::base64(b"foo"), "Zm9v");
        assert_eq!(super::base64(b"foobar"), "Zm9vYmFy");
    }

    use super::*;

    fn host() -> Host {
        let mut show = Show::new("Test", 1);
        for t in dmxcraft_core::library::builtin_fixture_types() {
            show.upsert_fixture_type(t);
        }
        Host::headless(Engine::new(show))
    }

    fn ok(h: &Host, name: &str, a: Value) -> Value {
        call(h, name, &a).unwrap_or_else(|e| panic!("{name}: {e}"))
    }

    #[test]
    fn obs_action_clips() {
        let h = host();
        let bank = ok(&h, "create_bank", json!({"name":"B"}))["bank"]
            .as_u64()
            .unwrap();
        let id = ok(
            &h,
            "create_super_scene",
            json!({"name":"S","bank":bank,"clips":[
                {"obs_scene":"Countdown","start_ms":0},
                {"obs_scene":"Live","obs_source":"camera","obs_visible":false,"start_ms":2000}
            ]}),
        )["id"]
            .as_u64()
            .unwrap();
        let sc = ok(&h, "get_scene", json!({"id":id}));
        let clips = sc["timeline"]["tracks"][0]["clips"]
            .as_array()
            .unwrap()
            .clone();
        assert_eq!(clips[0]["content"]["obs_scene"], "Countdown");
        assert_eq!(clips[1]["content"]["obs_visible"], false);
    }

    #[test]
    fn failed_super_scene_leaves_nothing() {
        let h = host();
        let bank = ok(&h, "create_bank", json!({"name":"B"}))["bank"]
            .as_u64()
            .unwrap();
        let r = call(
            &h,
            "create_super_scene",
            &json!({"name":"S","bank":bank,"clips":[{"audio_path":"/does/not/exist.mp3","start_ms":0}]}),
        );
        assert!(r.is_err());
        let list = ok(&h, "list_scenes", json!({"bank":bank}));
        assert!(list["scenes"].as_array().unwrap().is_empty());
    }

    #[test]
    fn full_workflow() {
        let h = host();
        let added = ok(
            &h,
            "add_fixtures",
            json!({"type_key":"generic/rgbw-par-6ch","count":4,"name":"PAR"}),
        );
        let list = added["added"].as_array().unwrap();
        assert_eq!(list.len(), 4);
        assert_eq!(list[0]["name"], "PAR 1");
        assert_eq!(list[1]["address"], 7);
        let ids: Vec<u64> = list.iter().map(|f| f["id"].as_u64().unwrap()).collect();

        let bank = ok(&h, "create_bank", json!({"name":"Colors"}))["bank"]
            .as_u64()
            .unwrap();
        let red = ok(
            &h,
            "create_scene",
            json!({
                "name":"Red","bank":bank,
                "steps":[{"fixtures":ids,"set":{"Dimmer":255,"color":"#ff0000"}}],
                "settings":{"fade_in_ms":500},
                "verbose":true
            }),
        );
        assert_eq!(red["kind"], "STATIC");
        assert_eq!(red["settings"]["fade_in_ms"], 500);
        let red_id = red["id"].as_u64().unwrap();
        let vals = red["step_list"][0]["values"].as_array().unwrap();
        assert!(vals.iter().any(|v| v["kind"] == "Red" && v["value"] == 255));
        assert!(vals.iter().any(|v| v["kind"] == "Green" && v["value"] == 0));

        // Sequence with two steps
        let chase = ok(
            &h,
            "create_scene",
            json!({
                "name":"Wechsel","bank":bank,
                "steps":[
                    {"fixtures":ids,"set":{"Dimmer":255,"color":"#0000ff"},"hold_ms":500},
                    {"values":[{"fixture":ids[0],"channel":"Dimmer","value":0}],"fade_ms":200}
                ]
            }),
        );
        assert_eq!(chase["steps"], 2);
        // Short form of the values and short answer without the channel list
        let short = ok(
            &h,
            "create_scene",
            json!({
                "name":"Kurz","bank":bank,
                "steps":[{"values":[[ids[0],"Dimmer",128],[ids[1],0,7]]}]
            }),
        );
        assert_eq!(short["values_per_step"][0], 2);
        assert!(short.get("step_list").is_none());
        let full = ok(&h, "get_scene", json!({"id": short["id"]}));
        let v = full["step_list"][0]["values"].as_array().unwrap();
        assert!(v.iter().any(|x| x["kind"] == "Dimmer" && x["value"] == 128));
        assert!(v.iter().any(|x| x["channel"] == 0 && x["value"] == 7));
        assert_eq!(chase["effects"][0], "Steps");

        // Effect with parameters
        let tpl = ok(&h, "describe_effect", json!({"kind":"ColorFx"}));
        assert!(tpl["template"]["color"].is_object());
        let fx = ok(
            &h,
            "add_effect",
            json!({"scene":red_id,"kind":"color fx","fixtures":ids}),
        );
        assert_eq!(fx["kind"], "FX");
        let eid = fx["effect_list"][0]["id"].as_u64().unwrap();
        ok(
            &h,
            "update_effect",
            json!({"scene":red_id,"effect":eid,"fixtures":[ids[0], ids[1]]}),
        );

        // SUPER SCENE with two blocks
        let sup = ok(
            &h,
            "create_super_scene",
            json!({
                "name":"Show","bank":bank,
                "clips":[{"scene":red_id,"start_ms":0,"duration_ms":2000},{"track":1,"scene":chase["id"],"start_ms":1000}],
                "verbose":true
            }),
        );
        assert_eq!(sup["kind"], "SUPER SCENE");
        assert_eq!(sup["timeline"]["tracks"][1]["clips"][0]["start_ms"], 1000);
        assert_eq!(sup["timeline"]["tracks"][0]["clips"][0]["fade_in_ms"], 500);
        let sup_id = sup["id"].as_u64().unwrap();
        assert!(call(
            &h,
            "add_timeline_clips",
            &json!({"scene":sup_id,"clips":[{"scene":sup_id,"start_ms":0}]})
        )
        .is_err());

        // Playback and output
        ok(&h, "play_scene", json!({"id":red_id}));
        h.engine
            .lock()
            .unwrap()
            .tick(std::time::Duration::from_millis(600));
        let out = ok(&h, "get_dmx_output", json!({"fixtures":[ids[0]]}));
        assert!(out["fixtures"][0]["values"].as_array().unwrap().len() == 6);
        ok(&h, "stop_scene", json!({}));
        ok(
            &h,
            "set_live_values",
            json!({"fixtures":[ids[0]],"set":{"Dimmer":128}}),
        );
        assert_eq!(h.engine.lock().unwrap().live().len(), 1);

        // Overview and cleanup
        let ov = ok(&h, "get_overview", json!({}));
        assert_eq!(ov["fixtures"], 4);
        ok(&h, "delete_scenes", json!({"ids":[chase["id"]]}));
        let s = ok(&h, "get_scene", json!({"id":sup_id}));
        assert_eq!(
            s["timeline"]["tracks"][1]["clips"]
                .as_array()
                .unwrap()
                .len(),
            0
        );
        ok(&h, "delete_fixtures", json!({"ids":[ids[3]]}));
        assert_eq!(
            ok(&h, "list_fixtures", json!({}))["fixtures"]
                .as_array()
                .unwrap()
                .len(),
            3
        );
    }

    #[test]
    fn helpful_errors() {
        let h = host();
        assert!(call(
            &h,
            "create_scene",
            &json!({"name":"x","bank":99,"steps":[{}]})
        )
        .unwrap_err()
        .contains("Bank"));
        assert!(
            call(&h, "add_fixtures", &json!({"type_key":"does/not/exist"}))
                .unwrap_err()
                .contains("search_fixture_library")
        );
        let r = call(&h, "search_fixture_library", &json!({"query":"rgbw"})).unwrap();
        assert!(!r["results"].as_array().unwrap().is_empty());
    }
}
