//! Utility's first Horizon mod_api port.
//!
//! The game-independent calculations remain in the root Rust crate. This
//! client adapter registers the original Java key bindings and exposes the
//! toggle state through Horizon's current mod API.

use mod_api::{
    self, ClientHost, ClientTickPostEvent, Hooks, KeyRegistration, Registrar,
    RegistrationContext, RenderFogEvent, RenderGuiPostEvent, SettingChangedEvent, SettingSpec, Text, Zoom, ZoomEvent,
};
use std::sync::Mutex;

mod hud;
use utilitymod::Config;

const ID: &str = "utilitymod";

const BINDS: &[(&str, &str, &str)] = &[
    ("armor_hud", "Armor HUD", "key.keyboard.j"),
    ("entity_hud", "Entity HUD", "key.keyboard.k"),
    ("fullbright", "Fullbright", "key.keyboard.l"),
    ("fog", "No Fog", "key.keyboard.n"),
    ("zoom", "Zoom", "key.keyboard.c"),
];

#[derive(Default)]
struct Utility {
    config: Mutex<Config>,
}

fn chat(host: &mut dyn ClientHost, label: &str, enabled: bool) {
    let state = if enabled { "activé" } else { "désactivé" };
    let _ = host.chat_line(Text::literal(format!("Utility — {label} {state}")));
}

#[mod_api::horizon_mod]
impl Hooks for Utility {
    fn register(&self, registrar: &mut dyn Registrar, cx: &RegistrationContext) {
        for (id, label, key) in BINDS {
            let mut mapping = KeyRegistration::new(id, key);
            mapping.label = format!("{ID}.key.{id}: {label}");
            registrar.register_key_mapping(mapping);
        }

        // These settings are stored by Horizon and survive game restarts.
        for (id, _label, _) in BINDS.iter().filter(|(id, _, _)| *id != "zoom") {
            let _ = registrar.register_setting(
                SettingSpec::toggle(
                    *id,
                    format!("setting.utilitymod.{id}"),
                    false,
                )
                .tooltip(format!("setting.utilitymod.{id}.tooltip")),
            );
            if let Ok(Some(value)) = cx.setting(ID, id) {
                if let Ok(mut config) = self.config.lock() {
                    let enabled = value == "true";
                    match *id {
                        "armor_hud" => config.armor_hud = enabled,
                        "entity_hud" => config.entity_hud = enabled,
                        "fullbright" => { config.fullbright = enabled; utilitymod::set_fullbright_enabled(enabled); },
                        "fog" => config.no_fog = enabled,
                        _ => {}
                    }
                }
            }
        }
        let _ = registrar.register_setting(
            SettingSpec::range("zoom_factor", "setting.utilitymod.zoom_factor", 1.0, 30.0, 0.1, 3.0)
                .tooltip("setting.utilitymod.zoom_factor.tooltip"),
        );
        let _ = registrar.register_setting(
            SettingSpec::range("zoom_turn", "setting.utilitymod.zoom_turn", 0.1, 1.0, 0.05, 0.45)
                .tooltip("setting.utilitymod.zoom_turn.tooltip"),
        );
    }

    /// Horizon's native zoom hook: hold C; strength and camera sensitivity are configurable.
    fn zoom(&self, host: &dyn ClientHost, _event: &mut ZoomEvent) -> Option<Zoom> {
        if !host.playing() || !host.is_down(ID, "zoom") {
            return None;
        }
        let factor = host.setting_f64("zoom_factor").unwrap_or(3.0).clamp(1.0, 30.0) as f32;
        let turn = host.setting_f64("zoom_turn").unwrap_or(0.45).clamp(0.1, 1.0) as f32;
        Some(Zoom { factor, smooth: false, turn })
    }

    /// Expand the final fog distances when No Fog is enabled.
    fn render_fog(&self, host: &dyn ClientHost, event: &mut RenderFogEvent<'_>) {
        if !host.playing() {
            return;
        }
        let enabled = self.config.lock().map(|config| config.no_fog).unwrap_or(false);
        if enabled {
            event.fog.environmental_start = 30_000_000.0;
            event.fog.environmental_end = 40_000_000.0;
            event.fog.render_distance_start = 30_000_000.0;
            event.fog.render_distance_end = 40_000_000.0;
            event.fog.sky_end = 40_000_000.0;
            event.fog.cloud_end = 40_000_000.0;
        }
    }

    fn setting_changed(&self, host: &mut dyn ClientHost, event: &mut SettingChangedEvent<'_>) {
        if event.mod_id != ID {
            return;
        }
        let Some(enabled) = host.setting_bool(event.key) else { return };
        if let Ok(mut config) = self.config.lock() {
            match event.key {
                "armor_hud" => config.armor_hud = enabled,
                "entity_hud" => config.entity_hud = enabled,
                "fullbright" => { config.fullbright = enabled; utilitymod::set_fullbright_enabled(enabled); },
                "fog" => config.no_fog = enabled,
                _ => {}
            }
        }
    }

    fn render_gui_post(&self, host: &dyn ClientHost, event: &mut RenderGuiPostEvent<'_>) {
        let (armor_enabled, entity_enabled) = self
            .config
            .lock()
            .map(|config| (config.armor_hud, config.entity_hud))
            .unwrap_or((false, false));
        hud::render(host, event, armor_enabled, entity_enabled);
    }

    fn client_tick_post(&self, host: &mut dyn ClientHost, _event: &mut ClientTickPostEvent) {
        if !host.playing() {
            return;
        }

        for (id, label, _) in BINDS {
            if !host.consume_click(ID, id) {
                continue;
            }

            let enabled = {
                let Ok(mut config) = self.config.lock() else {
                    continue;
                };
                match *id {
                    "armor_hud" => {
                        config.toggle_armor();
                        config.armor_hud
                    }
                    "entity_hud" => {
                        config.toggle_entity();
                        config.entity_hud
                    }
                    "fullbright" => {
                        config.toggle_fullbright();
                        config.fullbright
                    }
                    "fog" => {
                        config.toggle_fog();
                        config.no_fog
                    }
                    _ => continue,
                }
            };
            host.set_setting(ID, id, if enabled { "true" } else { "false" });
            chat(host, label, enabled);
        }
    }
}
