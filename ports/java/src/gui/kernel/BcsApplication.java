package bcs.gui.kernel;

import bcs.core.kernel.*;

public class BcsApplication extends BcsObject {
    private BcsEventLoop eventLoop;
    private String[] args;

    public String applicationName;
    public String applicationVersion;
    public String organizationName;
    public String organizationDomain;

    private static BcsApplication instance;

    public BcsApplication(String[] args) {
        super(null);
        this.eventLoop = new BcsEventLoop();
        this.args = args;
        BcsApplication.instance = this;
    }

    public static BcsApplication getInstance() {
        return instance;
    }

    public int exec() {
        System.out.println("BCS Application starting event loop...");
        this.eventLoop.exec();
        return 0;
    }

    public void quit() {
        System.out.println("BCS Application quitting...");
        this.eventLoop.quit();
    }

    public void processEvents() {
        // Abstract wrapper matching processEvents
    }

    public void postEvent(BcsObject receiver, BcsEvent e) {
        this.eventLoop.postEvent(e);
    }

    public boolean sendEvent(BcsObject receiver, BcsEvent e) {
        if (receiver != null) {
            return receiver.event(e);
        }
        return false;
    }
}
