namespace BCS.Gui.Text {
    public enum StyleHint {
        Helvetica = 0,
        Times = 1,
        Courier = 2,
        OldEnglish = 3,
        System = 4,
        AnyStyle = 5,
        Cursive = 6,
        Monospace = 7,
        Fantasy = 8
    }

    public enum StyleStrategy {
        PreferDefault = 0x0001,
        PreferBitmap = 0x0002,
        PreferDevice = 0x0004,
        PreferOutline = 0x0008,
        ForceOutline = 0x0010,
        PreferMatch = 0x0020,
        PreferQuality = 0x0040,
        PreferAntialias = 0x0080,
        NoAntialias = 0x0100,
        NoSubpixelAntialias = 0x0800,
        PreferNoShaping = 0x1000,
        ContextFontMerging = 0x2000,
        PreferTypoLineMetrics = 0x4000,
        NoFontMerging = 0x8000
    }

    public enum Weight {
        Thin = 0,
        ExtraLight = 12,
        Light = 25,
        Normal = 50,
        Medium = 57,
        DemiBold = 63,
        Bold = 75,
        ExtraBold = 81,
        Black = 87
    }

    public class BcsFont {
        public string Family { get; set; }

        private double _pointSize;
        private int _pixelSize;

        public double PointSize {
            get => _pointSize;
            set {
                _pointSize = value;
                _pixelSize = -1;
            }
        }

        public int PixelSize {
            get => _pixelSize;
            set {
                _pixelSize = value;
                _pointSize = -1;
            }
        }

        public Weight FontWeight { get; set; }
        public bool IsItalic { get; set; }
        public bool IsUnderline { get; set; }
        public bool IsStrikeOut { get; set; }
        public bool IsFixedPitch { get; set; }
        public StyleHint Hint { get; set; }
        public StyleStrategy Strategy { get; set; }

        public BcsFont(string family, double pointSize, Weight weight, bool italic) {
            Family = family;
            PointSize = pointSize; // also sets PixelSize to -1
            FontWeight = weight;
            IsItalic = italic;
            IsUnderline = false;
            IsStrikeOut = false;
            IsFixedPitch = false;
            Hint = StyleHint.AnyStyle;
            Strategy = StyleStrategy.PreferDefault;
        }
    }
}
