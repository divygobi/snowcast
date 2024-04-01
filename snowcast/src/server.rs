use std::{arch::global_asm, io::{Read, Write}, net::{TcpListener, TcpStream}};

pub struct Client{
    tcp_stream: TcpStream,
    station_number: u16,
    udp_port: u16,
}


fn main(){
    
    //open a tcp port to listen
    static TCP_PORT: u16 = 8080;
    let listener = TcpListener::bind(("localhost:8080"));
    let listener = match listener{
        Ok(listener) => listener,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("Server listening on port: {}", TCP_PORT);
    //let mut clients:Vec<Client> = vec![];
    loop{
        match listener.accept(){
            Ok((stream, _)) => {
                //create a buffer to store the incoming data
                handle_client(stream)
                }
            
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream){
    //create a buffer to store the incoming data
    loop {
        let mut buffer = [0; 3];
        stream.read(&mut buffer).expect("Failed to read from stream");
        //If there this is a hello message 

        if buffer[0] == 0{
            println!("Received hello message from client, sending welcome message");
            stream.write(&[1, 0, 4]).expect("Failed to write to stream");
        }else {
            println!("Received invalid message from client");
        }
    }
}