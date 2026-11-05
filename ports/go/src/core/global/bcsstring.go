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

// Length returns the number of characters in the string
func (s BcsString) Length() int {
	return len(s)
}

// IsEmpty checks if the string has a length of 0
func (s BcsString) IsEmpty() bool {
	return len(s) == 0
}

// Substr returns a substring starting at index with a given length
func (s BcsString) Substr(index, length int) BcsString {
    str := string(s)
    if index < 0 || index >= len(str) {
        return ""
    }
    end := index + length
    if end > len(str) {
        end = len(str)
    }
	return BcsString(str[index:end])
}

// ToUtf8 converts the BcsString to a standard UTF-8 byte slice
func (s BcsString) ToUtf8() []byte {
	return []byte(s)
}
