using System;
using System.Collections.Generic;
using BCS.Core.Global;

namespace BCS.Core.Tools {
    public class BcsCommandLineOption {
        public BcsString Name { get; set; }
        public BcsString Description { get; set; }
        public BcsString ValueName { get; set; }

        public BcsCommandLineOption(BcsString name, BcsString description, BcsString valueName) {
            Name = name;
            Description = description;
            ValueName = valueName;
        }
    }

    public class BcsCommandLineParser {
        private Dictionary<string, BcsCommandLineOption> options;
        private Dictionary<string, BcsString> values;

        public BcsCommandLineParser() {
            options = new Dictionary<string, BcsCommandLineOption>();
            values = new Dictionary<string, BcsString>();
        }

        public void AddOption(BcsCommandLineOption opt) {
            options[opt.Name.ToString()] = opt;
        }

        public void Process(string[] args) {
            for (int i = 0; i < args.Length; i++) {
                string arg = args[i];
                if (arg.Length > 2 && arg.StartsWith("--")) {
                    string flag = arg.Substring(2);
                    if (options.ContainsKey(flag)) {
                        values[flag] = new BcsString("true");
                    }
                }
            }
        }

        public bool IsSet(BcsString name) {
            return values.ContainsKey(name.ToString());
        }

        public BcsString Value(BcsString name) {
            if (values.TryGetValue(name.ToString(), out var val)) {
                return val;
            }
            return null;
        }

        public void ShowHelp() {
            Console.WriteLine("Usage:");
            foreach (var kvp in options) {
                Console.WriteLine($"  --{kvp.Key}\t{kvp.Value.Description}");
            }
            Environment.Exit(0);
        }
    }
}
