package global

// In Go, we use native pointers and garbage collection,
// but we define semantic aliases to match the BCS C++ API.

// BcsSharedPointer maps to a native pointer in Go (GC handles sharing)
type BcsSharedPointer[T any] *T

// BcsUniquePointer maps to a native pointer but implies exclusive ownership contextually
type BcsUniquePointer[T any] *T

// NewShared creates a new BcsSharedPointer
func NewShared[T any](val T) BcsSharedPointer[T] {
	return &val
}

// NewUnique creates a new BcsUniquePointer
func NewUnique[T any](val T) BcsUniquePointer[T] {
	return &val
}
