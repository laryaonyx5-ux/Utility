//! Utility's first Horizon mod_api port.
//!
//! The game-independent calculations remain in the root Rust crate. This
//! client adapter registers the original Java key bindings and exposes the
//! toggle state through Horizon's current mod API.

use mod_api::{
    self, ClientHost, ClientTickPostEvent, Hooks, KeyRegistration, Registrar,
    RegistrationContext, Text,
};
use std::sync::Mutex;
use utilitymod::Config;

const ID: &str = "utilitymod";

const BINDS: &[(&str, &str, &str)] = &[
    ("armor_hud", "Armor HUD", "key.keyboard.j"),
    ("entity_hud", "Entity HUD", "key.keyboard.k"),
    ("fullbright", "Fullbright", "key.keyboard.l"),
    ("fog", "No Fog", "key.keyboard.n"),
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
    fn register(&self, registrar: &mut dyn Registrar, _cx: &RegistrationContext) {
        for (id, label, key) in BINDS {
            let mut mapping = KeyRegistration::new(id, key);
            mapping.label = format!("{ID}.key.{id}: {label}");
            registrar.register_key_mapping(mapping);
        }
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
            chat(host, label, enabled);
        }
    }
}
