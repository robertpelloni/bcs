package io

import (
	"bcs/core/global"
	"bufio"
	"io"
	"os"
)

// BcsTextStream provides a convenient interface for reading and writing text.
type BcsTextStream struct {
	reader *bufio.Reader
	writer *bufio.Writer
}

func NewBcsTextStream(file *BcsFile) *BcsTextStream {
	// Assume BcsFile has an underlying os.File accessible or passed
	// This is a conceptual map for the port
	return &BcsTextStream{
		reader: bufio.NewReader(file.file),
		writer: bufio.NewWriter(file.file),
	}
}

func (s *BcsTextStream) ReadLine() (global.BcsString, error) {
	line, err := s.reader.ReadString('\n')
	if err != nil && err != io.EOF {
		return "", err
	}
	return global.NewBcsString(line), nil
}

func (s *BcsTextStream) ReadAll() (global.BcsString, error) {
	bytes, err := io.ReadAll(s.reader)
	if err != nil {
		return "", err
	}
	return global.NewBcsString(string(bytes)), nil
}

func (s *BcsTextStream) WriteString(str global.BcsString) error {
	_, err := s.writer.WriteString(string(str))
	if err != nil {
		return err
	}
	return s.writer.Flush()
}
