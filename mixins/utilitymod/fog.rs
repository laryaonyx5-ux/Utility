// Port of FogMixin: push every vanilla fog distance beyond the render range.
fn utilitymod_no_fog(
    game: &crate::game::Game<'_>,
    mut fog: horizon_client::fog::FogData,
) -> horizon_client::fog::FogData {
    if game.native_on("utilitymod")
        && game.native_setting("utilitymod", "fog").as_deref() == Some("on")
    {
        fog.environmental_start = 3.0e7;
        fog.environmental_end = 4.0e7;
        fog.render_distance_start = 3.0e7;
        fog.render_distance_end = 4.0e7;
        fog.sky_end = 4.0e7;
        fog.cloud_end = 4.0e7;
    }
    fog
}
