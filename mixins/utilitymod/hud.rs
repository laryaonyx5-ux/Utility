// Starling's GUI-post hook: draw live armor and target HUDs after the vanilla HUD.
fn utilitymod_hud(
    game: &crate::game::Game<'_>,
    draw: &mut horizon_client::hud::HudBatch,
    _corners: &mut horizon_client::extras_hud::Corners,
) {
    if !game.native_on("utilitymod") || !game.native_playing() {
        return;
    }
    if game.native_setting("utilitymod", "armor").as_deref() == Some("on") {
        utilitymod_armor_hud(game, draw);
    }
    if game.native_setting("utilitymod", "entity").as_deref() == Some("on") {
        utilitymod_entity_hud(game, draw);
    }
}

fn utilitymod_armor_hud(game: &crate::game::Game<'_>, draw: &mut horizon_client::hud::HudBatch) {
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

fn utilitymod_entity_hud(game: &crate::game::Game<'_>, draw: &mut horizon_client::hud::HudBatch) {
    let Some((index, distance, _point)) = game.mob_hit_within(64.0) else { return; };
    let Some((eye, _direction)) = game.aim() else { return; };
    let block_distance = game.pick().map_or(f64::INFINITY, |hit| {
        let d = [
            hit.point[0] - eye[0],
            hit.point[1] - eye[1],
            hit.point[2] - eye[2],
        ];
        (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
    });
    if distance > block_distance { return; }

    let mob = &game.mobs[index];
    let health = mob.health.max(0.0);
    let max_health = mob.attrs.max_health.max(1.0);
    let absorption = mob.absorption.max(0.0);
    let name = &mob.kind.id;
    let health_line = format!("{health:.1} / {max_health:.1} HP");
    let heart_slots = ((max_health / 2.0).ceil() as usize).clamp(1, 10);
    let full = (health / 2.0).floor() as usize;
    let half = health - full as f32 * 2.0 >= 1.0;
    let mut hearts = String::new();
    for slot in 0..heart_slots {
        if slot < full { hearts.push('♥'); }
        else if slot == full && half { hearts.push('♥'); }
        else { hearts.push('♡'); }
    }

    let atlas = &game.gui.atlas;
    let cx = draw.width / 2.0;
    let y = 28.0_f32;
    let name_width = horizon_client::hud::HudBatch::text_width(atlas, name);
    let health_width = horizon_client::hud::HudBatch::text_width(atlas, &health_line);
    let hearts_width = horizon_client::hud::HudBatch::text_width(atlas, &hearts);
    let total_width = name_width.max(health_width).max(hearts_width + if absorption > 0.0 { 42.0 } else { 0.0 });
    draw.fill(atlas, cx - total_width / 2.0 - 6.0, y - 4.0, total_width + 12.0, 48.0, 0x88000000);
    draw.text(atlas, name, cx - name_width / 2.0, y, 0xFFFFFFFF, true);
    draw.text(atlas, &health_line, cx - health_width / 2.0, y + 12.0, 0xFFFFFFFF, true);
    draw.text(atlas, &hearts, cx - hearts_width / 2.0, y + 24.0, 0xFFFF5555, true);
    if absorption > 0.0 {
        draw.text(atlas, &format!("+{absorption:.1}"), cx + hearts_width / 2.0 + 4.0, y + 24.0, 0xFFFFE040, true);
    }
}
