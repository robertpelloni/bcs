package bcs.gui.kernel;

// Assuming bcs.core.kernel.BcsEventLoop exists in package scope
import bcs.core.kernel.*;

public class BcsApplication {
    private BcsEventLoop eventLoop;
    private String[] args;

    public BcsApplication(String[] args) {
        // this.eventLoop = new BcsEventLoop();
        this.args = args;
    }

    public int exec() {
        System.out.println("BCS Application starting event loop...");
        // this.eventLoop.exec();
        return 0;
    }

    public void quit() {
        System.out.println("BCS Application quitting...");
    }
}
