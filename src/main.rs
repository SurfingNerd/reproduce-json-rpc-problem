use std::{error::Error, net::SocketAddr, thread::sleep, time::Duration};

use mio::net::{TcpListener};


fn main() {
    
    
    // Setup the server socket.
    let addr =  SocketAddr::from(([127, 0, 0, 1], 30301));

    match TcpListener::bind(&addr) {
        Ok(tcp_listener) => {
            //let x = mio::sys::TcpListener::new(listener);
            println!("Listening on: {}", addr);
            //let x  = SocketAddr::new(addr.ip(), addr.port());
            sleep(Duration::from_secs(100));
        }
        Err(e) => {
            println!("Failed to bind server socket: {}", e);
        }
    }

    
    
}