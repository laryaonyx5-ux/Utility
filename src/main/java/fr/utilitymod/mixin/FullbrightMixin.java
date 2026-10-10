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
 * Fullbright : on force le gamma (luminosité) très haut.
 * Zoom : on divise la valeur de l'option FOV tant que la touche de zoom est maintenue.
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
        } else if (ZoomState.active && mc.options.fov() == (Object) this
                && cir.getReturnValue() instanceof Integer base) {
            ZoomState.baseFov = base;
            double wanted = base / ZoomState.factor;
            // L'option est un entier >= 1 : au-delà, ZoomFovMixin termine le zoom sur le FOV flottant.
            int zoomed = wanted <= 1.0 ? 1 : (int) Math.max(1, Math.round(wanted));
            cir.setReturnValue((T) (Object) Integer.valueOf(zoomed));
        }
    }
}
