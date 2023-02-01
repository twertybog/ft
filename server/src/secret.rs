use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use k256::ecdh::SharedSecret;
use k256::Secp256k1;
use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
use rand_core::OsRng;
use tokio::net::TcpStream;

pub async fn get_secret(stream: Arc<Mutex<TcpStream>>) -> Vec<u8>{
    let secret = tokio::spawn(async move {
        //create server keys
        let server_secret = EphemeralSecret::random(OsRng);

        let server_public = EncodedPoint::from(server_secret.public_key());

        //get client pub key
        let mut client_public = [0;33];

        stream.lock().await
            .read(&mut client_public).await
            .expect("Client public key not recieved!");

        let client_public = PublicKey::from_sec1_bytes(&client_public[..])
            .expect("Client public key is invalid");

        //share pub key
        stream.lock().await
            .write_all(server_public.as_bytes()).await
            .expect("Server public key not sent!");

        server_secret
            .diffie_hellman(&client_public)
    });

    secret.await.expect("Secret not generated")
    .raw_secret_bytes()
    .to_vec()
}
