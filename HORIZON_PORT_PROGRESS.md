# Utility → Horizon (Rust) port

This branch is the current Horizon `.hmod` port. The old `.hnmod`/horizon-mixins package is legacy and is not the current installable package format.

## Implemented in Rust

- Current `horizon-mod-api` `Hooks` adapter registers J/K/L/N and reports toggle state in chat.
- Portable core contains feature toggle state, held-zoom math, durability calculations, health/absorption heart calculations, armor/equipment and food presentation models, entity HUD presentation/range model, fog-distance transformation, and Fullbright safety-state resolution.
- Regression tests cover the portable behavior.
- CI packages a source `.hmod` with its Rust core.

## Still not complete / not verified

- The adapter does not yet read live armor/inventory/entity data or draw HUD elements in Horizon.
- Actual renderer hook integration for Fullbright and No Fog is not implemented in the current `mod_api` adapter.
- Zoom hold + wheel input + camera/FOV/mouse sensitivity are not wired into the current adapter.
- The native adapter has not been successfully checked/built against the exact Horizon SDK, and no in-game test has been completed.

The current artifact is a source `.hmod` package, not a verified native compiled mod. Do not mark the port 100% complete until the missing Horizon client hooks are implemented, native compilation succeeds, and the mod is tested in-game.
