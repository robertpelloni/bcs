using System;

namespace BCS.Core.Global {
<<<<<<< HEAD
    // C# uses a GC, so BcsSharedPointer/BcsUniquePointer are semantic wrappers
=======
    // C# uses a GC, so BcsSharedPointer/BcsUniquePointer are semantic wrappers
>>>>>>> origin/master
    // or just direct object references. We provide a lightweight generic wrapper for parity.
    public class BcsSharedPointer<T> where T : class {
        public T Value { get; private set; }

        public BcsSharedPointer(T value) {
            Value = value;
        }
    }

    public class BcsUniquePointer<T> : IDisposable where T : IDisposable {
        public T Value { get; private set; }

        public BcsUniquePointer(T value) {
            Value = value;
        }

        public void Dispose() {
            if (Value != null) {
                Value.Dispose();
                Value = default(T);
            }
        }
    }
}
