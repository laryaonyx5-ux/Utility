// Starling's GUI-post hook: draw the armor/offhand rows after the vanilla HUD.
fn utilitymod_hud(
    game: &crate::game::Game<'_>,
    draw: &mut horizon_client::hud::HudBatch,
    _corners: &mut horizon_client::extras_hud::Corners,
) {
    if !game.native_on("utilitymod")
        || game.native_setting("utilitymod", "armor").as_deref() != Some("on")
        || !game.native_playing()
    {
        return;
    }

    let atlas = &game.gui.atlas;
    let x = 5.0_f32;
    let mut y = 8.0_f32;
    draw.fill(atlas, x, y, 170.0, 94.0, 0x88000000);
    draw.text(atlas, "Utility HUD - Armor", x + 5.0, y + 4.0, 0xFFFFFFFF, true);
    y += 18.0;

    let slots = ["Head", "Chest", "Legs", "Feet"];
    for (index, label) in slots.iter().enumerate() {
        if let Some(stack) = game.armor.get(index).and_then(Option::as_ref) {
            let line = format!("{label}: {} x{}", stack.item, stack.count);
            draw.text(atlas, &line, x + 5.0, y, 0xFFFFFFFF, true);
            y += 13.0;
        }
    }
    if let Some(stack) = game.offhand.as_ref() {
        let line = format!("Offhand: {} x{}", stack.item, stack.count);
        draw.text(atlas, &line, x + 5.0, y, 0xFFFFFFFF, true);
    }
}
