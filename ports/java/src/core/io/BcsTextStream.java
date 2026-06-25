package bcs.core.io;

import bcs.core.global.BcsString;
import java.io.*;

// BcsTextStream provides a convenient interface for reading and writing text.
public class BcsTextStream {
    private BufferedReader reader;
    private PrintWriter writer;

    public BcsTextStream(InputStream in, OutputStream out) {
        if (in != null) {
            this.reader = new BufferedReader(new InputStreamReader(in));
        }
        if (out != null) {
            this.writer = new PrintWriter(out, true);
        }
    }

    public BcsString readLine() throws IOException {
        if (this.reader != null) {
            return new BcsString(this.reader.readLine());
        }
        return new BcsString("");
    }

    public void writeString(BcsString str) {
        if (this.writer != null) {
            this.writer.print(str.toString());
            this.writer.flush();
        }
    }
}
