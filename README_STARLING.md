# Utility HUD — Starling Rust port

This branch is the native Rust adaptation of the Fabric mod in `main`.
The original Java/Fabric source and its `.jar` are unchanged.

## Native features currently connected

- J / K / L / N toggles for Armor HUD, Entity HUD, Fullbright and No Fog.
- Starling mod settings page for those four feature switches.
- C hold-to-zoom input, wheel scaling (1.2× per step), 3× start and 1×–30× limits.
- Zoom FOV modifier and mouse sensitivity adjustment.
- Armor/offhand text HUD with stack counts, durability values and low-durability warning.
- Entity HUD for mobs within 64 blocks, with block occlusion, health, hearts and absorption.

## Incomplete / not yet verified

- Armor item sprites, vanilla durability bars, food count and item component overrides.
- Entity translated names, player targets and vanilla heart sprites/layout for high-health mobs.
- Fullbright renderer lighting hook.
- No Fog renderer hook.
- Native build and in-game validation for the exact Starling 26.3 build.

## Build target

Starling/Horizon Minecraft 26.3. This is a trusted native Rust mod; only import it if you trust the source. Use Starling's Native Mods workflow to build it for the exact installed game build.

The source is a partial port, not a finished replacement for the Fabric `.jar`. See [STARLING_PORT.md](STARLING_PORT.md) for the implementation checklist.
