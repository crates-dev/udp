use crate::*;

pub async fn init() -> io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8080").await?;
    let mut buf = [0u8; 1024];
    loop {
        let (n, addr) = socket.recv_from(&mut buf).await?;
        let data = &buf[..n];
        println!("Received {} bytes from {}: {:?}", n, addr, data);
        socket.send_to(data, addr).await?;
    }
}
