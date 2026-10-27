package fr.utilitymod.mixin;

import fr.utilitymod.UtilityModClient;
import net.minecraft.client.renderer.Lightmap;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Force la luminosité maximale dans le calcul vanilla de la lightmap.
 * Les shaders qui recalculent leur éclairage entièrement en GLSL peuvent
 * encore appliquer leurs propres ombres et nécessitent un patch du shader.
 */
@Mixin(Lightmap.class)
public abstract class FullbrightLightmapMixin {
    @Inject(method = "getBrightness", at = @At("RETURN"), cancellable = true, require = 0)
    private static void utilitymod$maxLightmapBrightness(
            CallbackInfoReturnable<Float> cir) {
        if (UtilityModClient.fullbright) {
            cir.setReturnValue(1.0F);
        }
    }
}
