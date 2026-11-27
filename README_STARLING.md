# Utility HUD — Starling Rust port

This branch is the native Rust adaptation of the Fabric mod in `main`.
The original Java/Fabric source and its `.jar` are unchanged.

## Ported core

- Persistable feature state: armor HUD, entity HUD, fullbright, fog.
- Hold-to-zoom state, 3× starting magnification, 1.2× wheel multiplier,
  FOV division and matching mouse sensitivity divisor.
- Armor item durability values and 20% low-durability threshold.
- Entity health/absorption heart calculations, 64-block target range and
  10-heart row constant.

## Important status

The Rust behavior core is committed. It is **not yet a fully playable native
mod**: Starling's game-facing injection layer still needs to connect key
input, HUD draw calls, entity targeting, lighting and fog hooks to this core.
The Java mixins cannot be mechanically copied into Rust. Do not import this
branch and expect all five features to work until that adapter is completed
and built by Starling's native-mod pipeline.

## Build target

Starling/Horizon Minecraft 26.3. This native mod is a trusted Rust code mod;
only import it if you trust the source. Use Starling's Native Mods workflow
to build it for the exact installed game build.
