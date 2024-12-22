use tokio::net::UnixStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::io;


pub struct Handler {
    pub reader: BufReader<tokio::io::ReadHalf<UnixStream>>,
    pub writer: tokio::io::WriteHalf<UnixStream>,
}

impl Handler {
    pub fn new(stream: UnixStream) -> Self {
        let (reader, writer) = tokio::io::split(stream);
        Self {
            // Allows us to read data from the reader more efficiently (essentially the socket)
            reader: BufReader::new(reader),
            writer,
        }
    }

    pub async fn listen_for_messages(&mut self) -> io::Result<()> {
        let mut buffer = String::new();

        loop {
            buffer.clear();
            let bytes_read = self.reader.read_line(&mut buffer).await?;

            if bytes_read == 0 {
                println!("Connection closed by client");
                break;
            }
            
            println!("Received: {}", buffer.trim());

            if buffer.trim() == "hello from python"{
                self.send_message("Hello from Rust").await?;
            }
            
            if buffer.trim() == "hello from china"{
                self.send_message("Hello from Rust").await?;
            }

        }

        Ok(())
    }

    pub async fn send_message(&mut self, message: &str) -> io::Result<()> {
        // Write a message to the underlying stream represented by self.writer
        // self.writer is a mutable reference to a UnixListener that has been split 
        // Convert into bytes as most I/O operations deal with bytes instead of strings
        // write.all() means to only return once the whole message has been sent
        self.writer.write_all(message.as_bytes()).await?;
        // Flush forces any buffered data waiting to be transmitted out 
        self.writer.flush().await?;
        println!("Sent: {}", message);

        Ok(())
    }
}
