using System.Collections.Concurrent;
using BCS.Core.Global;
using BCS.Core.Kernel;

namespace BCS.Gui.Kernel {
    // BcsInputOwner maps the conceptual multi-user network ID to an input source
    public class BcsInputOwner {
        public string Id { get; set; }
        public string Name { get; set; }

        public BcsInputOwner(string id, string name) {
            Id = id;
            Name = name;
        }
    }

    // BcsInputArbitrator handles multi-user event streams and focus delegation natively
    public class BcsInputArbitrator {
        private ConcurrentDictionary<string, BcsInputOwner> _activeOwners;
        public BcsSignal<BcsInputOwner> FocusChanged { get; private set; }

        public BcsInputArbitrator() {
            _activeOwners = new ConcurrentDictionary<string, BcsInputOwner>();
            FocusChanged = new BcsSignal<BcsInputOwner>();
        }

        // RegisterOwner adds a new network or local user to the arbitrator's pool
        public void RegisterOwner(BcsInputOwner owner) {
            _activeOwners[owner.Id] = owner;
        }

        // RequestFocus attempts to route the target focus token to the specified owner
        public bool RequestFocus(string ownerId, BcsObject targetWidget) {
            if (!_activeOwners.TryGetValue(ownerId, out var owner) || targetWidget == null) {
                return false;
            }

            // Emit signal broadcasting focus shift logic
            FocusChanged.Emit(owner);
            return true;
        }
    }
}
