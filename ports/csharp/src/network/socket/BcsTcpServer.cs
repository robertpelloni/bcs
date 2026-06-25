using System;
using System.Net;
using System.Net.Sockets;
using System.Threading.Tasks;

namespace BCS.Network.Socket {
    // BcsTcpServer abstracts a network listener socket for C#
    public class BcsTcpServer {
        private TcpListener _listener;

        public void Listen(int port) {
            _listener = new TcpListener(IPAddress.Any, port);
            _listener.Start();
        }

        public async Task<BcsTcpSocket> AcceptAsync() {
            var client = await _listener.AcceptTcpClientAsync();
            var socket = new BcsTcpSocket();
            // In a real port, BcsTcpSocket would accept an existing TcpClient via constructor
            return socket;
        }

        public void Close() {
            _listener?.Stop();
        }
    }
}
