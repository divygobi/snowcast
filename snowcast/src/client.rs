mod server;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::net::UdpSocket;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use std::vec;
use std::mem;
use std::io::{BufRead};



fn main() {

    // Start server

    // Start stdin loop
    let mut count = 0;
    let stdin = io::stdin();
    println!("Hello goon, type start to start a new client!");


    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "start" {
            println!("Starting new client");
            start_client();
        } else {
            println!("Unknown command: {}", line);
        }
    }
}

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
                            let stdin = io::stdin();
                   
                            let line = stdin.lock().lines().next().unwrap().unwrap();
                            let station: i8 = line.parse().expect("Failed to parse station number");
                            let station: u8 = station as u8;
                            if station > buffer[2] {
                                println!("Invalid station number");
                                return;
                            }
                            stream.write(&[1, 0, station]).expect("Failed to write to stream");
                           
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
}

#[repr(C)]
struct InvalidCommand {
    reply_type: u8,
    string_size: u8,
    // reply_string: [char; n] // This is not valid Rust. See explanation below.
}

