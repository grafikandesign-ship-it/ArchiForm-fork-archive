# Ribbon icon suite

The 54 icon designs supplied in `stitch_archicad_modern_icon_suite/code.html` are embedded as 108 SVG assets in `assets/icons/suite`: one light and one dark variant per design. To regenerate them from a copy of the source HTML, run:

```sh
python scripts/extract_icon_suite.py path/to/code.html
```

`cc_app::icons::Icon::suite_source` maps the designs to ribbon and command palette icons. It selects the theme variant at paint time and retains the existing badge overlay for create, import, export and remove actions. Other interface icons continue to use the existing monochrome SVGs in `assets/icons/ui`.

The source contains one icon for several related commands (for example, both mirror commands use the mirror design). The room-tag design is currently used for the structural tag command, and the 3D material design for graphic overrides. These mappings can be refined when those commands receive dedicated designs.
