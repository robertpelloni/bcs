package io

import (
	"os"
)

// BcsDir abstracts standard directory operations
type BcsDir struct {
	path string
}

func NewBcsDir(path string) *BcsDir {
	return &BcsDir{path: path}
}

// Exists checks if the directory is present on the filesystem
func (d *BcsDir) Exists() bool {
	info, err := os.Stat(d.path)
	if os.IsNotExist(err) {
		return false
	}
	return info.IsDir()
}

// Mkdir creates the directory
func (d *BcsDir) Mkdir() error {
	return os.Mkdir(d.path, 0755)
}

// MkdirPath creates the directory and any necessary parents
func (d *BcsDir) MkdirPath() error {
	return os.MkdirAll(d.path, 0755)
}
