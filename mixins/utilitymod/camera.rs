fn utilitymod_fov(game: &crate::game::Game<'_>, modifier: &mut f32) {
    game.utilitymod_apply_fov(modifier);
}

fn utilitymod_mouse_turn(game: &crate::game::Game<'_>, sensitivity: &mut f64) {
    if game.native_on("utilitymod") && game.utilitymod_zoom_active() {
        *sensitivity /= f64::from(game.utilitymod_zoom_factor());
    }
}
