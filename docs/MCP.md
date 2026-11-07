# KI-Assistent (MCP)

DMXCraft enthält einen Server für das **Model Context Protocol (MCP)**. Damit
kann ein KI-Assistent wie Claude die laufende Show lesen und bedienen.

## Einschalten

*Bearbeiten → Einstellungen → KI-Assistent*: „MCP-Server einschalten“.
Der Server lauscht nur lokal auf `http://127.0.0.1:7373/mcp` (Port
einstellbar). Anfragen aus Webseiten (fremder `Origin`) werden abgewiesen.

## Verbinden

**Claude Code**

```
claude mcp add --transport http dmxcraft http://127.0.0.1:7373/mcp
```

**Claude Desktop** – in `claude_desktop_config.json` (der genaue Pfad zur
Programmdatei steht in den Einstellungen zum Kopieren bereit):

```json
{
  "mcpServers": {
    "dmxcraft": { "command": "/pfad/zu/dmxcraft", "args": ["mcp"] }
  }
}
```

`dmxcraft mcp` öffnet kein Fenster, sondern ist eine stdio-Brücke zum
laufenden DMXCraft. DMXCraft muss also gestartet und der Server
eingeschaltet sein.

**Andere Clients**: „Streamable HTTP“ mit der URL oben (JSON-Antworten, keine
Sitzungen).

## Werkzeuge

| Bereich | Werkzeuge |
|---|---|
| Überblick | `get_overview`, `list_fixtures`, `list_scenes`, `get_scene`, `get_dmx_output` |
| Patch | `search_fixture_library`, `add_fixtures`, `update_fixture`, `delete_fixtures` |
| Gruppen, Bänke | `create_group`, `delete_group`, `create_bank`, `update_bank`, `delete_bank` |
| Szenen | `create_scene`, `update_scene`, `delete_scenes`, `select_scene` |
| Effekte | `describe_effect`, `add_effect`, `update_effect`, `remove_effect` |
| SUPER SCENE | `create_super_scene`, `add_timeline_clips`, `remove_timeline_clips` |
| Wiedergabe | `play_scene`, `stop_scene`, `pause_scene`, `set_output` (Master, Blackout, Tempo) |
| Live | `set_live_values`, `clear_live_values` |
| Datei | `save_show` |

Kanalwerte gibt man entweder einzeln an (`{fixture, channel, value}`, Kanal
als Index, Art wie `"Red"` oder Name) oder bequem je Kanalart für mehrere
Geräte: `{"fixtures": [1,2,3], "set": {"Dimmer": 255, "color": "#ff8800"}}`.

Effekte werden über ihre JSON-Einstellungen gesteuert: `describe_effect`
liefert die Vorlage, `add_effect`/`update_effect` übernehmen davon nur die
angegebenen Felder.

## Sicherheit

* Nur lokal erreichbar (127.0.0.1).
* Ist DMXCraft gesperrt (Schloss), sind nur lesende Werkzeuge erlaubt.
* Jede Änderung erscheint sofort in der Oberfläche und lässt sich mit Strg+Z
  rückgängig machen; gespeichert wird erst mit `save_show`.

## Aufbau

`crates/mcp`: `tools.rs` (Werkzeuge auf der Engine), `protocol.rs`
(JSON-RPC/MCP), `server.rs` (HTTP), `bridge.rs` (stdio-Brücke).
Die App startet/stoppt den Server in `crates/app/src/mcp.rs`.
