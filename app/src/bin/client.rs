use app::common;
use app::config::Files;
use openssl::ssl::{SslConnector, SslMethod};
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let files = Files::new();

    let mut connector = SslConnector::builder(SslMethod::tls()).unwrap();
    connector.set_ca_file(files.root.cert).unwrap();
    let connector = connector.build();

    let stream = TcpStream::connect("localhost:8443").unwrap();
    let mut stream = connector.connect("localhost", stream).unwrap();

    stream.write_all(b"Hello").unwrap();
    common::handle_client_server_communication(&mut stream);
}

