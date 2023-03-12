use tokio::sync::Mutex;
use std::error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
use rand_core::OsRng;
use tokio::net::TcpStream;

pub async fn get_secret(stream: Arc<Mutex<TcpStream>>) -> Result<[u8;32], Box<dyn error::Error>>{
    let secret = tokio::spawn(async move{        
        let client_secret = EphemeralSecret::random(&mut OsRng);

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
    
    let mut sec = [0;32];
    secret.await?.extract::<sha2::Sha256>(None)
        .expand(&[], &mut sec)
        .expect("Invalid length!");
    Ok(sec)
}
