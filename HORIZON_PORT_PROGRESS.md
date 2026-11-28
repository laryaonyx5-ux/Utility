# Utility → Horizon (Rust) port

This branch is the current Horizon `.hmod` port. The old `.hnmod`/horizon-mixins package is legacy and is not the current installable package format.

## Implemented in Rust

- The current Horizon `mod_api::Hooks` adapter registers the original J/K/L/N feature toggles and C for zoom.
- J/K/L/N toggle Armor HUD, Entity HUD, Fullbright and No Fog, show chat feedback, and store their state in Horizon settings.
- Armor HUD draws equipped armor/offhand, item durability, low-durability alerts, a durability bar, offhand item totals, and the total food count.
- Entity HUD draws the target name, numeric health, vanilla heart sprites and absorption hearts.
- A native frame helper snapshots mob health and selects the nearest mob on a 64-block view ray, rejecting targets hidden behind a block.
- Fullbright is applied to Horizon's renderer each frame and Horizon's own brightness values are restored when Utility is disabled.
- No Fog changes Horizon's final environmental, render-distance, sky and cloud fog values.
- C is hold-to-zoom with the Java behavior: factor resets to 3 when zoom starts, the wheel changes the factor from 1 to 30, and the camera-turn scale is configurable.
- The portable core contains toggle state, zoom math, durability and health calculations, and regression tests.

## Build and verification

- The portable Rust core tests pass in GitHub Actions.
- GitHub Actions passes the portable core tests, packages the `.hmod`, and validates its archive and native manifest.
- Horizon's launcher compiles native code against the pinned game SDK; that launcher build has not been completed in this environment.
- The source package is not a tested native binary. The launcher must build it for the user's exact Horizon revision.

## Still not complete / not verified

- The native adapter and mixin helpers still need to pass the pinned SDK compile check.
- The `.hmod` must be installed and tested in Horizon in-game; this has not been verified.
- Targeting currently covers Horizon's native mob list; player and special multipart target behavior has not been verified.
- The later Starling/mobile port has not started.

Do not mark the port 100% complete until native compilation succeeds, the packaged mod installs, and the main features are tested in Horizon in-game.
