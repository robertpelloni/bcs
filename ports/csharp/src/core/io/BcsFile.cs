using System.IO;

namespace BCS.Core.IO {
    // BcsFile abstracts standard file I/O operations for C#
    public class BcsFile {
        private string _path;

        public BcsFile(string path) {
            _path = path;
        }

        public bool Exists() {
            return File.Exists(_path);
        }

        public string ReadAll() {
            return File.ReadAllText(_path);
        }

        public void WriteAll(string payload) {
            File.WriteAllText(_path, payload);
        }
    }
}
