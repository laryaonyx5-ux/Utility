package fr.utilitymod;

import net.minecraft.client.Minecraft;
import net.minecraft.client.renderer.GameRenderer;

import java.lang.reflect.Field;

/**
 * Zoom sans limite : au lieu du FOV (limite a 1 degre), on agrandit directement l'image
 * grace au champ "zoom" du GameRenderer (celui qui sert aux captures panoramiques).
 * Tout passe par reflexion : si le champ n'existe pas, rien ne plante.
 */
public final class ProjectionZoom {
    private ProjectionZoom() {}

    private static Field rendererField, zoomField;
    private static boolean failed = false;
    private static boolean applied = false;

    public static void apply(Minecraft mc) {
        if (failed) return;
        if (!ZoomState.active && !applied) return;
        try {
            if (zoomField == null) {
                for (Field f : Minecraft.class.getDeclaredFields()) {
                    if (f.getType() == GameRenderer.class) {
                        f.setAccessible(true);
                        rendererField = f;
                        break;
                    }
                }
                Field z = GameRenderer.class.getDeclaredField("zoom");
                if (rendererField == null || z.getType() != float.class) throw new NoSuchFieldException("zoom");
                z.setAccessible(true);
                zoomField = z;
            }
            Object gr = rendererField.get(mc);
            if (gr == null) return;
            zoomField.setFloat(gr, ZoomState.active ? (float) ZoomState.extra() : 1.0f);
            applied = ZoomState.active;
            ZoomState.fieldOk = true;
        } catch (Throwable t) {
            failed = true;
            ZoomState.fieldOk = false;
        }
    }
}
