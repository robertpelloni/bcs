package bcs.network.socket;

import java.io.IOException;
import java.net.ServerSocket;
import java.net.Socket;

// BcsTcpServer abstracts a network listener socket for Java
public class BcsTcpServer {
    private ServerSocket serverSocket;

    public void listen(int port) throws IOException {
        serverSocket = new ServerSocket(port);
    }

    public BcsTcpSocket accept() throws IOException {
        if (serverSocket != null) {
            Socket clientSocket = serverSocket.accept();
            BcsTcpSocket socket = new BcsTcpSocket();
            // Internal logic would set the underlying java.net.Socket
            return socket;
        }
        return null;
    }

    public void close() throws IOException {
        if (serverSocket != null && !serverSocket.isClosed()) {
            serverSocket.close();
        }
    }
}
