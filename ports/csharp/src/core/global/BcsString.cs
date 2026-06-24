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
