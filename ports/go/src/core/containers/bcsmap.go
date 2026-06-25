package containers

import "sync"

// BcsMap is a thread-safe generic hash map
type BcsMap[K comparable, V any] struct {
	mu    sync.RWMutex
	items map[K]V
}

// NewBcsMap creates a new empty map
func NewBcsMap[K comparable, V any]() *BcsMap[K, V] {
	return &BcsMap[K, V]{
		items: make(map[K]V),
	}
}

// Insert adds or updates a key-value pair
func (m *BcsMap[K, V]) Insert(key K, value V) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.items[key] = value
}

// Value retrieves a value by key
func (m *BcsMap[K, V]) Value(key K) (V, bool) {
	m.mu.RLock()
	defer m.mu.RUnlock()
	val, exists := m.items[key]
	return val, exists
}

// Contains checks if a key exists
func (m *BcsMap[K, V]) Contains(key K) bool {
	m.mu.RLock()
	defer m.mu.RUnlock()
	_, exists := m.items[key]
	return exists
}
