using BCS.Core.Kernel;
using BCS.Core.Tools;

namespace BCS.Gui.Widgets {
    // BcsWidget is the base class for all UI elements
    public class BcsWidget : BcsObject {
        public BcsRect Geometry { get; set; }
        public bool IsVisible { get; private set; }

        public BcsWidget(BcsWidget parent = null) {
            Geometry = new BcsRect(0, 0, 100, 100);
            IsVisible = true;
        }

        public void SetGeometry(int x, int y, int width, int height) {
            Geometry = new BcsRect(x, y, width, height);
        }

        public void Show() {
            IsVisible = true;
        }

        public void Hide() {
            IsVisible = false;
        }

        public override bool Event(BcsEvent e) {
            // Widget specific event routing
            return base.Event(e);
        }
    }
}
