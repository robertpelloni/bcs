using BCS.Core.Tools;

namespace BCS.Gui.Kernel {
    // BcsWindow represents a top-level native window
    public class BcsWindow {
        public string Title { get; set; }
        public BcsRect Geometry { get; set; }
        public bool IsVisible { get; private set; }

        public BcsWindow() {
            Title = string.Empty;
            Geometry = new BcsRect(0, 0, 800, 600);
            IsVisible = false;
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
    }
}
