namespace BCS.Gui.Layout {
    public enum BcsDirection {
        Horizontal,
        Vertical
    }

    // BcsBoxLayout organizes widgets into a horizontal or vertical array
    public class BcsBoxLayout : BcsLayout {
        public BcsDirection Direction { get; set; }

        public BcsBoxLayout(BcsDirection direction) {
            Direction = direction;
        }

        // Invalidate triggers an algorithmic recalculation of widget bounds
        public void Invalidate() {
            // Recompute sizes
        }
    }
}
