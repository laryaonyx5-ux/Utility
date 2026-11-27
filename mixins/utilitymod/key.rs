fn utilitymod_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    if !pressed || !game.native_on("utilitymod") { return; }
    let toggle = [
        ("armor", "Armor HUD"),
        ("entity", "Entity HUD"),
        ("fullbright", "Fullbright"),
        ("fog", "No Fog"),
    ];
    for (name, label) in toggle {
        let binding = game.native_mod_key("utilitymod", name, match name {
            "armor" => "key.keyboard.j",
            "entity" => "key.keyboard.k",
            "fullbright" => "key.keyboard.l",
            _ => "key.keyboard.n",
        });
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
    let zoom_key = game.native_mod_key("utilitymod", "zoom", "key.keyboard.c");
    if key == zoom_key {
        game.utilitymod_zoom(pressed);
    }
}
