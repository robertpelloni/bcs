package layout

// BcsBoxLayout organizes widgets into a horizontal or vertical array
type BcsBoxLayout struct {
	*BcsLayout
	Direction int // 0 = Horizontal, 1 = Vertical
}

func NewBcsBoxLayout(dir int) *BcsBoxLayout {
	return &BcsBoxLayout{
		BcsLayout: NewBcsLayout(),
		Direction: dir,
	}
}

// Invalidate triggers an algorithmic recalculation of widget bounds
func (b *BcsBoxLayout) Invalidate() {
	// Recompute sizes
}
