using System.IO;
using BCS.Core.Global;

namespace BCS.Core.IO {
    // BcsTextStream provides a convenient interface for reading and writing text.
    public class BcsTextStream {
        private StreamReader _reader;
        private StreamWriter _writer;

        public BcsTextStream(Stream stream) {
            _reader = new StreamReader(stream);
            _writer = new StreamWriter(stream) { AutoFlush = true };
        }

        public BcsString ReadLine() {
            return new BcsString(_reader.ReadLine());
        }

        public BcsString ReadAll() {
            return new BcsString(_reader.ReadToEnd());
        }

        public void WriteString(BcsString str) {
            _writer.Write(str.ToString());
        }
    }
}
