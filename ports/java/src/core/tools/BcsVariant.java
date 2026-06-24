package bcs.core.tools;

// BcsVariant acts as a dynamic type wrapper mapping natively to Java's `Object`
public class BcsVariant {
    private Object value;

    public BcsVariant(Object value) {
        this.value = value;
    }

    public Object getValue() {
        return value;
    }

    public void setValue(Object value) {
        this.value = value;
    }

    public boolean isNil() {
        return value == null;
    }

    @Override
    public String toString() {
        return value != null ? value.toString() : "Nil";
    }
}
