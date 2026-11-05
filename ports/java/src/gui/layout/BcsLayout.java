package bcs.gui.layout;

import bcs.gui.widgets.BcsWidget;
import java.util.ArrayList;
import java.util.List;

// BcsLayout governs spatial organization of UI components
public class BcsLayout {
    protected final List<BcsWidget> children;
    public int margin;
    public int spacing;

    public BcsLayout() {
        this.children = new ArrayList<>();
        this.margin = 0;
        this.spacing = 5;
    }

    public void addWidget(BcsWidget w) {
        this.children.add(w);
    }

    public void removeWidget(BcsWidget w) {
        this.children.remove(w);
    }
}
