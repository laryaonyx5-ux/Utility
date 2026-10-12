package fr.utilitymod.mixin;

import fr.utilitymod.UtilityModClient;
import fr.utilitymod.ZoomState;
import net.minecraft.client.Minecraft;
import net.minecraft.client.OptionInstance;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Fullbright : on force le gamma (luminosite) tres haut.
 * Zoom : ralentit la souris (sensibilite dynamique) ; et sert de secours pour le FOV si le mixin camera ne tourne pas.
 */
@Mixin(OptionInstance.class)
public abstract class FullbrightMixin<T> {
    @SuppressWarnings("unchecked")
    @Inject(method = "get", at = @At("RETURN"), cancellable = true, require = 0)
    private void utilitymod$get(CallbackInfoReturnable<T> cir) {
        Minecraft mc = Minecraft.getInstance();
        if (mc == null || mc.options == null) return;

        if (UtilityModClient.fullbright && mc.options.gamma() == (Object) this) {
            cir.setReturnValue((T) (Object) Double.valueOf(15.0));
        } else if (ZoomState.active && !ZoomState.cameraHooked && mc.options.fov() == (Object) this
                && cir.getReturnValue() instanceof Integer base) {
            // Secours (limite a 1 degre) : uniquement si le mixin camera n'a jamais tourne.
            int zoomed = (int) Math.max(1, Math.round(base / ZoomState.factor));
            cir.setReturnValue((T) (Object) Integer.valueOf(zoomed));
        } else if (ZoomState.active && cir.getReturnValue() instanceof Double s
                && ZoomState.isSensitivity(mc.options, this)) {
            // Sensibilite dynamique : la souris ralentit quand on zoome (formule vanilla : (0.6 s + 0.2)^3).
            double k = Math.max(1.0, ZoomState.factor);
            double t = Math.max(0.01, (s * 0.6 + 0.2) / Math.cbrt(k));
            cir.setReturnValue((T) (Object) Double.valueOf((t - 0.2) / 0.6));
        }
    }
}
