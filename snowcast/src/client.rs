mod server;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::net::UdpSocket;
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::thread;
use std::time::Duration;
use std::vec;
use std::mem;
use std::io::BufRead;
use std::process::Command;



//By trying to bind port 0 to a tcp/udp connection, the OS will assign a random port to the connection,
// this cilent binary should open the listener on the port assigned by the OS
fn main() {

    // Start lis
    let mut start_data_listener = Command::new("sh");
//
    // Start stdin loop
    let mut count = 0;
    let stdin = io::stdin();
    println!("Hello goon, type start to start a new client!");

    //need a dedicated thread for std in[ut, ]
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "start" {
            println!("Starting new client");
         //   thread::spawn(|| 
                start_client()
           // );
        } else {
            println!("Unknown command: {}", line);
        }
    }
}

//We need to have different clients being supported so they need to listen on different ports I think
fn start_client() {
    // Start client
    match TcpStream::connect("localhost:8080") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8080");

          //  loop {
                // Send "hello" to the server.
                println!("Sending hello message to server");

                stream.write(&[0, 0, 90]).expect("Failed to write to stream");
                
                // Wait for a second.
                thread::sleep(Duration::from_secs(1));

                // Read the server's response.
                let mut buffer = [0; 3];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        if buffer[0] == 1 {
                            println!("Received welcome message from server");
                            println!("Number of stations: {}", buffer[2]);
                            
                            println!("Enter the station number you want to listen to:");
                           // let stdin = io::stdin();
                            let station: u8 = 2;

                            // for line in stdin.lock().lines() {
                            //     let line = line.unwrap();
                                
                            //     let station_read: i8 = line.parse().expect("Failed to parse station number");
                            //     station = station_read as u8;
                            //     if station > buffer[2] {
                            //         println!("Invalid station number");
                            //         return;
                            //     }
                            println!("Requesting station {}", station);
                            stream.write(&[1, 0, station]).expect("Failed to write to stream");
                                //just read one line lul
                                
                           // }
                           
                        }
                        else{
                            println!("Received invalid message from server");
                        }
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        return;
                    }
                }
          //  }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}



pub struct Server{
    //Default port
    tcp_port: u16,
    //List of clients(vec of pointers clients)
    clients: Vec<Client>,
    //Path to data file 
    data_filepath: String,

}

impl Server {
    fn init_server(&mut self){
        self.tcp_port = 8080;
    }

    fn run_server(&mut self){
        //open a tcp port to listen
        let listener = TcpListener::bind(("localhost:{}", self.tcp_port));
        println!("")

    }
    
}

struct Client{
    control_tcp_port: u16,
    listener_udp_port: u16,
}


fn server(){
    //open tcp listener, this will open ports
    //create list of 


}

fn client_control(){
 //   let address: str& = ("localhost:{}", serverTcpPort);
}

fn client_listener(){

}


#[repr(C)]
struct HelloCommand {
    command_type: u8,
    udp_port: u16,
}

#[repr(C)]
struct SetStationCommand {
    command_type: u8,
    station_number: u16,
}

#[repr(C)]
struct WelcomeCommand {
    reply_type: u8,
    num_stations: u16,
}

#[repr(C)]
struct Announce {
    reply_type: u8,
    songname_size: u8,
    // songname: [char; n] // This is not valid Rust. See explanation below.