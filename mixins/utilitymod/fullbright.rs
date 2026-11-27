// Port of FullbrightLightmapMixin: force the eye lightmap brightness to 1.0.
fn utilitymod_fullbright(game: &crate::game::Game<'_>, intensity: f32) -> f32 {
    if game.native_on("utilitymod")
        && game.native_setting("utilitymod", "fullbright").as_deref() == Some("on")
    {
        1.0
    } else {
        intensity
    }
}
