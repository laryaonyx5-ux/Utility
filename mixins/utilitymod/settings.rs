// Starling's native mod settings page. The old hook is adapted into toggles.
fn utilitymod_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    if mod_id != "utilitymod" { return; }
    let enabled = |key: &str| game.native_setting("utilitymod", key).as_deref() == Some("on");
    buttons.push(format!("Armor HUD: {}", if enabled("armor") { "ON" } else { "OFF" }));
    buttons.push(format!("Entity HUD: {}", if enabled("entity") { "ON" } else { "OFF" }));
    buttons.push(format!("Fullbright: {}", if enabled("fullbright") { "ON" } else { "OFF" }));
    buttons.push(format!("No Fog: {}", if enabled("fog") { "ON" } else { "OFF" }));
}
