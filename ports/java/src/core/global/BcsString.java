package bcs.core.global;

public class BcsString {
    private final String value;

    public BcsString(String val) {
        this.value = (val != null) ? val : "";
    }

    public String toUpper() {
        return this.value.toUpperCase();
    }

    public String toLower() {
        return this.value.toLowerCase();
    }

    @Override
    public String toString() {
        return this.value;
    }
}
