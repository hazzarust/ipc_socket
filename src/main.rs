use crate::handler::Handler;
use tokio::process::Command;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::{mpsc, Mutex};
use std::path::Path;
use std::sync::Arc;
use crate::test_runner::spin_child_process;

mod handler;
mod test_runner;

#[tokio::main]
pub async fn main() {
    println!("Starting the program...");

    let python_script_path = Path::new("python/main.py");

    if !python_script_path.exists() {
        eprintln!("Python script not found: {:?}", python_script_path);
        return;
    }

    let handler = Arc::new(Mutex::new(spin_child_process(python_script_path).await));

    let (tx, mut rx) = mpsc::channel(32);

    // Clone `handler` for the listener task
    let handler_listener = Arc::clone(&handler);

    let listen_task = tokio::spawn(async move {
        let mut handler = handler_listener.lock().await;
        if let Err(e) = handler.listen_for_messages(tx).await {
            eprintln!("Error in listen_for_messages: {}", e);
        }
    });

    // Main loop to process received events
    // If we want non blocking, we need to move to seperate tokio::spawn
    while let Some(event) = rx.recv().await {
        let handler_main = Arc::clone(&handler);

        match event.as_str() {
            "hello from python" => {
                println!("Triggered by Python greeting");
                let mut handler = handler_main.lock().await;
                if let Err(e) = handler.send_message("Hello from Rust").await {
                    eprintln!("Error sending message: {}", e);
                }
            }
            "hello from china" => {
                println!("Triggered by China greeting");
                let mut handler = handler_main.lock().await;
                if let Err(e) = handler.send_message("Hello from Rust, China").await {
                    eprintln!("Error sending message: {}", e);
                }
            }
            _ => {
                println!("Unknown event: {}", event);
            }
        }
    }

    listen_task.await.unwrap();
    println!("End of main...");
}
