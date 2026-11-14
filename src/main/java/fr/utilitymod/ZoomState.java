package fr.utilitymod;

import net.minecraft.client.Minecraft;

/**
 * Zoom instantané : le FOV reste géré par Minecraft.
 * Le ralentissement de la souris est appliqué séparément par ZoomCameraMixin,
 * uniquement pendant le zoom. Les réglages vanilla ne sont jamais modifiés.
 */
public final class ZoomState {
    public static boolean active = false;
    public static double factor = 3.0;
    public static final double CAMERA_SPEED = 0.75;

    private ZoomState() {}

    public static void update(Minecraft mc, boolean shouldBeActive) {
        if (shouldBeActive && !active) {
            factor = 3.0;
            active = true;
        } else if (!shouldBeActive && active) {
            active = false;
        }
    }
}
