package bcs.core.global;

// Java uses GC, so these are semantic wrappers mapping the C++ ownership concepts.
public class BcsPointer {
<<<<<<< HEAD

=======

>>>>>>> origin/master
    public static class BcsSharedPointer<T> {
        private T value;

        public BcsSharedPointer(T value) {
            this.value = value;
        }

        public T get() { return value; }
    }

    public static class BcsUniquePointer<T extends AutoCloseable> implements AutoCloseable {
        private T value;

        public BcsUniquePointer(T value) {
            this.value = value;
        }

        public T get() { return value; }

        @Override
        public void close() throws Exception {
            if (value != null) {
                value.close();
                value = null;
            }
        }
    }
}
