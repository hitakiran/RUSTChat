//importing: 
use std::net::TcpListener;
use std::io::{Write, BufReader, BufRead};
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    //bind server to IP address - local host port 7878
    //listener waits for clients to connect to that address.
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Was not able to start the server. Sorry! :)");
    println!("Server is listening on 127.0.0.1:7878");// started!

    let clients = Arc::new(Mutex::new(Vec::new())); //shared client

    //loop through to take in incoming connections
    //when connected, it gets a stream (connection) from listener
    for stream_result in listener.incoming()  {
        if let Ok(stream) = stream_result { //check 
            println!("Client is connected!");

            let clone_stream = stream.try_clone().expect("Failed to clone stream"); //copy to ass it to the list
            let clone_clients = Arc::clone(&clients); //copy of shared client list

            clone_clients.lock().unwrap().push(clone_stream); //adding client to the list

            thread::spawn(move || {
                let mut reader = BufReader::new(stream);

                loop {
                    let mut buffer = String::new(); //empty string to hold incoming message form client

                    let result = reader.read_line(&mut buffer); //read each line

                    //check if reading or is message is empty - send error
                    if result.is_err() || buffer.is_empty() { //checking!!!
                        println!("Oops! Client disconnected!!");
                        break; //has to leave when client disconnects 
                    }
                    //if successful
                    println!("Message from the client: {}", buffer.trim());

                    //broadcasting to all clients & lock to access it safely later
                    let mut clients_locked = clone_clients.lock().unwrap();
                    //removing all the dead clients- disconnected ones
                    clients_locked.retain(|client| client.peer_addr().is_ok());

                    //send the received message to every connected client
                    for client in clients_locked.iter_mut() {
                        let _ = client.write_all(buffer.as_bytes()); //send as bytes
                    }

                }
            });

////////////////// This was Checkpoint 1 - connecting one client to the server - not needed anymroe
            /*
            let mut buffer = [0; 512]; //storage

            //read client messsage add it to buffer
            let result = stream.read(&mut buffer);
            result.expect("Can not read the client message!"); 

            //fromutf8_lossy : converts bytes to string
            let message = String::from_utf8_lossy(&buffer);

            println!("Message from the client: {}", message); //prints to terminal
            */
        }
    }

}
