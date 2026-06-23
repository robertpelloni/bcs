package bcs.core.global;

import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.function.Consumer;

// Maps the BCS Signal/Slot system to Java's Consumer event paradigm
public class BcsSignal<T> {
    private final List<Consumer<T>> listeners = new CopyOnWriteArrayList<>();

    public void connect(Consumer<T> listener) {
        listeners.add(listener);
    }

    public void disconnect(Consumer<T> listener) {
        listeners.remove(listener);
    }

    public void emit(T payload) {
        for (Consumer<T> listener : listeners) {
            listener.accept(payload);
        }
    }
}
