# Utility → Horizon (Rust) port

This branch is the current Horizon `.hmod` port. The old `.hnmod`/horizon-mixins package is legacy and is not the current installable package format.

## Implemented in Rust

- Current `horizon-mod-api` `Hooks` adapter registers J/K/L/N and C (zoom).
- J/K/L/N toggle the original Armor HUD, Entity HUD, Fullbright and No Fog state and post chat feedback.
- C uses Horizon's native zoom hook: hold-to-zoom, factor 3, instant transition, mouse turn scale 0.45.
- No Fog now modifies Horizon's final fog distances through its native `render_fog` hook when enabled.
- Portable core contains feature toggle state, held-zoom math, durability calculations, health/absorption heart calculations, armor/equipment and food presentation models, entity HUD presentation/range model, fog-distance transformation, and Fullbright safety-state resolution.
- Regression tests cover the portable behavior.
- CI has been changed to install Horizon's mod tooling, run `cargo horizon-mod check`, and build a validated `.hmod` instead of simply zipping the source. This SDK check is currently being worked through in CI.

## Still not complete / not verified

- Armor HUD and Entity HUD are not yet drawn in Horizon and live player inventory/target/entity data are not connected.
- Fullbright is not yet applied to Horizon's actual world-lighting/lightmap pipeline.
- Zoom wheel scaling beyond the fixed 3x and the original camera behavior are not wired; Horizon's native zoom hook currently supplies a fixed factor.
- The native adapter has not yet passed `cargo horizon-mod check` against its pinned SDK, and no in-game test has been completed.

Do not mark the port 100% complete until the missing HUD and lighting features are implemented, native compilation succeeds, and the mod is tested in-game. A source `.hmod` is not proof of a working mod.
