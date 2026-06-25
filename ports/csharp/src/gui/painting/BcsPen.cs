using BCS.Core.Global;

namespace BCS.Gui.Painting {
    // BcsPen defines how lines and outlines are drawn
    public class BcsPen {
        public BcsGlobalColor Color { get; set; }
        public int Width { get; set; }

        public BcsPen(BcsGlobalColor color, int width = 1) {
            Color = color;
            Width = width;
        }
    }

    // BcsBrush defines how shapes are filled
    public class BcsBrush {
        public BcsGlobalColor Color { get; set; }

        public BcsBrush(BcsGlobalColor color) {
            Color = color;
        }
    }
}
