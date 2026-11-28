//! Horizon-native rendering for Utility's armor and entity HUDs.
//!
//! This uses only the public Horizon client drawing and entity APIs. It does
//! not patch or modify Java sources.

use mod_api::{ClientHost, EquipmentSlot, RenderGuiPostEvent};
use utilitymod::EntityVitals;

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
        (EquipmentSlot::Head, "Tête"),
        (EquipmentSlot::Chest, "Plastron"),
        (EquipmentSlot::Legs, "Jambes"),
        (EquipmentSlot::Feet, "Bottes"),
        (EquipmentSlot::OffHand, "Main secondaire"),
    ];

    let inventory = player.inventory(host).unwrap_or_default();
    let food_count: u32 = inventory.iter()
        .filter(|stack| host.items().item_default_component(stack.item(), "minecraft:food").is_some())
        .map(|stack| stack.count())
        .sum();

    let mut rows: Vec<(String, mod_api::ItemStack)> = Vec::new();
    for (slot, label) in slots {
        if let Some(stack) = living.equipment(host, slot) {
            rows.push((label.to_owned(), stack));
        }
    }
    if rows.is_empty() && food_count == 0 {
        return;
    }

    let row_height = 24.0;
    let panel_width = 172.0;
    let panel_rows = rows.len() + usize::from(food_count > 0);
    let panel_height = panel_rows as f32 * row_height + 12.0;
    let x = 4.0;
    let y = (height * 0.5 - panel_height * 0.5).max(4.0);
    event.draw.fill([x, y, panel_width, panel_height], PANEL);

    for (index, (label, stack)) in rows.iter().enumerate() {
        let row_y = y + 8.0 + index as f32 * row_height;
        event.draw.item_stack(stack, [x + 4.0, row_y], 16.0);
        event.draw.text(label, [x + 24.0, row_y + 4.0], WHITE, true);

        let item_id = stack.item().id(host);
        let item_name = host.hover_name(stack);
        let damage_max = host.items().max_damage(stack.item()).unwrap_or(0);
        let damage = stack.damage();
        let detail = if damage_max > 0 {
            let remaining = damage_max.saturating_sub(damage);
            let ratio = remaining as f32 / damage_max as f32;
            if ratio <= 0.20 {
                event.draw.text("!!!", [x + 4.0, row_y - 8.0], ALERT, true);
            }
            let bar_color = if ratio <= 0.20 { 0xFFFF2020 } else if ratio <= 0.50 { 0xFFFFC040 } else { 0xFF40C040 };
            event.draw.fill([x + 6.0, row_y + 17.0, 13.0, 2.0], 0xFF000000);
            event.draw.fill([x + 6.0, row_y + 17.0, (13.0 * ratio).round().clamp(0.0, 13.0), 2.0], bar_color);
            format!("{remaining}/{damage_max}")
        } else if label == "Main secondaire" || stack.count() > 1 {
            let total: u32 = inventory.iter().filter(|item| item.same_item(stack)).map(|item| item.count()).sum::<u32>() + stack.count();
            format!("x{total}")
        } else {
            String::new()
        };

        // Keep the item id as a fallback when a custom stack has no translated name.
        let name = if item_name.is_empty() { item_id } else { item_name };
        let available = (panel_width - 30.0).max(1.0);
        let name = truncate_for_width(&*event.draw, &name, available);
        event.draw.text(&name, [x + 24.0, row_y + 13.0], 0xFFCCCCCC, true);
        if !detail.is_empty() {
            let detail_width = event.draw.text_width(&detail);
            let detail_x = x + panel_width - detail_width - 6.0;
            event.draw.text(&detail, [detail_x, row_y + 4.0], WHITE, true);
        }
    }
    if food_count > 0 {
        let row_y = y + 8.0 + rows.len() as f32 * row_height;
        let _ = event.draw.sprite("minecraft:hud/food_full", [0.0, 0.0, 9.0, 9.0], [x + 7.0, row_y + 4.0, 9.0, 9.0], None);
        event.draw.text(&format!("x{food_count}"), [x + 24.0, row_y + 4.0], WHITE, true);
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
    let Some(target_id) = utilitymod::target_entity_id() else { return };
    let Some(entity) = host.entity(target_id) else { return };
    let Some(vitals) = utilitymod::entity_vitals(target_id).or_else(|| {
        let living = host.living(entity)?;
        Some(EntityVitals::new(
            living.health(host)?,
            living.max_health(host)?,
            living.absorption(host).unwrap_or(0.0),
        ))
    }) else { return };
    let health = vitals.health;
    let max_health = vitals.max_health;
    let absorption = vitals.absorption.max(0.0);
    let name = entity.name(host).unwrap_or_else(|| "Entity".to_owned());

    let health_text = format!("{} / {} PV", fmt_health(health.max(0.0)), fmt_health(max_health.max(0.0)));
    let name_width = event.draw.text_width(&name);
    let health_width = event.draw.text_width(&health_text);
    let hearts = health <= 20.0;
    let heart_columns = if hearts {
        ((max_health.max(1.0) / 2.0).ceil() as usize).clamp(1, 10)
    } else {
        1
    };
    let absorption_count = (absorption / 2.0).ceil() as usize;
    let columns = heart_columns.max(absorption_count.min(10)).clamp(1, 10);
    let heart_width = columns as f32 * 8.0 + 1.0;
    let high_health_count = format!("{} / {} cœurs", fmt_health(health.max(0.0) / 2.0), fmt_health(max_health.max(0.0) / 2.0));
    let high_health_extra = if absorption > 0.0 { format!(" +{}", fmt_health(absorption / 2.0)) } else { String::new() };
    let high_health_width = if hearts { 0.0 } else {
        11.0 + event.draw.text_width(&high_health_count) + event.draw.text_width(&high_health_extra)
    };
    let box_width = name_width.max(health_width).max(heart_width).max(high_health_width) + 12.0;
    let health_rows = (heart_columns + 9) / 10;
    let absorption_rows = (absorption_count.min(30) + 9) / 10;
    let box_height = 8.0 + 11.0 + 11.0 + if hearts { (health_rows + absorption_rows) as f32 * 10.0 } else { 12.0 };
    let x = (width - box_width) * 0.5;
    let y = 28.0;
    event.draw.fill([x, y, box_width, box_height], PANEL);

    event.draw.text(&name, [(width - name_width) * 0.5, y + 4.0], WHITE, true);
    event.draw.text(&health_text, [(width - health_width) * 0.5, y + 15.0], WHITE, true);

    if !hearts {
        let group_x = (width - high_health_width) * 0.5;
        draw_heart(event, "full", [group_x, y + 27.0]);
        event.draw.text(&high_health_count, [group_x + 12.0, y + 28.0], WHITE, true);
        if !high_health_extra.is_empty() {
            let heart_count_width = event.draw.text_width(&high_health_count);
            event.draw.text(&high_health_extra, [group_x + 12.0 + heart_count_width, y + 28.0], GOLD, true);
        }
        return;
    }

    let x0 = width * 0.5 - (columns as f32 * 8.0 + 1.0) * 0.5;
    let hearts_y = y + 27.0;
    for i in 0..heart_columns {
        let at = [x0 + (i % 10) as f32 * 8.0, hearts_y + (i / 10) as f32 * 10.0];
        draw_heart(event, "container", at);
        if health >= 2.0 * i as f32 + 2.0 {
            draw_heart(event, "full", at);
        } else if health >= 2.0 * i as f32 + 1.0 {
            draw_heart(event, "half", at);
        }
    }

    for i in 0..absorption_count.min(30) {
        let at = [
            x0 + (i % 10) as f32 * 8.0,
            hearts_y + (health_rows + i / 10) as f32 * 10.0,
        ];
        draw_heart(event, "container", at);
        if absorption >= 2.0 * i as f32 + 2.0 {
            draw_heart(event, "absorbing_full", at);
        } else if absorption >= 2.0 * i as f32 + 1.0 {
            draw_heart(event, "absorbing_half", at);
        }
    }
}

fn fmt_health(value: f32) -> String {
    if value.is_finite() && value.fract().abs() < 0.0001 {
        format!("{}", value as i32)
    } else {
        format!("{value:.1}")
    }
}

fn draw_heart(event: &mut RenderGuiPostEvent<'_>, kind: &str, at: [f32; 2]) {
    let sprite = format!("minecraft:hud/heart/{kind}");
    if !event.draw.sprite(&sprite, [0.0, 0.0, 9.0, 9.0], [at[0], at[1], 9.0, 9.0], None) {
        let fallback = match kind {
            "container" => "♡",
            "half" | "absorbing_half" => "◐",
            "absorbing_full" => "♥",
            _ => "♥",
        };
        event.draw.text(fallback, at, if kind.starts_with("absorbing") { GOLD } else { WHITE }, true);
    }
}
