use app::common;
use app::config::Files;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn main() {
    let files = Files::new();

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    acceptor
        .set_private_key_file(files.server.key, SslFiletype::PEM)
        .unwrap();
    acceptor
        .set_certificate_chain_file(files.server.chain)
        .unwrap();
    acceptor.check_private_key().unwrap();
    let acceptor = Arc::new(acceptor.build());

    let listener = TcpListener::bind("localhost:8443").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let mut stream = acceptor.accept(stream).unwrap();
                    common::handle_client_server_communication(&mut stream);
                });
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
