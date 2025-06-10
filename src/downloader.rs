use crate::config::Config;
use crate::error::RytError;
use crate::ui::{ContentType, Format, Quality};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub struct Downloader {
    pub config: Config,
    ytdlp_cmd: String,
}

impl Downloader {
    pub fn new(config: Config) -> Result<Self> {
        config.ensure_download_dirs()?;

        let ytdlp_cmd = config
            .ytdlp_path
            .clone()
            .unwrap_or_else(|| "yt-dlp".to_string());

        Ok(Self { config, ytdlp_cmd })
    }

    pub async fn check_ytdlp(&self) -> Result<()> {
        let output = Command::new(&self.ytdlp_cmd)
            .arg("--version")
            .output()
            .await
            .map_err(|_| RytError::YtDlpNotFound)?;

        if !output.status.success() {
            return Err(RytError::YtDlpNotFound.into());
        }

        Ok(())
    }

    pub async fn download(
        &self,
        url: &str,
        content_type: ContentType,
        format: Format,
        quality: Option<Quality>,
    ) -> Result<()> {
        println!("🚀 Starting download...");

        let mut cmd = Command::new(&self.ytdlp_cmd);

        // Basic arguments
        cmd.arg(url);

        // Set output directory and template
        let output_dir = match content_type {
            ContentType::Single => self.config.download_dir.join("single-videos"),
            ContentType::Playlist => self.config.download_dir.join("playlists"),
        };

        let output_template = match content_type {
            ContentType::Single => "%(title)s.%(ext)s".to_string(),
            ContentType::Playlist => "%(playlist_title)s/%(title)s.%(ext)s".to_string(),
        };

        cmd.arg("-o").arg(output_dir.join(&output_template));

        // Format selection
        match format {
            Format::Audio => {
                cmd.arg("--extract-audio")
                    .arg("--audio-format")
                    .arg("best")
                    .arg("--audio-quality")
                    .arg("0");
            }
            Format::Video => {
                if let Some(quality) = quality {
                    match quality {
                        Quality::Best => {
                            cmd.arg("-f").arg("best");
                        }
                        _ => {
                            let height = quality.to_height();
                            let format_spec = format!(
                                "bestvideo[height<={}]+bestaudio/best[height<={}]",
                                height, height
                            );
                            cmd.arg("-f").arg(format_spec);
                        }
                    }
                }
            }
        }

        // Progress and output handling
        cmd.arg("--newline")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn()?;

        // Create progress bar
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {percent}% {msg}",
                )
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  "),
        );

        // Handle stdout for progress updates
        if let Some(stdout) = child.stdout.take() {
            let mut reader = BufReader::new(stdout).lines();

            while let Some(line) = reader.next_line().await? {
                if let Some(progress) = self.parse_progress(&line) {
                    pb.set_position(progress as u64);
                    pb.set_message(self.extract_status(&line));
                }
            }
        }

        let status = child.wait().await?;
        pb.finish_with_message("Download completed!");

        if status.success() {
            println!("✅ Download completed successfully!");
        } else {
            println!("❌ Download failed!");
            return Err(RytError::DownloadFailed.into());
        }

        Ok(())
    }

    fn parse_progress(&self, line: &str) -> Option<f32> {
        let re = Regex::new(r"\[download\]\s+(\d+(?:\.\d+)?)%").ok()?;
        if let Some(captures) = re.captures(line) {
            captures.get(1)?.as_str().parse().ok()
        } else {
            None
        }
    }

    fn extract_status(&self, line: &str) -> String {
        if line.contains("[download]") {
            if let Some(start) = line.find("] ") {
                return line[start + 2..].to_string();
            }
        }
        line.to_string()
    }
}
