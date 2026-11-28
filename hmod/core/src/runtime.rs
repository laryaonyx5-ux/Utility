//! Behavior ported from Config.java, ZoomState.java and HudRenderer.java.
//! The Horizon adapter feeds these value types from the game and renders them
//! through its native HUD hook.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, atomic::{AtomicBool, AtomicU32, Ordering}};

static FULLBRIGHT_ENABLED: AtomicBool = AtomicBool::new(false);
static TARGET_ENTITY_ID: AtomicU32 = AtomicU32::new(u32::MAX);

/// Set the client HUD's current 64-block entity target.
pub fn set_target_entity_id(id: Option<u32>) {
    TARGET_ENTITY_ID.store(id.unwrap_or(u32::MAX), Ordering::Relaxed);
}

/// The network id currently selected by Utility's native target ray.
pub fn target_entity_id() -> Option<u32> {
    match TARGET_ENTITY_ID.load(Ordering::Relaxed) {
        u32::MAX => None,
        id => Some(id),
    }
}

static ENTITY_VITALS: LazyLock<Mutex<HashMap<u32, EntityVitals>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Replace the last client-frame snapshot of living entity health values.
pub fn replace_entity_vitals(values: impl IntoIterator<Item = (u32, EntityVitals)>) {
    if let Ok(mut snapshot) = ENTITY_VITALS.lock() {
        snapshot.clear();
        snapshot.extend(values);
    }
}

/// Read the latest client-frame health snapshot for a network entity id.
pub fn entity_vitals(id: u32) -> Option<EntityVitals> {
    ENTITY_VITALS.lock().ok()?.get(&id).copied()
}

/// Shared switch read by the native renderer mixin and written by the mod API.
pub fn set_fullbright_enabled(enabled: bool) {
    FULLBRIGHT_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Current Utility fullbright switch for the native renderer.
pub fn fullbright_enabled() -> bool {
    FULLBRIGHT_ENABLED.load(Ordering::Relaxed)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Config {
    pub armor_hud: bool,
    pub entity_hud: bool,
    pub fullbright: bool,
    pub no_fog: bool,
}

impl Config {
    pub fn toggle_armor(&mut self) { self.armor_hud = !self.armor_hud; }
    pub fn toggle_entity(&mut self) { self.entity_hud = !self.entity_hud; }
    pub fn toggle_fullbright(&mut self) { self.fullbright = !self.fullbright; set_fullbright_enabled(self.fullbright); }
    pub fn toggle_fog(&mut self) { self.no_fog = !self.no_fog; }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Zoom {
    pub active: bool,
    pub factor: f64,
}

impl Default for Zoom {
    fn default() -> Self { Self { active: false, factor: 3.0 } }
}

impl Zoom {
    /// Called each tick; zoom is held, not toggled.
    pub fn update(&mut self, held: bool) {
        if held && !self.active {
            self.factor = 3.0;
            self.active = true;
        } else if !held {
            self.active = false;
        }
    }

    /// Same exponential wheel step as the Java implementation.
    pub fn scroll(&mut self, wheel_delta: f64) {
        if self.active && wheel_delta.is_finite() && wheel_delta != 0.0 {
            let next = self.factor * 1.2_f64.powf(wheel_delta);
            self.factor = if next.is_finite() { next.clamp(1.0, 30.0) } else { 30.0 };
        }
    }

    pub fn fov(&self, vanilla_fov: f32) -> f32 {
        if self.active { (vanilla_fov / self.factor.max(1.0) as f32).max(0.000001) }
        else { vanilla_fov }
    }

    pub fn mouse_divisor(&self) -> f64 {
        if self.active { self.factor.max(1.0) } else { 1.0 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HudRow {
    pub item_id: String,
    pub durability_remaining: Option<u32>,
    pub durability_max: Option<u32>,
    pub stack_count: u32,
    pub low_durability: bool,
    pub offhand: bool,
}

impl HudRow {
    pub fn from_item(item_id: &str, max_damage: u32, damage: u32, count: u32, offhand: bool) -> Self {
        let (remaining, max) = if max_damage > 0 {
            (Some(max_damage.saturating_sub(damage)), Some(max_damage))
        } else { (None, None) };
        let low_durability = remaining.zip(max).is_some_and(|(r, m)| r as f32 <= m as f32 * 0.20);
        Self {
            item_id: item_id.to_owned(),
            durability_remaining: remaining,
            durability_max: max,
            stack_count: count,
            low_durability,
            offhand,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EntityVitals {
    pub health: f32,
    pub max_health: f32,
    pub absorption: f32,
    pub health_full_hearts: u32,
    pub health_half_heart: bool,
    pub absorption_full_hearts: u32,
    pub absorption_half_heart: bool,
}

impl EntityVitals {
    pub fn new(health: f32, max_health: f32, absorption: f32) -> Self {
        Self {
            health,
            max_health,
            absorption,
            health_full_hearts: (health.max(0.0) / 2.0).floor() as u32,
            health_half_heart: health.max(0.0) % 2.0 >= 1.0,
            absorption_full_hearts: (absorption.max(0.0) / 2.0).floor() as u32,
            absorption_half_heart: absorption.max(0.0) % 2.0 >= 1.0,
        }
    }
}

/// Entity HUD range and normal heart row limits retained from the Java source.
pub const ENTITY_RANGE_BLOCKS: f64 = 64.0;
pub const HEARTS_PER_ROW: usize = 10;
pub const LOW_DURABILITY_RATIO: f32 = 0.20;
