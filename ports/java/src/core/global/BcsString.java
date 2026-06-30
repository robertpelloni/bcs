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

    public int length() {
        return this.value.length();
    }

    public boolean isEmpty() {
        return this.value.isEmpty();
    }

    public String substr(int index, int length) {
        if (index < 0 || index >= this.value.length()) return "";
        int end = Math.min(index + length, this.value.length());
        return this.value.substring(index, end);
    }

    public byte[] toUtf8() {
        return this.value.getBytes(java.nio.charset.StandardCharsets.UTF_8);
    }
}
