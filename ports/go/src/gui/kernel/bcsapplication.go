package kernel

import (
	"bcs/core/kernel"
	"fmt"
)

// BcsApplication manages the GUI application's control flow and main settings.
// Ported from C++ QCoreApplication/BcsApplication core
type BcsApplication struct {
	*kernel.BcsObject
	eventLoop *kernel.BcsEventLoop
	args      []string

	ApplicationName    string
	ApplicationVersion string
	OrganizationName   string
	OrganizationDomain string
}

// Global instance
var appInstance *BcsApplication

// NewBcsApplication initializes the GUI application state
func NewBcsApplication(args []string) *BcsApplication {
	app := &BcsApplication{
		BcsObject: kernel.NewBcsObject(nil),
		eventLoop: kernel.NewBcsEventLoop(),
		args:      args,
	}
	appInstance = app
	return app
}

// Instance returns the global application instance
func Instance() *BcsApplication {
	return appInstance
}

// Exec enters the main event loop and waits until exit() is called
func (app *BcsApplication) Exec() int {
	fmt.Println("BCS Application starting event loop...")
	app.eventLoop.Exec()
	return 0
}

// Quit requests the application to terminate
func (app *BcsApplication) Quit() {
	fmt.Println("BCS Application quitting...")
	app.eventLoop.Quit()
}

// ProcessEvents processes pending events in the queue
func (app *BcsApplication) ProcessEvents() {
	// Abstract wrapper matching processEvents
}

// PostEvent queues an event to a target receiver
func (app *BcsApplication) PostEvent(receiver *kernel.BcsObject, e *kernel.BcsEvent) {
	app.eventLoop.PostEvent(e)
}

// SendEvent directly dispatches an event to a receiver bypassing the queue
func (app *BcsApplication) SendEvent(receiver *kernel.BcsObject, e *kernel.BcsEvent) bool {
	if receiver != nil {
		return receiver.Event(e)
	}
	return false
}
