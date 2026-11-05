package thread

import (
	"sync"
	"time"
)

// BcsThread abstracts thread creation and joining, mapped to Goroutines
type BcsThread struct {
	wg       sync.WaitGroup
	quitChan chan struct{}
}

func NewBcsThread() *BcsThread {
	return &BcsThread{
		quitChan: make(chan struct{}),
	}
}

// Start executes the provided function in a new goroutine
func (t *BcsThread) Start(fn func()) {
	t.wg.Add(1)
	go func() {
		defer t.wg.Done()
		fn()
	}()
}

// Wait blocks until the spawned routine finishes
func (t *BcsThread) Wait() {
	t.wg.Wait()
}

// Sleep halts execution of the current routine for ms milliseconds
func Sleep(ms int) {
	time.Sleep(time.Duration(ms) * time.Millisecond)
}
