import socket
import time
import os

def main():
    # Path to the Unix socket file
    print("Starting Python script...")
    
    socket_path = os.getenv('SOCKET')

    if os.path.exists(socket_path):
        print(f"Socket file {socket_path} found. Attempting to connect...")
    else:
        print(f"Socket file {socket_path} not found.")

    # Create a Unix domain socket
    client_socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)

    # Sleep to simulate waiting for the server to be ready (remove if unnecessary)
    time.sleep(5)
    
    # Try to connect to the Unix socket
    try:
        client_socket.connect(socket_path)
        print("Connected to server.")
    except socket.error as e:
        print(f"Error connecting to socket: {e}")
        return

    # Send a message to the server
    message = "hello from china\n"
    try:
        client_socket.sendall(message.encode())
        print(f"Sent message: {message}")
    except socket.error as e:
        print(f"Error sending message: {e}")
        return

    # Receive the response from the server
    try:
        response = client_socket.recv(1024)  # Buffer size of 1024 bytes
        print(f"Received from server: {response.decode()}")
    except socket.error as e:
        print(f"Error receiving response: {e}")
        return

    # Close the socket connection
    client_socket.close()
    print("Connection closed.")

if __name__ == "__main__":
    main()
