# AI assistant (MCP)

DMXCraft contains a server for the **Model Context Protocol (MCP)**. It lets
an AI assistant such as Claude read and operate the running show.

## Turning it on

*Edit → Settings → MCP*: “Enable MCP server”.
The server only listens locally on `http://127.0.0.1:7373/mcp` (the port is
configurable). Requests from web pages (foreign `Origin`) are rejected.

## Connecting

**Claude Code**

```
claude mcp add --transport http dmxcraft http://127.0.0.1:7373/mcp
```

**Claude Desktop** – in `claude_desktop_config.json` (the exact path of the
executable can be copied from the settings):

```json
{
  "mcpServers": {
    "dmxcraft": { "command": "C:\\Users\\…\\AppData\\Roaming\\DmxCraft\\dmxcraft-mcp.exe", "args": [] }
  }
}
```

`dmxcraft-mcp` is a small stdio bridge to the running DMXCraft (no window);
for another port use `"args": ["--port", "7474"]`. At startup DMXCraft puts a
copy of it into its config folder. Claude Desktop keeps the bridge open all
the time – because the copy is used, `dmxcraft.exe` and the build folder stay
free (rebuilding/updating on Windows). DMXCraft must be running and the
server enabled. `dmxcraft mcp` still works, but locks `dmxcraft.exe` while
the client is running.

**Other clients**: “Streamable HTTP” with the URL above (JSON responses, no
sessions).

## Tools

| Area | Tools |
|---|---|
| Overview | `get_overview`, `list_fixtures`, `list_scenes`, `get_scene`, `get_dmx_output` |
| Patch | `search_fixture_library`, `add_fixtures`, `update_fixture`, `delete_fixtures` |
| Groups, banks | `create_group`, `delete_group`, `create_bank`, `update_bank`, `delete_bank` |
| Scenes | `create_scene`, `update_scene`, `delete_scenes`, `select_scene` |
| Effects | `describe_effect`, `add_effect`, `update_effect`, `remove_effect` |
| SUPER SCENE | `create_super_scene`, `set_scene_group`/`delete_scene_group` (SCENE GROUPS: scenes with offsets, as one block via `group` in `add_timeline_clips`), `add_timeline_clips`, `update_timeline` (block times, fades, `prepare` = pre-position before a block, `allow_overlap`), `remove_timeline_clips` (scene blocks only on light tracks, audio/video/OBS only on media tracks; blocks: scene, `audio_path`, `video_path` for OBS, `obs_scene`/`obs_source` for OBS actions, see `docs/OBS.md`) |
| Playback | `play_scene`, `stop_scene`, `pause_scene`, `set_output` (master, blackout, tempo) |
| Live | `set_live_values`, `clear_live_values` |
| 3D venue | `get_venue`, `set_venue`, `add_venue_object`, `update_venue_object`, `delete_venue_objects`, `place_fixtures` (position, hang on a truss, aim at a point) |
| 3D visualizer | `get_visualizer`, `set_visualizer` (window, quality, camera), `visualizer_screenshot` (PNG image of the view with the current DMX output) |
| Profile Builder | `list_profiles`, `get_profile`, `describe_profile_format`, `save_profile`, `delete_profile` |
| File | `save_show` |

Channel values are given either individually (`{fixture, channel, value}`,
channel as index, kind such as `"Red"` or name) or conveniently per channel
kind for several fixtures: `{"fixtures": [1,2,3], "set": {"Dimmer": 255, "color": "#ff8800"}}`.
For many values (e.g. when copying whole scenes) there is the short form
`[fixture, channel, value]`, such as `"values": [[3, 1, 255], [3, "Dimmer", 128]]`.

`create_scene`, `update_scene`, `add_effect`, `update_effect` and
`create_super_scene` answer with a short summary (ID, bank, kind, number of
values per step, effects, length of the timeline). The whole scene with all
channel values comes from `get_scene` or `"verbose": true`.

**3D visualizer:** `visualizer_screenshot` renders the view directly on the
graphics card (the window does not need to be open) and returns it as an
image – so the assistant can check positions, aiming and colors. Camera via
`view` (`front`, `back`, `left`, `right`, `top`, `overview`) and/or `yaw`,
`pitch`, `distance`, `target`, `fov`; `front` looks from the audience side
(+z); saved camera points via `point` (name or number).
`set_visualizer` sets the same values for the window and creates or removes
camera points with `save_point`/`delete_point`.

**Profile Builder:** `save_profile` saves to the config folder (`fixtures/`)
like the Profile Builder, makes the ranges gapless and lists problems
(missing mode, angle for pan …) instead of saving. Custom profiles show up in
`search_fixture_library` (with `own_profile`) and can be patched with
`add_fixtures`; fixtures already patched are updated on save.
`describe_profile_format` returns a template.

Effects are controlled through their JSON settings: `describe_effect` returns
the template, `add_effect`/`update_effect` only take over the fields given.

## Security

* Reachable locally only (127.0.0.1).
* While DMXCraft is locked (lock icon), only read-only tools are allowed.
* Every change shows up in the interface immediately and can be undone with
  Ctrl+Z; nothing is saved until `save_show`.

## Structure

`crates/mcp`: `tools.rs` (tools on the engine), `protocol.rs` (JSON-RPC/MCP),
`server.rs` (HTTP), `bridge.rs` (stdio bridge). The app starts/stops the
server in `crates/app/src/mcp.rs`.
