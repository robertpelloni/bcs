package containers

// BcsVector is a generic dynamically resizing array, mapped to Go slices.
type BcsVector[T any] struct {
	items []T
}

// NewBcsVector creates a new empty vector.
func NewBcsVector[T any]() *BcsVector[T] {
	return &BcsVector[T]{
		items: make([]T, 0),
	}
}

// Append adds an item to the end of the vector.
func (v *BcsVector[T]) Append(item T) {
	v.items = append(v.items, item)
}

// At returns the item at the specified index.
func (v *BcsVector[T]) At(index int) T {
	return v.items[index]
}

// Size returns the number of items in the vector.
func (v *BcsVector[T]) Size() int {
	return len(v.items)
}

// Clear removes all elements from the vector.
func (v *BcsVector[T]) Clear() {
	v.items = make([]T, 0)
}
