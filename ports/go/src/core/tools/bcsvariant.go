package tools

import "fmt"

// BcsVariant is a type-erased container, conceptually mapping to `interface{}` in Go
type BcsVariant struct {
	value interface{}
}

func NewBcsVariant(val interface{}) *BcsVariant {
	return &BcsVariant{value: val}
}

// Value retrieves the underlying value
func (v *BcsVariant) Value() interface{} {
	return v.value
}

// SetValue assigns a new type-erased value
func (v *BcsVariant) SetValue(val interface{}) {
	v.value = val
}

// IsNil checks if the variant has no underlying type/value
func (v *BcsVariant) IsNil() bool {
	return v.value == nil
}

func (v *BcsVariant) ToString() string {
	return fmt.Sprintf("%v", v.value)
}
