package bcs.network.socket;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.net.Socket;

// BcsTcpSocket abstracts a network client connection for Java
public class BcsTcpSocket {
    private Socket socket;
    private PrintWriter out;
    private BufferedReader in;

    public void connectToHost(String host, int port) throws IOException {
        socket = new Socket(host, port);
        out = new PrintWriter(socket.getOutputStream(), true);
        in = new BufferedReader(new InputStreamReader(socket.getInputStream()));
    }

    public void write(String data) {
        if (out != null) {
            out.print(data);
            out.flush();
        }
    }

    public String readLine() throws IOException {
        if (in != null) {
            return in.readLine();
        }
        return null;
    }

    public void disconnect() throws IOException {
        if (socket != null && !socket.isClosed()) {
            socket.close();
        }
    }
}
