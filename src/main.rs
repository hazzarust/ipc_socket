use test_runner::spin_child_process;
use std::path::Path;


mod handler;
mod test_runner;

#[tokio::main]
pub async fn main() { 

    println!("Starting the program...");

    // Path to python script
    let python_script_path = Path::new("python/main.py");
    
    let socket_path = "tmp/socket.sock";

    // If socket path exists already, make sure to delete it
    if std::path::Path::new(&socket_path).exists(){
        std::fs::remove_file(socket_path).expect("Failed to remove file"); 
    }

    // Make sure the script exists
    if !python_script_path.exists() {
        eprintln!("Python script not found: {:?}", python_script_path);
    }

    let mut handler = spin_child_process(python_script_path, socket_path).await;

    let listen_task = tokio::spawn(async move {
        handler.listen_for_messages().await.unwrap();
    });
    
    // Await the task here
    listen_task.await.unwrap();  // Block until listen_for_messages() completes

    println!("End of main...");
    
}

