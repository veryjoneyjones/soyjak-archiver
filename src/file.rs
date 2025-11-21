use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use std::{io, path::PathBuf};
use thiserror::Error;
use tokio::{fs, io::AsyncWriteExt};
use url::Url;

#[derive(Debug)]
pub struct File {
    pub url: String,
}

#[derive(Debug, Error)]
pub enum FileError {
    #[error("A network error occurred: {0}")]
    NetworkError(reqwest::Error),
    #[error("An IO error occurred: {0}")]
    IOError(io::Error),
}

impl File {
    pub fn new(url: impl Into<String>) -> Self {
        let url = url.into();

        if url.starts_with("https://soyjak.st/player.php?v=") {
            Self {
                url: {
                    let mut p = url
                        .split_once("?v=")
                        .map(|(_, s)| s)
                        .unwrap_or_default()
                        .to_string();
                    if let Some(idx) = p.find('&') {
                        p.truncate(idx);
                    }
                    format!("https://soyjak.st{}", p)
                },
            }
        } else {
            Self { url }
        }
    }

    pub async fn download_to_disk(&self, path: &PathBuf) -> Result<(), FileError> {
        let _ = fs::create_dir(&path).await;

        let response = reqwest::get(&self.url)
            .await
            .map_err(FileError::NetworkError)?;

        let total_size = response.content_length().unwrap();

        let file_name = Self::url_to_filename(&self.url);

        let mut full_path = path.clone();
        full_path.push(file_name);

        let mut file = tokio::fs::File::create(&full_path)
            .await
            .map_err(FileError::IOError)?;

        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
        .progress_chars("#>-"));
        pb.set_message(format!("Downloading {}", self.url));

        let mut stream = response.bytes_stream();

        let mut downloaded: u64 = 0;

        while let Some(item) = stream.next().await {
            // Map any reqwest::Error from the stream into our FileError::NetworkError
            let chunk = item.map_err(FileError::NetworkError)?;

            // Await the write and propagate any IO errors as FileError::IOError
            file.write_all(&chunk).await.map_err(FileError::IOError)?;

            let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
            downloaded = new;
            pb.set_position(new);
        }

        let path_lossy = full_path.to_string_lossy();

        let message = format!("Downloaded {} to {}", self.url, path_lossy);
        pb.finish_with_message(message);

        Ok(())
    }

    fn url_to_filename(url: impl Into<String>) -> String {
        let url = url.into();
        let url = Url::parse(&url).unwrap();
        url.path_segments()
            .and_then(|mut segments| segments.next_back())
            .unwrap_or("")
            .to_string()
    }
}
