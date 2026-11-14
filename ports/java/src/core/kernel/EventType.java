package bcs.core.kernel;

// EventType enumerates global event IDs modeled after C++ QEvent::Type.
// Values mirror ports/go (bcsevent.go) and the C# port; Child* are offset
// past User without colliding with it.
public enum EventType {
    None(0),
    Timer(1),
    MouseButtonPress(2),
    MouseButtonRelease(3),
    MouseButtonDblClick(4),
    MouseMove(5),
    KeyPress(6),
    KeyRelease(7),
    FocusIn(8),
    FocusOut(9),
    Enter(10),
    Leave(11),
    Paint(12),
    Move(13),
    Resize(14),
    Create(15),
    Destroy(16),
    Show(17),
    Hide(18),
    Close(19),
    Quit(20),
    ApplicationActivate(21),
    ApplicationDeactivate(22),
    User(1000),
    ChildAdded(1001),
    ChildRemoved(1002),
    DynamicPropertyChange(1003);

    public final int value;

    EventType(int value) {
        this.value = value;
    }
}
