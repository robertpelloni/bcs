package tools

import (
	"fmt"
	"reflect"
)

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

// GetType retrieves the string representation of the underlying type
func (v *BcsVariant) GetType() string {
	if v.IsNil() {
		return "Nil"
	}
	return reflect.TypeOf(v.value).String()
}

func (v *BcsVariant) ToString() string {
	return fmt.Sprintf("%v", v.value)
}

func (v *BcsVariant) ToInt() (int, bool) {
	if val, ok := v.value.(int); ok {
		return val, true
	}
	return 0, false
}

func (v *BcsVariant) ToFloat() (float64, bool) {
	if val, ok := v.value.(float64); ok {
		return val, true
	}
	return 0.0, false
}

func (v *BcsVariant) ToBool() (bool, bool) {
	if val, ok := v.value.(bool); ok {
		return val, true
	}
	return false, false
}
