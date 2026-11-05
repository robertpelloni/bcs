package thread

import "sync"

// BcsMutex maps exactly to Go's sync.Mutex
type BcsMutex struct {
	mu sync.Mutex
}

func (m *BcsMutex) Lock() {
	m.mu.Lock()
}

func (m *BcsMutex) Unlock() {
	m.mu.Unlock()
}
