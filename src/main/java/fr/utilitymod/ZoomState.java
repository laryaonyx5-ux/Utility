package fr.utilitymod;

public final class ZoomState {
    public static boolean active = false;
    public static double factor = 3.0;
    /** FOV de base (option du jeu) mémorisé pendant le zoom, 0 si pas de zoom. */
    public static double baseFov = 0.0;
    /** true si le zoom de projection (champ "zoom" du GameRenderer) fonctionne : il remplace alors le zoom FOV supplémentaire. */
    public static boolean fieldOk = false;
    private ZoomState() {}

    /**
     * Part du zoom que l'option FOV (entier, minimum 1 degré) ne peut pas faire.
     * Appliquée en plus sur le FOV flottant => zoom sans limite.
     */
    public static double extra() {
        if (baseFov <= 0.0) return 1.0;
        return Math.max(1.0, factor / baseFov);
    }
}
