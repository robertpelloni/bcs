using System;
using System.Threading;

namespace BCS.Core.Kernel {
    // BcsSystemSemaphore maps native Windows/Unix Semaphore concepts
    public class BcsSystemSemaphore {
        public string Key { get; private set; }
        private Semaphore _semaphore;

        public BcsSystemSemaphore(string key, int initialCount = 1, int maximumCount = 1) {
            Key = key;
            // Native cross-process semaphore
            _semaphore = new Semaphore(initialCount, maximumCount, key);
        }

        public bool Acquire() {
            return _semaphore.WaitOne();
        }

        public bool Release() {
            _semaphore.Release();
            return true;
        }

        public void SetKey(string key) {
            Key = key;
        }
    }

    // BcsSharedMemory maps MemoryMappedFiles
    public class BcsSharedMemory {
        public string Key { get; private set; }
        public int Size { get; private set; }
        public bool IsAttached { get; private set; }

        public BcsSharedMemory(string key) {
            Key = key;
            Size = 0;
            IsAttached = false;
        }

        public bool Attach() {
            IsAttached = true;
            return true;
        }

        public bool Detach() {
            IsAttached = false;
            return true;
        }

        public void SetKey(string key) {
            Key = key;
        }
    }
}
