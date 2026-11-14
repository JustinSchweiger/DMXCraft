# Touch surface

The **Touch** tab (at the top next to “Setup” and “Control”) holds a freely
designable control surface made of pages with a grid. Stage plan and
programmer stay visible below it. The surface is saved in the show (`touch`)
and can also be operated from a phone or tablet via the touch remote
(*Settings → Remote*).

## Adding elements

* **Alt + click** on a control places it on the current touch page:
  * scenes, bank chips, pause/step in the bank header,
  * groups, fixtures,
  * programmer faders,
  * buttons of the sidebar,
  * the **Sound** tab: player buttons, volume faders of the players, the
    master, crossfade and the pads. They work while another tab is shown.

  If there is no page yet, “Page 1” is created.
  * **Programmer faders** then control exactly the fixtures that were
    selected when the element was created.
  * **The programmer's color field or XY field** (Alt + click) creates a color
    wheel or XY pad for the selected fixtures.
* **Context menu** of a scene or of a fixture in the plan: “Add to touch
  surface”.
* **Edit → Add controls:**
  * label, button, fader, knob,
  * color wheel and XY pad, both for the current selection.

  A new button, fader or knob has no target yet (red dot). While it is
  selected, Alt + click on a control gives it that target.

## Editing

**Edit** at the top right shows the bar on the right:

* **Add:** button, fader, knob, label, color wheel, XY pad, live tracking.
* **Page** (when no element is selected):
  * background color,
  * background image (PNG/JPEG, saved in the show).
* **Grid:**
  * show,
  * snap,
  * step size 12–72 px.

  Without snapping, elements can be placed freely and may also overlap.
* **Element:**
  * label, text size, text color,
  * color (presets or a custom color),
  * icon from the icon library,
  * image,
  * size, flash key, “Take over selection” (color wheel/XY).

Elements can be

* dragged,
* resized with the handle at the bottom right,
* moved with the arrow keys,
* deleted with Delete.

The context menu shows the element's target (all channels for fixed
channels), plus duplicate, delete and color.

Pages:

* add with “+”,
* rename with a double-click,
* delete with a right-click.

The magnifiers change the view size (click the percentage: 100 %).

## Operating

* **Buttons** trigger their target like a key of the keyboard and MIDI
  mapping (see `MAPPING.md`). Active scenes and selected groups have a white
  frame.
* **Faders and knobs** set the value directly, e.g. master or fixed channels.
  Double-click: 0 ↔ full. The level appears in the element color.
* **Color wheel:** hue to the right, saturation downwards. It sets red, green
  and blue of the fixtures as live values.
* **XY pad:** sets pan and tilt as live values.
* **Live tracking:** the room from above with the targets of LIVE TRACKING
  (also in the web remote). Drag a target to move it, tap it to switch
  following on or off.

Controls of LIVE TRACKING, the SOUND tab, the programmer mode, “reset all”
and the scene sidebar (play/pause, start/end, direction, loop of the selected
scene) work whether they are shown or not. Other interface buttons (e.g.
programmer quick buttons) only work while the respective element is shown.
