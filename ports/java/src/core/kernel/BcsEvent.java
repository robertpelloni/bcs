package bcs.core.kernel;

public class BcsEvent implements Comparable<BcsEvent> {
    public int type;
    public int priority;

    public BcsEvent(int type) {
        this(type, 0);
    }

    public BcsEvent(int type, int priority) {
        this.type = type;
        this.priority = priority;
    }

    @Override
    public int compareTo(BcsEvent other) {
        // Higher priority first
        return Integer.compare(other.priority, this.priority);
    }
}
