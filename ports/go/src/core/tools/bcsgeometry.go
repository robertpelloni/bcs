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
