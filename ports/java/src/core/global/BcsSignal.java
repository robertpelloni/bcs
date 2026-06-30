package bcs.core.global;

import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.function.Consumer;

public class BcsSignal<T> {
    private final List<Consumer<T>> listeners;
    private static final ExecutorService executor = Executors.newCachedThreadPool();

    public BcsSignal() {
        this.listeners = new CopyOnWriteArrayList<>();
    }

    public void connect(Consumer<T> callback) {
        this.listeners.add(callback);
    }

    public void emit(T payload) {
        for (Consumer<T> listener : this.listeners) {
            executor.submit(() -> listener.accept(payload));
        }
    }
}
