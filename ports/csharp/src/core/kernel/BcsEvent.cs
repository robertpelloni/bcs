using System;
using System.Collections.Concurrent;
using System.Collections.Generic;

namespace BCS.Core.Kernel {
    public class BcsEvent : IComparable<BcsEvent> {
        public int Type { get; set; }
        public int Priority { get; set; }

        public BcsEvent(int type, int priority = 0) {
            Type = type;
            Priority = priority;
        }

        public int CompareTo(BcsEvent other) {
            // Higher priority first
            return other.Priority.CompareTo(this.Priority);
        }
    }

    public class BcsObject {
        public BcsObject Parent { get; set; }
        public List<BcsObject> Children { get; set; }
        private List<Func<BcsEvent, bool>> _eventFilters;

        public BcsObject(BcsObject parent = null) {
            Parent = parent;
            Children = new List<BcsObject>();
            _eventFilters = new List<Func<BcsEvent, bool>>();
            if (parent != null) {
                parent.Children.Add(this);
            }
        }

        public void InstallEventFilter(Func<BcsEvent, bool> filter) {
            _eventFilters.Add(filter);
        }

        public virtual bool Event(BcsEvent e) {
            foreach (var filter in _eventFilters) {
                if (filter(e)) {
                    return true;
                }
            }
            return false;
        }
    }

    public class BcsEventLoop {
        // In C# .NET 10+ we can use PriorityQueue, but for broader compatibility a BlockingCollection with a concurrent wrapper or custom wait is common.
        // We'll use a BlockingCollection of a concurrent priority structure, or a simple Monitor over a PriorityQueue.
        private PriorityQueue<BcsEvent, int> _queue;
        private readonly object _lock = new object();
        private bool _quit = false;

        public BcsEventLoop() {
            _queue = new PriorityQueue<BcsEvent, int>();
        }

        public void PostEvent(BcsEvent e) {
            lock (_lock) {
                // PriorityQueue dequeues lowest value first, so invert priority
                _queue.Enqueue(e, -e.Priority);
                System.Threading.Monitor.Pulse(_lock);
            }
        }

        public void Exec() {
            lock (_lock) {
                while (!_quit) {
                    while (_queue.Count == 0 && !_quit) {
                        System.Threading.Monitor.Wait(_lock);
                    }

                    if (_quit) break;

                    BcsEvent e = _queue.Dequeue();

                    System.Threading.Monitor.Exit(_lock);
                    // Dispatch event
                    System.Threading.Monitor.Enter(_lock);
                }
            }
        }

        public void Quit() {
            lock (_lock) {
                _quit = true;
                System.Threading.Monitor.PulseAll(_lock);
            }
        }
    }
}
