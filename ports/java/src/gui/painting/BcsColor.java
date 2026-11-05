package bcs.gui.painting;

public class BcsColor {
    public enum ColorSpec {
        Invalid, Rgb, Hsv, Cmyk, Hsl, ExtendedRgb
    }

    private ColorSpec spec;
    private float a;
    private float r;
    private float g;
    private float b;

    public BcsColor() {
        this.spec = ColorSpec.Invalid;
        this.a = 0.0f;
        this.r = 0.0f;
        this.g = 0.0f;
        this.b = 0.0f;
    }

    public BcsColor(int r, int g, int b, int a) {
        this.spec = ColorSpec.Rgb;
        setRgba(r, g, b, a);
    }

    public void setRgba(int r, int g, int b, int a) {
        this.r = clamp((float)r / 255.0f);
        this.g = clamp((float)g / 255.0f);
        this.b = clamp((float)b / 255.0f);
        this.a = clamp((float)a / 255.0f);
        this.spec = ColorSpec.Rgb;
    }

    private float clamp(float v) {
        if (v < 0.0f) return 0.0f;
        if (v > 1.0f) return 1.0f;
        return v;
    }

    public int r() { return (int)(this.r * 255.0f); }
    public int g() { return (int)(this.g * 255.0f); }
    public int b() { return (int)(this.b * 255.0f); }
    public int a() { return (int)(this.a * 255.0f); }

    public boolean isValid() {
        return this.spec != ColorSpec.Invalid;
    }
}
