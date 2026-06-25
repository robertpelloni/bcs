using System;
using System.IO;
using System.Net.Sockets;
using System.Threading.Tasks;

namespace BCS.Network.Socket {
    // BcsTcpSocket abstracts a network client connection for C#
    public class BcsTcpSocket {
        private TcpClient _client;
        private NetworkStream _stream;
        private StreamReader _reader;
        private StreamWriter _writer;

        public BcsTcpSocket() {
            _client = new TcpClient();
        }

        public async Task ConnectToHostAsync(string host, int port) {
            await _client.ConnectAsync(host, port);
            _stream = _client.GetStream();
            _reader = new StreamReader(_stream);
            _writer = new StreamWriter(_stream) { AutoFlush = true };
        }

        public async Task WriteAsync(string data) {
            await _writer.WriteAsync(data);
        }

        public async Task<string> ReadLineAsync() {
            return await _reader.ReadLineAsync();
        }

        public void Disconnect() {
            _stream?.Close();
            _client?.Close();
        }
    }
}
