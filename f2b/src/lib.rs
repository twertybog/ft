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

#[cfg(test)]

mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn read() {
        tokio::spawn(async move {
            assert_eq!(
                vec![56, 53, 52, 49, 53, 51, 53, 52, 53, 50, 51, 52, 49, 54, 53, 54],
                f2b_all("pass.txt").await.unwrap()
            );
            assert_eq!(
                vec![56, 53, 52, 49, 53, 51, 53, 52, 53, 50, 51, 52, 49, 54, 53, 54],
                f2b_part("pass.txt", 0, 16).await.unwrap()
            );
        });
    }
    #[test]
    fn over_read() {
        tokio::spawn(async move {
            assert_eq!(
                vec![56, 53, 52, 49, 53, 51, 53, 52, 53, 50, 51, 52, 49, 54, 53, 54],
                f2b_part("pass.txt", 0, 28).await.unwrap()
            );
        });
    }
    #[test]
    fn un_read() {
        tokio::spawn(async move {
            let vector: Vec<u8> = Vec::new();
            assert_eq!(vector, f2b_part("pass.txt", 19, 15).await.unwrap());
        });
    }
    #[test]
    fn mid_read() {
        tokio::spawn(async move {
            assert_eq!(
                vec![52, 49, 53, 51],
                f2b_part("pass.txt", 2, 4).await.unwrap()
            );
        });
    }
    #[test]
    fn write_to_file() {
        tokio::spawn(async move {
            let info = f2b_part("pass.txt", 0, 10).await.unwrap();
            let file = fs::File::create("info.txt").await.unwrap();
            b2f(&info, file.try_clone().await.unwrap()).await.unwrap();
            b2f(&info, file).await.unwrap();
            assert_eq!(
                vec![
                    56, 53, 52, 49, 53, 51, 53, 52, 53, 50, 56, 53, 52, 49, 53, 51, 53, 52, 53, 50
                ],
                f2b_all("info.txt").await.unwrap()
            );
            fs::remove_file("info.txt").await.unwrap();
        });
    }
}
