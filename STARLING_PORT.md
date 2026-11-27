# Utility HUD — Starling native port

This branch is for the Starling/Horizon native version of Utility HUD.
The original Fabric Java source and its `.jar` remain unchanged on `main`.

## Target package

- Game: Minecraft 26.3 / Starling's matching Horizon build
- Language: Rust
- Starling native package: `.hnmod`
- Mod id: `utilitymod`

## Features to port from the Fabric version

The source on `main` currently documents these default controls:

| Feature | Default | Required native integration |
|---|---:|---|
| Armor HUD | J | HUD rendering + player armor/inventory data |
| Entity HUD | K | HUD rendering + entity targeting/data |
| Fullbright | L | game lighting/render hook or supported setting |
| Fog toggle | N | fog/render hook |
| Zoom | C (hold + wheel) | key state + wheel + camera/FOV hook |

## Porting rules

1. Keep this branch independent from the Fabric `.jar`.
2. Reuse the user-facing behavior and defaults, not Java/Fabric implementation details.
3. Implement through Starling's documented native-mod hooks and settings API.
4. Do not claim a feature is working until it is connected to a real hook and built by Starling.
5. The native package must be imported and compiled by Starling's Native Mods workflow; copying a Rust source file into Mods is not sufficient.

## Status

Scaffold / API mapping only. The feature hooks and in-game validation are not complete yet.
