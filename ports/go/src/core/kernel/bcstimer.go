package kernel

import (
	"sync"
	"time"
)

// BcsTimer provides a cross-platform high-level timer mapping to C++ QTimer natively using goroutines
type BcsTimer struct {
	*BcsObject
	interval     time.Duration
	singleShot   bool
	active       bool
	timer        *time.Timer
	ticker       *time.Ticker
	quit         chan struct{}
	mu           sync.Mutex
	TimeoutEvent func() // Simple native callback hook
}

func NewBcsTimer(parent *BcsObject) *BcsTimer {
	return &BcsTimer{
		BcsObject:  NewBcsObject(parent),
		interval:   0,
		singleShot: false,
		active:     false,
		quit:       make(chan struct{}),
	}
}

func (t *BcsTimer) SetInterval(msec int) {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.interval = time.Duration(msec) * time.Millisecond
}

func (t *BcsTimer) SetSingleShot(singleShot bool) {
	t.mu.Lock()
	defer t.mu.Unlock()
	t.singleShot = singleShot
}

func (t *BcsTimer) Start() {
	t.mu.Lock()
	defer t.mu.Unlock()

	if t.active {
		return
	}
	t.active = true
	t.quit = make(chan struct{})

	go func() {
		if t.singleShot {
			t.timer = time.NewTimer(t.interval)
			select {
			case <-t.timer.C:
				if t.TimeoutEvent != nil {
					t.TimeoutEvent()
				}
				t.mu.Lock()
				t.active = false
				t.mu.Unlock()
			case <-t.quit:
				if !t.timer.Stop() {
					<-t.timer.C
				}
			}
		} else {
			t.ticker = time.NewTicker(t.interval)
			for {
				select {
				case <-t.ticker.C:
					if t.TimeoutEvent != nil {
						t.TimeoutEvent()
					}
				case <-t.quit:
					t.ticker.Stop()
					return
				}
			}
		}
	}()
}

func (t *BcsTimer) Stop() {
	t.mu.Lock()
	defer t.mu.Unlock()

	if !t.active {
		return
	}
	t.active = false
	close(t.quit)
}

func (t *BcsTimer) IsActive() bool {
	t.mu.Lock()
	defer t.mu.Unlock()
	return t.active
}
