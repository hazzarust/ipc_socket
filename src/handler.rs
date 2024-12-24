use tokio::sync::mpsc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io;
use tokio::net::UnixStream;

pub struct Handler {
    pub reader: BufReader<tokio::io::ReadHalf<UnixStream>>,
    pub writer: tokio::io::WriteHalf<UnixStream>,
}

impl Handler {
    pub fn new(stream: UnixStream) -> Self {
        let (reader, writer) = tokio::io::split(stream);
        Self {
            reader: BufReader::new(reader),
            writer,
        }
    }

    pub async fn listen_for_messages(
        &mut self,
        sender: mpsc::Sender<String>, // Send events to main
    ) -> io::Result<()> {
        let mut buffer = String::new();

        loop {
            buffer.clear();
            let bytes_read = self.reader.read_line(&mut buffer).await?;

            if bytes_read == 0 {
                println!("Connection closed by client");
                break;
            }

            let message = buffer.trim().to_string();
            println!("Received: {}", message);
            sender.send(message).await.unwrap();
                
        }

        Ok(())
    }

    pub async fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write_all(message.as_bytes()).await?;
        self.writer.flush().await?;
        println!("Sent: {}", message);

        Ok(())
    }
}
