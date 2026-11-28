// Capture current mob health and the first visible mob along Utility's
// 64-block target ray. The block clip prevents the HUD from seeing through walls.
fn utilitymod_entity_vitals_frame(game: &mut crate::game::Game<'_>) {
    let values = game.mobs.iter()
        .filter(|mob| !mob.removed)
        .map(|mob| (
            mob.id,
            utilitymod::EntityVitals::new(mob.health, mob.attrs.max_health, mob.absorption),
        ));
    utilitymod::replace_entity_vitals(values);

    let target_id = game.aim().and_then(|(eye, direction)| {
        let (index, entity_distance, _) =
            game.mob_hit_within(utilitymod::ENTITY_RANGE_BLOCKS)?;

        let block_distance = game.clip_ray(eye, direction, utilitymod::ENTITY_RANGE_BLOCKS)
            .map(|hit| {
                let dx = hit.point[0] - eye[0];
                let dy = hit.point[1] - eye[1];
                let dz = hit.point[2] - eye[2];
                (dx * dx + dy * dy + dz * dz).sqrt()
            })
            .unwrap_or(f64::INFINITY);

        if block_distance + 0.0001 < entity_distance {
            return None;
        }
        game.mobs.iter().nth(index).map(|mob| mob.id)
    });
    utilitymod::set_target_entity_id(target_id);
}
