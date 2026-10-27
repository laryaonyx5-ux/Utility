package fr.utilitymod.mixin;

import fr.utilitymod.UtilityModClient;
import net.minecraft.client.renderer.LightTexture;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/**
 * Compatibilité Fullbright avec les rendus qui réutilisent la luminosité
 * calculée par Minecraft pour construire la lightmap.
 *
 * Le gamma seul est ignoré par certains shaders. Ici, quand Fullbright est
 * activé, chaque niveau d'éclairage renvoie la luminosité maximale.
 * Un shader qui recalcule entièrement son éclairage en GLSL peut encore
 * appliquer ses propres ombres : cette injection ne modifie pas le pack.
 */
@Mixin(LightTexture.class)
public abstract class FullbrightLightmapMixin {
    @Inject(method = "getBrightness", at = @At("RETURN"), cancellable = true, require = 0)
    private static void utilitymod$maxLightmapBrightness(
            CallbackInfoReturnable<Float> cir) {
        if (UtilityModClient.fullbright) {
            cir.setReturnValue(1.0F);
        }
    }
}
