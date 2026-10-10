package fr.utilitymod.mixin;

import fr.utilitymod.ZoomState;
import net.minecraft.client.renderer.GameRenderer;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

@Mixin(GameRenderer.class)
public abstract class ZoomFovMixin {
    @Inject(method = "getFov", at = @At("RETURN"), cancellable = true, require = 0)
    private void utilitymod$zoom(CallbackInfoReturnable<Float> cir) {
        if (ZoomState.active) {
            cir.setReturnValue(Math.max(0.0001f, cir.getReturnValue() / (float) ZoomState.factor));
        }
    }
}
