package fr.utilitymod.mixin;

import fr.utilitymod.ZoomState;
import net.minecraft.client.Camera;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Zoom instantané basé uniquement sur le FOV calculé par Minecraft.
 * Aucune modification de yaw/pitch, de sensibilité ou de position caméra.
 */
@Mixin(Camera.class)
public abstract class ZoomFovMixin {
    @Inject(method = "calculateFov", at = @At("RETURN"), cancellable = true, require = 0)
    private void utilitymod$instantZoom(CallbackInfoReturnable<Float> cir) {
        if (!ZoomState.active) return;

        float vanillaFov = cir.getReturnValue();
        double factor = Math.max(1.0, ZoomState.factor);
        float zoomedFov = (float) (vanillaFov / factor);

        cir.setReturnValue(Math.max(1.0E-6f, zoomedFov));
    }
}
