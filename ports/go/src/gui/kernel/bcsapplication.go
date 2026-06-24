package kernel

import (
	"bcs/core/kernel"
	"fmt"
)

// BcsApplication manages the GUI application's control flow and main settings.
type BcsApplication struct {
	eventLoop *kernel.BcsEventLoop
	args      []string
}

// NewBcsApplication initializes the GUI application state
func NewBcsApplication(args []string) *BcsApplication {
	return &BcsApplication{
		eventLoop: kernel.NewBcsEventLoop(),
		args:      args,
	}
}

// Exec enters the main event loop and waits until exit() is called
func (app *BcsApplication) Exec() int {
	fmt.Println("BCS Application starting event loop...")
	app.eventLoop.Exec()
	return 0
}

// Quit requests the application to terminate
func (app *BcsApplication) Quit() {
	// Post a quit event to the event loop
	fmt.Println("BCS Application quitting...")
}
