package bcs.gui.kernel;

import bcs.core.global.BcsSignal;
import bcs.core.kernel.BcsEvent; // BcsObject conceptual parent
import bcs.core.kernel.*; // Assuming BcsObject is here
import java.util.concurrent.ConcurrentHashMap;

// BcsInputOwner maps the conceptual multi-user network ID to an input source
class BcsInputOwner {
    public String id;
    public String name;

    public BcsInputOwner(String id, String name) {
        this.id = id;
        this.name = name;
    }
}

// BcsInputArbitrator handles multi-user event streams and focus delegation natively
public class BcsInputArbitrator {
    private final ConcurrentHashMap<String, BcsInputOwner> activeOwners;
    public final BcsSignal<BcsInputOwner> focusChanged;

    public BcsInputArbitrator() {
        this.activeOwners = new ConcurrentHashMap<>();
        this.focusChanged = new BcsSignal<>();
    }

    // RegisterOwner adds a new network or local user to the arbitrator's pool
    public void registerOwner(BcsInputOwner owner) {
        activeOwners.put(owner.id, owner);
    }

    // RequestFocus attempts to route the target focus token to the specified owner
    public boolean requestFocus(String ownerId, Object targetWidget) { // Object used for BcsObject proxy
        BcsInputOwner owner = activeOwners.get(ownerId);

        if (owner == null || targetWidget == null) {
            return false;
        }

        // Emit signal broadcasting focus shift logic
        focusChanged.emit(owner);
        return true;
    }
}
