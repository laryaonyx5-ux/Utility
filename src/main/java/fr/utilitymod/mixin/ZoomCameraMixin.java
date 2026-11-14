package fr.utilitymod.mixin;

import fr.utilitymod.ZoomState;
import net.minecraft.client.MouseHandler;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/**
 * Ralentit les deltas accumulés juste avant leur application à la caméra.
 * Les deltas sont consommés ensuite par Minecraft : ils ne sont donc réduits
 * qu'une seule fois. Hors zoom, les valeurs vanilla restent intactes.
 */
@Mixin(MouseHandler.class)
public abstract class ZoomCameraMixin {
    @Shadow
    private double accumulatedDX;

    @Shadow
    private double accumulatedDY;

    @Inject(method = "handleAccumulatedMovement", at = @At("HEAD"))
    private void utilitymod$slowZoomCamera(CallbackInfo ci) {
        if (!ZoomState.active) return;

        this.accumulatedDX *= ZoomState.CAMERA_SPEED;
        this.accumulatedDY *= ZoomState.CAMERA_SPEED;
    }
}
