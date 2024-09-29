# Summary
A sample client-server application for TLS authentication and secure messaging, built with OpenSSL and Rust.

# Project structure
- certs: Contains certificates and RSA keys used by the app to provide TLS authentication and message encryption, generated using OpenSSL CLI.
- app: Contains the app's source-code.

# Running
Start the server:
`cargo run --bin server`

Provide the private-key passphrase (obviously I'll not provide my own, so you'll need to generate your own private-keys and certificates)

Start the client:
`cargo run --bin client`

The client will send a "Hello" message to the server, then the server will ask for a message to send to the client; after that, the communication will continue indefinitely.

# Reference
The following material was used for guidance:

[Cyber Hashira's Youtube playlist about OpenSSL CLI](https://youtube.com/playlist?list=PLgBMtP0_D_afzNG7Zs2jr8FSoyeU4yqhi&si=dJRkcRinVw_nAwDq)

[OpenSSL Rust lib documentation (especially the sample client-server TLS auth code)](https://docs.rs/openssl/0.10.66/openssl/ssl/index.html)
