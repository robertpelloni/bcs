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

    public String getType() {
        return value != null ? value.getClass().getSimpleName() : "Nil";
    }

    @Override
    public String toString() {
        return value != null ? value.toString() : "Nil";
    }

    public Integer toInt() {
        if (value instanceof Integer) {
            return (Integer) value;
        }
        return null;
    }

    public Double toFloat() {
        if (value instanceof Double) {
            return (Double) value;
        }
        return null;
    }

    public Boolean toBool() {
        if (value instanceof Boolean) {
            return (Boolean) value;
        }
        return null;
    }
}
