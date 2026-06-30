using System;
using BCS.Core.Kernel;

namespace BCS.Gui.Kernel {
    // BcsApplication manages the GUI application's control flow
    public class BcsApplication : BcsObject {
        private BcsEventLoop _eventLoop;
        private string[] _args;

        public string ApplicationName { get; set; }
        public string ApplicationVersion { get; set; }
        public string OrganizationName { get; set; }
        public string OrganizationDomain { get; set; }

        public static BcsApplication Instance { get; private set; }

        public BcsApplication(string[] args) : base(null) {
            _eventLoop = new BcsEventLoop();
            _args = args;
            Instance = this;
        }

        public int Exec() {
            Console.WriteLine("BCS Application starting event loop...");
            _eventLoop.Exec();
            return 0;
        }

        public void Quit() {
            Console.WriteLine("BCS Application quitting...");
            _eventLoop.Quit();
        }

        public void ProcessEvents() {
            // Abstract wrapper matching processEvents
        }

        public void PostEvent(BcsObject receiver, BcsEvent e) {
            _eventLoop.PostEvent(e);
        }

        public bool SendEvent(BcsObject receiver, BcsEvent e) {
            if (receiver != null) {
                return receiver.Event(e);
            }
            return false;
        }
    }
}
