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
- Native GUI-post hook draws an armor/offhand text panel from the live player equipment.

## Still incomplete

- Entity HUD: raycast to a living target, block occlusion, target name and health/absorption hearts.
- Armor HUD: item icons, exact damage/max durability, low-durability alert, food count and item-bar rendering.
- Fullbright: a renderer lighting/lightmap hook.
- No Fog: a renderer fog hook that preserves lava, powder-snow, blindness and other intended special fog.
- Validate the key event's press/release behavior against the pinned Starling build.
- Build the native mod with Starling's pipeline and test it in-game.

The current native adapter is partial. The toggle controls and zoom FOV/mouse hooks are wired in source, and the armor panel is a first working rendering pass; the remaining features above are not implemented or verified. Do not mark the port complete until Starling builds it and the listed features are tested.
