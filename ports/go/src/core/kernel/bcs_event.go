package kernel

// BcsEvent represents the base class for all framework events
type BcsEvent struct {
	Type int
}

func NewBcsEvent(eventType int) *BcsEvent {
	return &BcsEvent{Type: eventType}
}

// BcsObject is the base class for all event-receiving objects
type BcsObject struct {
	Parent   *BcsObject
	Children []*BcsObject
}

func NewBcsObject(parent *BcsObject) *BcsObject {
	obj := &BcsObject{Parent: parent}
	if parent != nil {
		parent.Children = append(parent.Children, obj)
	}
	return obj
}

func (o *BcsObject) Event(e *BcsEvent) bool {
	// Base event processing
	return false
}

// BcsEventLoop provides the execution loop for event dispatching
type BcsEventLoop struct {
	queue chan *BcsEvent
}

func NewBcsEventLoop() *BcsEventLoop {
	return &BcsEventLoop{
		queue: make(chan *BcsEvent, 1024),
	}
}

func (l *BcsEventLoop) PostEvent(e *BcsEvent) {
	l.queue <- e
}

func (l *BcsEventLoop) Exec() {
	for e := range l.queue {
		// Dispatch event to application roots
		_ = e
	}
}
