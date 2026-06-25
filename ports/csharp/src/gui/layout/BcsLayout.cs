using System.Collections.Generic;
using BCS.Gui.Widgets;

namespace BCS.Gui.Layout {
    // BcsLayout governs spatial organization of UI components
    public class BcsLayout {
        protected List<BcsWidget> _children;
        public int Margin { get; set; }
        public int Spacing { get; set; }

        public BcsLayout() {
            _children = new List<BcsWidget>();
            Margin = 0;
            Spacing = 5;
        }

        public void AddWidget(BcsWidget w) {
            _children.Add(w);
        }

        public void RemoveWidget(BcsWidget w) {
            _children.Remove(w);
        }
    }
}
