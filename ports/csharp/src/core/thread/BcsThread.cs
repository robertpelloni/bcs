using System.Threading;
using System.Threading.Tasks;

namespace BCS.Core.Thread {
    // BcsThread abstracts thread execution, mapped to C# Tasks and Threads
    public class BcsThread {
        private Task _task;

        public void Start(System.Action action) {
            _task = Task.Run(action);
        }

        public void Wait() {
            if (_task != null) {
                _task.Wait();
            }
        }

        public static void Sleep(int ms) {
            System.Threading.Thread.Sleep(ms);
        }
    }
}
