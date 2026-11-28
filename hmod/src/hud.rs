//! Horizon-native rendering for Utility's armor and entity HUDs.
//!
//! This uses only the public Horizon client drawing and entity APIs. It does
//! not patch or modify Java sources.

use mod_api::{ClientHost, CrosshairTarget, EquipmentSlot, RenderGuiPostEvent};
use utilitymod::{EntityHud, EntityVitals};

const PANEL: u32 = 0x78000000;
const WHITE: u32 = 0xFFFFFFFF;
const GOLD: u32 = 0xFFFFE040;
const ALERT: u32 = 0xFFFF2020;

pub fn render(host: &dyn ClientHost, event: &mut RenderGuiPostEvent<'_>, armor_enabled: bool, entity_enabled: bool) {
    if !host.playing() {
        return;
    }

    let [width, height] = event.draw.size();
    if armor_enabled {
        render_armor(host, event, height);
    }
    if entity_enabled {
        render_target(host, event, width);
    }
}

fn render_armor(host: &dyn ClientHost, event: &mut RenderGuiPostEvent<'_>, height: f32) {
    let Some(player) = host.local_player() else { return };
    let Some(living) = host.living(player.entity()) else { return };

    let slots = [
        (EquipmentSlot::Head, "Head"),
        (EquipmentSlot::Chest, "Chest"),
        (EquipmentSlot::Legs, "Legs"),
        (EquipmentSlot::Feet, "Feet"),
        (EquipmentSlot::OffHand, "Offhand"),
    ];

    let mut rows: Vec<(String, mod_api::ItemStack)> = Vec::new();
    for (slot, label) in slots {
        if let Some(stack) = living.equipment(host, slot) {
            rows.push((label.to_owned(), stack));
        }
    }
    if rows.is_empty() {
        return;
    }

    let row_height = 22.0;
    let panel_width = 172.0;
    let panel_height = rows.len() as f32 * row_height + 8.0;
    let x = 4.0;
    let y = (height * 0.5 - panel_height * 0.5).max(4.0);
    event.draw.fill([x, y, panel_width, panel_height], PANEL);

    for (index, (label, stack)) in rows.iter().enumerate() {
        let row_y = y + 4.0 + index as f32 * row_height;
        event.draw.item_stack(stack, [x + 4.0, row_y], 16.0);
        event.draw.text(label, [x + 24.0, row_y + 4.0], WHITE, true);

        let item_id = stack.item().id(host);
        let item_name = host.hover_name(stack);
        let damage_max = host.items().max_damage(stack.item()).unwrap_or(0);
        let damage = stack.damage();
        let detail = if damage_max > 0 {
            let remaining = damage_max.saturating_sub(damage);
            if remaining as f32 <= damage_max as f32 * 0.20 {
                event.draw.text("!!!", [x + 88.0, row_y + 4.0], ALERT, true);
            }
            format!("{remaining}/{damage_max}")
        } else if stack.count() > 1 {
            format!("x{}", stack.count())
        } else {
            String::new()
        };

        // Keep the item id as a fallback when a custom stack has no translated name.
        let name = if item_name.is_empty() { item_id } else { item_name };
        let available = (panel_width - 30.0).max(1.0);
        let name = truncate_for_width(&event.draw, &name, available);
        event.draw.text(&name, [x + 24.0, row_y + 13.0], 0xFFCCCCCC, true);
        if !detail.is_empty() {
            event.draw.text(&detail, [x + panel_width - event.draw.text_width(&detail) - 6.0, row_y + 4.0], WHITE, true);
        }
    }
}

fn truncate_for_width(draw: &dyn mod_api::DrawContext, text: &str, max_width: f32) -> String {
    if draw.text_width(text) <= max_width {
        return text.to_owned();
    }
    let mut out = String::new();
    for ch in text.chars() {
        let mut candidate = out.clone();
        candidate.push(ch);
        candidate.push('…');
        if draw.text_width(&candidate) > max_width {
            break;
        }
        out.push(ch);
    }
    out.push('…');
    out
}

fn render_target(host: &dyn ClientHost, event: &mut RenderGuiPostEvent<'_>, width: f32) {
    let Some(CrosshairTarget::Entity { entity, .. }) = host.crosshair_target() else { return };
    let Some(living) = host.living(entity) else { return };
    let Some(health) = living.health(host) else { return };
    let Some(max_health) = living.max_health(host) else { return };
    let absorption = living.absorption(host).unwrap_or(0.0);
    let name = entity.name(host).unwrap_or_else(|| "Entity".to_owned());

    let vitals = EntityVitals::new(health, max_health, absorption);
    let hud = EntityHud {
        entity_name: name.clone(),
        distance_blocks: 0.0,
        vitals,
    };
    let health_text = hud.health_text();
    let hearts = hud.compact_hearts();
    let extra = if absorption > 0.0 {
        format!("  +{:.1} absorption", absorption)
    } else {
        String::new()
    };

    let lines = [name, health_text, hearts + &extra];
    let line_height = 11.0;
    let box_width = lines.iter().map(|line| event.draw.text_width(line)).fold(0.0_f32, f32::max) + 12.0;
    let box_height = lines.len() as f32 * line_height + 8.0;
    let x = (width - box_width) * 0.5;
    let y = 28.0;
    event.draw.fill([x, y, box_width, box_height], PANEL);
    for (i, line) in lines.iter().enumerate() {
        let text_x = (width - event.draw.text_width(line)) * 0.5;
        event.draw.text(line, [text_x, y + 4.0 + i as f32 * line_height], if i == 2 && absorption > 0.0 { GOLD } else { WHITE }, true);
    }
}
