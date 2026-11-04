# ArchiForm detachment plan

ArchiForm is now maintained as a fork with its own workflow. The original ConstructCAD repository remains a reference source, not an automatic parent for development merges.

## Baseline

- Upstream reference: `francisreboucas/constructcad`, `upstream/main` at `v0.9.1` (`14c9305`).
- ArchiForm repository: `grafikandesign-ship-it/constructcad`.
- Development branch: `sketchdown/workflow-foundation`.
- ArchiForm branch tip published before this plan: `60d201a`.
- The fork `main` branch has been fast-forwarded to the upstream `v0.9.1` baseline.

## Separation rules

1. Continue architecture work only on `sketchdown/workflow-foundation`.
2. Do not merge `upstream/main` automatically into the ArchiForm development branch.
3. Keep the `upstream` remote for read-only comparison and selective code review.
4. Bring an upstream change into ArchiForm only after checking that it does not undo the ArchiForm interface, naming, localization or workflow.
5. Publish only committed, portable-tested changes from the ArchiForm branch.
6. Do not bump the application version or publish a release until the Architecture group is accepted.

## Remaining Architecture gate

The first autonomous ArchiForm milestone is complete when these workflows are usable together:

- free mass and BIM wall creation;
- openings hosted by BIM walls and converted wall-shaped masses;
- doors and windows with editable parameters and plan/3D previews;
- layers and automatic architectural assignment;
- PDF, image and DXF underlays with calibration;
- properties, materials and wall compounds;
- IFC export for the architectural model;
- Windows portable build and a user walkthrough on a clean project.

After that walkthrough, create the first ArchiForm-only release baseline. Upstream changes are then evaluated one at a time instead of being merged as a branch update.

## Local work not yet published

The working tree may contain local changes for the current Architecture group. They must be reviewed and committed as ArchiForm changes before they are pushed. Updating the fork baseline must never discard those files.
