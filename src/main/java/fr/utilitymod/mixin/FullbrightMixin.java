package fr.utilitymod.mixin;

import fr.utilitymod.UtilityModClient;
import net.minecraft.client.Minecraft;
import net.minecraft.client.OptionInstance;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

/** Fullbright : on force la valeur de gamma (luminosité) à une valeur énorme. */
@Mixin(OptionInstance.class)
public abstract class FullbrightMixin<T> {
    @SuppressWarnings("unchecked")
    @Inject(method = "get", at = @At("HEAD"), cancellable = true, require = 0)
    private void utilitymod$fullbright(CallbackInfoReturnable<T> cir) {
        if (!UtilityModClient.fullbright) return;
        Minecraft mc = Minecraft.getInstance();
        if (mc != null && mc.options != null && mc.options.gamma() == (Object) this) {
            cir.setReturnValue((T) (Object) Double.valueOf(15.0));
        }
    }
}
