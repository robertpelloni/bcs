namespace BCS.Gui.Painting {
    public enum ColorSpec {
        Invalid = 0,
        Rgb = 1,
        Hsv = 2,
        Cmyk = 3,
        Hsl = 4,
        ExtendedRgb = 5
    }

    public class BcsColor {
        public ColorSpec Spec { get; private set; }
        private float _a;
        private float _r;
        private float _g;
        private float _b;

        public BcsColor() {
            Spec = ColorSpec.Invalid;
            _a = 0.0f;
            _r = 0.0f;
            _g = 0.0f;
            _b = 0.0f;
        }

        public BcsColor(int r, int g, int b, int a = 255) {
            Spec = ColorSpec.Rgb;
            SetRgba(r, g, b, a);
        }

        public void SetRgba(int r, int g, int b, int a = 255) {
            _r = Clamp(r / 255.0f);
            _g = Clamp(g / 255.0f);
            _b = Clamp(b / 255.0f);
            _a = Clamp(a / 255.0f);
            Spec = ColorSpec.Rgb;
        }

        private float Clamp(float v) {
            if (v < 0.0f) return 0.0f;
            if (v > 1.0f) return 1.0f;
            return v;
        }

        public int R => (int)(_r * 255.0f);
        public int G => (int)(_g * 255.0f);
        public int B => (int)(_b * 255.0f);
        public int A => (int)(_a * 255.0f);

        public bool IsValid => Spec != ColorSpec.Invalid;
    }
}
