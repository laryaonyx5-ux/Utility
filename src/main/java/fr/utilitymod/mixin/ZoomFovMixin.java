package fr.utilitymod.mixin;

import fr.utilitymod.ZoomState;
import net.minecraft.client.Camera;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Zoom infini : on divise le FOV flottant calcule par la camera (Camera.calculateFov, verifie dans le dump 26.3).
 * Contrairement a l'option FOV (entier >= 1), il n'y a aucune limite.
 */
@Mixin(Camera.class)
public abstract class ZoomFovMixin {
    @Inject(method = "calculateFov", at = @At("RETURN"), cancellable = true, require = 0)
    private void utilitymod$zoom(CallbackInfoReturnable<Float> cir) {
        ZoomState.cameraHooked = true;
        if (!ZoomState.active) return;
        float v = cir.getReturnValue();
        cir.setReturnValue(Math.max(1.0E-6f, (float) (v / ZoomState.factor)));
    }
}
