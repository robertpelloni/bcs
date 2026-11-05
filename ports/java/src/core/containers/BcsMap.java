package bcs.core.containers;

import java.util.concurrent.ConcurrentHashMap;
import java.util.Optional;

// BcsMap maps to ConcurrentHashMap for thread-safe key-value storage
public class BcsMap<K, V> {
    private final ConcurrentHashMap<K, V> items;

    public BcsMap() {
        this.items = new ConcurrentHashMap<>();
    }

    public void insert(K key, V value) {
        this.items.put(key, value);
    }

    public Optional<V> value(K key) {
        return Optional.ofNullable(this.items.get(key));
    }

    public boolean contains(K key) {
        return this.items.containsKey(key);
    }
}
