use anyhow::Result;
use clap::{Parser, Subcommand};

mod config;
mod downloader;
mod error;
mod ui;
mod utils;

use config::Config;
use downloader::Downloader;
use error::RytError;
use ui::UserInterface;

#[derive(Parser)]
#[command(name = "ryt")]
#[command(about = "A user-friendly media downloader built on yt-dlp")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// URL to download
    #[arg(short, long)]
    url: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Download media from URL
    Download {
        /// URL to download
        url: Option<String>,
    },
    /// Manage configuration
    Config,
    /// View download history
    History,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load_or_create()?;
    let ui = UserInterface::new();
    let downloader = Downloader::new(config.clone())?;

    // Check if yt-dlp is available
    if let Err(e) = downloader.check_ytdlp().await {
        ui.error(&format!("yt-dlp check failed: {}", e));
        ui.info("Please install yt-dlp: https://github.com/yt-dlp/yt-dlp#installation");
        return Ok(());
    }

    match cli.command {
        Some(Commands::Download { url }) => {
            handle_download(url.or(cli.url), &ui, &downloader).await?;
        }
        Some(Commands::Config) => {
            handle_config(&ui, &config).await?;
        }
        Some(Commands::History) => {
            handle_history(&ui, &config).await?;
        }
        None => {
            if let Some(url) = cli.url {
                handle_download(Some(url), &ui, &downloader).await?;
            } else {
                handle_interactive(&ui, &downloader).await?;
            }
        }
    }

    Ok(())
}

async fn handle_download(
    url: Option<String>,
    ui: &UserInterface,
    downloader: &Downloader,
) -> Result<()> {
    let url = match url {
        Some(u) => u,
        None => ui.get_url()?,
    };

    if !utils::is_valid_url(&url) {
        ui.error("Invalid URL format or unsupported platform");
        ui.info("Supported platforms: YouTube, Vimeo, SoundCloud, Twitch, TikTok, and more");
        return Err(RytError::InvalidUrl.into());
    }

    let content_type = ui.get_content_type()?;
    let format = ui.get_format()?;
    let quality = if format == ui::Format::Video {
        Some(ui.get_quality()?)
    } else {
        None
    };

    downloader
        .download(&url, content_type, format, quality)
        .await?;
    Ok(())
}

async fn handle_config(ui: &UserInterface, _config: &Config) -> Result<()> {
    ui.info("Configuration management coming soon!");
    ui.info("Current features planned:");
    ui.info("  • Set default download directory");
    ui.info("  • Configure default quality settings");
    ui.info("  • Set custom yt-dlp path");
    ui.info("  • Manage concurrent downloads");
    Ok(())
}

async fn handle_history(ui: &UserInterface, _config: &Config) -> Result<()> {
    ui.info("Download history coming soon!");
    ui.info("Planned features:");
    ui.info("  • View past downloads");
    ui.info("  • Re-download previous URLs");
    ui.info("  • Export download history");
    Ok(())
}

async fn handle_interactive(ui: &UserInterface, downloader: &Downloader) -> Result<()> {
    ui.welcome();

    loop {
        match ui.get_main_action()? {
            ui::MainAction::Download => {
                let url = ui.get_url()?;
                if let Err(e) = handle_download(Some(url), ui, downloader).await {
                    ui.error(&format!("Download failed: {}", e));
                    continue;
                }
            }
            ui::MainAction::Config => {
                handle_config(ui, &downloader.config).await?;
            }
            ui::MainAction::History => {
                handle_history(ui, &downloader.config).await?;
            }
            ui::MainAction::Exit => break,
        }

        if !ui.continue_prompt()? {
            break;
        }
    }

    ui.goodbye();
    Ok(())
}
