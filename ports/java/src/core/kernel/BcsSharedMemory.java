package bcs.core.kernel;

// BcsSharedMemory abstracts IPC shared memory
public class BcsSharedMemory {
    private String key;
    private int size;
    private boolean attached;

    public BcsSharedMemory(String key) {
        this.key = key;
        this.size = 0;
        this.attached = false;
    }

    public boolean attach() {
        this.attached = true;
        return true;
    }

    public boolean detach() {
        this.attached = false;
        return true;
    }

    public boolean isAttached() {
        return this.attached;
    }

    public int getSize() {
        return this.size;
    }

    public String getKey() {
        return this.key;
    }

    public void setKey(String key) {
        this.key = key;
    }
}
