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
