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
    let secret = tokio::spawn(async move{        
        let client_secret = EphemeralSecret::random(OsRng);

        let client_public = EncodedPoint::from(client_secret.public_key());

        //share pub key
        stream.lock().await
            .write_all(client_public.as_ref()).await
            .expect("Public key not sent!");

        //get server pub key
        let mut server_public = [0;33];

        stream.lock().await
            .read(&mut server_public).await
            .expect("Server public key not recieved!");

        let server_public = PublicKey::from_sec1_bytes(server_public.as_ref())
            .expect("Server pubblic key is invalid!");

        client_secret.diffie_hellman(&server_public)
    });
    secret.await
        .expect("Secret not generated")
        .raw_secret_bytes()
        .to_vec()
}
