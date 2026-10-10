package fr.utilitymod.mixin;

import fr.utilitymod.UtilityModClient;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.ModifyVariable;

/**
 * Fog : on repousse tous les paramètres de brouillard très loin (Overworld, Nether, End, eau, lave).
 * Les 6 floats de updateBuffer sont : envStart, envEnd, renderDistStart, renderDistEnd, skyEnd, cloudEnd.
 */
@Mixin(targets = "net.minecraft.client.renderer.fog.FogRenderer")
public abstract class FogMixin {
    private static final float START = 3.0E7f;
    private static final float END = 4.0E7f;

    @ModifyVariable(method = "updateBuffer", at = @At("HEAD"), argsOnly = true, ordinal = 0, require = 0)
    private float utilitymod$envStart(float v) { return UtilityModClient.noFog ? START : v; }

    @ModifyVariable(method = "updateBuffer", at = @At("HEAD"), argsOnly = true, ordinal = 1, require = 0)
    private float utilitymod$envEnd(float v) { return UtilityModClient.noFog ? END : v; }

    @ModifyVariable(method = "updateBuffer", at = @At("HEAD"), argsOnly = true, ordinal = 2, require = 0)
    private float utilitymod$rdStart(float v) { return UtilityModClient.noFog ? START : v; }

    @ModifyVariable(method = "updateBuffer", at = @At("HEAD"), argsOnly = true, ordinal = 3, require = 0)
    private float utilitymod$rdEnd(float v) { return UtilityModClient.noFog ? END : v; }

    @ModifyVariable(method = "updateBuffer", at = @At("HEAD"), argsOnly = true, ordinal = 4, require = 0)
    private float utilitymod$skyEnd(float v) { return UtilityModClient.noFog ? END : v; }

    @ModifyVariable(method = "updateBuffer", at = @At("HEAD"), argsOnly = true, ordinal = 5, require = 0)
    private float utilitymod$cloudEnd(float v) { return UtilityModClient.noFog ? END : v; }
}
