using System.Threading;

namespace Bcs.Core.Thread {
    // BcsMutex maps to C# Monitor/lock semantics
    public class BcsMutex {
        private readonly object _lockObj = new object();

        public void Lock() {
            Monitor.Enter(_lockObj);
        }

        public void Unlock() {
            Monitor.Exit(_lockObj);
        }
    }
}
