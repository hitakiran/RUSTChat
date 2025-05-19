RUSTChat
- My project is a terminal-based client-server application written in Rust. It allows multiple clients to connect to a server and exchange messages in real time.

---
Few Prerequisites:
Before running the project, make sure to have the following installed: 
- RUST
- Git (for cloning)

---
Running the Project:
1. Clone the Repository
```bash
git clone https://github.com/hitakiran/cs128H_finalproject.git
cd cs128H_finalproject
```


2. Run the Server
- Open one terminal and run:
  
```bash
cargo run --bin server
```

- You should see:
```bash
Server is listening on 127.0.0.1:7878
```


3. Run the Client
- Open a new terminal window in the same directory and run:
```bash
cargo run --bin client
```

- You should see:
```bash
Server is Connected! Type a message to send:
```

4. Open more Clients (Optional)
- Repeat step 3 in more terminals to test with multiple clients connected at once.


---

EXAMPLE:


## Server Terminal:
```bash
Server is listening on 127.0.01:7878
Client is connected!
Message from the client: Hello world!
```

## Client Terminal:
```bash
Server is Connected! Type a message to send: 
Hello world!
Message is sent!
> Hello world!
```
