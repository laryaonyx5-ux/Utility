// Adds Utility HUD to Starling's native Options > Mods list.
fn utilitymod_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "utilitymod".into(),
        title: "Utility HUD 1.1.0".into(),
        description: "Armor HUD, Entity HUD, Fullbright, Fog and hold-to-zoom.".into(),
        keys: Vec::new(),
    });
}
