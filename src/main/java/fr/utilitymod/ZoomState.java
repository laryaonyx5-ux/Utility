package fr.utilitymod;

import net.minecraft.client.Minecraft;

/**
 * Zoom instantané : seul le champ de vision est modifié.
 * La caméra, sa rotation et la sensibilité restent celles de Minecraft.
 */
public final class ZoomState {
    public static boolean active = false;
    public static double factor = 3.0;
    public static boolean cameraHooked = false;

    private ZoomState() {}

    /**
     * Désactive le lissage cinématique pour conserver le mouvement normal
     * de la caméra Minecraft, pendant et après l'utilisation du zoom.
     */
    public static void update(Minecraft mc, boolean shouldBeActive) {
        mc.options.smoothCamera = false;

        if (shouldBeActive && !active) {
            factor = 3.0;
        }

        active = shouldBeActive;
    }
}
