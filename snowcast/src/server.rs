use std::{ io::{Read, Write}, net::{TcpListener, TcpStream}, thread};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
pub struct Client{
    tcp_stream: TcpStream,
    station_number: u16,
    udp_port: u16,
   // currently_broadcasting: bool
}

impl Client{
    fn new(tcp_stream: TcpStream) -> Client{
        let client = Client{
            tcp_stream: tcp_stream,
            station_number: 0,
            udp_port: 0,
          //  currently_broadcasting: false,
        };
        return client;
    }

    // fn clone(self) -> Client{
    //     return Client{
    //         tcp_stream: self.tcp_stream,
    //         station_number: self.station_number,
    //         udp_port: self.udp_port,
    //         currently_broadcasting: self.currently_broadcasting,
    //     }
        
    // }
}



fn main(){
    
    //open a tcp port to listen
    static TCP_PORT: u16 = 8080;

    //Create a hashmap to store the clients, this needs to be 
 //   let mut station_map: HashMap<u16,Vec<Arc<Mutex<Client>>>> = HashMap::new();
    let (tx, rx): (Sender<Client>, Receiver<Client>) = mpsc::channel();
    


    //LOOK FOR NEW CONNECTIONS
    thread::spawn(move ||
        {let listener = TcpListener::bind("localhost:8080");
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
            Ok((stream, _)) => {
                //create a buffer to store the incoming data
                handle_client(&stream, tx.clone())
                }
            
            //TODO infinitely prints on recieving on a closed chanell, should 
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }});

    //BROADCAST TO CURRENT CURRENT CONNECTIONS.
    loop{
        //for each station, broadcast the data to the clients
        println!("Waiting for new client to broadcast to");
        match rx.recv(){
            Ok(client) => {
                let announcement = "New song is playing";
                send_announcement_to_client(&client.tcp_stream, announcement);

                // let mut client = client.clone();
           
                thread::spawn(move || {
                    send_data_to_client(&client);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    
}

fn handle_client(stream: &TcpStream, tx: Sender<Client> ){
    //create a buffer to store the incoming data
    let mut udp_port: u16 = 0;
    let mut stream = stream;
    loop {
        println!("Waiting for data from client");
        let mut buffer = [0; 3];
        stream.read(&mut buffer).expect("Failed to read from stream");
        //If there this is a hello message 
        if buffer[0] == 0{
            println!("Received hello message from client, sending welcome message");
            udp_port = buffer[2] as u16;
            //last byte is the number of stations
            stream.write(&[1, 0, 4]).expect("Failed to write to stream");
        }
        //Else if its a set station message
        else if buffer[0] == 1{
            println!("Received request for station {}", buffer[2]);
            let mut client: Client = Client::new(stream.try_clone().unwrap());
            client.udp_port = udp_port; 
            client.station_number = buffer[2] as u16;
            tx.send(client).expect("Failed to send client to broadcast");
            return;
        }
        else {
            println!("Received invalid message from client");
            return;
        }
    }

}


fn send_announcement_to_client(stream:& TcpStream, announcement: &str){
    //send announcement to client
    let mut stream = stream;
    let announcement_msg_bytes = create_annoucment_in_bytes(announcement);
    let mut announcement_bytes = vec![2, announcement.len().try_into().unwrap()];
    for byte in announcement_msg_bytes.bytes(){
        announcement_bytes.push(byte.unwrap());
    }
    stream.write(&announcement_bytes).expect("Failed to send to announcement");
   // stream.write(&[2, 0, 0]).expect("Failed to send to announcement");
    return;

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
    print!("Sending data to client on station number {}, on udp port {}", client.station_number, client.udp_port);
  //  send_announcement_to_client(client, "announcement");
    
}

