use std::net::{TcpStream, Shutdown, TcpListener};
use std::io::Read;

pub trait TcpServer {
    #[tokio::main]
    async fn start(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    self.handle_client(stream)
                }
                Err(e) => {
                    println!("Error: {}", e);
                    /* connection failed */
                }
            }
        }
        // close the socket server
        drop(listener);
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        let mut data = vec![0; 4096]; // using 50 byte buffer
        while match stream.read(&mut data) {
            Ok(size) => {
                // echo everything!
                // stream.write(&data[0..size]).unwrap();
                self.on_receive(&data[0..size].to_vec());
                true
            },
            Err(_) => {
                println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {}
    }

    fn on_receive(&mut self, mut buf: &Vec<u8>)  {
        println!("STD Packet received");
        println!("{:#01x?}", buf)
    }
}
