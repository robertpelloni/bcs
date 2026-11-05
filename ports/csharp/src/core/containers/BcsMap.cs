using System.Collections.Concurrent;

namespace BCS.Core.Containers {
    // BcsMap maps to ConcurrentDictionary for thread-safe key-value storage
    public class BcsMap<K, V> {
        private ConcurrentDictionary<K, V> _items;

        public BcsMap() {
            _items = new ConcurrentDictionary<K, V>();
        }

        public void Insert(K key, V value) {
            _items[key] = value;
        }

        public bool TryGetValue(K key, out V value) {
            return _items.TryGetValue(key, out value);
        }

        public bool Contains(K key) {
            return _items.ContainsKey(key);
        }
    }
}
