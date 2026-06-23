package global

import "sync"

// BcsSignal maps the C++ signal/slot paradigm to Go channels and callbacks.
type BcsSignal struct {
	mu        sync.RWMutex
	listeners []func(interface{})
}

// Connect adds a listener callback to the signal.
func (s *BcsSignal) Connect(callback func(interface{})) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.listeners = append(s.listeners, callback)
}

// Emit broadcasts the payload to all connected listeners.
func (s *BcsSignal) Emit(payload interface{}) {
	s.mu.RLock()
	defer s.mu.RUnlock()
	for _, listener := range s.listeners {
		go listener(payload) // Asynchronous execution default
	}
}
