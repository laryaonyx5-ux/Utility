fn utilitymod_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if !game.native_on("utilitymod") {
        game.utilitymod_zoom(false);
        return;
    }

    let zoom_key = game.native_mod_key("utilitymod", "zoom", "key.keyboard.c");
    if key == zoom_key {
        if game.native_playing() { game.utilitymod_zoom(pressed); }
        else { game.utilitymod_zoom(false); }
        return;
    }
    if !pressed || !game.native_playing() { return; }

    let toggle = [
        ("armor", "Armor HUD", "key.keyboard.j"),
        ("entity", "Entity HUD", "key.keyboard.k"),
        ("fullbright", "Fullbright", "key.keyboard.l"),
        ("fog", "No Fog", "key.keyboard.n"),
    ];
    for (name, label, default) in toggle {
        let binding = game.native_mod_key("utilitymod", name, default);
        if key == binding {
            let value = if game.native_setting("utilitymod", name).as_deref() == Some("on") { "off" } else { "on" };
            game.native_set_setting("utilitymod", name, value);
            game.native_client_action_bar(
                horizon_client::text::Text::literal(format!("{label}: {}", if value == "on" { "activé" } else { "désactivé" })),
                true,
            );
            return;
        }
    }
}
