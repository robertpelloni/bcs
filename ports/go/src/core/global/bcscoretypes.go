package global

import "fmt"

// BcsGlobalColor defines standard core colors
type BcsGlobalColor int

const (
	BcsColorWhite BcsGlobalColor = iota
	BcsColorBlack
	BcsColorRed
	BcsColorGreen
	BcsColorBlue
	BcsColorTransparent
)

// BcsWarning handles internal framework warnings
func BcsWarning(msg string, args ...interface{}) {
	fmt.Printf("[BCS Warning] "+msg+"\n", args...)
}

// BcsCritical handles critical framework failures
func BcsCritical(msg string, args ...interface{}) {
	fmt.Printf("[BCS Critical] "+msg+"\n", args...)
}
