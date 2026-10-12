package fr.utilitymod;

import java.lang.reflect.Field;

public final class ZoomState {
    public static boolean active = false;
    public static double factor = 3.0;
    /** true des que le mixin de la camera a tourne au moins une fois : il gere alors tout le zoom. */
    public static boolean cameraHooked = false;
    private ZoomState() {}

    private static Field sensField;
    private static boolean sensFailed = false;

    /** Vrai si "inst" est l'option de sensibilite de la souris (recherche par reflexion, sans risque de plantage). */
    public static boolean isSensitivity(Object options, Object inst) {
        if (sensFailed) return false;
        try {
            if (sensField == null) {
                sensField = options.getClass().getDeclaredField("sensitivity");
                sensField.setAccessible(true);
            }
            return sensField.get(options) == inst;
        } catch (Throwable t) {
            sensFailed = true;
            return false;
        }
    }
}
