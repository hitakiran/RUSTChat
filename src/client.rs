use std::net::TcpStream;
use std::io::{self, Write};

fn main() {
    let connection = TcpStream::connect("127.0.0.1:7878"); //connect to server

    if let Ok(mut stream) = connection {
        println!("Server is Connected! Type a message to send:");

        let mut input = String::new();//hold user message

        let user_input = io::stdin().read_line(&mut input); //read from terminal

        if user_input.is_ok() {
            let send_result = stream.write_all(input.as_bytes()); //convert the string to bytes
            println!("Message is sent!");
        } else {
            println!("Message is NOT sent!");
        }
    } else {
        println!("Can not connect to server!");
    }
}
