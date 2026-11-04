# ArchiForm DAE Parametric Converter

Offline, browser-based first-pass converter for SketchUp Collada (`.dae`) models. Start it manually by running `start_dae_converter.bat` in the repository root. The launcher opens a visible terminal running Python's standard-library HTTP server, bound only to `127.0.0.1`; close that terminal or press Ctrl+C when finished.

The browser reads the selected DAE locally. It displays its named scene groups and their geometry, allows free rotation, reads the DAE unit scale and lets the user adjust overall width, height and depth. Export preserves each named group's triangulated surface as a detailed `ParametricMesh` in the ArchiForm `.ccomp`; shared scale expressions resize all groups around the opening center and floor while retaining their relative positions.

## Current limits

- The source triangles and named groups are preserved; textures/material mapping, curved surface smoothing and hardware recognition are not yet transferred. Geometry scales proportionally on X, Y and Z, so changing proportions can stretch details.
- The generated door uses ArchiForm's `Door` component kind and a host opening. Review the output in ArchiForm before using it in a project.
- No network calls or remote AI are used. AI-assisted recognition can be added later behind an explicitly configured local or approved provider.
- The converter has been kept separate from the Rust application and its existing uncommitted Architecture work.
