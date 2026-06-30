package bcs.gui.widgets;

import bcs.core.kernel.BcsEvent;
import bcs.core.kernel.BcsObject; // Assume BcsObject is ported and public
import bcs.core.tools.BcsGeometry.BcsRect;

// BcsWidget is the base class for all UI elements
public class BcsWidget extends bcs.core.kernel.BcsObject {
    private BcsRect geometry;
    private boolean visible;

    // Comprehensive UI Representation properties
    public String toolTip;
    public String description;
    public String label;

    public BcsWidget(BcsWidget parent) {
        super(parent);
        this.geometry = new BcsRect(0, 0, 100, 100);
        this.visible = true;
        this.toolTip = "";
        this.description = "";
        this.label = "";
    }

    public void setGeometry(int x, int y, int width, int height) {
        this.geometry = new BcsRect(x, y, width, height);
    }

    public BcsRect getGeometry() {
        return this.geometry;
    }

    public void show() {
        this.visible = true;
    }

    public void hide() {
        this.visible = false;
    }

    @Override
    public boolean event(BcsEvent e) {
        // Widget specific event routing
        return super.event(e);
    }
}
