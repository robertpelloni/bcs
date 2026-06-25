using System;
using BCS.Core.Kernel;

namespace BCS.Gui.Kernel {
    // BcsApplication manages the GUI application's control flow
    public class BcsApplication {
        private BcsEventLoop _eventLoop;
        private string[] _args;

        public BcsApplication(string[] args) {
            _eventLoop = new BcsEventLoop();
            _args = args;
        }

        public int Exec() {
            Console.WriteLine("BCS Application starting event loop...");
            _eventLoop.Exec();
            return 0;
        }

        public void Quit() {
            Console.WriteLine("BCS Application quitting...");
            // Post quit event
        }
    }
}
