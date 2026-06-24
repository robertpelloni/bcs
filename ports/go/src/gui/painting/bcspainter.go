package painting

import (
	"bcs/core/tools"
)

// BcsPainter abstracts native hardware-accelerated and software drawing ops
type BcsPainter struct {
	pen   *BcsPen
	brush *BcsBrush
}

func NewBcsPainter() *BcsPainter {
	return &BcsPainter{}
}

func (p *BcsPainter) SetPen(pen *BcsPen) {
	p.pen = pen
}

func (p *BcsPainter) SetBrush(brush *BcsBrush) {
	p.brush = brush
}

func (p *BcsPainter) DrawRect(rect *tools.BcsRect) {
	// Dispatch native draw rect
}

func (p *BcsPainter) DrawLine(p1, p2 *tools.BcsPoint) {
	// Dispatch native draw line
}
