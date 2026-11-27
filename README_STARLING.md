# Utility HUD — Starling Rust port

This branch contains a native Rust adaptation of the Fabric mod on `main`.
The original Java/Fabric source and its `.jar` are unchanged.

## Ported source features

- J / K / L / N toggles for Armor HUD, Entity HUD, Fullbright and No Fog.
- Starling mod-list row and settings-page source for the four toggles.
- C hold-to-zoom source, wheel scaling (1.2× per step), 3× start and 1×–30× limits.
- Zoom FOV modifier and mouse-sensitivity adjustment.
- Armor/offhand durability and stack-count HUD source, with low-durability warning.
- Food stack count source.
- Entity HUD source for mobs within 64 blocks, block occlusion, health, hearts and absorption.

## Native package

Run:

```sh
bash scripts/package-starling.sh
```

This creates `utilitymod-1.1.0.hmod`, the native-mod archive format used by Starling. The GitHub Actions workflow runs the portable Rust core tests and uploads the archive as an artifact.

## Validation status

The portable Rust core has unit tests. The injected Starling mixin sources have **not** been built against the pinned Horizon revision or validated in-game in this repository. Therefore this branch is a work in progress, not a verified 100% replacement for the Fabric mod.

To build and test the native adapter, import the source through Starling's Native Mods workflow for the exact installed game build and resolve any compiler diagnostics from the mixin pipeline.
