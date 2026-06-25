package bcs.core.tools;

public class BcsGeometry {

    // BcsPoint defines an X/Y coordinate
    public static class BcsPoint {
        public int x;
        public int y;

        public BcsPoint(int x, int y) {
            this.x = x;
            this.y = y;
        }
    }

    // BcsSize defines width and height dimensions
    public static class BcsSize {
        public int width;
        public int height;

        public BcsSize(int width, int height) {
            this.width = width;
            this.height = height;
        }
    }

    // BcsRect defines a 2D bounding box
    public static class BcsRect {
        public int x;
        public int y;
        public int width;
        public int height;

        public BcsRect(int x, int y, int width, int height) {
            this.x = x;
            this.y = y;
            this.width = width;
            this.height = height;
        }
    }
}
