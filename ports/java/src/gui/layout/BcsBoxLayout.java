package bcs.gui.layout;

// BcsBoxLayout organizes widgets into a horizontal or vertical array
public class BcsBoxLayout extends BcsLayout {
    public enum BcsDirection {
        HORIZONTAL,
        VERTICAL
    }

    public BcsDirection direction;

    public BcsBoxLayout(BcsDirection direction) {
        this.direction = direction;
    }

    // Invalidate triggers an algorithmic recalculation of widget bounds
    public void invalidate() {
        // Recompute sizes
    }
}
