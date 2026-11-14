using System;

namespace Bcs.Core.Kernel {
    // EventType enumerates global event IDs modeled after C++ QEvent::Type.
    // Values mirror ports/go (bcsevent.go); Child* are offset past User
    // without colliding with it (the Go port collides at 1000 — do not copy that).
    public enum EventType {
        None = 0,
        Timer = 1,
        MouseButtonPress = 2,
        MouseButtonRelease = 3,
        MouseButtonDblClick = 4,
        MouseMove = 5,
        KeyPress = 6,
        KeyRelease = 7,
        FocusIn = 8,
        FocusOut = 9,
        Enter = 10,
        Leave = 11,
        Paint = 12,
        Move = 13,
        Resize = 14,
        Create = 15,
        Destroy = 16,
        Show = 17,
        Hide = 18,
        Close = 19,
        Quit = 20,
        ApplicationActivate = 21,
        ApplicationDeactivate = 22,
        User = 1000,
        ChildAdded = 1001,
        ChildRemoved = 1002,
        DynamicPropertyChange = 1003
    }

    public class BcsEvent : IComparable<BcsEvent> {
        public int Type { get; set; }
        public int Priority { get; set; }

        public BcsEvent(int type, int priority = 0) {
            Type = type;
            Priority = priority;
        }

        public int CompareTo(BcsEvent other) {
            // Higher priority first
            return other.Priority.CompareTo(this.Priority);
        }
    }

    // Maps to C++ QTimerEvent: delivered to BcsObject.TimerEvent.
    public class BcsTimerEvent : BcsEvent {
        public int TimerId { get; private set; }

        public BcsTimerEvent(int timerId, int priority = 0)
            : base((int)EventType.Timer, priority) {
            TimerId = timerId;
        }
    }

    // Maps to C++ QChildEvent: delivered to BcsObject.ChildEvent.
    public class BcsChildEvent : BcsEvent {
        public BcsObject Child { get; private set; }

        public bool Added {
            get { return Type == (int)EventType.ChildAdded; }
        }

        public bool Removed {
            get { return Type == (int)EventType.ChildRemoved; }
        }

        public BcsChildEvent(EventType type, BcsObject child, int priority = 0)
            : base((int)type, priority) {
            Child = child;
        }
    }
}
