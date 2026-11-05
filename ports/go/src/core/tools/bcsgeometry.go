package tools

// BcsPoint defines an X/Y coordinate
type BcsPoint struct {
	X int
	Y int
}

func NewBcsPoint(x, y int) *BcsPoint {
	return &BcsPoint{X: x, Y: y}
}

// BcsSize defines width and height dimensions
type BcsSize struct {
	Width  int
	Height int
}

func NewBcsSize(w, h int) *BcsSize {
	return &BcsSize{Width: w, Height: h}
}

// BcsRect defines a 2D bounding box
type BcsRect struct {
	X      int
	Y      int
	Width  int
	Height int
}

func NewBcsRect(x, y, w, h int) *BcsRect {
	return &BcsRect{X: x, Y: y, Width: w, Height: h}
}

func (r *BcsRect) Contains(p *BcsPoint) bool {
	return p.X >= r.X && p.X <= r.X+r.Width &&
		p.Y >= r.Y && p.Y <= r.Y+r.Height
}

func (r *BcsRect) Intersects(other *BcsRect) bool {
	return r.X < other.X+other.Width && r.X+r.Width > other.X &&
		r.Y < other.Y+other.Height && r.Y+r.Height > other.Y
}
