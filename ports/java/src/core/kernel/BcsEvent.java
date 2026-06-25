package bcs.core.kernel;

import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;

// BcsEvent represents the base class for all framework events
public class BcsEvent {
    public final int type;

    public BcsEvent(int type) {
        this.type = type;
    }
}

// BcsObject is the base class for event receivers
class BcsObject {
    public boolean event(BcsEvent e) {
        return false;
    }
}

// BcsEventLoop provides the execution loop for event dispatching
class BcsEventLoop {
    private final BlockingQueue<BcsEvent> queue;

    public BcsEventLoop() {
        queue = new LinkedBlockingQueue<>(1024);
    }

    public void postEvent(BcsEvent e) {
        queue.offer(e);
    }

    public void exec() {
        try {
            while (true) {
                BcsEvent e = queue.take();
                // Dispatch event
            }
        } catch (InterruptedException ex) {
            Thread.currentThread().interrupt();
        }
    }
}
