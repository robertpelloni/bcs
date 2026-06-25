package bcs.gui.kernel;

import bcs.core.tools.BcsGeometry.BcsRect;

// BcsWindow represents a top-level native window
public class BcsWindow {
    private String title;
    private BcsRect geometry;
    private boolean visible;

    public BcsWindow() {
        this.title = "";
        this.geometry = new BcsRect(0, 0, 800, 600);
        this.visible = false;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public void setGeometry(int x, int y, int width, int height) {
        this.geometry = new BcsRect(x, y, width, height);
    }

    public void show() {
        this.visible = true;
    }

    public void hide() {
        this.visible = false;
    }
}
