# Utility → Horizon (Rust) port

This branch is the current Horizon mod API port. The old `.hnmod`/horizon-mixins package is legacy and is not the installable deliverable for current Horizon launchers.

## First implemented slice

- New `hmod/` project using Horizon's `horizon-mod-api` and `mod_api::Hooks`.
- Registered original key bindings J/K/L/N.
- Added client tick hook to toggle the Rust core's armor HUD, entity HUD, fullbright and no-fog config flags.
- Sends an in-game chat confirmation for each toggle.
- CI workflow attempts to fetch the pinned SDK, check, build and publish the resulting `.hmod` as an Actions artifact.

## Not yet ported / verified

- Actual armor HUD drawing and equipment data.
- Entity HUD target selection, health and absorption rendering.
- Fullbright and fog changes to the renderer.
- Zoom key hold, wheel scaling, camera/FOV and mouse sensitivity integration.
- Native build and in-game test on the pinned Horizon build.

This is a real first Rust API implementation, but it is not the complete Utility feature port yet. Do not mark the project finished until the package workflow succeeds and the mod is tested in Horizon.
