package kernel

import (
	"bcs/core/tools"
)

// BcsWindow represents a top-level native window
type BcsWindow struct {
	title  string
	rect   *tools.BcsRect
	visible bool
}

func NewBcsWindow() *BcsWindow {
	return &BcsWindow{
		rect: tools.NewBcsRect(0, 0, 800, 600),
	}
}

func (w *BcsWindow) SetTitle(title string) {
	w.title = title
}

func (w *BcsWindow) SetGeometry(x, y, width, height int) {
	w.rect = tools.NewBcsRect(x, y, width, height)
}

func (w *BcsWindow) Show() {
	w.visible = true
}

func (w *BcsWindow) Hide() {
	w.visible = false
}
