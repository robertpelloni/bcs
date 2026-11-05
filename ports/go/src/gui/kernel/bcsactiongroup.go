package kernel

import (
	"github.com/robertpelloni/bcs/ports/go/core/kernel"
)

type ExclusionPolicy int

const (
	ExclusionPolicyNone              ExclusionPolicy = 0
	ExclusionPolicyExclusive         ExclusionPolicy = 1
	ExclusionPolicyExclusiveOptional ExclusionPolicy = 2
)

// BcsActionGroup groups actions together, primarily for mutual exclusivity
type BcsActionGroup struct {
	*kernel.BcsObject
	actions         []*BcsAction
	exclusionPolicy ExclusionPolicy
	enabled         bool
	visible         bool

	// Signals
	Triggered func(action *BcsAction)
	Hovered   func(action *BcsAction)
}

func NewBcsActionGroup(parent *kernel.BcsObject) *BcsActionGroup {
	return &BcsActionGroup{
		BcsObject:       kernel.NewBcsObject(parent),
		actions:         make([]*BcsAction, 0),
		exclusionPolicy: ExclusionPolicyExclusive,
		enabled:         true,
		visible:         true,
	}
}

func (g *BcsActionGroup) AddAction(action *BcsAction) {
	g.actions = append(g.actions, action)

	// Intercept action toggling to enforce exclusivity
	originalToggled := action.Toggled
	action.Toggled = func(checked bool) {
		if checked && g.exclusionPolicy == ExclusionPolicyExclusive {
			for _, a := range g.actions {
				if a != action && a.IsChecked() {
					a.SetChecked(false)
				}
			}
		}
		if originalToggled != nil {
			originalToggled(checked)
		}
	}

	// Forward trigger
	originalTriggered := action.Triggered
	action.Triggered = func(checked bool) {
		if g.Triggered != nil {
			g.Triggered(action)
		}
		if originalTriggered != nil {
			originalTriggered(checked)
		}
	}
}

func (g *BcsActionGroup) Actions() []*BcsAction {
	return g.actions
}

func (g *BcsActionGroup) CheckedAction() *BcsAction {
	for _, a := range g.actions {
		if a.IsChecked() {
			return a
		}
	}
	return nil
}

func (g *BcsActionGroup) SetExclusionPolicy(policy ExclusionPolicy) {
	g.exclusionPolicy = policy
}

func (g *BcsActionGroup) GetExclusionPolicy() ExclusionPolicy {
	return g.exclusionPolicy
}
