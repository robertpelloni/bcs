using System;
using System.Collections.Generic;
using BCS.Core.Kernel;

namespace BCS.Gui.Kernel {
    public enum ExclusionPolicy {
        None,
        Exclusive,
        ExclusiveOptional
    }

    public class BcsActionGroup : BcsObject {
        private List<BcsAction> _actions;
        public ExclusionPolicy Policy { get; set; }
        public bool IsEnabled { get; set; }
        public bool IsVisible { get; set; }

        public event Action<BcsAction> Triggered;

        public BcsActionGroup(BcsObject parent = null) : base(parent) {
            _actions = new List<BcsAction>();
            Policy = ExclusionPolicy.Exclusive;
            IsEnabled = true;
            IsVisible = true;
        }

        public void AddAction(BcsAction action) {
            _actions.Add(action);

            action.Toggled += (checkedState) => {
                if (checkedState && Policy == ExclusionPolicy.Exclusive) {
                    foreach (var a in _actions) {
                        if (a != action && a.IsChecked) {
                            a.IsChecked = false;
                        }
                    }
                }
            };

            action.Triggered += (checkedState) => {
                Triggered?.Invoke(action);
            };
        }

        public IReadOnlyList<BcsAction> Actions => _actions.AsReadOnly();

        public BcsAction CheckedAction() {
            foreach (var a in _actions) {
                if (a.IsChecked) {
                    return a;
                }
            }
            return null;
        }
    }
}
