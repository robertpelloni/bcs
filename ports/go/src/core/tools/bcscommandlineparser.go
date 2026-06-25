package tools

import (
	"bcs/core/global"
	"fmt"
	"os"
)

// BcsCommandLineOption defines a single command line flag or parameter
type BcsCommandLineOption struct {
	Name        global.BcsString
	Description global.BcsString
	ValueName   global.BcsString
}

// BcsCommandLineParser handles standard cross-platform argument parsing
type BcsCommandLineParser struct {
	options map[string]BcsCommandLineOption
	values  map[string]global.BcsString
}

func NewBcsCommandLineParser() *BcsCommandLineParser {
	return &BcsCommandLineParser{
		options: make(map[string]BcsCommandLineOption),
		values:  make(map[string]global.BcsString),
	}
}

// AddOption registers a new valid command line argument
func (p *BcsCommandLineParser) AddOption(opt BcsCommandLineOption) {
	p.options[string(opt.Name)] = opt
}

// Process ingests os.Args and parses them against registered options
func (p *BcsCommandLineParser) Process(args []string) {
	for i := 0; i < len(args); i++ {
		arg := args[i]
		if len(arg) > 2 && arg[:2] == "--" {
			flag := arg[2:]
			if _, exists := p.options[flag]; exists {
				// Simple boolean flag logic
				p.values[flag] = "true"
			}
		}
	}
}

// IsSet checks if a flag was passed during application execution
func (p *BcsCommandLineParser) IsSet(name global.BcsString) bool {
	_, exists := p.values[string(name)]
	return exists
}

// Value returns the value associated with a parsed flag
func (p *BcsCommandLineParser) Value(name global.BcsString) global.BcsString {
	return p.values[string(name)]
}

// ShowHelp outputs the registered options to standard out
func (p *BcsCommandLineParser) ShowHelp() {
	fmt.Println("Usage:")
	for name, opt := range p.options {
		fmt.Printf("  --%s\t%s\n", name, opt.Description)
	}
	os.Exit(0)
}
