//importing
use std::net::TcpStream;
use std::io::{self, Write, BufRead, BufReader};
use std::thread;


fn main() {
    let connection = TcpStream::connect("127.0.0.1:7878"); //connect to server

    if let Ok(stream) = connection {
        println!("Server is Connected! Type a message to send:");
        //cloning the stream so that one can read and one can write
        let mut write_stream = stream.try_clone().expect("Failed to clone the stream");
        let read_stream = stream; //the main original will be used for reading

        //thread to receive messages from server and to print it
        thread::spawn(move || {
            let reader = BufReader::new(read_stream);
            for line in reader.lines() {
                if let Ok(message) = line {
                    println!("> {}", message.trim()); //display incoming message
                }
            }
        });

        let stdin = io::stdin();
        loop {
            let mut input = String::new();//holds user message
            let user_input = stdin.read_line(&mut input); //read from terminal
            
            if user_input.is_ok() { //sends the message over the network to the server with bytes
                let send_result = write_stream.write_all(input.as_bytes()); //convert the string to bytes
                if send_result.is_ok() {
                    println!("Message is sent!");
                } else {
                    println!("Message is NOT sent!");
                    break; //break loop if sending fails
                }
            } else {
                println!("Failed to read user input!");
            }
        } 
    } else {
        println!("Can not connect to server!");
    }
}
