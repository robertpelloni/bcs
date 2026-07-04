package text

type StyleHint int

const (
	Helvetica  StyleHint = 0
	SansSerif  StyleHint = 0
	Times      StyleHint = 1
	Serif      StyleHint = 1
	Courier    StyleHint = 2
	TypeWriter StyleHint = 2
	OldEnglish StyleHint = 3
	Decorative StyleHint = 3
	System     StyleHint = 4
	AnyStyle   StyleHint = 5
	Cursive    StyleHint = 6
	Monospace  StyleHint = 7
	Fantasy    StyleHint = 8
)

type StyleStrategy int

const (
	PreferDefault         StyleStrategy = 0x0001
	PreferBitmap          StyleStrategy = 0x0002
	PreferDevice          StyleStrategy = 0x0004
	PreferOutline         StyleStrategy = 0x0008
	ForceOutline          StyleStrategy = 0x0010
	PreferMatch           StyleStrategy = 0x0020
	PreferQuality         StyleStrategy = 0x0040
	PreferAntialias       StyleStrategy = 0x0080
	NoAntialias           StyleStrategy = 0x0100
	NoSubpixelAntialias   StyleStrategy = 0x0800
	PreferNoShaping       StyleStrategy = 0x1000
	ContextFontMerging    StyleStrategy = 0x2000
	PreferTypoLineMetrics StyleStrategy = 0x4000
	NoFontMerging         StyleStrategy = 0x8000
)

type Weight int

const (
	Thin       Weight = 0
	ExtraLight Weight = 12
	Light      Weight = 25
	Normal     Weight = 50
	Medium     Weight = 57
	DemiBold   Weight = 63
	Bold       Weight = 75
	ExtraBold  Weight = 81
	Black      Weight = 87
)

type BcsFont struct {
	family        string
	pointSize     float64
	pixelSize     int
	weight        Weight
	italic        bool
	underline     bool
	strikeOut     bool
	fixedPitch    bool
	styleHint     StyleHint
	styleStrategy StyleStrategy
}

func NewBcsFont(family string, pointSize float64, weight Weight, italic bool) *BcsFont {
	return &BcsFont{
		family:        family,
		pointSize:     pointSize,
		pixelSize:     -1,
		weight:        weight,
		italic:        italic,
		underline:     false,
		strikeOut:     false,
		fixedPitch:    false,
		styleHint:     AnyStyle,
		styleStrategy: PreferDefault,
	}
}

func (f *BcsFont) SetFamily(family string) {
	f.family = family
}

func (f *BcsFont) Family() string {
	return f.family
}

func (f *BcsFont) SetPointSize(size float64) {
	f.pointSize = size
	f.pixelSize = -1 // mutually exclusive
}

func (f *BcsFont) PointSize() float64 {
	return f.pointSize
}

func (f *BcsFont) SetPixelSize(size int) {
	f.pixelSize = size
	f.pointSize = -1 // mutually exclusive
}

func (f *BcsFont) PixelSize() int {
	return f.pixelSize
}

func (f *BcsFont) SetWeight(weight Weight) {
	f.weight = weight
}

func (f *BcsFont) Weight() Weight {
	return f.weight
}

func (f *BcsFont) SetItalic(italic bool) {
	f.italic = italic
}

func (f *BcsFont) Italic() bool {
	return f.italic
}
