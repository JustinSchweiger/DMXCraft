# OBS integration

DMXCraft controls OBS (version 28 and later) through its built-in WebSocket
server (obs-websocket 5). Video blocks reach OBS in one of two ways
(*Settings → OBS → Videos*):

* **Browser source, in sync with DMXCraft** (default) – DMXCraft serves a
  small page on this computer (`http://127.0.0.1:8421/`, the next free port
  if it is taken) that OBS shows in a browser source. The page plays the
  video blocks and follows DMXCraft's clock, so picture, sound and light stay
  together. No window opens and DMXCraft does not decode the video – OBS's
  browser does (with hardware decoding).
* **Media source, played by OBS** – OBS plays the file in a media source and
  DMXCraft only tells it which file, from which position and at which speed;
  larger drift is corrected by seeking.

## Setup

1. In OBS: *Tools → WebSocket Server Settings*, enable the server, note the
   port (default 4455) and the password.
2. In DMXCraft: *Edit → Settings → OBS*, turn on “Connect to OBS”, enter the
   address (`localhost` if OBS runs on the same computer), port and password.
   The data is saved with the show.
3. The connection state is shown in the settings and as “OBS” in the title
   bar (green = connected, red = disconnected; a click opens the settings).
   DMXCraft reconnects every few seconds after a drop.

## Browser source

* When the show has video blocks and OBS is connected, DMXCraft creates the
  browser source (default “DMXCraft Video Output”, canvas size) in the
  current OBS scene; position, crop or move it into other scenes like any
  source. After a restart DMXCraft points it to the current address again.
  The source can also be added by hand with the address from the settings.
* Where no video plays the page is transparent; fade-in and fade-out of a
  block fade the picture. Blocks on upper tracks lie on top, so overlapping
  blocks cross-fade.
* Small deviations are evened out by playing up to 8 % faster or slower,
  only jumps of more than 0.4 s seek. Blocks that start within 3 s are
  loaded ahead and wait at their first frame.
* *Delay picture* shifts the picture against sound and light (positive =
  later), e.g. when the sound output has a long buffer.
* The browser plays H.264/AAC in MP4, VP8/VP9 in WebM and similar web
  formats. Files it cannot play are listed in the settings; convert them to
  H.264 MP4 (e.g. with HandBrake or ffmpeg).
* The page only answers on this computer and only serves the video files of
  the running show.

## Video blocks

* Drag a video file (MP4, MOV, MKV, WebM …) onto the timeline of a SUPER SCENE
  or choose *Add video …* in the track context menu. The length is read from
  MP4/MOV.
* Media source mode: when playback reaches the block, DMXCraft puts the file
  into the media source (default “DMXCraft Video”, changeable per block) and starts it at the
  right position. If the source is missing in OBS, it is created in the
  block's OBS scene (otherwise in the current scene).
* Optionally OBS switches to a scene when the block starts (block properties
  → OBS scene).
* Pause, seeking with the ruler, speed and level (volume of the source)
  follow the timeline; in media source mode drift of more than about 0.6 s
  is corrected. At the
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
as for any sound block) and OBS shows the picture muted. Linked blocks move,
trim, split, select and delete together; “Unlink video and sound” in the
block menu separates them (the video stays muted in OBS, the sound block
keeps playing in DMXCraft and can be moved or deleted on its own).
