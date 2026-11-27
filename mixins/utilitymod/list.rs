// Registers Utility HUD and its five configurable native key mappings.
fn utilitymod_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    mods.push(crate::game::native_hooks::NativeMod {
        id: "utilitymod".into(),
        title: "Utility HUD 1.1.0".into(),
        description: "Armor HUD, Entity HUD, Fullbright, Fog and hold-to-zoom.".into(),
        keys: vec![
            ["armor".into(), "Armor HUD".into(), "key.keyboard.j".into()],
            ["entity".into(), "Entity HUD".into(), "key.keyboard.k".into()],
            ["fullbright".into(), "Fullbright".into(), "key.keyboard.l".into()],
            ["zoom".into(), "Zoom (maintenir)".into(), "key.keyboard.c".into()],
            ["fog".into(), "No Fog".into(), "key.keyboard.n".into()],
        ],
    });
}
