package painting

type ColorSpec int

const (
	InvalidColorSpec  ColorSpec = 0
	RgbColorSpec      ColorSpec = 1
	HsvColorSpec      ColorSpec = 2
	CmykColorSpec     ColorSpec = 3
	HslColorSpec      ColorSpec = 4
	ExtendedRgbColorSpec ColorSpec = 5
)

// BcsColor represents an RGBA color structure
type BcsColor struct {
	spec ColorSpec
	a    float32
	r    float32
	g    float32
	b    float32
}

func NewBcsColor() *BcsColor {
	return &BcsColor{
		spec: InvalidColorSpec,
		a:    0,
		r:    0,
		g:    0,
		b:    0,
	}
}

func NewBcsColorRgba(r, g, b, a int) *BcsColor {
	color := &BcsColor{
		spec: RgbColorSpec,
	}
	color.SetRgba(r, g, b, a)
	return color
}

func (c *BcsColor) SetRgba(r, g, b, a int) {
	c.r = clamp(float32(r)/255.0)
	c.g = clamp(float32(g)/255.0)
	c.b = clamp(float32(b)/255.0)
	c.a = clamp(float32(a)/255.0)
	c.spec = RgbColorSpec
}

func clamp(v float32) float32 {
	if v < 0.0 {
		return 0.0
	}
	if v > 1.0 {
		return 1.0
	}
	return v
}

func (c *BcsColor) R() int {
	return int(c.r * 255.0)
}

func (c *BcsColor) G() int {
	return int(c.g * 255.0)
}

func (c *BcsColor) B() int {
	return int(c.b * 255.0)
}

func (c *BcsColor) A() int {
	return int(c.a * 255.0)
}

func (c *BcsColor) IsValid() bool {
	return c.spec != InvalidColorSpec
}
