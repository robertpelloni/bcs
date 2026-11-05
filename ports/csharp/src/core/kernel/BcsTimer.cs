using System;
using System.Threading;
using System.Threading.Tasks;

namespace BCS.Core.Kernel {
    // BcsTimer provides a cross-platform high-level timer mapping to C++ QTimer natively using Task.Delay
    public class BcsTimer : BcsObject {
        private int _interval;
        private bool _singleShot;
        private bool _active;
        private CancellationTokenSource _cts;

        public Action TimeoutEvent { get; set; }

        public BcsTimer(BcsObject parent = null) : base(parent) {
            _interval = 0;
            _singleShot = false;
            _active = false;
        }

        public void SetInterval(int msec) {
            _interval = msec;
        }

        public void SetSingleShot(bool singleShot) {
            _singleShot = singleShot;
        }

        public void Start() {
            if (_active) return;
            _active = true;
            _cts = new CancellationTokenSource();
            var token = _cts.Token;

            Task.Run(async () => {
                try {
                    if (_singleShot) {
                        await Task.Delay(_interval, token);
                        if (!token.IsCancellationRequested) {
                            TimeoutEvent?.Invoke();
                            _active = false;
                        }
                    } else {
                        while (_active && !token.IsCancellationRequested) {
                            await Task.Delay(_interval, token);
                            if (!token.IsCancellationRequested) {
                                TimeoutEvent?.Invoke();
                            }
                        }
                    }
                } catch (TaskCanceledException) {
                    // Timer stopped
                }
            }, token);
        }

        public void Stop() {
            if (!_active) return;
            _active = false;
            _cts?.Cancel();
        }

        public bool IsActive() {
            return _active;
        }
    }
}
