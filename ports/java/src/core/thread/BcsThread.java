package bcs.core.thread;

// BcsThread abstracts standard thread execution for Java
public class BcsThread {
    private Thread thread;

    public void start(Runnable runnable) {
        this.thread = new Thread(runnable);
        this.thread.start();
    }

    public void waitThread() {
        if (this.thread != null) {
            try {
                this.thread.join();
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }
    }

    public static void sleep(int ms) {
        try {
            Thread.sleep(ms);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }
}
