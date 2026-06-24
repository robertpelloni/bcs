package bcs.core.global;

public class BcsCoreTypes {
    public enum BcsGlobalColor {
        WHITE,
        BLACK,
        RED,
        GREEN,
        BLUE,
        TRANSPARENT
    }

    public static void warning(String msg) {
        System.out.println("[BCS Warning] " + msg);
    }

    public static void critical(String msg) {
        System.out.println("[BCS Critical] " + msg);
    }
}
