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
    // BcsAlignment mapping (using int constants since Java Enums don't support bitwise easily)
    public static final int ALIGN_LEFT = 0x0001;
    public static final int ALIGN_RIGHT = 0x0002;
    public static final int ALIGN_HCENTER = 0x0004;
    public static final int ALIGN_JUSTIFY = 0x0008;
    public static final int ALIGN_TOP = 0x0020;
    public static final int ALIGN_BOTTOM = 0x0040;
    public static final int ALIGN_VCENTER = 0x0080;
    public static final int ALIGN_CENTER = ALIGN_VCENTER | ALIGN_HCENTER;

    public enum BcsOrientation {
        HORIZONTAL(1),
        VERTICAL(2);

        public final int value;
        private BcsOrientation(int value) {
            this.value = value;
        }
    }
}
