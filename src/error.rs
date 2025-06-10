use thiserror::Error;

#[derive(Error, Debug)]
pub enum RytError {
    #[error("yt-dlp is not installed or not found in PATH")]
    YtDlpNotFound,

    #[error("Download failed")]
    DownloadFailed,

    #[error("Invalid URL format")]
    InvalidUrl,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
