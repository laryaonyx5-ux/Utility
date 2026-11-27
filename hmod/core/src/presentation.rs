//! Render-ready presentation data for the Utility HUD.
//! Horizon's version-specific client adapter must translate these values into
//! its native draw calls and live game state.

use crate::{EntityVitals, HudRow};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArmorEntry {
    pub slot: &'static str,
    pub row: HudRow,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArmorHud {
    pub equipment: Vec<ArmorEntry>,
    pub food_total: u32,
}

impl ArmorHud {
    pub fn add_equipment(&mut self, slot: &'static str, item_id: &str, max_damage: u32, damage: u32, count: u32, offhand: bool) {
        self.equipment.push(ArmorEntry {
            slot,
            row: HudRow::from_item(item_id, max_damage, damage, count, offhand),
        });
    }

    pub fn add_food_stack(&mut self, count: u32) {
        self.food_total = self.food_total.saturating_add(count);
    }

    pub fn is_empty(&self) -> bool {
        self.equipment.is_empty() && self.food_total == 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntityHud {
    pub entity_name: String,
    pub distance_blocks: f64,
    pub vitals: EntityVitals,
}

impl EntityHud {
    pub fn is_in_range(&self) -> bool {
        self.distance_blocks.is_finite() && self.distance_blocks >= 0.0 && self.distance_blocks <= crate::runtime::ENTITY_RANGE_BLOCKS
    }

    pub fn health_text(&self) -> String {
        format!("{:.1} / {:.1} HP", self.vitals.health.max(0.0), self.vitals.max_health.max(0.0))
    }

    pub fn compact_hearts(&self) -> String {
        let max = self.vitals.max_health.max(1.0);
        if max > 20.0 {
            return format!("♥ {:.1}/{:.1}", self.vitals.health.max(0.0) / 2.0, max / 2.0);
        }
        let slots = ((max / 2.0).ceil() as usize).clamp(1, 10);
        let full = (self.vitals.health.max(0.0) / 2.0).floor() as usize;
        let half = self.vitals.health.max(0.0) - full as f32 * 2.0 >= 1.0;
        (0..slots).map(|i| if i < full { '♥' } else if i == full && half { '♥' } else { '♡' }).collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FogDistances {
    pub environmental_start: f32,
    pub environmental_end: f32,
    pub render_start: f32,
    pub render_end: f32,
    pub sky_end: f32,
    pub cloud_end: f32,
}

impl FogDistances {
    pub fn without_fog(mut self, enabled: bool) -> Self {
        if enabled {
            self.environmental_start = 30_000_000.0;
            self.environmental_end = 40_000_000.0;
            self.render_start = 30_000_000.0;
            self.render_end = 40_000_000.0;
            self.sky_end = 40_000_000.0;
            self.cloud_end = 40_000_000.0;
        }
        self
    }
}

/// Utility Fullbright must never override Horizon's held safety-disable state.
pub fn resolve_fullbright(horizon_enabled: bool, utility_enabled: bool, safety_held: bool) -> bool {
    (horizon_enabled || utility_enabled) && !safety_held
}

#[cfg(test)]]
mod tests {
    use super::*;

    #[test]
    fn armor_hud_counts_food_and_equipment() {
        let mut hud = ArmorHud::default();
        hud.add_equipment("Head", "minecraft:iron_helmet", 165, 100, 1, false);
        hud.add_food_stack(16);
        hud.add_food_stack(7);
        assert_eq!(hud.food_total, 23);
        assert_eq!(hud.equipment[0].row.durability_remaining, Some(65));
        assert!(!hud.is_empty());
    }

    #[test]
    fn entity_hud_enforces_range_and_formats_hearts() {
        let hud = EntityHud { entity_name: "minecraft:zombie".into(), distance_blocks: 12.0, vitals: EntityVitals::new(5.0, 20.0, 0.0) };
        assert!(hud.is_in_range());
        assert_eq!(hud.compact_hearts(), "♥♥♡♡♡♡♡♡♡♡");
        assert_eq!(hud.health_text(), "5.0 / 20.0 HP");
    }

    #[test]
    fn fog_and_fullbright_respect_disabled_states() {
        let fog = FogDistances { environmental_start: 1.0, environmental_end: 2.0, render_start: 3.0, render_end: 4.0, sky_end: 5.0, cloud_end: 6.0 };
        assert_eq!(fog.without_fog(false), fog);
        assert_eq!(fog.without_fog(true).cloud_end, 40_000_000.0);
        assert!(!resolve_fullbright(true, true, true));
        assert!(resolve_fullbright(false, true, false));
    }
}
