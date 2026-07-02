package kernel

import (
	"container/heap"
	"sync"
)

// BcsEvent represents the base class for all framework events
type BcsEvent struct {
	Type     int
	Priority int // Higher number means higher priority
}

func NewBcsEvent(eventType int) *BcsEvent {
	return &BcsEvent{Type: eventType, Priority: 0}
}

func NewBcsEventWithPriority(eventType int, priority int) *BcsEvent {
	return &BcsEvent{Type: eventType, Priority: priority}
}

// BcsObject is the base class for all event-receiving objects
type BcsObject struct {
	Parent       *BcsObject
	Children     []*BcsObject
	eventFilters []func(*BcsEvent) bool
}

func NewBcsObject(parent *BcsObject) *BcsObject {
	obj := &BcsObject{Parent: parent, eventFilters: make([]func(*BcsEvent) bool, 0)}
	if parent != nil {
		parent.Children = append(parent.Children, obj)
	}
	return obj
}

func (o *BcsObject) InstallEventFilter(filter func(*BcsEvent) bool) {
	o.eventFilters = append(o.eventFilters, filter)
}

func (o *BcsObject) Event(e *BcsEvent) bool {
	// First process event filters
	for _, filter := range o.eventFilters {
		if filter(e) {
			return true // Handled
		}
	}
	// Base event processing
	return false
}

// priorityQueue implementation for BcsEventLoop using container/heap
type priorityQueue []*BcsEvent

func (pq priorityQueue) Len() int { return len(pq) }
func (pq priorityQueue) Less(i, j int) bool {
	// We want higher priority to be popped first, meaning it needs to be at the front of the heap
	return pq[i].Priority > pq[j].Priority
}
func (pq priorityQueue) Swap(i, j int) { pq[i], pq[j] = pq[j], pq[i] }
func (pq *priorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(*BcsEvent))
}
func (pq *priorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

// BcsEventLoop provides the execution loop for event dispatching
type BcsEventLoop struct {
	queue priorityQueue
	mu    sync.Mutex
	cond  *sync.Cond
	quit  bool
}

func NewBcsEventLoop() *BcsEventLoop {
	loop := &BcsEventLoop{
		queue: make(priorityQueue, 0),
		quit:  false,
	}
	loop.cond = sync.NewCond(&loop.mu)
	return loop
}

func (l *BcsEventLoop) PostEvent(e *BcsEvent) {
	l.mu.Lock()
	defer l.mu.Unlock()

	heap.Push(&l.queue, e)

	l.cond.Signal()
}

func (l *BcsEventLoop) Exec() {
	l.mu.Lock()
	defer l.mu.Unlock()

	for !l.quit {
		for len(l.queue) == 0 && !l.quit {
			l.cond.Wait()
		}

		if l.quit {
			break
		}

		// Pop highest priority event
		e := heap.Pop(&l.queue).(*BcsEvent)

		l.mu.Unlock()
		// Dispatch event to application roots
		_ = e
		l.mu.Lock()
	}
}

func (l *BcsEventLoop) Quit() {
	l.mu.Lock()
	defer l.mu.Unlock()
	l.quit = true
	l.cond.Broadcast()
}
