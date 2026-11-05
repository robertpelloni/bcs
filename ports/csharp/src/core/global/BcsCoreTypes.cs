using System;

namespace BCS.Core.Global {
    public enum BcsGlobalColor {
        White,
        Black,
        Red,
        Green,
        Blue,
        Transparent
    }

    public static class BcsCoreLogging {
        public static void Warning(string msg) {
            Console.WriteLine($"[BCS Warning] {msg}");
        }

        public static void Critical(string msg) {
            Console.WriteLine($"[BCS Critical] {msg}");
        }
    }
}

namespace BCS.Core.Global {
    [System.Flags]
    public enum BcsAlignment {
        AlignLeft = 0x0001,
        AlignRight = 0x0002,
        AlignHCenter = 0x0004,
        AlignJustify = 0x0008,
        AlignTop = 0x0020,
        AlignBottom = 0x0040,
        AlignVCenter = 0x0080,
        AlignCenter = AlignVCenter | AlignHCenter
    }

    public enum BcsOrientation {
        Horizontal = 0x1,
        Vertical = 0x2
    }
}
