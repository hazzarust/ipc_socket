
use crate::handler::Handler;
use tokio::process::Command;
use tokio::net::{UnixListener, UnixStream};
use std::path::Path;

pub async fn spin_child_process(python_script_path: &Path) -> Handler{

    let socket_path = "tmp/socket.sock";

    if std::path::Path::new(&socket_path).exists() {
        std::fs::remove_file(socket_path).expect("Failed to remove file");
    }

    let mut child = Command::new("python3")
        .arg(python_script_path)  // Provide the path to the Python script
        .env("SOCKET", socket_path)
        .stdout(std::process::Stdio::piped()) // Pipe the output so we can capture it
        .stderr(std::process::Stdio::piped()) // Capture any error output
        .spawn()  // Start the child process
        .expect("Failed to start Python process");
    
    // Set up the UnixListener binding 
    // A UnixListener listens for incoming connections on a Unix domain socket on the 
    // local machine and allows handling of these connections when they are established.
    let listener = UnixListener::bind(socket_path).expect("Failed to bind socket");
    println!("Server is listening on {}", socket_path);

    // Accept the first connection
    let (socket, _) = listener.accept().await.unwrap();
    println!("Connection accepted!");

    //Create Handler with accepted connection
    let handler = Handler::new(socket);

    handler
}