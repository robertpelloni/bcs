package bcs.gui.painting;

import bcs.core.tools.BcsGeometry.BcsPoint;
import bcs.core.tools.BcsGeometry.BcsRect;

// BcsPainter abstracts native hardware-accelerated and software drawing ops
public class BcsPainter {
    private BcsPen pen;
    private BcsBrush brush;

    public void setPen(BcsPen pen) {
        this.pen = pen;
    }

    public void setBrush(BcsBrush brush) {
        this.brush = brush;
    }

    public void drawRect(BcsRect rect) {
        // Dispatch native draw rect
    }

    public void drawLine(BcsPoint p1, BcsPoint p2) {
        // Dispatch native draw line
    }
}
