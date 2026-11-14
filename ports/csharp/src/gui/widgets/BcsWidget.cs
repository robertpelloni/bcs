using Bcs.Core.Kernel;
using Bcs.Core.Tools;

namespace Bcs.Gui.Widgets {
    // BcsWidget is the base class for all UI elements
    public class BcsWidget : BcsObject {
        public BcsRect Geometry { get; set; }
        public bool IsVisible { get; private set; }

        // Comprehensive UI Representation properties
        public string ToolTip { get; set; }
        public string Description { get; set; }
        public string Label { get; set; }

        public BcsWidget(BcsWidget parent = null) : base(parent) {
            Geometry = new BcsRect(0, 0, 100, 100);
            IsVisible = true;
            ToolTip = string.Empty;
            Description = string.Empty;
            Label = string.Empty;
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
            // Widget specific event routing (mirrors QWidget::event):
            // typed handlers first, then the object-level fallback.
            if (e.Type == (int)EventType.MouseButtonRelease) {
                MouseReleaseEvent(e);
                return true;
            }
            return base.Event(e);
        }

        // Mirrors QWidget::mouseReleaseEvent; override in subclasses.
        protected virtual void MouseReleaseEvent(BcsEvent e) { }
    }
}
