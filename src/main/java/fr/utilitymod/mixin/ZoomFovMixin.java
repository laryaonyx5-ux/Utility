package fr.utilitymod.mixin;

import fr.utilitymod.ZoomState;
import net.minecraft.client.renderer.GameRenderer;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/** Zoom infini : divise le FOV flottant par la part de zoom que l'option entière ne peut pas faire. */
@Mixin(GameRenderer.class)
public abstract class ZoomFovMixin {
    @Inject(method = "getFov", at = @At("RETURN"), cancellable = true, require = 0)
    private void utilitymod$zoom(CallbackInfoReturnable<Float> cir) {
        if (!ZoomState.active) return;
        double extra = ZoomState.extra();
        if (extra > 1.0) {
            cir.setReturnValue(Math.max(1.0E-6f, (float) (cir.getReturnValue() / extra)));
        }
    }
}
