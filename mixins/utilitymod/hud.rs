// Starling's GUI-post hook: draw live armor/offhand rows after the vanilla HUD.
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

    let slots = ["Head", "Chest", "Legs", "Feet"];
    let mut rows: Vec<(String, bool)> = Vec::new();
    for (index, label) in slots.iter().enumerate() {
        if let Some(stack) = game.armor.get(index).and_then(Option::as_ref) {
            let max = game.items().get(&stack.item).map_or(0_u32, |item| item.max_damage as u32);
            if max > 0 {
                let remaining = max.saturating_sub(stack.damage);
                let low = remaining as f32 <= max as f32 * 0.20;
                rows.push((format!("{label}: {}  {remaining}/{max}", stack.item), low));
            } else {
                rows.push((format!("{label}: {} x{}", stack.item, stack.count), false));
            }
        }
    }
    if let Some(stack) = game.offhand.as_ref() {
        let max = game.items().get(&stack.item).map_or(0_u32, |item| item.max_damage as u32);
        if max > 0 {
            let remaining = max.saturating_sub(stack.damage);
            let low = remaining as f32 <= max as f32 * 0.20;
            rows.push((format!("Offhand: {}  {remaining}/{max}", stack.item), low));
        } else {
            rows.push((format!("Offhand: {} x{}", stack.item, stack.count), false));
        }
    }
    if rows.is_empty() { return; }

    let atlas = &game.gui.atlas;
    let x = 5.0_f32;
    let y = 8.0_f32;
    let height = 20.0 + rows.len() as f32 * 13.0;
    draw.fill(atlas, x, y, 210.0, height, 0x88000000);
    draw.text(atlas, "Utility HUD - Armor", x + 5.0, y + 4.0, 0xFFFFFFFF, true);
    for (index, (line, low)) in rows.iter().enumerate() {
        let color = if *low { 0xFFFF2020 } else { 0xFFFFFFFF };
        let alert = if *low { "!!! " } else { "" };
        draw.text(atlas, &format!("{alert}{line}"), x + 5.0, y + 18.0 + index as f32 * 13.0, color, true);
    }
}
