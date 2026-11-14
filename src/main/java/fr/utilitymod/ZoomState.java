package fr.utilitymod;

import net.minecraft.client.Minecraft;

/**
 * Zoom instantané : le FOV reste géré par Minecraft.
 * Pendant le zoom, la sensibilité est légèrement réduite pour ralentir
 * le déplacement de la caméra sans activer le lissage cinématique.
 */
public final class ZoomState {
    public static boolean active = false;
    public static double factor = 3.0;
    public static boolean cameraHooked = false;

    private static boolean smoothCameraBeforeZoom = false;
    private static double sensitivityBeforeZoom = 0.5;

    private ZoomState() {}

    /**
     * Réduit temporairement la sensibilité pendant le zoom, puis restaure
     * les réglages exacts de Minecraft lorsque le zoom est relâché.
     */
    public static void update(Minecraft mc, boolean shouldBeActive) {
        if (shouldBeActive && !active) {
            factor = 3.0;
            smoothCameraBeforeZoom = mc.options.smoothCamera;
            sensitivityBeforeZoom = mc.options.sensitivity().get();

            // 75 % de la sensibilité normale : léger ralentissement, sans inertie.
            mc.options.smoothCamera = false;
            mc.options.sensitivity().set(sensitivityBeforeZoom * 0.75);
            active = true;
            return;
        }

        if (!shouldBeActive && active) {
            mc.options.sensitivity().set(sensitivityBeforeZoom);
            mc.options.smoothCamera = smoothCameraBeforeZoom;
            active = false;
        }
    }
}
