package fr.utilitymod;

import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.client.renderer.RenderPipelines;
import net.minecraft.core.component.DataComponents;
import net.minecraft.resources.Identifier;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EquipmentSlot;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.entity.projectile.ProjectileUtil;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.level.ClipContext;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.EntityHitResult;
import net.minecraft.world.phys.HitResult;
import net.minecraft.world.phys.Vec3;

public final class HudRenderer {
    private static final double RANGE = 64.0;
    private static final Identifier HEART_CONTAINER = Identifier.withDefaultNamespace("hud/heart/container");
    private static final Identifier HEART_FULL = Identifier.withDefaultNamespace("hud/heart/full");
    private static final Identifier FOOD_ICON = Identifier.withDefaultNamespace("hud/food_full");
    private static final Identifier HEART_HALF = Identifier.withDefaultNamespace("hud/heart/half");

    private static Entity target = null;

    private HudRenderer() {}

    // ---------- Cible (Entity HUD) ----------
    public static void updateTarget(Minecraft mc) {
        target = null;
        if (!UtilityModClient.entityHud || mc.player == null || mc.level == null) return;
        Player p = mc.player;
        Vec3 eye = p.getEyePosition(1.0f);
        Vec3 look = p.getViewVector(1.0f);
        Vec3 end = eye.add(look.scale(RANGE));
        AABB box = p.getBoundingBox().expandTowards(look.scale(RANGE)).inflate(1.0);
        EntityHitResult hit = ProjectileUtil.getEntityHitResult(
                p, eye, end, box, e -> !e.isSpectator() && e.isPickable(), RANGE * RANGE);
        if (hit == null) return;
        HitResult block = mc.level.clip(new ClipContext(
                eye, end, ClipContext.Block.COLLIDER, ClipContext.Fluid.NONE, p));
        double blockDist = block.getType() == HitResult.Type.MISS
                ? Double.MAX_VALUE : block.getLocation().distanceToSqr(eye);
        if (eye.distanceToSqr(hit.getLocation()) < blockDist) target = hit.getEntity();
    }

    // ---------- Rendu ----------
    public static void render(GuiGraphicsExtractor g) {
        Minecraft mc = Minecraft.getInstance();
        if (mc.player == null) return;
        if (UtilityModClient.entityHud) renderEntity(g, mc);
        if (UtilityModClient.armorHud) renderArmor(g, mc);
    }

    private static void renderEntity(GuiGraphicsExtractor g, Minecraft mc) {
        if (!(target instanceof LivingEntity le) || !le.isAlive()) return;
        int cx = mc.getWindow().getGuiScaledWidth() / 2;
        int y = 30;
        String name = le.getName().getString();
        g.text(mc.font, name, cx - mc.font.width(name) / 2, y, 0xFFFFFFFF, true);
        y += 12;

        float hp = le.getHealth();
        float max = le.getMaxHealth();

        // <= 10 coeurs actuels : rangee de coeurs comme en survie. Sinon : un coeur + "vie/vie max".
        if (hp <= 20.0f) {
            int n = Math.min(10, (int) Math.ceil(max / 2.0f));
            int x0 = cx - (n * 8 + 1) / 2;
            for (int i = 0; i < n; i++) {
                int x = x0 + i * 8;
                g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_CONTAINER, x, y, 9, 9);
                if (hp >= 2 * i + 2) {
                    g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_FULL, x, y, 9, 9);
                } else if (hp >= 2 * i + 1) {
                    g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_HALF, x, y, 9, 9);
                }
            }
        } else {
            String s = fmt(hp / 2.0f) + "/" + fmt(max / 2.0f);
            int w = 11 + mc.font.width(s);
            int x = cx - w / 2;
            g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_CONTAINER, x, y, 9, 9);
            g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_FULL, x, y, 9, 9);
            g.text(mc.font, s, x + 11, y + 1, 0xFFFFFFFF, true);
        }
    }

    private static String fmt(float v) {
        return (v == Math.floor(v)) ? String.valueOf((int) v) : String.format("%.1f", v);
    }

    private static void renderArmor(GuiGraphicsExtractor g, Minecraft mc) {
        Player p = mc.player;
        int x = 4;
        int y = mc.getWindow().getGuiScaledHeight() / 2 - 60;

        EquipmentSlot[] slots = { EquipmentSlot.HEAD, EquipmentSlot.CHEST, EquipmentSlot.LEGS, EquipmentSlot.FEET };
        for (EquipmentSlot s : slots) {
            ItemStack st = p.getItemBySlot(s);
            if (!st.isEmpty()) { row(g, mc, st, x, y, false); y += 20; }
        }

        ItemStack off = p.getOffhandItem();
        if (!off.isEmpty()) { row(g, mc, off, x, y, true); y += 20; }

        int food = countFood(p);
        g.blitSprite(RenderPipelines.GUI_TEXTURED, FOOD_ICON, x + 3, y + 4, 9, 9);
        g.text(mc.font, "x" + food, x + 20, y + 4, 0xFFFFFFFF, true);
    }

    /** Ligne pour un objet : durabilité si possible, sinon total dans l'inventaire. */
    private static void row(GuiGraphicsExtractor g, Minecraft mc, ItemStack st, int x, int y, boolean offhand) {
        String text;
        if (st.getMaxDamage() > 0) {
            text = (st.getMaxDamage() - st.getDamageValue()) + "/" + st.getMaxDamage();
        } else if (offhand && st.getMaxStackSize() > 1) {
            text = "x" + countItem(mc.player, st);
        } else {
            text = "";
        }
        row(g, mc, st, x, y, text);
    }

    private static void row(GuiGraphicsExtractor g, Minecraft mc, ItemStack st, int x, int y, String text) {
        g.item(st, x, y);
        if (st.isBarVisible()) {
            int w = st.getBarWidth();
            int c = st.getBarColor() | 0xFF000000;
            g.fill(x + 2, y + 13, x + 15, y + 15, 0xFF000000);
            g.fill(x + 2, y + 13, x + 2 + w, y + 14, c);
        }
        if (!text.isEmpty()) g.text(mc.font, text, x + 20, y + 4, 0xFFFFFFFF, true);
    }

    private static int countItem(Player p, ItemStack ref) {
        int total = 0;
        for (int i = 0; i < p.getInventory().getContainerSize(); i++) {
            ItemStack s = p.getInventory().getItem(i);
            if (!s.isEmpty() && ItemStack.isSameItem(s, ref)) total += s.getCount();
        }
        return total;
    }

    private static int countFood(Player p) {
        int total = 0;
        for (int i = 0; i < p.getInventory().getContainerSize(); i++) {
            ItemStack s = p.getInventory().getItem(i);
            if (!s.isEmpty() && s.get(DataComponents.FOOD) != null) total += s.getCount();
        }
        return total;
    }
}
