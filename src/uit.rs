use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start() -> Result<() , Box<dyn std::error::Error>> {
    let tcp_accpetor =
        tokio::net::TcpListener::bind("192.168.1.14:4444").await?;
    println!("It's listening for connections in 192.168.1.14:4444.");
    loop {
        let (mut socket , addr) = tcp_accpetor.accept().await?;
        println!("Connection received from {}", addr);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Connection closed by {}" , addr);
                        return; }
                    Ok(n) => n,
                    Err(e) =>{
                        eprintln!("Failed to read from socket: {}", e);
                        return;
                    }
                };
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Failed to write to socket: {}", e);
                    return;
                }
            }

        });
    }

}