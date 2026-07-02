package bcs.core.tools;

import bcs.core.global.BcsString;
import java.util.HashMap;
import java.util.Map;

public class BcsCommandLineParser {

    public static class BcsCommandLineOption {
        public BcsString name;
        public BcsString description;
        public BcsString valueName;

        public BcsCommandLineOption(BcsString name, BcsString description, BcsString valueName) {
            this.name = name;
            this.description = description;
            this.valueName = valueName;
        }
    }

    private Map<String, BcsCommandLineOption> options;
    private Map<String, BcsString> values;

    public BcsCommandLineParser() {
        this.options = new HashMap<>();
        this.values = new HashMap<>();
    }

    public void addOption(BcsCommandLineOption opt) {
        this.options.put(opt.name.toString(), opt);
    }

    public void process(String[] args) {
        for (int i = 0; i < args.length; i++) {
            String arg = args[i];
            if (arg.length() > 2 && arg.startsWith("--")) {
                String flag = arg.substring(2);
                if (this.options.containsKey(flag)) {
                    this.values.put(flag, new BcsString("true"));
                }
            }
        }
    }

    public boolean isSet(BcsString name) {
        return this.values.containsKey(name.toString());
    }

    public BcsString value(BcsString name) {
        return this.values.get(name.toString());
    }

    public void showHelp() {
        System.out.println("Usage:");
        for (Map.Entry<String, BcsCommandLineOption> entry : this.options.entrySet()) {
            System.out.println("  --" + entry.getKey() + "\t" + entry.getValue().description);
        }
        System.exit(0);
    }
}
