using System;

namespace BCS.Core.Global {
    public class BcsString {
        private string _value;

        public BcsString(string val) {
            _value = val ?? string.Empty;
        }

        public string ToUpper() {
            return _value.ToUpperInvariant();
        }

        public string ToLower() {
            return _value.ToLowerInvariant();
        }
<<<<<<< HEAD

=======

>>>>>>> origin/master
        public override string ToString() {
            return _value;
        }
    }
}

namespace BCS.Core.Global {
    public partial class BcsString {
        public int Length() {
            return _value.Length;
        }

        public bool IsEmpty() {
            return string.IsNullOrEmpty(_value);
        }

        public string Substr(int index, int length) {
            if (index < 0 || index >= _value.Length) return string.Empty;
            int len = System.Math.Min(length, _value.Length - index);
            return _value.Substring(index, len);
        }

        public byte[] ToUtf8() {
            return System.Text.Encoding.UTF8.GetBytes(_value);
        }
    }
}
