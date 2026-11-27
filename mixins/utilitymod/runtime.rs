// Game-side state and adapters for Utility HUD's Starling native build.
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

static ZOOM_ACTIVE: AtomicBool = AtomicBool::new(false);
static ZOOM_FACTOR_BITS: AtomicU32 = AtomicU32::new(3.0_f32.to_bits());

impl Game<'_> {
    pub(crate) fn utilitymod_zoom(&mut self, held: bool) {
        let was_active = ZOOM_ACTIVE.load(Ordering::Relaxed);
        if held && !was_active {
            ZOOM_FACTOR_BITS.store(3.0_f32.to_bits(), Ordering::Relaxed);
        }
        ZOOM_ACTIVE.store(held, Ordering::Relaxed);
    }

    pub(crate) fn utilitymod_zoom_active(&self) -> bool {
        ZOOM_ACTIVE.load(Ordering::Relaxed)
    }

    pub(crate) fn utilitymod_zoom_factor(&self) -> f32 {
        f32::from_bits(ZOOM_FACTOR_BITS.load(Ordering::Relaxed)).clamp(1.0, 30.0)
    }

    pub(crate) fn utilitymod_zoom_scroll(&mut self, steps: f32) {
        let old = self.utilitymod_zoom_factor();
        let next = (old * 1.2_f32.powf(steps)).clamp(1.0, 30.0);
        ZOOM_FACTOR_BITS.store(next.to_bits(), Ordering::Relaxed);
    }

    pub(crate) fn utilitymod_apply_fov(&self, modifier: &mut f32) {
        if self.native_on("utilitymod") && self.utilitymod_zoom_active() {
            *modifier /= self.utilitymod_zoom_factor().max(1.0);
        }
    }
}
