use std::net::UdpSocket;
//This will get run 


fn main() {
    // Create a UDP socket bound to any available address and port
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");

    let port = socket.local_addr().unwrap().port();

    print!("Listening on port {}", port);


    loop {
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((size, _)) => {
                let message = String::from_utf8_lossy(&buf[..size]);
                println!("Received message: {}", message);
            }
            Err(e) => {
                eprintln!("Failed to receive message: {}", e);
            }
        }
    }
}
