use app::common::{self, HOST};
use app::config::Files;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = Files::new();

    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    acceptor.set_private_key_file(files.server.key, SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file(files.server.chain)?;
    acceptor.check_private_key()?;
    let acceptor = Arc::new(acceptor.build());

    let listener = TcpListener::bind(HOST)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || {
                    let mut stream = acceptor.accept(stream)
                        .expect("Error accepting connection");
                    common::handle_client_server_communication(&mut stream)
                        .expect("Error handling client communication");
                });
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    Ok(())
}
