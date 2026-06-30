package bcs.core.kernel;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

public class BcsObject {
    public BcsObject parent;
    public List<BcsObject> children;
    private List<Predicate<BcsEvent>> eventFilters;

    public BcsObject(BcsObject parent) {
        this.parent = parent;
        this.children = new ArrayList<>();
        this.eventFilters = new ArrayList<>();
        if (parent != null) {
            parent.children.add(this);
        }
    }

    public void installEventFilter(Predicate<BcsEvent> filter) {
        this.eventFilters.add(filter);
    }

    public boolean event(BcsEvent e) {
        for (Predicate<BcsEvent> filter : eventFilters) {
            if (filter.test(e)) {
                return true;
            }
        }
        return false;
    }
}
