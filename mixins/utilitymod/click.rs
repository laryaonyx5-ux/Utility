fn utilitymod_settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    if mod_id != "utilitymod" { return; }
    let key = match button {
        0 => "armor",
        1 => "entity",
        2 => "fullbright",
        3 => "fog",
        _ => return,
    };
    let value = if game.native_setting("utilitymod", key).as_deref() == Some("on") { "off" } else { "on" };
    game.native_set_setting("utilitymod", key, value);
}
