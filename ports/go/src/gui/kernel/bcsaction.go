package kernel

import (
	"github.com/robertpelloni/bcs/ports/go/core/kernel"
)

// BcsAction represents an actionable item (e.g. in a menu or toolbar)
// Ported from C++ QAction
type BcsAction struct {
	*kernel.BcsObject
	text      string
	enabled   bool
	visible   bool
	checkable bool
	checked   bool

	// Signals
	Triggered func(checked bool)
	Toggled   func(checked bool)
}

func NewBcsAction(parent *kernel.BcsObject) *BcsAction {
	return &BcsAction{
		BcsObject: kernel.NewBcsObject(parent),
		enabled:   true,
		visible:   true,
		checkable: false,
		checked:   false,
	}
}

func (a *BcsAction) SetText(text string) {
	a.text = text
}

func (a *BcsAction) Text() string {
	return a.text
}

func (a *BcsAction) SetEnabled(enabled bool) {
	a.enabled = enabled
}

func (a *BcsAction) IsEnabled() bool {
	return a.enabled
}

func (a *BcsAction) SetCheckable(checkable bool) {
	a.checkable = checkable
}

func (a *BcsAction) IsCheckable() bool {
	return a.checkable
}

func (a *BcsAction) SetChecked(checked bool) {
	if !a.checkable {
		return
	}
	if a.checked != checked {
		a.checked = checked
		if a.Toggled != nil {
			a.Toggled(a.checked)
		}
	}
}

func (a *BcsAction) IsChecked() bool {
	return a.checked
}

func (a *BcsAction) Trigger() {
	if !a.enabled {
		return
	}
	if a.checkable {
		a.SetChecked(!a.checked)
	}
	if a.Triggered != nil {
		a.Triggered(a.checked)
	}
}
