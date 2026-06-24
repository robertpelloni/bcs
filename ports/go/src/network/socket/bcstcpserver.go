package socket

import (
	"net"
)

// BcsTcpServer abstracts a network listener socket
type BcsTcpServer struct {
	listener net.Listener
}

func NewBcsTcpServer() *BcsTcpServer {
	return &BcsTcpServer{}
}

// Listen starts listening on the specified port
func (s *BcsTcpServer) Listen(port string) error {
	listener, err := net.Listen("tcp", ":"+port)
	if err != nil {
		return err
	}
	s.listener = listener
	return nil
}

// Accept blocks until a new client connection is established
func (s *BcsTcpServer) Accept() (*BcsTcpSocket, error) {
	conn, err := s.listener.Accept()
	if err != nil {
		return nil, err
	}
	return &BcsTcpSocket{conn: conn}, nil
}

// Close stops the server from listening
func (s *BcsTcpServer) Close() error {
	if s.listener != nil {
		return s.listener.Close()
	}
	return nil
}
