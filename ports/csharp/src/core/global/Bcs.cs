using System;

namespace BCS.Core.Global {
    public static class BcsContext {
        public const string Version = "0.2.1";

        // C# uses native types, but we provide semantic aliases if needed via struct/record,
        // or just rely on the standard library for now as direct mappings.

        public static void InitBcs() {
            // Global initialization logic for BCS framework
        }
    }
}
