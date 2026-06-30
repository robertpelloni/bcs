package widgets

import (
	"bcs/core/kernel"
	"bcs/core/tools"
)

// BcsWidget is the base class for all UI elements
type BcsWidget struct {
	*kernel.BcsObject
	rect    *tools.BcsRect
	visible bool

	// Comprehensive UI Representation properties
	ToolTip     string
	Description string
	Label       string
}

func NewBcsWidget(parent *BcsWidget) *BcsWidget {
	var parentObj *kernel.BcsObject
	if parent != nil {
		parentObj = parent.BcsObject
	}

	return &BcsWidget{
		BcsObject:   kernel.NewBcsObject(parentObj),
		rect:        tools.NewBcsRect(0, 0, 100, 100),
		visible:     true,
		ToolTip:     "",
		Description: "",
		Label:       "",
	}
}

func (w *BcsWidget) SetGeometry(x, y, width, height int) {
	w.rect = tools.NewBcsRect(x, y, width, height)
}

func (w *BcsWidget) Geometry() *tools.BcsRect {
	return w.rect
}

func (w *BcsWidget) Show() {
	w.visible = true
}

func (w *BcsWidget) Hide() {
	w.visible = false
}

// Override Event processing for Widgets
func (w *BcsWidget) Event(e *kernel.BcsEvent) bool {
	// Propagate to BcsObject or handle UI-specific painting/input
	return w.BcsObject.Event(e)
}
