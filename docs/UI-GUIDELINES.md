# DMXCraft – UI guidelines

Binding rules for every change to the interface (`crates/app`).
Goal: a calm, dense desk layout that can be operated in the dark during a
show – with its own look (do not copy graphics, icons or color schemes of
other programs).

## 1. Principles

1. **Live first.** Everything needed during a show lives under CONTROL and is
   reachable with one click. Setting up (patch, editing the stage plan,
   zones) belongs under SETUP.
2. **Fill the space.** Panels, faders and canvases use the available space
   completely (`ui.available_height()`/`available_width()`); no fixed
   heights except for header rows and buttons.
3. **One control, one meaning.** The same actions look the same everywhere
   (e.g. “release” always as right-click + context menu entry).
4. **Make state visible.** Active scenes, programmer channels that are set,
   the selection and the output state are recognizable without hovering.
5. **No surprises during shows.** Never put destructive actions on a simple
   left-click without confirmation, and never next to frequently used buttons.

## 2. Design tokens (`ui/theme.rs`)

Colors, spacing, radii and font sizes come **exclusively** from `ui::theme`.
No `Color32::from_rgb(..)` literals in views.

Surface and text colors depend on the chosen **theme** (Midnight, Studio, … –
Settings → Customize) and are read as functions: `theme::bg()`,
`theme::surface()`, `theme::text()` … New themes are another `Palette` entry
in `theme.rs` plus a `Theme` variant with label and description; give each
theme its own accent so they are clearly distinct. The user can override the
accent and the work-area background (`theme::Overrides`) – never cache theme
colors, always read them through the functions. On work areas use
`canvas_line()`, `canvas_dim()`, `handle()`, `lamp()` …, which follow the
brightness of the (possibly overridden) canvas. Signal colors (`DANGER`,
`OK`, `WARN`) stay the same in all themes.

**Terms:** the columns under CONTROL are called **banks** in the interface
(`Zone` in the code). Inline renaming (chips, patch) focuses the field only
at the start – never call `request_focus()` every frame.

**Language:** every visible text goes through `t("…")` or
`tr!("… {name}", name = x)` (English source, German translation in
`i18n/de.rs`). Do not assemble texts that need translating with `format!` –
name the placeholders so the translation can change their order. Show data
(fixture names, scenes, zones) is not translated.

| Token | Use |
|---|---|
| `bg()` | window background, canvas |
| `surface()` | panels |
| `raised()` | cards, fader tracks, buttons |
| `raised_hover()` | hover state |
| `border()` | 1 px edges of cards/panels |
| `accent()` | selection, active values, focus (cyan from the logo) |
| `TEXT` / `TEXT_DIM` / `TEXT_FAINT` | primary, secondary, disabled text |
| `DANGER`, `OK`, `WARN` | blackout/error, output active, warning |

Spacing on a 4 px grid: `SP_XS=4`, `SP_S=8`, `SP_M=12`, `SP_L=16`.
Radii: `R_S=4` (small buttons), `R_M=8` (cards, faders).
Fonts: `FS_CAPTION=10.5` (section titles, UPPER CASE), `FS_BODY=13`,
`FS_TITLE=15`.

## 3. Layout

* **Custom title bar** (`titlebar.rs`, window without an operating system
  bar): menu on the left (File, Edit, View, Tools, Mappings, Help) · modes
  Setup/Control/Touch/Sound; in the middle “DMXCraft · show name” (= file
  name, `*` for unsaved changes); on the right the load bar “LOAD” (status
  dot, details in the tooltip), lock button, window buttons. No fps or
  fixture/scene counts.
* **No status bar.** Short messages (`AppState::info/error`) appear on the
  right of the title bar and fade out after 5 s (errors 10 s); update notices
  too.
* **Borderless:** areas touch each other and are only separated by 1 px
  divider lines (panel borders). Inner padding of an area `SP_XS`–`SP_S`, no
  outer frames or rounding around whole areas (canvas, grid).
* CONTROL: scene grid at the top (flexible), stage plan + programmer at the
  bottom (height via splitter, 40 % by default).
* **No area headings** (“Stage”, “Programmer”, “Patch” …) – they cost space
  and say nothing new. A toolbar above an area uses
  `widgets::section_header(ui, "", …)` (actions on the right). Where a label
  is needed (e.g. “GROUPS |” before pills), always use
  `theme::heading_text()`.
* **Lock:** while the interface is locked (`AppState::locked`) only CONTROL
  is available; setup, scene/zone editing, context menus for changes,
  settings and opening shows are off. New controls that change something in
  the patch or in scenes must respect `locked`.
* Content has `SP_S` padding; `SP_XS` between cards.

## 4. Components (`ui/widgets/`)

Reusable elements are their own types with `impl egui::Widget` or a
`show(self, ui) -> Response` method. They know **neither** the engine nor
the app state – only their input values.

* **Fader** (programmer): status dot at the top (color of the mode,
  clickable = store/release the channel), channel number, icon of the channel
  kind (name only as a tooltip on the icon), value or “OFF” (click = type in,
  drag up/down = change), track with handle. Mouse wheel ±2, double-click
  0 ↔ full, right-click menu (release, 0/25/50/75 %/full). In all categories
  except “All channels” one fader controls the channel kind of **all**
  selected fixtures.
* **SceneCard**: name in the scene color, type in upper case (STATIC,
  SEQUENCE, FX), the selection strip on the right (click = select, drag =
  move to another bank), main area = start/stop. Bar at the bottom: run time
  or ramp while fading in/out; small icons for sequence and flash.
* **Knob**: label below, value **only while turning** as a hint at the mouse
  pointer; accent in the color of the object (e.g. scene).
* **Time field** (`widgets::time_field`): `00h00m00s00` (hours only where it
  makes sense, e.g. duration), every number by dragging up/down, hundredths in
  steps of 4.
* **Dialog** (`widgets::Dialog`): **every** window goes through this, never
  `egui::Window` directly. Header: title centered in the heading font, close
  icon on the right. Content on `SURFACE`. Action buttons (Cancel, Close,
  Save …) **always in the footer at the bottom right**, the main action far
  right in `ACCENT` (also via Enter), Esc closes.
  Only focus the first field via the `first` flag – never `request_focus()`
  every frame, otherwise Tab does not work.
* **Windows with tabs** (model: `settings_window.rs`): `Dialog` with
  `.flush()` and a fixed `.height()`, tab bar on the left (`BG`, active tab
  `SURFACE` with an accent strip, icon + text), content on the right
  scrollable. The window keeps its size when switching tabs.
* **Forms** (`widgets::form` + `Form::row`): label on the left (`TEXT_DIM`,
  fixed column width, no colon), control on the right at full width, all rows
  the same height (`INPUT_H`).
* **Input fields** always via `widgets::text_field()` (text vertically
  centered, fixed height, background `BG`), never `TextEdit::singleline`
  directly.
* **Menus** (`widgets::menu`): main menu and **all** context menus only with
  `MenuItem` (icon column, text, shortcut muted on the right, arrow for
  submenus), `menu::item`, `menu::sub`, `menu::separator`, `menu::title`;
  every menu starts with `menu::begin`. Never `ui.button`/`ui.menu_button`
  in menus. Icons where they help; the top level of the main menu without an
  icon column (`plain()`). Delete directly and in red (`danger()`), without
  confirmation – Ctrl+Z undoes it; fixtures additionally via Delete. Clicks on
  disabled entries do nothing and keep the menu open
  (`menu::context`/`menu::dropdown`, never `resp.context_menu`). Do not add
  placeholder entries for features that do not exist yet. Shortcut texts come
  from `titlebar::keys`. “…” at the end means: a dialog opens; files
  **always** via the system dialog (`rfd`, own thread), never via an own path
  field.
* **Toggle** (switch): for all on/off options – **no checkboxes**.
* **RangeSlider**: vertical slider with lower and upper bound (e.g. dimmer
  limit).
* **LimitPad**: XY pad with handles for ranges (pan/tilt limits).
* **SegmentedTabs**: for mode and category switches.
* **XyPad**, **Swatch** (color swatch), **StageCanvas** (stage plan in the
  select/edit modes).

## 5. Interaction

* Every command is reachable through the main menu; frequent ones also
  directly in the view or via shortcut (Ctrl+S, Ctrl+O, F11, B).
* Right-clicking fixtures opens the same menu everywhere
  (`views/fixture_menu.rs`).
* Hover: `RAISED_HOVER`; pressed/active: muted `ACCENT`; selection: 2 px
  border in `ACCENT`.
* Multi-selection: Ctrl/Cmd or Shift + click, dragging a box.
* Dragging only starts after the egui threshold; hits are determined at the
  **press point** (`press_origin`), not at the current position.
* Context menus for rare actions (edit, delete, release).
* **No hint or empty-state texts** (“Select a fixture …”, “No scene
  selected”, “Select fixtures and use + …”, “no channels in this category”):
  if there is nothing, the area stays empty. Help belongs in tooltips, if
  needed on an info icon (`Icon::Info`).
* Elements that appear depending on the selection (e.g. DIP switches) must
  not change the height of their row.
* All areas of a page can be resized at their dividers. Widgets only set the
  mouse cursor when they really need it (e.g. grab while moving) – never a
  blanket `Default`, otherwise the resize cursor of the dividers disappears.
* Tools and selections with few fixed options (shapes, arrange, repeat) as
  icon buttons with a tooltip instead of dropdowns.
* **Delete** acts on the last clicked element (`AppState::focus`: scenes,
  bank, effect) – never on something else that happens to be selected too.
* **Inline renaming:** the field appears in place, the whole name is
  selected (`dialog::select_all`, also after a double-click), Enter applies,
  Esc cancels. New objects (scene, bank, group) start right in renaming.
* **Drag & drop** where it is natural: scenes by their strip into other
  banks, fixtures from the library into the patch, tracks between playlists.
  The target shows an insertion mark or free/occupied (green/red), a label
  without line breaks at the mouse pointer.
* **Sidebars** (e.g. scene properties): label on the left, control
  **right-aligned**; dropdowns with one-word options (Off/All/Bank …). Long
  lists to choose from (scenes, banks) open their own window with a column
  per bank and multi-selection.
* Controls of a scene use the **scene color** as the accent (exception:
  play/pause).
* Values you need to read (times, run time) are large enough and
  high-contrast; the current time on the left in the object color, the total
  time on the right in the text color.
* Stage plan: fixtures without light are black (like a closed dimmer), strobe
  blinks, every fixture shape is distinguishable.
* Window and area sizes are saved (eframe persistence).

## 6. Text & icons

* Language: English source, short, imperative verbs (“Save”, “Release”);
  every text also gets a German translation.
* Only characters the built-in font renders reliably: Latin letters, digits,
  common punctuation, `·`, `–`, `“”`, `„“`. **No emojis or arrow/symbol
  characters** (▶ ❚ ● … etc.) in text.
* **Icons** come uniformly from the Phosphor icon font (MIT,
  `assets/fonts/Phosphor*.ttf`) via `widgets::Icon` – add new icons there as
  a variant with the matching glyph. Icons must fit their function (e.g. a
  gear for settings, not a magnifier). Draw by hand only if there is no
  matching glyph.
* Lighting terms (EDIT, LIVE, BLIND, FX, STEPS …) stay as they are in both
  languages.

## 7. UI architecture

* Every view is a type that implements the `views::View` trait and keeps its
  own UI state as fields.
* Shared state (selection, messages, paths) lives in `state::AppState`;
  views get it via `ViewCtx` (`engine`, `state`, `frame`).
* Views change the show only through `Engine`/`Show` methods from
  `dmxcraft-core`; logic that does not look like UI belongs in the core and
  gets tests.

## 8. Checks before every commit

1. `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`,
   `cargo test --workspace`.
2. Start the app under Xvfb and check the affected views with screenshots
   (do the faders fill the height? cut-off texts? missing glyphs?).
3. Run `graphify update .` so the code graph stays up to date.
