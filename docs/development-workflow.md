# ArchiForm development workflow

The active feature group is **Architecture**. Work on walls, openings, stairs, roofs and related architectural controls stays in the same development version until the group is reviewed and accepted.

## During development

- Keep the version in `Cargo.toml` and `Cargo.lock` unchanged. Do not create a release tag or publish an update after each feature.
- Test a ready change with the **Windows portable** build only. Build `cc_app` and `cc_cli` in release mode with `cargo build --release --locked -p cc_app -p cc_cli`; package or copy `archiform.exe` as `ArchiForm.exe` with `ccad.exe` into the local portable folder. Do not build an MSI, DMG, Debian package or AppImage for development testing.
- Run focused local checks relevant to the change. A push to `sketchdown/workflow-foundation` runs formatting only; it does not launch the cross-platform test matrix or IFC validation.
- Keep each testable change in a signed-off commit. Publishing the development branch does not publish an application release.

## Closing a feature group

After the user confirms that the group is complete, run the full CI manually with `workflow_dispatch`. Resolve any failures, then bump the version once, update the changelog and publish a version tag. The tag triggers the multi-platform release workflow and makes the new version available to the app updater.

## ArchiForm repository separation

ArchiForm uses the original ConstructCAD repository as a reference only. Do not merge `upstream/main` automatically into the ArchiForm development branch. Selective upstream changes may be reviewed and ported when they preserve ArchiForm's interface and workflow. The baseline and the Architecture completion gate are documented in [the detachment plan](archiform-detachment.md).
