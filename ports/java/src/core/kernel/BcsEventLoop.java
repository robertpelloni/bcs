package bcs.core.kernel;

import java.util.concurrent.PriorityBlockingQueue;

public class BcsEventLoop {
    private PriorityBlockingQueue<BcsEvent> queue;
    private final BcsEventDispatcher dispatcher;
    private volatile boolean quit = false;

    public BcsEventLoop() {
        this(null, null);
    }

    // Dispatcher-backed loop (used by BcsKernel/BcsApplication).
    public BcsEventLoop(BcsEventDispatcher dispatcher, BcsObject parent) {
        this.queue = new PriorityBlockingQueue<>(1024);
        this.dispatcher = dispatcher;
    }

    public void postEvent(BcsEvent e) {
        if (dispatcher != null) {
            dispatcher.postEvent(e);
        } else {
            this.queue.put(e);
        }
    }

    public int exec() {
        if (dispatcher != null) {
            dispatcher.exec();
            return 0;
        }
        try {
            while (!quit) {
                BcsEvent e = this.queue.take();
                // Dispatch event to application roots
            }
        } catch (InterruptedException ex) {
            Thread.currentThread().interrupt();
        }
        return 0;
    }

    public void quit() {
        this.quit = true;
        BcsEvent quitEvent = new BcsEvent(-1, Integer.MAX_VALUE);
        this.postEvent(quitEvent); // Unblock queue.take()
    }
}
