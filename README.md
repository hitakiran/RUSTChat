# RUSTChat
**Working on Project Individually**

Simple Chat Built Using Rust

Project Name: RustCHAT

Project Owner: Hita Kiran Amba
netID: hamba2

---

Project Introduction/Idea:

The goal of this project is to build a simple chat program using Rust, consisting of a server and client. The server will manage incoming messages and broadcast them to all connected clients. The client will allow users to send messages to the server and receive messages in real-time.

This project will help me learn about network communication in Rust. While this project could eventually work on multiple machines, I will focus on getting it to work on a single machine. I aim to gain hands-on experience with networking, multi-threading, and message handling in Rust.

---

Technical Overview:
1. Server Process
   - The server will listen for incoming client connections and manage the distribution of messages to connected clients (Use TCP to handle connections).
   - The server will handle multiple client connections concurrently using multithreading in Rust.

2. Client Process:
   - The client will connect to the server.
   - The client will allow users to input and send messages to the server, while also displaying messages received from the server.
   - Features like sending and viewing messages in real-time.
  
Breakdown of Tasks: 
1. TCP Connection: Set up TCP for the server and client to communicate on a single machine.
2. Message Handling: The server gets messages from clients and sends them to all connected clients.
3. Client Communication: The client lets the user send and receive messages.
4. Concurrency: The server uses threads to handle multiple clients at once without issues.

---

CHECKPOINTS: 

Checkpoint 1: 
- Implement a basic TCP server that can accept a client connection.
- Implement the client that can connect to the server and send messages.
- Test basic communication between the server and client on the same machine.

Checkpoint 2:
- Implement message broadcasting to all clients from the server.
- Add real-time message display on the client side.
- Test with multiple clients on the same machine to verify concurrent message handling.

---

Possible Challenges: 
Issues I forsee running into include: 
- Understanding and implementing TCP communication in Rust, including managing connections on a single machine.
- Properly handling multiple clients concurrently and ensuring thread safety.
- Making sure to handle potential errors such as failed connections or message delivery issues.
- Creating a simple but functional interface for the clients to send and receive messages, making sure it is intuitive and reliable.

---

REFERENCES: (While working on the project): 
https://www.youtube.com/watch?v=JiuouCJQzSQ
https://www.youtube.com/watch?v=F27PLin3TV0


