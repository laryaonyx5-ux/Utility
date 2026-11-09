package fr.utilitymod.mixin;

import fr.utilitymod.UtilityModClient;
import net.minecraft.client.Camera;
import net.minecraft.world.level.material.FogType;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Makes Iris shaderpacks stop applying their separate underwater/lava fog
 * while Utility Mod's Fog toggle is enabled.
 */
@Mixin(Camera.class)
public abstract class CameraMixin {
    @Inject(method = "getFluidInCamera", at = @At("RETURN"), cancellable = true)
    private void utilitymod$clearFluidFog(CallbackInfoReturnable<FogType> cir) {
        if (UtilityModClient.noFog) {
            FogType fluid = cir.getReturnValue();
            if (fluid == FogType.WATER || fluid == FogType.LAVA) {
                cir.setReturnValue(FogType.NONE);
            }
        }
    }
}
