//! Utility HUD — Rust port core for Starling/Horizon.
 //!
 //! This crate ports the behavior/state calculations from the Fabric Java mod.
 //! Game-facing injection adapters live under `mixins/utilitymod`; they must
 //! be compiled against the exact Starling/Horizon source revision pinned in
 //! horizon-native.json.

pub mod runtime;
pub mod presentation;

pub use runtime::{Config, EntityVitals, HudRow, Zoom, entity_vitals, fullbright_enabled, replace_entity_vitals, set_fullbright_enabled};
pub use presentation::{ArmorHud, EntityHud, FogDistances, resolve_fullbright};

/// Native-mod identity used by the Starling compatibility mod list.
pub const MOD_ID: &str = "utilitymod";
pub const VERSION: &str = "1.1.0";

#[cfg(test)]
mod tests {
    use super::runtime::*;

    #[test]
    fn native_entity_vitals_snapshot_replaces_old_entities() {
        replace_entity_vitals([(42, EntityVitals::new(7.0, 20.0, 2.0))]);
        assert_eq!(entity_vitals(42), Some(EntityVitals::new(7.0, 20.0, 2.0)));
        assert_eq!(entity_vitals(43), None);
        replace_entity_vitals([]);
        assert_eq!(entity_vitals(42), None);
    }

    #[test]
    fn toggles_persist_as_config_values() {
        let mut config = Config::default();
        config.toggle_armor();
        config.toggle_entity();
        config.toggle_fullbright();
        config.toggle_fog();
        assert_eq!(config, Config {
            armor_hud: true,
            entity_hud: true,
            fullbright: true,
            no_fog: true,
        });
    }

    #[test]
    fn zoom_matches_java_wheel_and_hold_behavior() {
        let mut zoom = Zoom::default();
        zoom.update(true);
        assert!(zoom.active);
        assert_eq!(zoom.factor, 3.0);
        zoom.scroll(1.0);
        assert!((zoom.factor - 3.6).abs() < 0.0001);
        assert!((zoom.mouse_divisor() - 3.6).abs() < 0.0001);
        zoom.scroll(100.0);
        assert_eq!(zoom.factor, 30.0);
        zoom.scroll(-100.0);
        assert_eq!(zoom.factor, 1.0);
        zoom.scroll(f64::NAN);
        assert_eq!(zoom.factor, 1.0);
        zoom.update(false);
        assert!(!zoom.active);
    }

    #[test]
    fn armor_hud_marks_low_durability_and_computes_remaining() {
        let row = HudRow::from_item("minecraft:iron_helmet", 20, 17, 1, false);
        assert_eq!(row.durability_remaining, Some(3));
        assert_eq!(row.durability_max, Some(20));
        assert!(row.low_durability);
    }

    #[test]
    fn entity_health_uses_full_and_half_hearts() {
        let vitals = EntityVitals::new(7.0, 20.0, 3.0);
        assert_eq!(vitals.health_full_hearts, 3);
        assert!(vitals.health_half_heart);
        assert_eq!(vitals.absorption_full_hearts, 1);
        assert!(vitals.absorption_half_heart);
    }

    #[test]
    fn nondamageable_items_have_no_durability_alert() {
        let row = HudRow::from_item("minecraft:apple", 0, 0, 12, false);
        assert_eq!(row.durability_remaining, None);
        assert_eq!(row.durability_max, None);
        assert!(!row.low_durability);
        assert_eq!(row.stack_count, 12);
    }

    #[test]
    fn zoom_wheel_does_nothing_when_not_held() {
        let mut zoom = Zoom::default();
        zoom.scroll(5.0);
        assert_eq!(zoom.factor, 3.0);
        assert!(!zoom.active);
    }

    #[test]
    fn zero_health_and_absorption_never_make_negative_hearts() {
        let vitals = EntityVitals::new(-4.0, 10.0, -2.0);
        assert_eq!(vitals.health_full_hearts, 0);
        assert!(!vitals.health_half_heart);
        assert_eq!(vitals.absorption_full_hearts, 0);
        assert!(!vitals.absorption_half_heart);
    }
}
