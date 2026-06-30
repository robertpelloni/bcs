using System;
using System.Collections.Generic;
using System.Threading.Tasks;

namespace BCS.Core.Global {
    public class BcsSignal<T> {
        private readonly List<Action<T>> _listeners = new List<Action<T>>();
        private readonly object _lock = new object();

        public void Connect(Action<T> callback) {
            lock (_lock) {
                _listeners.Add(callback);
            }
        }

        public void Emit(T payload) {
            List<Action<T>> listenersCopy;
            lock (_lock) {
                listenersCopy = new List<Action<T>>(_listeners);
            }

            foreach (var listener in listenersCopy) {
                Task.Run(() => listener(payload));
            }
        }
    }
}
