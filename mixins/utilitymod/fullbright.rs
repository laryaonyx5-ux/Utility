// Override the final frame's night-vision brightening factor while Fullbright is enabled.
// A very large factor makes the renderer's vanilla night-vision curve approach its
// maximum without permanently changing the user's gamma option.
fn utilitymod_fullbright(game: &crate::game::Game<'_>, intensity: f32) -> f32 {
    if game.native_on("utilitymod")
        && game.native_setting("utilitymod", "fullbright").as_deref() == Some("on")
    {
        1000.0
    } else {
        intensity
    }
}
