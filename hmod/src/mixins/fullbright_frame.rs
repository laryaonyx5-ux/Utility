// Utility's native fullbright override. It changes only the renderer's
// frame-local brightness fields and restores Horizon's own values when off.
fn utilitymod_fullbright_frame(game: &mut crate::game::Game<'_>) {
    let enabled = utilitymod::fullbright_enabled();
    let (horizon_fullbright, horizon_gamma) =
        (game.settings.extras.fullbright, game.settings.gamma);

    let Some(renderer) = game.renderer.as_mut() else {
        return;
    };

    if enabled {
        renderer.fullbright = true;
    } else {
        renderer.fullbright = horizon_fullbright;
        renderer.gamma = horizon_gamma;
    }
}
