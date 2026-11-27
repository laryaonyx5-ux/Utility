// Override the final frame's night-vision brightening factor while Fullbright is enabled.
// Force the eye's lightmap brightness to its maximum without permanently changing gamma.
fn utilitymod_fullbright(game: &crate::game::Game<'_>, intensity: f32) -> f32 {
    if game.native_on("utilitymod")
        && game.native_setting("utilitymod", "fullbright").as_deref() == Some("on")
    {
        1.0
    } else {
        intensity
    }
}
