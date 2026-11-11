# OBS integration

DMXCraft controls OBS (version 28 and later) through its built-in WebSocket
server (obs-websocket 5). OBS plays the videos itself – DMXCraft only tells it
which file, from which position and at which speed.

## Setup

1. In OBS: *Tools → WebSocket Server Settings*, enable the server, note the
   port (default 4455) and the password.
2. In DMXCraft: *Edit → Settings → OBS*, turn on “Connect to OBS”, enter the
   address (`localhost` if OBS runs on the same computer), port and password.
   The data is saved with the show.
3. The connection state is shown in the settings and as “OBS” in the title
   bar (green = connected, red = disconnected; a click opens the settings).
   DMXCraft reconnects every few seconds after a drop.

## Video blocks

* Drag a video file (MP4, MOV, MKV, WebM …) onto the timeline of a SUPER SCENE
  or choose *Add video …* in the track context menu. The length is read from
  MP4/MOV.
* When playback reaches the block, DMXCraft puts the file into the media
  source (default “DMXCraft Video”, changeable per block) and starts it at the
  right position. If the source is missing in OBS, it is created in the
  block's OBS scene (otherwise in the current scene).
* Optionally OBS switches to a scene when the block starts (block properties
  → OBS scene).
* Pause, seeking with the ruler, speed and level (volume of the source)
  follow the timeline; drift of more than about 0.3 s is corrected. At the
  end of the block the source is stopped.
* Trimming from the left/right moves the start within the file, as for audio.

## OBS actions

Track context menu → *Add OBS action*. When the block is reached (only during
playback):

* **Switch scene** – e.g. jump to the video scene for a countdown.
* **Show / hide source** – toggle a source in a scene.

## Tips

* Video output to a projector: in OBS right-click the preview →
  *Fullscreen Projector*.
* The video's sound runs through OBS's audio mixer and therefore also ends up
  in the recording. For silent videos pull the block's level down to 0.

## Sound of videos

When a video block is added (or a SUPER SCENE with older video blocks is
shown), DMXCraft checks the file for a sound track (MP4/MOV with AAC and
other formats the audio decoder reads). If there is one, a linked sound
block is put on the media track below: DMXCraft plays the sound (sound
master, mute, fade out, MIDI faders, beat analysis and audio triggers work
as for any sound block) and OBS plays the picture muted. Linked blocks move,
trim, split, select and delete together; “Unlink video and sound” in the
block menu separates them (the video stays muted in OBS, the sound block
keeps playing in DMXCraft and can be moved or deleted on its own).
