# 3D visualizer

*Tools → 3D visualizer* opens the visualizer in its own window (e.g. on a
second screen; fullscreen via the button at the top right).

## Technology

* Rendering happens on the graphics card with **wgpu** – on Windows and Linux
  via **Vulkan** (otherwise DirectX 12), on macOS via Metal. The most powerful
  graphics card is chosen; its name and backend are shown at the bottom right
  of the image.
* Scene into an HDR target with 4× multisampling, all fixtures as spotlights
  with light spots on the surfaces, light beams added up in the haze, ACES
  tonemapping. Light beams end at the first obstacle.
* Shadows: the brightest fixtures get one shadow map each (depth texture
  array, soft filtering); structures cast shadows.
* Bloom: bright spots (lenses, light spots) glow softly (downsample/upsample
  in five steps).
* Quality levels: Low (60 % resolution, no anti-aliasing, 32 lights, no
  shadows/bloom), Medium (4×, 96 lights, 8 shadows at 512 px, bloom), High
  (256 lights, 16 shadows at 1024 px), Ultra (150 % resolution, 32 shadows).
* Values come directly from the DMX output: dimmer, RGB/W/amber/UV (also per
  cell for bars), strobe, pan/tilt (with fine channel) for moving heads.
  Moving heads hang by default (rotation 0 = beam down); pan and tilt turn
  the way real hanging heads do. Their angles come from the profile
  (*Physical*: pan and tilt range) and the output after the fixture limits,
  so a wrong tilt range shows up as beams that are too high or too low.
* **Value ranges** of the channels (from OFL, QLC+ and the Profile Builder):
  shutter open/closed and strobe rate, color wheel slots with their color,
  gobo slots. Without ranges it is approximated (strobe below 10 open, color
  wheel with eight typical colors, gobo with six patterns: star, dots, rings,
  breakup, bars, spiral – in the light spot and as shafts in the beam).
  Older shows get the ranges from the library or the custom profiles when
  they are opened (same key and same channel names).
* **Focus**: the focus channel sets a focus distance from 1.5 to 20 m; light
  spot and gobo are sharp where the beam hits at that distance, soft
  otherwise.

## Venue

* Belongs to the show and can be saved as **`.dmxvenue`** (JSON) and loaded
  into other shows. Units: meters, y up, rotations in degrees (X, then Y,
  then Z).
* Shapes: box, plane, cylinder, truss (three-/four-point), groups.
* **Build**: add objects, select them in the object tree, change
  placement/size/color/opacity/gloss/glow; move objects and fixtures on the
  floor with the mouse (5 cm grid, Ctrl: free).
* **Layout**: on the left room, add and the object tree (groups can be
  expanded; a click in the image expands down to the object) or, in the
  *Fixtures* tab, the fixture list with search (Ctrl/Shift for
  multi-selection); docked on the right the properties of the selection in
  collapsible sections. Undo/redo at the top next to *Build/Live* (or
  Ctrl+Z/Ctrl+Y).
* **Folders**: in the object tree groups are folders – create them with “+”
  or a right-click, rename them (double-click or menu), sort objects in by
  dragging (their placement in the room is kept), “Selection into new
  folder”, sort by name. The fixture list has its own folders (only for
  sorting, saved in the venue), drag into a folder, sort by address or name.
* **Multi-selection** (Ctrl/Shift): handles and dragging act on all selected
  objects and fixtures; rotating turns each one in place.
* **Library**: bundled objects (riser, stage, stairs, DJ booth, speaker,
  subwoofer, truss tower and frame, screen, tables, bar, person) and your
  own: select an object or folder, “Save selection as custom object”
  (including models and images, in the config folder under `objects/`).
* **Images (textures)**: choose a bundled image in the material (wooden
  planks, stage floor, concrete, carpet, bricks, tiles, molton) or your own
  PNG/JPEG; tile size in meters; for folders it applies to everything inside.
  The color tints the image. glTF models bring their images (base color).
* **Handles** on the selection: the red/green/blue axis moves along the
  room's x/y/z, the three rings rotate around the room's x/y/z (5 cm or 5°
  grid, Ctrl: free) – also for objects in rotated groups and for fixtures.
* **Import custom 3D models** (glTF/GLB): geometry with all node transforms,
  the base colors and images of the materials are embedded in the show; the
  model stands with its bottom edge on the floor. Up to 2 million triangles.
* **Aiming fixtures**: aim at a target point (numbers or click in the image),
  straight down, hang on the nearest truss – each for all selected fixtures.
* Fixtures without their own 3D placement are placed automatically below the
  ceiling according to the stage plan.

## Camera points

On the right of the bar (next to fullscreen): “+” saves the current view as a
camera point (rename it right away, Enter). A click on a point – or keys
1–9 – moves the camera there smoothly; right-click: overwrite with the
current view, rename, delete; double-click: rename. The point where the
camera currently is is highlighted. If the points do not fit into the bar
(narrow window) they appear as a dropdown. Camera points belong to the venue
(saved in the show and in `.dmxvenue`) and can also be used by the AI
assistant (`set_visualizer`, `visualizer_screenshot` with `camera.point`).
Whenever the visualizer is opened (tab, own window, another show loaded) the
camera starts at the first camera point; without one at the room overview.

## Import from Easy View 2

Expects the file as XML. Room, structures (boxes, planes), trusses (as own
trusses), environment and the placement of the fixtures are taken over;
fixtures are matched to the fixtures in the patch by universe and address.
For fixtures without a counterpart DMXCraft suggests a type from the library
(manufacturer and model from the Easy View library name, channel count if
given, otherwise generic) and creates them at the right address on request.
Easy View models, textures and the embedded Daslight fixture library are not
taken over.

## Next steps

Color slots and gobo images per fixture (Profile Builder), cells from the
physical fixture data, normal/roughness maps.
