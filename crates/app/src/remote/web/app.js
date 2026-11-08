// DMXCraft-Fernbedienung: zeigt die Seiten des Touch-Modus und bedient sie.
// Zeichnet wie die Desktop-Oberfläche (views/touch.rs, paint_item).
"use strict";

const UNIT = 72, GAP = 6;
const de = (navigator.language || "de").toLowerCase().startsWith("de");
const T = de ? {
  connecting: "Verbinde …", offline: "Verbindung zu DMXCraft unterbrochen", since: "Seit {s} s – verbinde automatisch neu …",
  pin: "PIN eingeben", wrongPin: "PIN falsch", empty: "Diese Show hat noch keine Touch-Seiten.\nIn DMXCraft im Modus „Touch“ anlegen.",
  loading: "Lade Layout …",
} : {
  connecting: "Connecting …", offline: "Connection to DMXCraft lost", since: "For {s} s – reconnecting automatically …",
  pin: "Enter PIN", wrongPin: "Wrong PIN", empty: "This show has no touch pages yet.\nCreate them in DMXCraft in “Touch” mode.",
  loading: "Loading layout …",
};

const $ = (id) => document.getElementById(id);
const cv = $("cv"), g = cv.getContext("2d"), main = $("main");
let pin = localStorage.getItem("dmxcraft-pin") || "";
let layout = null, rev = -1, state = null;
let pageId = Number(localStorage.getItem("dmxcraft-page")) || null;
let scale = 1, offX = 0, offY = 0, dpr = 1;
// Kleinste Darstellung: darunter wird die Seite verschoben statt verkleinert
const MIN_SCALE = 0.7;
// darunter (passend eingepasst) wird auf schmalen Bildschirmen neu angeordnet
const FLOW_BELOW = 0.75;
let flow = null;
let panX = 0, panY = 0, contentW = 0, contentH = 0, pan = null;
let online = null, pinNeeded = false;  // null = noch unbekannt
const images = new Map();          // id -> Image
const pointers = new Map();        // pointerId -> {item, ...}
const local = new Map();           // item id -> {v|rgb|xy, until}
const pending = new Map();         // item id -> cmd
let flushTimer = null, dirty = true;

// ------------------------------------------------------------ Netzwerk

// Ohne Antwort so lange gilt die Verbindung als getrennt. Ohne eigenes
// Zeitlimit würde der Browser bei weggefallenem Rechner bis zum TCP-Timeout
// warten (auf Handys oft 1–2 Minuten).
const REQUEST_TIMEOUT = 1500;
const OFFLINE_AFTER = 2500;
let lastOk = performance.now(), offlineSince = 0;

function api(path, opts = {}) {
  const headers = Object.assign({}, opts.headers || {});
  if (pin) headers["X-Pin"] = pin;
  const ctrl = new AbortController();
  const timer = setTimeout(() => ctrl.abort(), REQUEST_TIMEOUT);
  return fetch(path, Object.assign({}, opts, { headers, cache: "no-store", signal: ctrl.signal }))
    .finally(() => clearTimeout(timer));
}

async function loadLayout() {
  const r = await api("/api/layout");
  if (r.status === 401) { askPin(!!pin); return false; }
  const j = await r.json();
  if (!j.layout) return false;
  rev = j.rev;
  layout = j.layout;
  applyTheme(layout.theme);
  $("name").textContent = layout.name || layout.show || "DMXCraft";
  document.title = (layout.name || "DMXCraft") + " – Touch";
  if (!layout.pages.some((p) => p.id === pageId)) pageId = layout.pages.length ? layout.pages[0].id : null;
  buildTabs();
  layoutCanvas();
  return true;
}

async function poll() {
  try {
    if (pinNeeded) return;
    const r = await api("/api/state");
    lastOk = performance.now();
    if (r.status === 401) { askPin(!!pin); return; }
    const j = await r.json();
    setOnline(true);
    if (!layout || j.layout !== rev) {
      if (!(await loadLayout()) && !layout) message(T.loading);
    }
    if (j.values) { state = j; dirty = true; }
  } catch (e) {
    // einzelne Aussetzer (WLAN) überbrücken, der Wächter entscheidet
    if (performance.now() - lastOk > OFFLINE_AFTER) setOnline(false);
  } finally {
    setTimeout(poll, document.hidden ? 1000 : online ? 120 : 700);
  }
}

// Wächter: auch eine hängende Anfrage fällt so nach kurzer Zeit auf
setInterval(() => {
  if (!document.hidden && !pinNeeded && performance.now() - lastOk > OFFLINE_AFTER) setOnline(false);
  if (!online) updateOffline();
}, 500);

// nach dem Aufwecken aus dem Hintergrund nicht sofort „getrennt“ melden
document.addEventListener("visibilitychange", () => {
  if (!document.hidden) lastOk = performance.now();
});

function send(cmd) {
  pending.set(cmd.item + ":" + cmd.t, cmd);
  if (!flushTimer) flushTimer = setTimeout(flush, 35);
}

function sendNow(cmd) {
  pending.set(cmd.item + ":" + cmd.t, cmd);
  flush();
}

function flush() {
  clearTimeout(flushTimer);
  flushTimer = null;
  if (!pending.size) return;
  const body = JSON.stringify([...pending.values()]);
  pending.clear();
  api("/api/cmd", { method: "POST", body, headers: { "Content-Type": "application/json" } })
    .catch(() => setOnline(false));
}

function setOnline(ok) {
  if (ok !== online) {
    online = ok;
    $("dot").classList.toggle("ok", ok);
    $("msg").classList.toggle("offline", !ok);
    if (!ok) {
      offlineSince = performance.now();
      // laufende Bedienung abbrechen, nichts nachsenden
      pointers.clear();
      pan = null;
      pending.clear();
      if (navigator.vibrate) navigator.vibrate([60, 60, 60]);
      dirty = true;
    }
  }
  if (!ok) updateOffline();
  else if (layout && layout.pages.length) message(null);
  else if (layout) message(T.empty);
}

function updateOffline() {
  const secs = Math.round((performance.now() - offlineSince) / 1000);
  message(T.offline + (secs >= 3 ? "\n" + T.since.replace("{s}", secs) : ""));
}

function message(text) {
  const m = $("msg");
  m.classList.toggle("show", !!text || pinNeeded);
  $("msgtext").textContent = text || "";
}

function askPin(wrong) {
  pinNeeded = true;
  $("pin").classList.add("show");
  message(wrong ? T.wrongPin : T.pin);
  $("pinval").value = "";
  $("pinval").focus();
}

$("pin").addEventListener("submit", (ev) => {
  ev.preventDefault();
  pin = $("pinval").value.trim();
  localStorage.setItem("dmxcraft-pin", pin);
  pinNeeded = false;
  $("pin").classList.remove("show");
  message(T.connecting);
  rev = -1;
  layout = null;
  lastOk = performance.now();
});

$("full").addEventListener("click", () => {
  const d = document.documentElement;
  if (document.fullscreenElement || document.webkitFullscreenElement) {
    (document.exitFullscreen || document.webkitExitFullscreen).call(document);
  } else if (d.requestFullscreen || d.webkitRequestFullscreen) {
    (d.requestFullscreen || d.webkitRequestFullscreen).call(d);
  }
});
if (!(document.documentElement.requestFullscreen || document.documentElement.webkitRequestFullscreen)) {
  $("full").style.display = "none";
}

// ------------------------------------------------------------ Darstellung

let th = { bg: "#16181c", surface: "#1d2025", raised: "#272b31", raised_hover: "#30353c",
  border: "#353a42", accent: "#3d8bfd", text: "#e8eaed", text_dim: "#9aa0a8", danger: "#e5484d" };

function applyTheme(t) {
  if (!t) return;
  th = t;
  const s = document.documentElement.style;
  s.setProperty("--bg", t.bg); s.setProperty("--surface", t.surface);
  s.setProperty("--raised", t.raised); s.setProperty("--border", t.border);
  s.setProperty("--accent", t.accent); s.setProperty("--text", t.text);
  s.setProperty("--dim", t.text_dim); s.setProperty("--danger", t.danger);
  s.setProperty("--on-accent", textOn(t.accent));
  document.querySelector('meta[name="theme-color"]').content = t.surface;
}

function buildTabs() {
  const tabs = $("tabs");
  tabs.innerHTML = "";
  for (const p of layout.pages) {
    const b = document.createElement("button");
    b.className = "tab" + (p.id === pageId ? " on" : "");
    b.textContent = p.name;
    b.onclick = () => {
      pageId = p.id;
      panX = panY = 0;
      localStorage.setItem("dmxcraft-page", p.id);
      buildTabs();
      layoutCanvas();
    };
    tabs.appendChild(b);
  }
  tabs.style.visibility = layout.pages.length > 1 ? "visible" : "hidden";
}

function page() {
  return layout && layout.pages.find((p) => p.id === pageId);
}

/** Seite in den sichtbaren Bereich einpassen. */
function layoutCanvas() {
  dpr = window.devicePixelRatio || 1;
  const w = main.clientWidth, h = main.clientHeight;
  cv.width = Math.round(w * dpr);
  cv.height = Math.round(h * dpr);
  cv.style.width = w + "px";
  cv.style.height = h + "px";
  const p = page();
  flow = null;
  if (p && p.items.length) {
    const right = Math.max(...p.items.map((i) => i.x + i.w)) * UNIT - GAP;
    const bottom = Math.max(...p.items.map((i) => i.y + i.h)) * UNIT - GAP;
    const pad = 8;
    scale = Math.max(Math.min((w - 2 * pad) / right, (h - 2 * pad) / bottom, 2.5), MIN_SCALE);
    const fit = Math.min((w - 2 * pad) / right, (h - 2 * pad) / bottom);
    if (fit < FLOW_BELOW && w < 760) {
      // zu klein (Handy hochkant): Elemente neu anordnen, senkrecht scrollen
      flow = reflow(p.items, w - 2 * pad);
      scale = 1;
      contentW = w;
      contentH = flow.height + 2 * pad;
    } else {
      contentW = right * scale + 2 * pad;
      contentH = bottom * scale + 2 * pad;
    }
  } else {
    scale = 1; contentW = contentH = 0;
  }
  placeContent();
}

/** Seite mittig oder (wenn größer als der Bildschirm) verschoben anzeigen. */
function placeContent() {
  const w = main.clientWidth, h = main.clientHeight, pad = 8;
  panX = clamp(panX, 0, Math.max(0, contentW - w));
  panY = clamp(panY, 0, Math.max(0, contentH - h));
  offX = contentW <= w ? (w - contentW) / 2 + pad : pad - panX;
  offY = contentH <= h ? (h - contentH) / 2 + pad : pad - panY;
  dirty = true;
}

window.addEventListener("resize", layoutCanvas);
window.addEventListener("orientationchange", () => setTimeout(layoutCanvas, 200));

/**
 * Handy-Ansicht: Elemente in Lesereihenfolge (oben links nach unten rechts)
 * auf ein Raster passend zur Bildschirmbreite packen. Halbe Zellen als
 * Raster: Schaltfläche/Drehregler 2×2, Fader 2×6, Überschrift volle Breite
 * × 1, Farbrad/XY volle Breite.
 */
function reflow(items, width) {
  const cols = Math.max(2, Math.round(width / 120));
  const n = cols * 2;                 // Halbzellen je Zeile
  const g = width / n;                // Kantenlänge einer Halbzelle
  const order = readingOrder(items);
  const used = [];                    // used[row][col]
  const free = (r, c, w, h) => {
    for (let y = r; y < r + h; y++)
      for (let x = c; x < c + w; x++) if (used[y] && used[y][x]) return false;
    return true;
  };
  const take = (r, c, w, h) => {
    for (let y = r; y < r + h; y++) {
      used[y] = used[y] || [];
      for (let x = c; x < c + w; x++) used[y][x] = true;
    }
  };
  const rects = new Map();
  let from = 0, bottom = 0;
  for (const i of order) {
    let w, h;
    switch (i.kind) {
      case "label": w = n; h = 1; break;
      case "fader": w = 2; h = 6; break;
      case "color": case "xy": w = n; h = Math.max(4, Math.round(n * 0.7)); break;
      default: w = i.w >= 2 * i.h && n >= 4 ? 4 : 2; h = 2;
    }
    let r = from, c = 0;
    search: for (;; r++) {
      for (c = 0; c + w <= n; c++) if (free(r, c, w, h)) break search;
    }
    take(r, c, w, h);
    // Reihenfolge erhalten: nicht vor die Zeile des letzten Elements springen
    from = w === n ? r + h : r;
    bottom = Math.max(bottom, r + h);
    rects.set(i.id, { x: c * g, y: r * g, w: w * g - GAP, h: h * g - GAP });
  }
  return { rects, height: bottom * g - GAP };
}

/**
 * Lesereihenfolge nach Blöcken (XY-Schnitt): erst an durchgehenden
 * waagerechten Lücken in Zeilen teilen, sonst an senkrechten in Spalten –
 * so bleiben z. B. nebeneinanderstehende Fader und Schaltflächengruppen
 * zusammen.
 */
function readingOrder(items) {
  if (items.length <= 1) return items;
  const cut = (lo, hi) => {
    // Schnittstellen: Kanten, die kein Element überspannt
    const edges = [...new Set(items.flatMap((i) => [lo(i), hi(i)]))].sort((a, b) => a - b);
    const groups = [];
    let rest = items;
    for (const e of edges) {
      if (rest.some((i) => lo(i) < e - 1e-3 && hi(i) > e + 1e-3)) continue;
      const before = rest.filter((i) => hi(i) <= e + 1e-3);
      if (before.length && before.length < rest.length) {
        groups.push(before);
        rest = rest.filter((i) => hi(i) > e + 1e-3);
      }
    }
    if (!groups.length) return null;
    groups.push(rest);
    return groups;
  };
  const groups = cut((i) => i.y, (i) => i.y + i.h) || cut((i) => i.x, (i) => i.x + i.w);
  if (!groups) return [...items].sort((a, b) => a.y - b.y || a.x - b.x);
  return groups.flatMap(readingOrder);
}

function rectOf(i) {
  if (flow) return flow.rects.get(i.id);
  return { x: i.x * UNIT, y: i.y * UNIT, w: i.w * UNIT - GAP, h: i.h * UNIT - GAP };
}

function image(id) {
  if (id == null) return null;
  let img = images.get(id);
  if (!img) {
    img = new Image();
    img.onload = () => { dirty = true; };
    img.src = "/img/" + id + (pin ? "?pin=" + encodeURIComponent(pin) : "");
    images.set(id, img);
  }
  return img.complete && img.naturalWidth ? img : null;
}

function rr(x, y, w, h, r) {
  g.beginPath();
  g.roundRect ? g.roundRect(x, y, w, h, r) : g.rect(x, y, w, h);
}

function cover(img, r, alpha) {
  const s = Math.max(r.w / img.naturalWidth, r.h / img.naturalHeight);
  const sw = r.w / s, sh = r.h / s;
  g.save();
  rr(r.x, r.y, r.w, r.h, 8);
  g.clip();
  g.globalAlpha = alpha;
  g.drawImage(img, (img.naturalWidth - sw) / 2, (img.naturalHeight - sh) / 2, sw, sh, r.x, r.y, r.w, r.h);
  g.restore();
}

function hexRgb(h) {
  return [parseInt(h.slice(1, 3), 16), parseInt(h.slice(3, 5), 16), parseInt(h.slice(5, 7), 16)];
}

function textOn(hex) {
  const [r, gg, b] = hexRgb(hex);
  return 0.299 * r + 0.587 * gg + 0.114 * b > 150 ? "#141414" : "#ffffff";
}

function dim(hex, f) {
  const [r, gg, b] = hexRgb(hex);
  return `rgb(${Math.round(r * f)},${Math.round(gg * f)},${Math.round(b * f)})`;
}

function font(size) { return `${size}px DMXText, system-ui, sans-serif`; }

/** Text umbrechen (wie egui-Layout mit Breite). */
function wrap(text, maxW) {
  const lines = [];
  for (const para of text.split("\n")) {
    let line = "";
    for (const word of para.split(" ")) {
      const t = line ? line + " " + word : word;
      if (line && g.measureText(t).width > maxW) { lines.push(line); line = word; } else line = t;
    }
    lines.push(line);
  }
  return lines;
}

function wrappedText(text, cx, cy, size, color, maxW, shadow) {
  g.font = font(size);
  g.textAlign = "center";
  g.textBaseline = "middle";
  const lines = wrap(text, maxW);
  const lh = size * 1.25;
  let y = cy - (lines.length - 1) * lh / 2;
  for (const l of lines) {
    if (shadow) { g.fillStyle = "rgba(0,0,0,0.78)"; g.fillText(l, cx + 1, y + 1); }
    g.fillStyle = color;
    g.fillText(l, cx, y);
    y += lh;
  }
}

function valueOf(i) {
  const l = local.get(i.id);
  if (l && l.v != null && (l.active || l.until > performance.now())) return l.v;
  return state && state.values[i.id] != null ? state.values[i.id] : 0;
}

function colorOf(i) {
  const l = local.get(i.id);
  if (l && l.rgb && (l.active || l.until > performance.now())) return l.rgb;
  return state && state.colors[i.id];
}

function xyOf(i) {
  const l = local.get(i.id);
  if (l && l.xy && (l.active || l.until > performance.now())) return l.xy;
  return state && state.xy[i.id];
}

function isHot(i) {
  for (const p of pointers.values()) if (p.item.id === i.id) return true;
  return false;
}

function draw() {
  requestAnimationFrame(draw);
  if (!dirty) {
    // lokale Werte laufen ab -> einmal neu zeichnen
    const now = performance.now();
    let expire = false;
    for (const l of local.values()) if (!l.active && l.until && l.until < now) { l.until = 0; expire = true; }
    if (!expire) return;
  }
  dirty = false;
  g.setTransform(1, 0, 0, 1, 0, 0);
  const p = page();
  g.fillStyle = p && p.background_color ? p.background_color : th.bg;
  g.fillRect(0, 0, cv.width, cv.height);
  if (!p) return;
  const bgImg = image(p.background);
  g.setTransform(dpr, 0, 0, dpr, 0, 0);
  if (bgImg) {
    const w = cv.width / dpr, h = cv.height / dpr;
    const s = Math.max(w / bgImg.naturalWidth, h / bgImg.naturalHeight);
    g.drawImage(bgImg, (w - bgImg.naturalWidth * s) / 2, (h - bgImg.naturalHeight * s) / 2,
      bgImg.naturalWidth * s, bgImg.naturalHeight * s);
  }
  g.setTransform(dpr * scale, 0, 0, dpr * scale, dpr * offX, dpr * offY);
  for (const i of p.items) paintItem(i, rectOf(i), isHot(i));
}

function paintItem(i, r, hot) {
  const base = i.color;
  const label = i.label || "";
  const auto = i.kind === "label" ? clamp(r.h * 0.42, 13, 48) : clamp(Math.min(r.w, r.h) * 0.16, 11, 22);
  const size = i.text_size > 0 ? i.text_size : auto;
  const accent = base || th.accent;
  const cx = r.x + r.w / 2, cy = r.y + r.h / 2;
  switch (i.kind) {
    case "label":
    case "button": {
      const on = i.kind === "button" && state && state.on.includes(i.id);
      let fill = base || (i.kind === "button" ? th.raised : null);
      if (fill) {
        g.fillStyle = hot ? dim(fill, 0.7) : fill;
        rr(r.x, r.y, r.w, r.h, 8);
        g.fill();
      }
      const img = image(i.image);
      if (img) cover(img, { x: r.x + 1, y: r.y + 1, w: r.w - 2, h: r.h - 2 }, hot ? 0.7 : 1);
      if (i.kind === "button") {
        if (on) {
          g.strokeStyle = "#ffffff"; g.lineWidth = 3;
          rr(r.x + 1.5, r.y + 1.5, r.w - 3, r.h - 3, 7);
          g.stroke();
          g.fillStyle = "#ffffff";
          rr(r.x + 8, r.y + 6, r.w - 16, 3, 1.5);
          g.fill();
        } else {
          g.strokeStyle = th.border; g.lineWidth = 1;
          rr(r.x + 0.5, r.y + 0.5, r.w - 1, r.h - 1, 8);
          g.stroke();
        }
      }
      const tc = i.text_color || (img ? "#ffffff" : fill ? textOn(fill) : th.text);
      let ty = cy;
      if (i.icon) {
        const isz = label ? Math.max(Math.min(r.w, r.h) * 0.32, 12) : Math.max(Math.min(r.w, r.h) * 0.5, 14);
        const icy = label ? cy - isz * 0.45 : cy;
        g.font = `${isz}px DMXIcons`;
        g.textAlign = "center";
        g.textBaseline = "middle";
        g.fillStyle = tc;
        g.fillText(i.icon, cx, icy);
        ty = cy + isz * 0.5;
      }
      if (label) wrappedText(label, cx, ty, size, tc, r.w - 12, img && !i.text_color);
      if (i.kind === "button" && !i.active) {
        g.fillStyle = th.danger;
        g.beginPath(); g.arc(r.x + r.w - 9, r.y + 9, 3.5, 0, Math.PI * 2); g.fill();
      }
      break;
    }
    case "fader": {
      const v = valueOf(i);
      g.fillStyle = th.raised; rr(r.x, r.y, r.w, r.h, 8); g.fill();
      const f = faderParts(r);
      const tc = i.text_color || th.text;
      g.font = font(16); g.textAlign = "center"; g.textBaseline = "middle"; g.fillStyle = tc;
      g.fillText(Math.round(v * 100), cx, r.y + 15);
      const slotH = f.track.h - 6;
      const slot = { x: cx - 3, y: f.track.y + (f.track.h - slotH) / 2, w: 6, h: slotH };
      g.fillStyle = th.bg; rr(slot.x, slot.y, slot.w, slot.h, 3); g.fill();
      const hy = f.travel.y + f.travel.h - f.travel.h * v;
      g.fillStyle = accent; rr(slot.x, hy, slot.w, slot.y + slot.h - hy, 3); g.fill();
      const hw = Math.min(r.w - 16, 40);
      g.fillStyle = th.raised_hover; rr(cx - hw / 2, hy - 17, hw, 34, 4); g.fill();
      g.strokeStyle = hot ? th.text : th.text_dim; g.lineWidth = 1.5;
      rr(cx - hw / 2 + 0.75, hy - 16.25, hw - 1.5, 32.5, 4); g.stroke();
      g.strokeStyle = th.text; g.lineWidth = 2.5;
      g.beginPath(); g.moveTo(cx - hw / 2 + 7, hy); g.lineTo(cx + hw / 2 - 7, hy); g.stroke();
      if (label) {
        g.font = font(Math.min(size, 14)); g.textBaseline = "bottom"; g.fillStyle = tc;
        const lines = wrap(label, r.w - 6);
        let y = r.y + r.h - 6 - (lines.length - 1) * Math.min(size, 14) * 1.25;
        for (const l of lines) { g.fillText(l, cx, y); y += Math.min(size, 14) * 1.25; }
      }
      break;
    }
    case "knob": {
      const v = valueOf(i);
      g.fillStyle = th.raised; rr(r.x, r.y, r.w, r.h, 8); g.fill();
      const tc = i.text_color || th.text;
      const availH = r.h - (label ? 22 : 8);
      const rad = Math.max(Math.min(availH, r.w - 12) * 0.5, 12);
      const kx = cx, ky = r.y + 4 + availH / 2;
      const start = Math.PI * 0.75, sweep = Math.PI * 1.5;
      const ring = clamp(rad * 0.14, 3, 7);
      g.lineWidth = ring; g.lineCap = "round";
      g.strokeStyle = th.bg;
      g.beginPath(); g.arc(kx, ky, rad - ring, start, start + sweep); g.stroke();
      if (v > 0.001) {
        g.strokeStyle = accent;
        g.beginPath(); g.arc(kx, ky, rad - ring, start, start + sweep * v); g.stroke();
      }
      g.lineCap = "butt";
      const body = rad - ring * 2.2;
      g.fillStyle = hot ? th.raised_hover : th.raised;
      g.beginPath(); g.arc(kx, ky, body, 0, Math.PI * 2); g.fill();
      g.strokeStyle = th.border; g.lineWidth = 1; g.stroke();
      const a = start + sweep * v;
      g.strokeStyle = th.text; g.lineWidth = 2.5;
      g.beginPath();
      g.moveTo(kx + Math.cos(a) * body * 0.35, ky + Math.sin(a) * body * 0.35);
      g.lineTo(kx + Math.cos(a) * body * 0.85, ky + Math.sin(a) * body * 0.85);
      g.stroke();
      g.font = font(12); g.textAlign = "center"; g.textBaseline = "top"; g.fillStyle = th.text_dim;
      g.fillText(Math.round(v * 100), kx, ky + body * 0.35 + 4);
      if (label) {
        g.font = font(Math.min(size, 14)); g.textBaseline = "middle"; g.fillStyle = tc;
        g.fillText(label, cx, r.y + r.h - 11);
      }
      break;
    }
    case "color": {
      panel(i, r);
      const f = contentRect(r, i);
      const grad = g.createLinearGradient(f.x, 0, f.x + f.w, 0);
      for (let k = 0; k <= 12; k++) grad.addColorStop(k / 12, `hsl(${k * 30},100%,50%)`);
      g.fillStyle = grad; g.fillRect(f.x, f.y, f.w, f.h);
      const wg = g.createLinearGradient(0, f.y, 0, f.y + f.h);
      wg.addColorStop(0, "rgba(255,255,255,0)"); wg.addColorStop(1, "rgba(255,255,255,1)");
      g.fillStyle = wg; g.fillRect(f.x, f.y, f.w, f.h);
      const c = colorOf(i);
      if (c) {
        const [h, s] = rgbToHs(c);
        const mx = f.x + h * f.w, my = f.y + (1 - s) * f.h;
        g.lineWidth = 2;
        g.strokeStyle = "#000"; g.beginPath(); g.arc(mx, my, 8, 0, Math.PI * 2); g.stroke();
        g.strokeStyle = "#fff"; g.beginPath(); g.arc(mx, my, 6, 0, Math.PI * 2); g.stroke();
      }
      break;
    }
    case "xy": {
      panel(i, r);
      const f = contentRect(r, i);
      g.fillStyle = th.bg; rr(f.x, f.y, f.w, f.h, 4); g.fill();
      g.strokeStyle = th.raised; g.lineWidth = 1;
      for (let k = 1; k < 4; k++) {
        g.beginPath();
        g.moveTo(f.x + f.w * k / 4, f.y); g.lineTo(f.x + f.w * k / 4, f.y + f.h);
        g.moveTo(f.x, f.y + f.h * k / 4); g.lineTo(f.x + f.w, f.y + f.h * k / 4);
        g.stroke();
      }
      const p = xyOf(i);
      if (p) {
        const mx = f.x + p[0] * f.w, my = f.y + p[1] * f.h;
        g.globalAlpha = 0.5; g.strokeStyle = accent;
        g.beginPath(); g.moveTo(mx, f.y); g.lineTo(mx, f.y + f.h); g.moveTo(f.x, my); g.lineTo(f.x + f.w, my); g.stroke();
        g.globalAlpha = 1;
        g.fillStyle = accent; g.beginPath(); g.arc(mx, my, 8, 0, Math.PI * 2); g.fill();
        g.strokeStyle = "#fff"; g.lineWidth = 2; g.stroke();
      }
      break;
    }
  }
}

function panel(i, r) {
  g.fillStyle = th.raised; rr(r.x, r.y, r.w, r.h, 8); g.fill();
  if (i.own_label) {
    g.font = font(14); g.textAlign = "left"; g.textBaseline = "middle";
    g.fillStyle = i.text_color || th.text;
    g.fillText(i.label, r.x + 10, r.y + 15);
  }
}

function contentRect(r, i) {
  const top = i.own_label ? 28 : 8;
  return { x: r.x + 8, y: r.y + top, w: r.w - 16, h: r.h - top - 8 };
}

function faderParts(r) {
  const track = { x: r.x, y: r.y + 30, w: r.w, h: r.h - 56 };
  const travel = { x: track.x, y: track.y + 17, w: track.w, h: track.h - 34 };
  return { track, travel };
}

function clamp(v, a, b) { return Math.min(Math.max(v, a), b); }

function rgbToHs([r, gg, b]) {
  r /= 255; gg /= 255; b /= 255;
  const mx = Math.max(r, gg, b), mn = Math.min(r, gg, b), d = mx - mn;
  let h = 0;
  if (d > 0) {
    if (mx === r) h = ((gg - b) / d) % 6;
    else if (mx === gg) h = (b - r) / d + 2;
    else h = (r - gg) / d + 4;
    h /= 6;
    if (h < 0) h += 1;
  }
  return [h, mx > 0 ? d / mx : 0];
}

function hsToRgb(h, s) {
  const f = (n) => {
    const k = (n + h * 6) % 6;
    return Math.round(255 * (1 - s * Math.max(0, Math.min(k, 4 - k, 1))));
  };
  return [f(5), f(3), f(1)];
}

// ------------------------------------------------------------ Bedienung

function toLayout(ev) {
  const b = cv.getBoundingClientRect();
  return { x: (ev.clientX - b.left - offX) / scale, y: (ev.clientY - b.top - offY) / scale };
}

function hit(pt) {
  const p = page();
  if (!p) return null;
  for (let k = p.items.length - 1; k >= 0; k--) {
    const i = p.items[k];
    const r = rectOf(i);
    if (pt.x >= r.x && pt.x <= r.x + r.w && pt.y >= r.y && pt.y <= r.y + r.h) return i;
  }
  return null;
}

function setLocal(i, patch) {
  const l = local.get(i.id) || {};
  Object.assign(l, patch);
  local.set(i.id, l);
  dirty = true;
}

cv.addEventListener("pointerdown", (ev) => {
  const pt = toLayout(ev);
  const i = hit(pt);
  if (!i || i.kind === "label") {
    // freie Fläche: Seite verschieben, wenn sie nicht ganz passt
    if (contentW > main.clientWidth || contentH > main.clientHeight) {
      cv.setPointerCapture(ev.pointerId);
      pan = { id: ev.pointerId, x: ev.clientX, y: ev.clientY, px: panX, py: panY };
    }
    return;
  }
  ev.preventDefault();
  cv.setPointerCapture(ev.pointerId);
  const r = rectOf(i);
  const ptr = { item: i, start: pt, last: pt, v0: valueOf(i), cx: ev.clientX, cy: ev.clientY };
  pointers.set(ev.pointerId, ptr);
  switch (i.kind) {
    case "button":
      if (scrollable()) {
        // erst kurz abwarten: Wischen über eine Schaltfläche scrollt die Seite
        ptr.pending = true;
        ptr.timer = setTimeout(() => {
          if (pointers.get(ev.pointerId) === ptr && ptr.pending) {
            ptr.pending = false;
            pressDown(i);
          }
        }, 130);
      } else {
        pressDown(i);
      }
      break;
    case "fader": {
      const f = faderParts(r);
      const v = valueOf(i);
      const hy = f.travel.y + f.travel.h - f.travel.h * v;
      // außerhalb des Griffs: Griff springt zum Finger
      if (Math.abs(pt.y - hy) > 20) {
        ptr.v0 = clamp((f.travel.y + f.travel.h - pt.y) / f.travel.h, 0, 1);
        setValue(i, ptr.v0);
      }
      setLocal(i, { active: true });
      break;
    }
    case "knob":
      setLocal(i, { active: true });
      break;
    case "color":
    case "xy":
      setLocal(i, { active: true });
      fieldMove(i, r, pt, true);
      break;
  }
  dirty = true;
});

cv.addEventListener("pointermove", (ev) => {
  if (pan && pan.id === ev.pointerId) {
    panX = pan.px - (ev.clientX - pan.x);
    panY = pan.py - (ev.clientY - pan.y);
    placeContent();
    return;
  }
  const ptr = pointers.get(ev.pointerId);
  if (!ptr) return;
  ev.preventDefault();
  if (ptr.pending && Math.hypot(ev.clientX - ptr.cx, ev.clientY - ptr.cy) > 10) {
    // doch gewischt: Schaltfläche nicht auslösen, sondern verschieben
    clearTimeout(ptr.timer);
    pointers.delete(ev.pointerId);
    pan = { id: ev.pointerId, x: ptr.cx, y: ptr.cy, px: panX, py: panY };
    panX = pan.px - (ev.clientX - pan.x);
    panY = pan.py - (ev.clientY - pan.y);
    placeContent();
    return;
  }
  const pt = toLayout(ev);
  const i = ptr.item, r = rectOf(i);
  ptr.last = pt;
  switch (i.kind) {
    case "fader": {
      const f = faderParts(r);
      setValue(i, clamp(ptr.v0 - (pt.y - ptr.start.y) / f.travel.h, 0, 1));
      break;
    }
    case "knob":
      setValue(i, clamp(ptr.v0 - (pt.y - ptr.start.y) / 160 + (pt.x - ptr.start.x) / 320, 0, 1));
      break;
    case "color":
    case "xy":
      fieldMove(i, r, pt, false);
      break;
  }
});

function release(ev) {
  if (pan && pan.id === ev.pointerId) pan = null;
  const ptr = pointers.get(ev.pointerId);
  if (!ptr) return;
  pointers.delete(ev.pointerId);
  const i = ptr.item;
  if (i.kind === "button") {
    if (ptr.pending) {
      // kurzer Tipp: Drücken und Loslassen zusammen senden
      clearTimeout(ptr.timer);
      if (navigator.vibrate) navigator.vibrate(8);
      sendBatch([{ t: "press", item: i.id, down: true }, { t: "press", item: i.id, down: false }]);
    } else {
      sendNow({ t: "press", item: i.id, down: false });
    }
  } else {
    // letzten Wert sofort senden, Anzeige kurz halten bis der Server ihn meldet
    flush();
    setLocal(i, { active: false, until: performance.now() + 600 });
  }
  dirty = true;
}
cv.addEventListener("pointerup", release);
cv.addEventListener("pointercancel", release);
cv.addEventListener("contextmenu", (ev) => ev.preventDefault());

function pressDown(i) {
  if (navigator.vibrate) navigator.vibrate(8);
  sendNow({ t: "press", item: i.id, down: true });
}

function scrollable() {
  return contentW > main.clientWidth + 1 || contentH > main.clientHeight + 1;
}

function sendBatch(cmds) {
  flush();
  api("/api/cmd", { method: "POST", body: JSON.stringify(cmds), headers: { "Content-Type": "application/json" } })
    .catch(() => setOnline(false));
}

function setValue(i, v) {
  v = Math.round(v * 1000) / 1000;
  setLocal(i, { v });
  send({ t: "value", item: i.id, v });
}

function fieldMove(i, r, pt, now) {
  const f = contentRect(r, i);
  const x = clamp((pt.x - f.x) / f.w, 0, 1), y = clamp((pt.y - f.y) / f.h, 0, 1);
  if (i.kind === "color") {
    const rgb = hsToRgb(Math.min(x, 0.9999), 1 - y);
    setLocal(i, { rgb });
    (now ? sendNow : send)({ t: "color", item: i.id, rgb });
  } else {
    setLocal(i, { xy: [x, y] });
    (now ? sendNow : send)({ t: "xy", item: i.id, x, y });
  }
}

// ------------------------------------------------------------ Start

message(T.connecting);
layoutCanvas();
document.fonts && document.fonts.load("16px DMXIcons").then(() => { dirty = true; });
document.fonts && document.fonts.load("16px DMXText").then(() => { dirty = true; });
requestAnimationFrame(draw);
poll();
