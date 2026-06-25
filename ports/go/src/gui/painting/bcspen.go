package painting

import "bcs/core/global"

// BcsPen defines how lines and outlines are drawn
type BcsPen struct {
	color global.BcsGlobalColor
	width int
}

func NewBcsPen(color global.BcsGlobalColor, width int) *BcsPen {
	return &BcsPen{color: color, width: width}
}

// BcsBrush defines how shapes are filled
type BcsBrush struct {
	color global.BcsGlobalColor
}

func NewBcsBrush(color global.BcsGlobalColor) *BcsBrush {
	return &BcsBrush{color: color}
}
