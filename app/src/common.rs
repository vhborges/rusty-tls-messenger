use openssl::ssl::SslStream;
use std::io::{stdin, stdout, Read, Write};
use std::net::TcpStream;

pub const IP: &str = "localhost";
pub const HOST: &str = "localhost:8443";

pub fn handle_client_server_communication(
    stream: &mut SslStream<TcpStream>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input_buffer = vec![0; 1024];
    while stream.read(&mut input_buffer)? > 0 {
        println!("Received: {}", String::from_utf8_lossy(&input_buffer));
        print!("What should I send back? ");
        stdout().flush()?;

        let mut output_buffer = String::new();
        stdin().read_line(&mut output_buffer)?;
        let output_buffer = output_buffer
            .strip_suffix("\r\n")
            .or(output_buffer.strip_suffix("\n"))
            .unwrap_or(output_buffer.as_str());
        stream.write_all(output_buffer.as_bytes())?;

        input_buffer = vec![0; 1024];
    }

    Ok(())
}
