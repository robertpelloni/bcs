package bcs.core.containers;

import java.util.ArrayList;

// BcsVector is mapped to Java's native ArrayList with framework-specific semantics
public class BcsVector<T> {
    private final ArrayList<T> items;

    public BcsVector() {
        this.items = new ArrayList<>();
    }

    public void append(T item) {
        this.items.add(item);
    }

    public T at(int index) {
        return this.items.get(index);
    }

    public int size() {
        return this.items.size();
    }

    public void clear() {
        this.items.clear();
    }
}
