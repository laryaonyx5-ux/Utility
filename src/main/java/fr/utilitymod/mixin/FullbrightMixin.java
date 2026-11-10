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
 * Le zoom garde la sensibilite et la rotation normales de Minecraft.
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
            // Secours si le mixin camera ne tourne pas : zoom sans animation.
            int zoomed = (int) Math.max(1, Math.round(base / ZoomState.factor));
            cir.setReturnValue((T) (Object) Integer.valueOf(zoomed));
        }
    }
}
