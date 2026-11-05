package kernel

import (
	"testing"
)

func TestBcsAction(t *testing.T) {
	action := NewBcsAction(nil)
	action.SetText("Test")

	if action.Text() != "Test" {
		t.Error("Action text not set correctly")
	}

	triggered := false
	action.Triggered = func(checked bool) {
		triggered = true
	}

	action.Trigger()
	if !triggered {
		t.Error("Trigger callback was not invoked")
	}

	action.SetCheckable(true)
	action.Trigger()
	if !action.IsChecked() {
		t.Error("Checkable action did not toggle on trigger")
	}
}
