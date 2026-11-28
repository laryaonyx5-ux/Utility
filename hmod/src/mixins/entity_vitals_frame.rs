// Capture the living mobs' current client-side health once per frame.
// The public mod API intentionally exposes another entity's health only on
// the server; this native observation feeds Utility's read-only HUD snapshot.
fn utilitymod_entity_vitals_frame(game: &mut crate::game::Game<'_>) {
    let values = game.mobs.iter()
        .filter(|mob| !mob.removed)
        .map(|mob| (
            mob.id,
            utilitymod::EntityVitals::new(mob.health, mob.attrs.max_health, mob.absorption),
        ));
    utilitymod::replace_entity_vitals(values);
}
