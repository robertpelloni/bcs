using System;
using BCS.Core.Kernel;

namespace BCS.Gui.Kernel {
    // BcsAction represents an actionable item (e.g. in a menu or toolbar)
    public class BcsAction : BcsObject {
        public string Text { get; set; }
        public bool IsEnabled { get; set; }
        public bool IsVisible { get; set; }
        public bool IsCheckable { get; set; }

        private bool _isChecked;
        public bool IsChecked {
            get => _isChecked;
            set {
                if (!IsCheckable) return;
                if (_isChecked != value) {
                    _isChecked = value;
                    Toggled?.Invoke(_isChecked);
                }
            }
        }

        // Signals mapped to C# events
        public event Action<bool> Triggered;
        public event Action<bool> Toggled;

        public BcsAction(BcsObject parent = null) : base(parent) {
            Text = string.Empty;
            IsEnabled = true;
            IsVisible = true;
            IsCheckable = false;
            _isChecked = false;
        }

        public void Trigger() {
            if (!IsEnabled) return;
            if (IsCheckable) {
                IsChecked = !IsChecked; // This fires Toggled
            }
            Triggered?.Invoke(IsChecked);
        }
    }
}
