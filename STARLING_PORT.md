# Utility HUD — Starling native port

This branch is for the Starling/Horizon native version of Utility HUD.
The original Fabric Java source and its `.jar` remain unchanged on `main`.

## Target

- Minecraft 26.3 / matching Starling-Horizon build
- Rust native mod, id `utilitymod`
- Native package manifest: `horizon-native.json`
- Pinned Horizon mixins: `0d6b103c70633d8fd9443d8d0acd3aadaacedb06`

## Implemented in this branch

- Portable Rust calculations and unit tests for feature state, armor durability, entity hearts and zoom.
- Native Starling mod-list row with configurable J/K/L/C/N bindings.
- Native settings page toggles for Armor HUD, Entity HUD, Fullbright and No Fog.
- Input dispatch for the four toggles; state is stored in Starling's mod settings.
- Hold-C zoom state, starts at 3×, wheel multiplier 1.2× and clamp 1×–30×.
- FOV hook applies the zoom factor; mouse-turn hook reduces sensitivity by the same factor.
- Native armor/offhand HUD reads live equipment, shows stack counts and item durability, and flags items at 20% durability or below.
- Native Entity HUD raycasts mobs up to 64 blocks, checks the block hit for occlusion, and displays the mob ID, health, hearts and absorption.

## Still incomplete

- Armor HUD: item sprites, vanilla durability bar, food count, and max-damage overrides from item components.
- Entity HUD: proper translated display names, player targets, and vanilla heart sprites/rows for high-health entities.
- Fullbright: a renderer lighting/lightmap hook.
- No Fog: a renderer fog hook that preserves lava, powder-snow, blindness and other intended special fog.
- Validate key press/release and the hook signatures against the pinned Starling build.
- Build the native mod with Starling's pipeline and test it in-game.

The current native adapter is still partial. The key/settings, zoom camera hooks, armor text/durability panel and mob health overlay are implemented in source, but fullbright and no-fog rendering are not connected, and the native build has not been run. Do not mark the port complete until Starling builds it and the remaining features are implemented and tested.
