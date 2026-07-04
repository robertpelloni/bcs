package kernel

import (
	"testing"
	"time"
)

func TestEventLoopLifecycle(t *testing.T) {
	loop := NewBcsEventLoop()

	// Start loop in background
	go loop.Exec()

	// Post a basic event
	event := NewBcsEventWithPriority(1, 10)
	loop.PostEvent(event)

	// Let it run momentarily
	time.Sleep(50 * time.Millisecond)

	// Stop loop
	loop.Quit()
}

func TestObjectEventRouting(t *testing.T) {
	parent := NewBcsObject(nil)
	child := NewBcsObject(parent)

	if len(parent.Children) != 1 || parent.Children[0] != child {
		t.Errorf("Object hierarchy not linked correctly")
	}

	handled := false
	child.InstallEventFilter(func(e *BcsEvent) bool {
		handled = true
		return true
	})

	event := NewBcsEvent(1)
	child.Event(event)

	if !handled {
		t.Errorf("Event filter failed to execute")
	}
}
