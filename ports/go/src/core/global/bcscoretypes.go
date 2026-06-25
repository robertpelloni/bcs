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

// BcsAlignment defines vertical and horizontal layout constraints
type BcsAlignment int

const (
	AlignLeft BcsAlignment = 0x0001
	AlignRight BcsAlignment = 0x0002
	AlignHCenter BcsAlignment = 0x0004
	AlignJustify BcsAlignment = 0x0008
	AlignTop BcsAlignment = 0x0020
	AlignBottom BcsAlignment = 0x0040
	AlignVCenter BcsAlignment = 0x0080
	AlignCenter BcsAlignment = AlignVCenter | AlignHCenter
)

// BcsOrientation defines linear layout bounds
type BcsOrientation int

const (
	Horizontal BcsOrientation = 0x1
	Vertical BcsOrientation = 0x2
)
