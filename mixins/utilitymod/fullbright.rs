// After Horizon updates its own fullbright state, include Utility HUD's toggle.
// Preserve Horizon's multiplayer safety gate and the user's built-in Fullbright option.
fn utilitymod_fullbright(game: &mut crate::game::Game<'_>, _result: ()) {
    let utility_enabled = game.native_on("utilitymod")
        && game.native_setting("utilitymod", "fullbright").as_deref() == Some("on");
    if let Some(renderer) = game.renderer.as_mut() {
        renderer.fullbright =
            (game.settings.extras.fullbright || utility_enabled) && !game.settings.fullbright_held;
    }
}
