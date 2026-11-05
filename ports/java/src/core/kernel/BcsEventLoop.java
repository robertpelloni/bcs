package bcs.core.kernel;

import java.util.concurrent.PriorityBlockingQueue;

public class BcsEventLoop {
    private PriorityBlockingQueue<BcsEvent> queue;
    private volatile boolean quit = false;

    public BcsEventLoop() {
        this.queue = new PriorityBlockingQueue<>(1024);
    }

    public void postEvent(BcsEvent e) {
        this.queue.put(e);
    }

    public void exec() {
        try {
            while (!quit) {
                BcsEvent e = this.queue.take();
                // Dispatch event to application roots
            }
        } catch (InterruptedException ex) {
            Thread.currentThread().interrupt();
        }
    }

    public void quit() {
        this.quit = true;
        BcsEvent quitEvent = new BcsEvent(-1, Integer.MAX_VALUE);
        this.postEvent(quitEvent); // Unblock queue.take()
    }
}
