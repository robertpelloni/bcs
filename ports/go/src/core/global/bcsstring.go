package global

import "strings"

// BcsString represents the core string type, mapped to Go's native string
type BcsStringUtf8 string

// NewBcsString creates a new BcsString
func NewBcsString(val string) BcsString {
	return BcsString(val)
}

// ToUpper converts the string to uppercase
func (s BcsString) ToUpper() BcsString {
	return BcsString(strings.ToUpper(string(s)))
}

// ToLower converts the string to lowercase
func (s BcsString) ToLower() BcsString {
	return BcsString(strings.ToLower(string(s)))
}
