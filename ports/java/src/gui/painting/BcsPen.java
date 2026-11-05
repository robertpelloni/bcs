package bcs.gui.painting;

import bcs.core.global.BcsCoreTypes.BcsGlobalColor;

// BcsPen defines how lines and outlines are drawn
public class BcsPen {
    public BcsGlobalColor color;
    public int width;

    public BcsPen(BcsGlobalColor color, int width) {
        this.color = color;
        this.width = width;
    }
}

// BcsBrush defines how shapes are filled
class BcsBrush {
    public BcsGlobalColor color;

    public BcsBrush(BcsGlobalColor color) {
        this.color = color;
    }
}
