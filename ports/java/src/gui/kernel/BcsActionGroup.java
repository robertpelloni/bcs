package bcs.gui.kernel;

import bcs.core.kernel.BcsObject;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Consumer;

public class BcsActionGroup extends BcsObject {
    public enum ExclusionPolicy {
        None,
        Exclusive,
        ExclusiveOptional
    }

    private List<BcsAction> actions;
    private ExclusionPolicy exclusionPolicy;
    private boolean enabled;
    private boolean visible;

    // Callbacks
    private Consumer<BcsAction> triggered;

    public BcsActionGroup(BcsObject parent) {
        super(parent);
        this.actions = new ArrayList<>();
        this.exclusionPolicy = ExclusionPolicy.Exclusive;
        this.enabled = true;
        this.visible = true;
    }

    public void addAction(BcsAction action) {
        this.actions.add(action);

        // Intercept action toggling
        action.setToggledCallback(checked -> {
            if (checked && this.exclusionPolicy == ExclusionPolicy.Exclusive) {
                for (BcsAction a : this.actions) {
                    if (a != action && a.isChecked()) {
                        a.setChecked(false);
                    }
                }
            }
        });

        // Forward trigger
        action.setTriggeredCallback(checked -> {
            if (this.triggered != null) {
                this.triggered.accept(action);
            }
        });
    }

    public List<BcsAction> getActions() {
        return this.actions;
    }

    public BcsAction checkedAction() {
        for (BcsAction a : this.actions) {
            if (a.isChecked()) {
                return a;
            }
        }
        return null;
    }

    public void setExclusionPolicy(ExclusionPolicy policy) {
        this.exclusionPolicy = policy;
    }

    public ExclusionPolicy getExclusionPolicy() {
        return this.exclusionPolicy;
    }

    public void setTriggeredCallback(Consumer<BcsAction> cb) {
        this.triggered = cb;
    }
}
