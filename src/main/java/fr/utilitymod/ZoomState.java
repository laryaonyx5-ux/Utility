package fr.utilitymod;

import net.minecraft.client.Minecraft;

/**
 * Etat du zoom : aucune interpolation et aucun pilotage de la rotation.
 * Le zoom ne modifie que le champ de vision dans ZoomFovMixin.
 */
public final class ZoomState {
    public static boolean active = false;
    public static double factor = 3.0;

    private static boolean smoothCameraBeforeZoom = false;

    private ZoomState() {}

    /**
     * Désactive le mode caméra cinématique pendant le zoom, puis restaure
     * exactement le réglage que le joueur avait avant d'activer le zoom.
     */
    public static void update(Minecraft mc, boolean shouldBeActive) {
        if (shouldBeActive) {
            if (!active) {
                factor = 3.0;
                smoothCameraBeforeZoom = mc.options.smoothCamera;
            }

            // Le zoom ne doit jamais activer le lissage de la caméra.
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
