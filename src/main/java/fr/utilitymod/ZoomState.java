package fr.utilitymod;

import net.minecraft.client.Minecraft;

/**
 * Etat du zoom instantané.
 * La rotation et la sensibilité de la souris restent celles de Minecraft.
 */
public final class ZoomState {
    public static boolean active = false;
    public static double factor = 3.0;
    public static boolean cameraHooked = false;

    private static boolean smoothCameraBeforeZoom = false;

    private ZoomState() {}

    /**
     * Coupe le mode caméra cinématique pendant le zoom et restaure le réglage
     * précédent lorsque la touche de zoom est relâchée.
     */
    public static void update(Minecraft mc, boolean shouldBeActive) {
        if (shouldBeActive) {
            if (!active) {
                factor = 3.0;
                smoothCameraBeforeZoom = mc.options.smoothCamera;
            }

            mc.options.smoothCamera = false;
            active = true;
            return;
        }

        if (active) {
            mc.options.smoothCamera = smoothCameraBeforeZoom;
            active = false;
        }
    }
}
