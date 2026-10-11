package fr.utilitymod;

import net.fabricmc.loader.api.FabricLoader;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Properties;

/** Sauvegarde l'état des touches (HUD armure, HUD entité, fullbright, brouillard) dans config/utilitymod.properties. */
public final class Config {
    private Config() {}

    private static Path file() {
        return FabricLoader.getInstance().getConfigDir().resolve("utilitymod.properties");
    }

    public static void load() {
        Path f = file();
        if (!Files.exists(f)) return;
        try (InputStream in = Files.newInputStream(f)) {
            Properties pr = new Properties();
            pr.load(in);
            UtilityModClient.armorHud   = Boolean.parseBoolean(pr.getProperty("armorHud", "false"));
            UtilityModClient.entityHud  = Boolean.parseBoolean(pr.getProperty("entityHud", "false"));
            UtilityModClient.fullbright = Boolean.parseBoolean(pr.getProperty("fullbright", "false"));
            UtilityModClient.noFog      = Boolean.parseBoolean(pr.getProperty("noFog", "false"));
        } catch (IOException ignored) {
        }
    }

    public static void save() {
        Properties pr = new Properties();
        pr.setProperty("armorHud", Boolean.toString(UtilityModClient.armorHud));
        pr.setProperty("entityHud", Boolean.toString(UtilityModClient.entityHud));
        pr.setProperty("fullbright", Boolean.toString(UtilityModClient.fullbright));
        pr.setProperty("noFog", Boolean.toString(UtilityModClient.noFog));
        try (OutputStream out = Files.newOutputStream(file())) {
            pr.store(out, "Utility HUD - etat des touches");
        } catch (IOException ignored) {
        }
    }
}
