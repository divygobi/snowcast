use std::{arch::global_asm, collections::{hash_map, HashMap}, io::{Read, Write}, net::{TcpListener, TcpStream}, os::macos::raw::stat, sync::{Arc, Mutex},thread};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
pub struct Client{
    tcp_stream: TcpStream,
    station_number: u16,
    udp_port: u16,
    currently_broadcasting: bool
}

impl Client{
    fn new(tcp_stream: TcpStream) -> Client{
        let client = Client{
            tcp_stream: tcp_stream,
            station_number: 0,
            udp_port: 0,
            currently_broadcasting: false,
        };
        return client;
    }

    fn clone(self) -> Client{
        return Client{
            tcp_stream: self.tcp_stream,
            station_number: self.station_number,
            udp_port: self.udp_port,
            currently_broadcasting: self.currently_broadcasting,
        }
        
    }
}



fn main(){
    
    //open a tcp port to listen
    static TCP_PORT: u16 = 8080;

    //Create a hashmap to store the clients, this needs to be 
    let mut station_map: HashMap<u16,Vec<Arc<Mutex<Client>>>> = HashMap::new();
    let (tx, rx): (Sender<Client>, Receiver<Client>) = mpsc::channel();
    


    //LOOK FOR NEW CONNECTIONS
    thread::spawn(move ||
        {let listener = TcpListener::bind(("localhost:8080"));
        let listener = match listener{
            Ok(listener) => listener,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };
        loop{
        
        println!("Server listening on port: {}", TCP_PORT);
        match listener.accept(){
            Ok((&stream, _)) => {
                //create a buffer to store the incoming data
                handle_client(stream, tx.clone())
                }
            
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }});

    //BROADCAST TO CURRENT CURRENT CONNECTIONS.
    loop{
        //for each station, broadcast the data to the clients
        match rx.recv(){
            Ok(client) => {
                thread::spawn(move || {
                    let client = client.clone();
                    let station_number = client.station_number;
                    let announcement = "New song is playing";
                    send_announcement_to_client(client, announcement);
                    //send_data_to_client(&client);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        
    
    }
    
}

fn handle_client(&stream: TcpStream, tx: Sender<Client> ){
    //create a buffer to store the incoming data
    let mut client: Client = Client::new(stream);
    loop {
        let mut buffer = [0; 3];
        stream.read(&mut buffer).expect("Failed to read from stream");
        //If there this is a hello message 
        if buffer[0] == 0{
            println!("Received hello message from client, sending welcome message");
            client.udp_port = buffer[2] as u16;
            client.tcp_stream.write(&[1, 0, 4]).expect("Failed to write to stream");
        }
        //Else if its a set station message
        else if buffer[0] == 1{
            println!("Received request for station {}", buffer[2]);
            client.station_number = buffer[2] as u16;
            tx.send(client).unwrap();
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


    //GET DATA FROM FILE
    //SONG DATA, SONG NAME, SONG LENGTH

    //IF NEW SONG, SEND ANNOUNCEMENT TO CLIENT CONTROL(TCP)

    //TRASIMIT contiously DATA TO CLIENT LISTENER(UDP) 

    
}

