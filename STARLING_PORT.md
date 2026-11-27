# Utility HUD — Starling native port

This branch is for the Starling/Horizon native version of Utility HUD.
The original Fabric Java source and its `.jar` remain unchanged on `main`.

## Target

- Minecraft 26.3 / matching Starling-Horizon build
- Rust native mod, id `utilitymod`
- Native package manifest: `horizon-native.json`
- Pinned Horizon mixins: `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`

## Port status

Implemented and committed on this branch:
- Rust core state for Armor HUD, Entity HUD, Fullbright and Fog toggles.
- Hold-to-zoom state: starts at 3×, wheel multiplier 1.2×, FOV calculation and mouse divisor.
- Armor durability calculation and low-durability threshold (20%).
- Entity health/absorption heart calculations, 64-block target range and 10-heart row constant.
- Starling native mod-list registration.
- Unit tests for the portable Rust calculations.

Not yet complete:
- Native key registration and J/K/L/N/C input dispatch.
- Actual HUD drawing and entity raycast/block occlusion.
- Game-backed armor/inventory/food reads.
- Lighting/fullbright and fog hooks.
- Camera FOV, mouse movement and scroll hook wiring.
- Native settings UI and persistent config.
- A successful Starling native build and in-game validation.

The Rust behavior core is not, by itself, a playable port. Do not mark the mod complete or tell the user all five features work until the remaining adapters are implemented and Starling builds it.
