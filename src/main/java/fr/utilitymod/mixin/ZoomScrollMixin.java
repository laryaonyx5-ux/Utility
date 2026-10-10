package fr.utilitymod.mixin;

import fr.utilitymod.ZoomState;
import net.minecraft.client.MouseHandler;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

@Mixin(MouseHandler.class)
public abstract class ZoomScrollMixin {
    @Inject(method = "onScroll", at = @At("HEAD"), cancellable = true, require = 0)
    private void utilitymod$scroll(long window, double xOffset, double yOffset, CallbackInfo ci) {
        if (!ZoomState.active || yOffset == 0) return;
        ZoomState.factor = Math.max(1.0, ZoomState.factor * Math.pow(1.2, yOffset));
        ci.cancel();
    }
}
