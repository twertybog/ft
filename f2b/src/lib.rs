use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncSeekExt;
use tokio::io::AsyncWriteExt;
use tokio::io::{BufWriter, SeekFrom};

//converting file into vec of bytes
pub async fn f2b_all(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    let file = (fs::read(path)).await?;
    Ok(file)
}

//the same as f2b_all() but it can convert a part of a file
pub async fn f2b_part(
    path: &str,
    start: u64,
    bytes: u64,
) -> Result<Vec<u8>, Box<dyn std::error::Error + 'static>> {
    let file = fs::File::open(path).await?;

    let reader = &mut tokio::io::BufReader::new(file);

    reader.seek(SeekFrom::Start(start)).await?;

    let mut take = reader.take(bytes);

    let mut buf: Vec<u8> = Vec::new();
    take.read_to_end(&mut buf).await?;

    Ok(buf)
}

//write bytes in file
pub async fn b2f(bytes: &Vec<u8>, file: File) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = BufWriter::new(file);

    buffer.write(&bytes).await?;

    buffer.flush().await?;

    Ok(())
}