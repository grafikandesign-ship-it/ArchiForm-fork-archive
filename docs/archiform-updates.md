# ArchiForm updates

ArchiForm checks only the releases of `grafikandesign-ship-it/constructcad`. It never downloads an update until the user selects **Update Now**. The download and `SHA256SUMS.txt` must both come from that fork; the package is checked before installation.

## First installation on Windows

The development executable in `target/debug` is not a release installation and cannot update itself. Install the first ArchiForm release from the fork's [Releases page](https://github.com/grafikandesign-ship-it/constructcad/releases), using either the Windows MSI or the portable ZIP. The MSI installs under `Program Files/ArchiForm` with its own Windows product identity. Keep project `.ccad` files outside the program folder.

## Publish the next version

1. Work on the `sketchdown/workflow-foundation` branch, merge any desired upstream changes there, and run the relevant checks.
2. Update `[workspace.package] version` in `Cargo.toml`; refresh `Cargo.lock` with Cargo; add a matching `## [x.y.z]` entry to `CHANGELOG.md`.
3. Commit and push the branch. Then create a tag on that exact commit and push it:

   ```sh
   git tag v0.8.2
   git push origin v0.8.2
   ```

   Replace `0.8.2` with the version you set in both files. The release workflow refuses a tag that differs from the Cargo version.
4. GitHub Actions builds the packages, writes `SHA256SUMS.txt`, and publishes a release when all builds succeed. Check the release page before installing it.
5. ArchiForm checks once a day when the preference is enabled, or immediately through **File → Check for Updates**. Choose **Update Now** to download and install. Windows MSI and portable ZIP installations support this path.

Pushing a normal commit does not change installed copies. A new release tag and a successful release workflow are required. The original ConstructCAD repository stays available as `upstream` for selectively importing changes; its releases are not used by ArchiForm's updater.
