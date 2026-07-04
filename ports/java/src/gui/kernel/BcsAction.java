package bcs.gui.kernel;

import bcs.core.kernel.BcsObject;
import java.util.function.Consumer;

// BcsAction represents an actionable item (e.g. in a menu or toolbar)
public class BcsAction extends BcsObject {
    private String text;
    private boolean enabled;
    private boolean visible;
    private boolean checkable;
    private boolean checked;

    // Callbacks
    private Consumer<Boolean> triggered;
    private Consumer<Boolean> toggled;

    public BcsAction(BcsObject parent) {
        super(parent);
        this.text = "";
        this.enabled = true;
        this.visible = true;
        this.checkable = false;
        this.checked = false;
    }

    public void setText(String text) {
        this.text = text;
    }

    public String getText() {
        return this.text;
    }

    public void setEnabled(boolean enabled) {
        this.enabled = enabled;
    }

    public boolean isEnabled() {
        return this.enabled;
    }

    public void setCheckable(boolean checkable) {
        this.checkable = checkable;
    }

    public boolean isCheckable() {
        return this.checkable;
    }

    public void setChecked(boolean checked) {
        if (!this.checkable) return;
        if (this.checked != checked) {
            this.checked = checked;
            if (this.toggled != null) {
                this.toggled.accept(this.checked);
            }
        }
    }

    public boolean isChecked() {
        return this.checked;
    }

    public void setTriggeredCallback(Consumer<Boolean> cb) {
        this.triggered = cb;
    }

    public void setToggledCallback(Consumer<Boolean> cb) {
        this.toggled = cb;
    }

    public void trigger() {
        if (!this.enabled) return;
        if (this.checkable) {
            this.setChecked(!this.checked);
        }
        if (this.triggered != null) {
            this.triggered.accept(this.checked);
        }
    }
}
