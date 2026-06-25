package bcs.core.io;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.Path;

// BcsFile abstracts standard file I/O operations for Java
public class BcsFile {
    private final Path path;

    public BcsFile(String pathStr) {
        this.path = Paths.get(pathStr);
    }

    public boolean exists() {
        return Files.isRegularFile(this.path);
    }

    public String readAll() throws IOException {
        return new String(Files.readAllBytes(this.path));
    }

    public void writeAll(String payload) throws IOException {
        Files.write(this.path, payload.getBytes());
    }
}
