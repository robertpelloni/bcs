package bcs.core.thread;

import java.util.concurrent.locks.ReentrantLock;

// BcsMutex maps to Java's ReentrantLock for explicit locking
public class BcsMutex {
    private final ReentrantLock lock = new ReentrantLock();

    public void lock() {
        lock.lock();
    }

    public void unlock() {
        lock.unlock();
    }
}
