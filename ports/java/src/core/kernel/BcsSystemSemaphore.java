package bcs.core.kernel;

import java.util.concurrent.Semaphore;

// BcsSystemSemaphore abstracts IPC semaphores
public class BcsSystemSemaphore {
    private String key;
    private Semaphore semaphore;

    public BcsSystemSemaphore(String key) {
        this.key = key;
        this.semaphore = new Semaphore(1); // Standard Java lacks true cross-process named semaphores cleanly without JNI
    }

    public boolean acquire() {
        try {
            this.semaphore.acquire();
            return true;
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            return false;
        }
    }

    public boolean release() {
        this.semaphore.release();
        return true;
    }

    public String getKey() {
        return this.key;
    }

    public void setKey(String key) {
        this.key = key;
    }
}
