use rand_core::OsRng;
use k256::{EncodedPoint, PublicKey, ecdh::EphemeralSecret};
use std::io as sio;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod commands;
use commands::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = TcpStream::connect("127.0.0.1:4956").await?;

    let (mut rd, mut wr) = tokio::io::split(socket);

    let get_secret = tokio::spawn(async move{
        let client_secret = EphemeralSecret::random(OsRng);

        let client_public = EncodedPoint::from(client_secret.public_key());

        //share pub key
        wr.write_all(client_public.as_ref()).await
            .expect("Public key not sent!");

        //get server pub key
        let mut server_public = Vec::new();

        rd.read_buf(&mut server_public).await
            .expect("Server public key not recieved!");

        let server_public = PublicKey::from_sec1_bytes(server_public.as_ref())
            .expect("Server pubblic key is invalid!");

        client_secret.diffie_hellman(&server_public)
    });

    println!("{:?}\n", get_secret.await?.raw_secret_bytes());

    Ok(())
}
