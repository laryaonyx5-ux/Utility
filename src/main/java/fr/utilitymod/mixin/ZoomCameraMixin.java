package fr.utilitymod.mixin;

import fr.utilitymod.ZoomState;
import net.minecraft.client.MouseHandler;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

/**
 * Reproduit la sensibilité relative de Zoomify à 100 % :
 * les mouvements sont divisés par le facteur de zoom.
 * Aucun lissage ni inertie n'est ajouté.
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

        double divisor = Math.max(1.0, ZoomState.factor);
        this.accumulatedDX /= divisor;
        this.accumulatedDY /= divisor;
    }
}
