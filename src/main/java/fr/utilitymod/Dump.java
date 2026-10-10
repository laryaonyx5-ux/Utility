package fr.utilitymod;

import net.fabricmc.loader.api.FabricLoader;
import net.minecraft.client.Minecraft;

import java.io.File;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.jar.JarFile;

/** Écrit utilitymod-dump.txt (dans le dossier du jeu) avec les noms réels de méthodes de cette version. */
public final class Dump {
    private Dump() {}

    private static final String FILTER = "(?i).*(fov|fog|zoom|scroll|hide|camera|extract|gamma|screen|overlay).*";

    public static void run() {
        StringBuilder sb = new StringBuilder("Minecraft dump for Utility HUD\n");
        List<String> fogClasses = new ArrayList<>();
        try {
            File jar = new File(Minecraft.class.getProtectionDomain().getCodeSource().getLocation().toURI());
            try (JarFile jf = new JarFile(jar)) {
                jf.stream().map(e -> e.getName()).filter(n -> n.endsWith(".class")
                        && n.startsWith("net/minecraft/client/renderer/fog/"))
                        .forEach(n -> fogClasses.add(n.replace('/', '.').replace(".class", "")));
            }
        } catch (Throwable t) {
            sb.append("jar scan failed: ").append(t).append('\n');
        }

        List<String> others = List.of(
                "net.minecraft.client.renderer.fog.FogRenderer",
                "net.minecraft.client.renderer.GameRenderer",
                "net.minecraft.client.Camera",
                "net.minecraft.client.MouseHandler",
                "net.minecraft.client.OptionInstance",
                "net.minecraft.client.Options",
                "net.minecraft.client.gui.Gui",
                "net.minecraft.client.Minecraft");

        for (String n : fogClasses) dump(sb, n, false);
        for (String n : others) if (!fogClasses.contains(n)) dump(sb, n, true);

        try {
            Path out = FabricLoader.getInstance().getGameDir().resolve("utilitymod-dump.txt");
            Files.writeString(out, sb.toString());
        } catch (Throwable ignored) {}
    }

    private static void dump(StringBuilder sb, String name, boolean filter) {
        try {
            Class<?> c = Class.forName(name, false, Dump.class.getClassLoader());
            sb.append("\n== ").append(name).append('\n');
            for (Method m : c.getDeclaredMethods()) {
                String line = m.toGenericString();
                if (!filter || line.matches(FILTER)) sb.append("M ").append(line).append('\n');
            }
            for (Field f : c.getDeclaredFields()) {
                String line = f.toGenericString();
                if (!filter || line.matches(FILTER)) sb.append("F ").append(line).append('\n');
            }
        } catch (Throwable t) {
            sb.append("\n== ").append(name).append(" : ").append(t).append('\n');
        }
    }
}
