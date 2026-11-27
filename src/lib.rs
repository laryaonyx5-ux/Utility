//! Utility HUD — Rust port core for Starling/Horizon.
 //!
 //! This crate ports the behavior/state calculations from the Fabric Java mod.
 //! Game-facing injection adapters live under `mixins/utilitymod`; they must
 //! be compiled against the exact Starling/Horizon source revision pinned in
 //! horizon-native.json.

pub mod runtime;

pub use runtime::{Config, EntityVitals, HudRow, Zoom};

/// Native-mod identity used by the Starling compatibility mod list.
pub const MOD_ID: &str = "utilitymod";
pub const VERSION: &str = "1.1.0";

#[cfg(test)]
mod tests {
    use super::runtime::*;

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
}
