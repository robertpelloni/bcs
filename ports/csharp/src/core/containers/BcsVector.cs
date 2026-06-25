using System.Collections.Generic;

namespace BCS.Core.Containers {
    // BcsVector is mapped to C#'s native List<T> with framework-specific semantics
    public class BcsVector<T> {
        private List<T> _items;

        public BcsVector() {
            _items = new List<T>();
        }

        public void Append(T item) {
            _items.Add(item);
        }

        public T At(int index) {
            return _items[index];
        }

        public int Size() {
            return _items.Count;
        }

        public void Clear() {
            _items.Clear();
        }
    }
}
