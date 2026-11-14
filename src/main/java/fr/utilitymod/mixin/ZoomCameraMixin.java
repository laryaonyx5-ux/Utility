package fr.utilitymod.mixin;

import fr.utilitymod.ZoomState;
import net.minecraft.client.MouseHandler;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.ModifyVariable;

/**
 * Ralentit uniquement les deltas réels de souris pendant le zoom.
 * Aucun lissage temporel : zéro mouvement souris = zéro mouvement caméra.
 */
@Mixin(MouseHandler.class)
public abstract class ZoomCameraMixin {
    @ModifyVariable(
            method = "onMove(JDDDD)V",
            at = @At("HEAD"),
            argsOnly = true,
            ordinal = 2,
            require = 0
    )
    private double utilitymod$slowZoomMouseX(double deltaX) {
        return ZoomState.active ? deltaX * ZoomState.CAMERA_SPEED : deltaX;
    }

    @ModifyVariable(
            method = "onMove(JDDDD)V",
            at = @At("HEAD"),
            argsOnly = true,
            ordinal = 3,
            require = 0
    )
    private double utilitymod$slowZoomMouseY(double deltaY) {
        return ZoomState.active ? deltaY * ZoomState.CAMERA_SPEED : deltaY;
    }
}
