package bcs.gui.text;

public class BcsFont {
    public enum StyleHint {
        Helvetica(0),
        Times(1),
        Courier(2),
        OldEnglish(3),
        System(4),
        AnyStyle(5),
        Cursive(6),
        Monospace(7),
        Fantasy(8);

        private final int value;
        StyleHint(int value) { this.value = value; }
        public int getValue() { return value; }
    }

    public enum StyleStrategy {
        PreferDefault(0x0001),
        PreferBitmap(0x0002),
        PreferDevice(0x0004),
        PreferOutline(0x0008),
        ForceOutline(0x0010),
        PreferMatch(0x0020),
        PreferQuality(0x0040),
        PreferAntialias(0x0080),
        NoAntialias(0x0100),
        NoSubpixelAntialias(0x0800),
        PreferNoShaping(0x1000),
        ContextFontMerging(0x2000),
        PreferTypoLineMetrics(0x4000),
        NoFontMerging(0x8000);

        private final int value;
        StyleStrategy(int value) { this.value = value; }
        public int getValue() { return value; }
    }

    public enum Weight {
        Thin(0),
        ExtraLight(12),
        Light(25),
        Normal(50),
        Medium(57),
        DemiBold(63),
        Bold(75),
        ExtraBold(81),
        Black(87);

        private final int value;
        Weight(int value) { this.value = value; }
        public int getValue() { return value; }
    }

    private String family;
    private double pointSize;
    private int pixelSize;
    private Weight weight;
    private boolean italic;
    private boolean underline;
    private boolean strikeOut;
    private boolean fixedPitch;
    private StyleHint styleHint;
    private StyleStrategy styleStrategy;

    public BcsFont(String family, double pointSize, Weight weight, boolean italic) {
        this.family = family;
        this.pointSize = pointSize;
        this.pixelSize = -1;
        this.weight = weight;
        this.italic = italic;
        this.underline = false;
        this.strikeOut = false;
        this.fixedPitch = false;
        this.styleHint = StyleHint.AnyStyle;
        this.styleStrategy = StyleStrategy.PreferDefault;
    }

    public void setFamily(String family) {
        this.family = family;
    }

    public String getFamily() {
        return this.family;
    }

    public void setPointSize(double size) {
        this.pointSize = size;
        this.pixelSize = -1;
    }

    public double getPointSize() {
        return this.pointSize;
    }

    public void setPixelSize(int size) {
        this.pixelSize = size;
        this.pointSize = -1;
    }

    public int getPixelSize() {
        return this.pixelSize;
    }

    public void setWeight(Weight weight) {
        this.weight = weight;
    }

    public Weight getWeight() {
        return this.weight;
    }

    public void setItalic(boolean italic) {
        this.italic = italic;
    }

    public boolean isItalic() {
        return this.italic;
    }
}
