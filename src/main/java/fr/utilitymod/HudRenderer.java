package fr.utilitymod;

import java.lang.reflect.Field;

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
    private static final float JADE_MAX_HEALTH_FOR_ICONS = 40.0f; // au-dessus : un coeur + nombre
    private static final int HEARTS_PER_ROW = 10;
    private static final Identifier ABSORB_FULL = Identifier.withDefaultNamespace("hud/heart/absorbing_full");
    private static final Identifier ABSORB_HALF = Identifier.withDefaultNamespace("hud/heart/absorbing_half");
    private static final Identifier HEART_CONTAINER = Identifier.withDefaultNamespace("hud/heart/container");
    private static final Identifier HEART_FULL = Identifier.withDefaultNamespace("hud/heart/full");
    private static final Identifier FOOD_ICON = Identifier.withDefaultNamespace("hud/food_full");
    private static final Identifier HEART_HALF = Identifier.withDefaultNamespace("hud/heart/half");

    private static final int ROW_H = 24; // place pour les "!!!" au-dessus de chaque icône
    private static final float LOW_DURABILITY = 0.20f; // <= 20 % = dans le rouge
    private static final int ALERT_COLOR = 0xFFFF2020;

    private static Entity target = null;
    private static String targetSource = "rayon";

    // TEMPORAIRE : ligne de controle sous le nom (type d entite + vie brute). Mettre false pour la retirer.
    private static final boolean DEBUG_LINE = true;

    private static Field crossField, hitField;
    private static boolean reflInit = false;

    /** Cible calculee par Minecraft lui-meme (comme Jade). Lecture par reflexion : pas de plantage si le champ n existe pas. */
    private static Entity vanillaTarget(Minecraft mc) {
        try {
            if (!reflInit) {
                reflInit = true;
                for (Field f : Minecraft.class.getDeclaredFields()) {
                    if (f.getName().equals("hitResult") && HitResult.class.isAssignableFrom(f.getType())) {
                        f.setAccessible(true); hitField = f;
                    }
                    if (f.getName().equals("crosshairPickEntity") && Entity.class.isAssignableFrom(f.getType())) {
                        f.setAccessible(true); crossField = f;
                    }
                }
            }
            if (hitField != null && hitField.get(mc) instanceof EntityHitResult r) return r.getEntity();
            if (crossField != null && crossField.get(mc) instanceof Entity e) return e;
        } catch (Throwable ignored) {
        }
        return null;
    }

    private HudRenderer() {}

    // ---------- Cible (Entity HUD) ----------
    public static void updateTarget(Minecraft mc) {
        target = null;
        if (!UtilityModClient.entityHud || mc.player == null || mc.level == null) return;
        if (hitField != null || crossField != null || !reflInit) {
            Entity v = vanillaTarget(mc);
            if (hitField != null || crossField != null) {
                targetSource = "vanilla";
                target = (v instanceof LivingEntity) ? v : null;
                return;
            }
        }
        targetSource = "rayon";
        Player p = mc.player;
        Vec3 eye = p.getEyePosition(1.0f);
        Vec3 look = p.getViewVector(1.0f);
        Vec3 end = eye.add(look.scale(RANGE));
        AABB box = p.getBoundingBox().expandTowards(look.scale(RANGE)).inflate(1.0);
        EntityHitResult hit = ProjectileUtil.getEntityHitResult(
                p, eye, end, box,
                e -> !e.isSpectator() && e.isPickable() && e instanceof LivingEntity
                        && e != p.getVehicle() && e != p,
                RANGE * RANGE);
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
        float absorb = le.getAbsorptionAmount();

        // Valeur numérique toujours affichée juste au-dessus de la barre de coeurs.
        String healthText = fmt(hp) + " / " + fmt(max) + " PV";
        g.text(mc.font, healthText, cx - mc.font.width(healthText) / 2, y, 0xFFFFFFFF, true);
        y += 11;

        if (max > JADE_MAX_HEALTH_FOR_ICONS) {
            // Pour les entités ayant beaucoup de vie, conserver l'affichage compact.
            String s = fmt(hp) + " / " + fmt(max) + " PV";
            String extra = absorb > 0 ? " +" + fmt(absorb) : "";
            int w = 11 + mc.font.width(s) + mc.font.width(extra);
            int x = cx - w / 2;
            g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_CONTAINER, x, y, 9, 9);
            g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_FULL, x, y, 9, 9);
            g.text(mc.font, s, x + 11, y + 1, 0xFFFFFFFF, true);
            if (!extra.isEmpty()) g.text(mc.font, extra, x + 11 + mc.font.width(s), y + 1, 0xFFFFE040, true);
            return;
        }

        // Comme Jade : rangees de 10 coeurs, puis les coeurs d'absorption (jaunes) en dessous.
        int n = (int) Math.ceil(max / 2.0f);
        int a = absorb > 0 ? (int) Math.ceil(absorb / 2.0f) : 0;
        int cols = Math.min(HEARTS_PER_ROW, Math.max(n, a));
        int x0 = cx - (cols * 8 + 1) / 2;
        int healthRows = (n + HEARTS_PER_ROW - 1) / HEARTS_PER_ROW;

        for (int i = 0; i < n; i++) {
            int x = x0 + (i % HEARTS_PER_ROW) * 8;
            int yy = y + (i / HEARTS_PER_ROW) * 10;
            g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_CONTAINER, x, yy, 9, 9);
            if (hp >= 2 * i + 2) {
                g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_FULL, x, yy, 9, 9);
            } else if (hp >= 2 * i + 1) {
                g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_HALF, x, yy, 9, 9);
            }
        }
        for (int j = 0; j < a; j++) {
            int x = x0 + (j % HEARTS_PER_ROW) * 8;
            int yy = y + (healthRows + j / HEARTS_PER_ROW) * 10;
            g.blitSprite(RenderPipelines.GUI_TEXTURED, HEART_CONTAINER, x, yy, 9, 9);
            if (absorb >= 2 * j + 2) {
                g.blitSprite(RenderPipelines.GUI_TEXTURED, ABSORB_FULL, x, yy, 9, 9);
            } else if (absorb >= 2 * j + 1) {
                g.blitSprite(RenderPipelines.GUI_TEXTURED, ABSORB_HALF, x, yy, 9, 9);
            }
        }
    }

    private static String fmt(float v) {
        return (v == Math.floor(v)) ? String.valueOf((int) v) : String.format("%.1f", v);
    }

    private static void renderArmor(GuiGraphicsExtractor g, Minecraft mc) {
        Player p = mc.player;
        int x = 4;
        int y = mc.getWindow().getGuiScaledHeight() / 2 - 70;

        EquipmentSlot[] slots = { EquipmentSlot.HEAD, EquipmentSlot.CHEST, EquipmentSlot.LEGS, EquipmentSlot.FEET };
        for (EquipmentSlot s : slots) {
            ItemStack st = p.getItemBySlot(s);
            if (!st.isEmpty()) { row(g, mc, st, x, y, false); y += ROW_H; }
        }

        ItemStack off = p.getOffhandItem();
        if (!off.isEmpty()) { row(g, mc, off, x, y, true); y += ROW_H; }

        int food = countFood(p);
        if (food > 0) {
            g.blitSprite(RenderPipelines.GUI_TEXTURED, FOOD_ICON, x + 3, y + 4, 9, 9);
            g.text(mc.font, "x" + food, x + 20, y + 4, 0xFFFFFFFF, true);
        }
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
        int max = st.getMaxDamage();
        if (max > 0 && (max - st.getDamageValue()) <= max * LOW_DURABILITY) {
            int w = mc.font.width("!!!");
            g.text(mc.font, "!!!", x + 8 - w / 2, y - 8, ALERT_COLOR, true);
        }
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
