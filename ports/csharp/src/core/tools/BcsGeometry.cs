namespace BCS.Core.Tools {
    // BcsPoint defines an X/Y coordinate
    public struct BcsPoint {
        public int X { get; set; }
        public int Y { get; set; }

        public BcsPoint(int x, int y) {
            X = x;
            Y = y;
        }
    }

    // BcsSize defines width and height dimensions
    public struct BcsSize {
        public int Width { get; set; }
        public int Height { get; set; }

        public BcsSize(int width, int height) {
            Width = width;
            Height = height;
        }
    }

    // BcsRect defines a 2D bounding box
    public struct BcsRect {
        public int X { get; set; }
        public int Y { get; set; }
        public int Width { get; set; }
        public int Height { get; set; }

        public BcsRect(int x, int y, int width, int height) {
            X = x;
            Y = y;
            Width = width;
            Height = height;
        }

        public bool Contains(BcsPoint p) {
            return p.X >= X && p.X <= X + Width &&
                   p.Y >= Y && p.Y <= Y + Height;
        }

        public bool Intersects(BcsRect other) {
            return X < other.X + other.Width && X + Width > other.X &&
                   Y < other.Y + other.Height && Y + Height > other.Y;
        }
    }
}
