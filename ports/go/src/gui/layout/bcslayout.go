package layout

import "bcs/gui/widgets"

// BcsLayout governs spatial organization of UI components
type BcsLayout struct {
	children []*widgets.BcsWidget
	margin   int
	spacing  int
}

func NewBcsLayout() *BcsLayout {
	return &BcsLayout{
		children: make([]*widgets.BcsWidget, 0),
		margin:   0,
		spacing:  5,
	}
}

func (l *BcsLayout) AddWidget(w *widgets.BcsWidget) {
	l.children = append(l.children, w)
}

func (l *BcsLayout) RemoveWidget(w *widgets.BcsWidget) {
	for i, child := range l.children {
		if child == w {
			l.children = append(l.children[:i], l.children[i+1:]...)
			break
		}
	}
}
