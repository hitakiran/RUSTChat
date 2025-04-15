//importing: 

use std::net::TcpListener;
use std::io::Read;

fn main() {
    //bind server to IP address - local host port 7878
    //listener waits for clients to connect to that address.
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Was not able to start the server. Sorry! :)");
    println!("Server is listening on 127.0.0.1:7878");// started!

    //loop through to take in incoming connections
    //when connected, it gets a stream (connection) from listener
    for stream_result in listener.incoming()  {
        if let Ok(mut stream) = stream_result { //check 
            println!("Client is connected!");

            let mut buffer = [0; 512]; //storage

            //read client messsage add it to buffer
            let result = stream.read(&mut buffer);
            result.expect("Can not read the client message!"); 

            //fromutf8_lossy : converts bytes to string
            let message = String::from_utf8_lossy(&buffer);

            println!("Message from the client: {}", message); //prints to terminal
        }
    }

}
