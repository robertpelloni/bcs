using System.Collections.Concurrent;

namespace BCS.Core.Kernel {
    // BcsEvent represents the base class for all framework events
    public class BcsEvent {
        public int Type { get; }

        public BcsEvent(int type) {
            Type = type;
        }
    }

    // BcsObject is the base class for event receivers
    public class BcsObject {
        public virtual bool Event(BcsEvent e) {
            return false;
        }
    }

    // BcsEventLoop provides the execution loop for event dispatching
    public class BcsEventLoop {
        private BlockingCollection<BcsEvent> _queue;

        public BcsEventLoop() {
            _queue = new BlockingCollection<BcsEvent>(1024);
        }

        public void PostEvent(BcsEvent e) {
            _queue.Add(e);
        }

        public void Exec() {
            foreach (var e in _queue.GetConsumingEnumerable()) {
                // Dispatch event
            }
        }
    }
}
