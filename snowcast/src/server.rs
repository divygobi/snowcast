use std::{arch::global_asm, collections::hash_map, io::{Read, Write}, net::{TcpListener, TcpStream}, os::macos::raw::stat};

pub struct Client{
    tcp_stream: TcpStream,
    station_number: u16,
    udp_port: u16,
}

impl Client{
    fn new(tcp_stream: TcpStream) -> Client{
        let client = Client{
            tcp_stream: tcp_stream,
            station_number: 0,
            udp_port: 0,
        };
        return client;
    }
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




    //ONE THREAD NEEDS TO CHECK FOR NEW TCP STREAM
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

    //NOW NEED TO START A THREAD RUN UDP CONNECTIONS TO SEND DATA TO LISTENERS AND TCP CONNECTIONS TO SEND ANNOUCEMENTS TO CLIENTS
    
}

fn handle_client(stream: TcpStream){
    //create a buffer to store the incoming data
    let mut client: Client = Client::new(stream);
    loop {
        let mut buffer = [0; 3];
        client.tcp_stream.read(&mut buffer).expect("Failed to read from stream");
        //If there this is a hello message 

        if buffer[0] == 0{
            println!("Received hello message from client, sending welcome message");
            client.udp_port = buffer[2] as u16;
            client.tcp_stream.write(&[1, 0, 4]).expect("Failed to write to stream");
        }else if buffer[0] == 1{
            println!("Received request for station {}", buffer[2]);
            client.station_number = buffer[2] as u16;
            
            break;
        }
        else {
            println!("Received invalid message from client");
            return;
        }
    }

}


fn send_announcement_to_client(mut client: Client, announcement: &str){
    //send announcement to client

    let announcement_bytes = create_annoucment_in_bytes(announcement);
    client.tcp_stream.write(&announcement_bytes).expect("Failed to send to announcement");
 //   client.tcp_stream.write(&[2, 0]).expect("Failed to send to announcement");
}

fn create_annoucment_in_bytes(announcement: &str) -> Vec<u8>{
    let mut announcement_bytes: Vec<u8> = vec![];
    announcement_bytes.push(1);
    announcement_bytes.push(announcement.len() as u8);
    for byte in announcement.bytes(){
        announcement_bytes.push(byte);
    }
    return announcement_bytes;
}
fn send_data_to_client(client: &Client){
    //send data to client
    //let udp_socket = UdpSocket::bind();

    
}

