package bcs.core.kernel;

import java.util.Timer;
import java.util.TimerTask;

// BcsTimer provides a cross-platform high-level timer mapping to C++ QTimer natively using java.util.Timer
public class BcsTimer extends BcsObject {
    private int interval;
    private boolean singleShot;
    private boolean active;
    private Timer timer;
    private Runnable timeoutEvent;

    public BcsTimer(BcsObject parent) {
        super(parent);
        this.interval = 0;
        this.singleShot = false;
        this.active = false;
    }

    public void setInterval(int msec) {
        this.interval = msec;
    }

    public void setSingleShot(boolean singleShot) {
        this.singleShot = singleShot;
    }

    public void setTimeoutEvent(Runnable callback) {
        this.timeoutEvent = callback;
    }

    public synchronized void start() {
        if (this.active) return;
        this.active = true;
        this.timer = new Timer(true); // Run as daemon

        TimerTask task = new TimerTask() {
            @Override
            public void run() {
                if (timeoutEvent != null) {
                    timeoutEvent.run();
                }
                if (singleShot) {
                    active = false;
                }
            }
        };

        if (this.singleShot) {
            this.timer.schedule(task, this.interval);
        } else {
            this.timer.scheduleAtFixedRate(task, this.interval, this.interval);
        }
    }

    public synchronized void stop() {
        if (!this.active) return;
        this.active = false;
        if (this.timer != null) {
            this.timer.cancel();
            this.timer = null;
        }
    }

    public synchronized boolean isActive() {
        return this.active;
    }
}
