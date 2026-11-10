# Keyboard and MIDI mapping

## Turning mapping on

There are two ways to turn mapping on:

* the two icons to the left of the lock in the title bar,
* the **Mappings** menu.

There is one mapping for the **keyboard** (red) and one for **MIDI** (violet).

## Mappable elements

All mappable elements are highlighted in color:

* **Scenes:** scene cards, bank chips (“All” and every bank) and the pause, back and forward buttons in the bank header.
* **Scene sidebar:**
  * the tabs,
  * play/pause,
  * the dimmer, speed, phase and size knobs,
  * start/end, forward/backward, the repeat mode,
  * the “Release at end” and “Flash mode” switches.
* **Programmer:**
  * the Edit, Blind and Live buttons,
  * beams off, on and center,
  * the two reset buttons,
  * the categories,
  * every fader,
  * the color presets.
* **Stage:**
  * every fixture in the plan,
  * the groups,
  * the FX button.

  For a fixture you choose under “Target” what the key does: toggle the fixture's selection or control a single channel, e.g. dimmer, pan or red. The channel is set as a live value, independent of what is currently selected. A knob (CC) sets the value directly. A key toggles between 0 and full, as a flash key only while it is held.
* **Global commands** via “+” in the window: blackout, master, stop all scenes, next/previous bank, selected scene.

Clicking a highlighted element does not trigger anything. It selects the element or creates a new mapping and waits for the input.

Two kinds of elements behave differently:

* **Scenes, banks and global commands** always work.
* **Interface elements** (programmer, sidebar, stage) act like a click or a slider movement. That only works while the element is shown. The programmer's dimmer fader therefore acts on the fixtures currently selected.

**Programmer faders** are mapped together with the fixtures currently selected: the mapping controls exactly their channel as a live value, even if the fixtures are no longer selected later (“Target: Fixed”). If you prefer a fader that always acts on the current selection, choose “Selected fixtures (follows the selection)” under “Target”. The same fader can be mapped several times with different fixture selections.

Sliders (faders, knobs) follow a MIDI controller (CC) directly. A key toggles a slider between 0 and full.

## Window

**Presets:**

* Clear,
* import and export as a `.dmxmap` file.

**List:**

* Columns category, target and key or MIDI input.
* Clicking the “Category” header reverses the sort order.
* Right-click or Delete removes the mapping.

**Settings** of the selected mapping:

* Input with the “Reassign” button.
* For scenes the target: the scene, the selected scene, no. n in this bank or no. n in the bank currently shown.
* Key mode and flash key.
* For MIDI: device, channel, note or CC and the number.

**MIDI output** (LEDs):

* device,
* channel and velocity for each of the states off and on.
* For known controllers the channel list shows the meaning, and the velocity can be picked as a color from the palette.
* **Preview:** while a mapping is selected, its key on the controller shows the state currently being edited (off/on).

The LED of a deleted or reassigned mapping is switched off. This also happens when all mappings are cleared or imported.

## Setting up MIDI controllers (Settings → MIDI)

All connected MIDI inputs are opened automatically; the search runs every 3 s. “+” sets up a device. Type and LED output are detected from its name:

* **Type:** generic controller or **Akai APC mini mk2**.
  * For the APC, DMXCraft knows the color table from Akai's published communication protocol: 128 colors, the velocity picks the color.
  * The MIDI channel picks the light behavior: 1–7 brightness 10–100 %, 8–11 pulsing, 12–16 blinking.
* **LED output:** newly learned keys send their feedback to this output. The “on” color is the scene color if it is known.
* **Clear LEDs on connect:** at program start and when the device is plugged in, all LEDs are switched off and the current states are sent afterwards. “Clear now” does the same immediately.

Mappings are saved in the show (`mappings`), controllers in the program settings. Mappings also work while the interface is locked. Keys are ignored while an input field has focus.

## Making new elements mappable

A new element registers with `mapping::button(ui, &resp, "key", on, || (category, target))`. A slider uses `mapping::fader(…)`. Knobs have `Knob::map(…)`, switches `Toggle::map(…)`.

The key must stay fixed because it is saved in the show.
