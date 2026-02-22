use tokio::fs;
use std::path::Path;

pub async fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(Path::new(path)).await
}

pub async fn write_file(path: &str, data: &str) -> Result<(), std::io::Error> {
    fs::write(Path::new(path), data).await
}
