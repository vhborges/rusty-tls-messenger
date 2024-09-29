use app::common::{self, HOST, IP};
use app::config::Files;
use openssl::ssl::{SslConnector, SslMethod};
use std::io::Write;
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = Files::new();

    let mut connector = SslConnector::builder(SslMethod::tls())?;
    connector.set_ca_file(files.root.cert)?;
    let connector = connector.build();

    let stream = TcpStream::connect(HOST)?;
    let mut stream = connector.connect(IP, stream)?;

    stream.write_all(b"Hello")?;
    common::handle_client_server_communication(&mut stream)?;

    Ok(())
}
