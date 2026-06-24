package socket

import (
	"bufio"
	"net"
)

// BcsTcpSocket abstracts a network client connection
type BcsTcpSocket struct {
	conn net.Conn
}

func NewBcsTcpSocket() *BcsTcpSocket {
	return &BcsTcpSocket{}
}

func (s *BcsTcpSocket) ConnectToHost(host string, port string) error {
	conn, err := net.Dial("tcp", host+":"+port)
	if err != nil {
		return err
	}
	s.conn = conn
	return nil
}

func (s *BcsTcpSocket) Write(data []byte) (int, error) {
	return s.conn.Write(data)
}

func (s *BcsTcpSocket) ReadLine() (string, error) {
	reader := bufio.NewReader(s.conn)
	return reader.ReadString('\n')
}

func (s *BcsTcpSocket) Disconnect() error {
	if s.conn != nil {
		return s.conn.Close()
	}
	return nil
}
