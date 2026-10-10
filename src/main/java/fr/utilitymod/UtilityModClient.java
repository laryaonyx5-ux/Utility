package fr.utilitymod;

import com.mojang.blaze3d.platform.InputConstants;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents;
import net.fabricmc.fabric.api.client.keymapping.v1.KeyMappingHelper;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.hud.VanillaHudElements;
import net.minecraft.client.KeyMapping;
import net.minecraft.client.Minecraft;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;

public class UtilityModClient implements ClientModInitializer {
    public static final String MOD_ID = "utilitymod";

    public static boolean armorHud = false;
    public static boolean entityHud = false;
    public static boolean fullbright = false;
    public static boolean noFog = false;

    private static KeyMapping.Category category;
    private static KeyMapping kArmor, kEntity, kFullbright, kZoom, kFog;

    private static KeyMapping key(String id, int keyCode) {
        return KeyMappingHelper.registerKeyMapping(new KeyMapping(
                "key." + MOD_ID + "." + id, keyCode, category));
    }

    @Override
    public void onInitializeClient() {
        // 26.3 : GLFW a été remplacé par SDL -> toujours utiliser InputConstants.KEY_*
        category = KeyMapping.Category.register(Identifier.fromNamespaceAndPath(MOD_ID, "main"));
        kArmor      = key("armor_hud",  InputConstants.KEY_J);
        kEntity     = key("entity_hud", InputConstants.KEY_K);
        kFullbright = key("fullbright", InputConstants.KEY_L);
        kZoom       = key("zoom",       InputConstants.KEY_C);
        kFog        = key("fog",        InputConstants.KEY_N);

        ClientTickEvents.END_CLIENT_TICK.register(UtilityModClient::tick);

        HudElementRegistry.attachElementBefore(
                VanillaHudElements.CHAT,
                Identifier.fromNamespaceAndPath(MOD_ID, "hud"),
                (graphics, delta) -> HudRenderer.render(graphics));
    }

    private static void tick(Minecraft mc) {
        while (kArmor.consumeClick())      { armorHud   = !armorHud;   msg(mc, "Armor HUD", armorHud); }
        while (kEntity.consumeClick())     { entityHud  = !entityHud;  msg(mc, "Entity HUD", entityHud); }
        while (kFullbright.consumeClick()) { fullbright = !fullbright; msg(mc, "Fullbright", fullbright); }
        while (kFog.consumeClick())        { noFog      = !noFog;      msg(mc, "Fog", noFog); }

        boolean was = ZoomState.active;
        ZoomState.active = kZoom.isDown() && mc.mouseHandler.isMouseGrabbed() && mc.player != null;
        if (ZoomState.active && !was) ZoomState.factor = 3.0;

        HudRenderer.updateTarget(mc);
    }

    private static void msg(Minecraft mc, String name, boolean on) {
        if (mc.player != null) {
            mc.player.sendSystemMessage(Component.literal(name + (on ? " activé" : " désactivé")));
        }
    }
}
