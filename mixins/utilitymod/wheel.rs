fn utilitymod_wheel(game: &mut crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    if !game.native_on("utilitymod") || !game.utilitymod_zoom_active() || !steps.is_finite() || steps == 0.0 {
        return;
    }
    game.utilitymod_zoom_scroll(steps);
    *claimed = true;
}
