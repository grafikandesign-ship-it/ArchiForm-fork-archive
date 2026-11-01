"""Extract the supplied light/dark ribbon SVGs from the icon suite HTML."""

from pathlib import Path
import html
import re
import sys

if len(sys.argv) != 2:
    raise SystemExit("usage: python scripts/extract_icon_suite.py path/to/code.html")
source = Path(sys.argv[1]).read_text(encoding="utf-8")
cards = re.findall(r'<article\b[^>]*data-name="([^"]+)"[^>]*>(.*?)</article>', source, re.S)
names = """wall door window floor stair roof room railing select column beam footing rebar analytical grid-beams pipe plumbing-fixture duct outlet circuit sizing move mirror offset trim rotate measure align floor-plan elevation section dimension text room-tag schedule view-3d shader visibility hide-isolate zoom-fit new open save ifc-import cad-import pdf-export svg-export settings project-info units levels global-materials phases macro""".split()
assert len(cards) == len(names) == 54
out = Path(__file__).resolve().parent.parent / "assets/icons/suite"
out.mkdir(parents=True, exist_ok=True)
for i, ((label, body), name) in enumerate(zip(cards, names, strict=True), 1):
    variants = re.findall(r'<svg\b.*?</svg>', body, re.S)
    assert len(variants) == 2, (i, label)
    for theme, svg in zip(("light", "dark"), variants, strict=True):
        svg = html.unescape(svg)
        svg = svg.replace("viewbox=", "viewBox=")
        svg = svg.replace("currentColor", "#1e293b" if theme == "light" else "#b8c4ff")
        svg = re.sub(r'\sclass="[^"]*"', "", svg)
        svg = svg.replace("<svg ", '<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" ' , 1)
        svg = re.sub(r'<!--.*?-->', "", svg, flags=re.S)
        path = out / f"{name}-{theme}.svg"
        path.write_text(svg.strip() + "\n", encoding="utf-8")
    print(f"{i:02} {name}: {html.unescape(label)}")
