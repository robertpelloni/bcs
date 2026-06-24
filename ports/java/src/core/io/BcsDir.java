package bcs.core.io;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.Path;

// BcsDir abstracts standard directory operations for Java
public class BcsDir {
    private final Path path;

    public BcsDir(String pathStr) {
        this.path = Paths.get(pathStr);
    }

    public boolean exists() {
        return Files.isDirectory(this.path);
    }

    public void mkdir() throws IOException {
        Files.createDirectory(this.path);
    }

    public void mkdirPath() throws IOException {
        Files.createDirectories(this.path);
    }
}
