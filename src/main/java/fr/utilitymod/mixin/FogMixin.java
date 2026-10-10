package fr.utilitymod.mixin;

import fr.utilitymod.UtilityModClient;
import net.minecraft.client.renderer.fog.FogData;
import net.minecraft.client.renderer.fog.FogRenderer;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Fog : on repousse tous les paramètres de brouillard très loin
 * (Overworld, Nether, End, eau, lave, neige poudreuse, cécité...).
 * Noms vérifiés avec le dump de la 26.3 : FogRenderer.setupFog(...) renvoie un FogData.
 */
@Mixin(FogRenderer.class)
public abstract class FogMixin {
    private static final float START = 3.0E7f;
    private static final float END = 4.0E7f;

    private static void utilitymod$clear(FogData d) {
        d.environmentalStart = START;
        d.environmentalEnd = END;
        d.renderDistanceStart = START;
        d.renderDistanceEnd = END;
        d.skyEnd = END;
        d.cloudEnd = END;
    }

    @Inject(method = "setupFog", at = @At("RETURN"), require = 0)
    private void utilitymod$setupFog(CallbackInfoReturnable<FogData> cir) {
        if (UtilityModClient.noFog && cir.getReturnValue() != null) {
            utilitymod$clear(cir.getReturnValue());
        }
    }

    @Inject(method = "updateBuffer(Lnet/minecraft/client/renderer/fog/FogData;)V", at = @At("HEAD"), require = 0)
    private void utilitymod$updateBuffer(FogData data, CallbackInfo ci) {
        if (UtilityModClient.noFog && data != null) {
            utilitymod$clear(data);
        }
    }
}
