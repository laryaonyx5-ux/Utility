# Utility HUD (Fabric, Minecraft 26.3)

Touches par défaut (changeables dans Options > Contrôles > Touches > "Utility HUD") :
J Armor HUD · K Entity HUD · L Fullbright · N Fog · C Zoom (maintenir + molette)

## Compiler (Java 25 requis)
Méthode simple (recommandée) :
1. https://fabricmc.net/develop/template/ -> Minecraft 26.3, mod id `utilitymod`, package `fr.utilitymod`.
2. Remplace le dossier `src/main` du template par celui de ce zip.
3. `./gradlew build` -> jar dans `build/libs/` (pas celui en -sources).

Méthode directe : ce zip contient aussi build.gradle / gradle.properties
(Loom 1.17, Fabric API 0.162.0+26.3, Loader 0.19.5). Il faut juste ajouter le wrapper :
`gradle wrapper --gradle-version 9.6.0` puis `./gradlew build`.

Mets le jar + Fabric API 0.162.0+26.3 dans `.minecraft/mods`.
