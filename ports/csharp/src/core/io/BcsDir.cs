using System.IO;

namespace BCS.Core.IO {
    // BcsDir abstracts standard directory operations for C#
    public class BcsDir {
        private string _path;

        public BcsDir(string path) {
            _path = path;
        }

        public bool Exists() {
            return Directory.Exists(_path);
        }

        public void Mkdir() {
            Directory.CreateDirectory(_path); // CreateDirectory handles full paths implicitly in C#
        }

        public void MkdirPath() {
            Directory.CreateDirectory(_path);
        }
    }
}
