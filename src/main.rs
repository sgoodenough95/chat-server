use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener, sync::broadcast};

#[tokio::main]
async fn main() {
    // Initializing TCP listener
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // Initiializing broadcast channel
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let  (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();

        // Subscribe to transmission events
        let mut rx = tx.subscribe();

        // Spawn thread to allow for concurrency
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
    
            let mut line = String::new();
    
            loop {
                // Enabling multiple concurrent branches
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}