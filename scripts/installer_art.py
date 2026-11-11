"""Generate the branded WiX installer bitmaps (crates/app/wix/Banner.bmp 493x58,
Dialog.bmp 493x312) and License.rtf from LICENSE.

Usage: python scripts/installer_art.py <repo root>  (needs Pillow)
"""
from PIL import Image, ImageDraw, ImageFont, ImageFilter
import sys
root = sys.argv[1]
icon = Image.open(f"{root}/crates/app/assets/icon-512.png").convert("RGBA")
font = f"{root}/crates/app/assets/fonts/NotoSans-SemiBold.ttf"
BG = (11, 14, 18); ACC = (25, 227, 240); DIM = (12, 92, 100)

def glow_panel(w, h):
    im = Image.new("RGB", (w, h), BG)
    # soft accent light from the top-left, like a beam
    g = Image.new("L", (w, h), 0)
    d = ImageDraw.Draw(g)
    for i in range(12):
        x = -40 + i * 6
        d.polygon([(x, -10), (x + 30, -10), (w + 60, h + 40), (w - 40, h + 40)], fill=10 + i * 4)
    g = g.filter(ImageFilter.GaussianBlur(18))
    beam = Image.new("RGB", (w, h), DIM)
    im = Image.composite(beam, im, g)
    return im

# Dialog (welcome/finish): left 164 px panel, rest white (text is drawn there)
W, H = 493, 312
dlg = Image.new("RGB", (W, H), (255, 255, 255))
panel = glow_panel(164, H)
ic = icon.resize((96, 96), Image.LANCZOS)
panel.paste(ic, ((164 - 96) // 2, 70), ic)
d = ImageDraw.Draw(panel)
f1 = ImageFont.truetype(font, 21); f2 = ImageFont.truetype(font, 11)
t = "DMXCraft"; tw = d.textlength(t, font=f1)
d.text(((164 - tw) / 2, 178), t, font=f1, fill=(232, 236, 240))
t2 = "DMX lighting control"; tw2 = d.textlength(t2, font=f2)
d.text(((164 - tw2) / 2, 208), t2, font=f2, fill=ACC)
d.rectangle([163, 0, 163, H], fill=ACC)
dlg.paste(panel, (0, 0))
dlg.save(f"{root}/crates/app/wix/Dialog.bmp")

# Banner (other pages): white, title text is drawn left by Windows; logo right
W, H = 493, 58
ban = Image.new("RGB", (W, H), (255, 255, 255))
strip = glow_panel(150, H)
ic = icon.resize((38, 38), Image.LANCZOS)
strip.paste(ic, (150 - 38 - 12, 10), ic)
ban.paste(strip, (W - 150, 0))
# soft edge between white and the strip
d = ImageDraw.Draw(ban)
for i in range(24):
    a = i / 24
    col = tuple(int(255 * (1 - a) + c * a) for c in BG)
    d.line([(W - 150 - 24 + i, 0), (W - 150 - 24 + i, H)], fill=col)
d.rectangle([0, H - 2, W, H], fill=ACC)
ban.save(f"{root}/crates/app/wix/Banner.bmp")
print("ok")

# License.rtf for the license page (plain text → RTF, Windows-1252 + \u escapes)
def rtf_escape(text):
    out = []
    for ch in text:
        o = ord(ch)
        if ch in "\\{}":
            out.append("\\" + ch)
        elif ch == "\n":
            out.append("\\par\n")
        elif o < 128:
            out.append(ch)
        else:
            out.append(f"\\u{o if o < 32768 else o - 65536}?")
    return "".join(out)

lic = open(f"{root}/LICENSE", encoding="utf-8").read()
rtf = ("{\\rtf1\\ansi\\ansicpg1252\\deff0{\\fonttbl{\\f0\\fswiss Segoe UI;}}"
       "\\viewkind4\\uc1\\pard\\f0\\fs16 " + rtf_escape(lic) + "}")
open(f"{root}/crates/app/wix/License.rtf", "w", encoding="ascii").write(rtf)
print("License.rtf ok")
