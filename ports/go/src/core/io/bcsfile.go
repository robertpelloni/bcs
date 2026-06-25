package io

import (
	"os"
)

// BcsFile abstracts standard file I/O operations
type BcsFile struct {
	path string
	file *os.File
}

func NewBcsFile(path string) *BcsFile {
	return &BcsFile{path: path}
}

// Exists checks if the file is present on the filesystem
func (f *BcsFile) Exists() bool {
	info, err := os.Stat(f.path)
	if os.IsNotExist(err) {
		return false
	}
	return !info.IsDir()
}

// ReadAll loads the entire file into a string
func (f *BcsFile) ReadAll() (string, error) {
	bytes, err := os.ReadFile(f.path)
	if err != nil {
		return "", err
	}
	return string(bytes), nil
}

// WriteAll writes the payload string to the file
func (f *BcsFile) WriteAll(payload string) error {
	return os.WriteFile(f.path, []byte(payload), 0644)
}
