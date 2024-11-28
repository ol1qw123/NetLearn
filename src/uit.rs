use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufReadExt};

pub async fn start() -> Result<() , Box<dyn std::error::Error>> {
    println!("Enter the address and the port as in (192.168.1.1:6666)");

    let mut buffer_for_addr = String::new();

    
    let mut reader = tokio::io::BufReader::new(tokio::io::stdin());

    
    reader.read_line(&mut buffer_for_addr).await?;

    
    let address = buffer_for_addr.trim();
    let tcp_accpetor =
        tokio::net::TcpListener::bind(address).await?;
    println!("It's listening for connections in {}." , address);
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
