using BCS.Core.Tools;

namespace BCS.Gui.Painting {
    // BcsPainter abstracts native hardware-accelerated and software drawing ops
    public class BcsPainter {
        public BcsPen Pen { get; set; }
        public BcsBrush Brush { get; set; }

        public BcsPainter() {
        }

        public void DrawRect(BcsRect rect) {
            // Dispatch native draw rect
        }

        public void DrawLine(BcsPoint p1, BcsPoint p2) {
            // Dispatch native draw line
        }
    }
}
