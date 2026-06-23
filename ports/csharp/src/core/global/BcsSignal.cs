using System;

namespace BCS.Core.Global {
    // Maps the C++ BCS Signal to a standard C# event delegate architecture
    public class BcsSignal<T> {
        public event Action<T> OnEmit;

        public void Connect(Action<T> handler) {
            OnEmit += handler;
        }

        public void Disconnect(Action<T> handler) {
            OnEmit -= handler;
        }

        public void Emit(T payload) {
            OnEmit?.Invoke(payload);
        }
    }
}
