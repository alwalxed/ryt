use anyhow::Result;
use console::style;
use dialoguer::{Confirm, Input, Select};

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    Single,
    Playlist,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Video,
    Audio,
}

#[derive(Debug, Clone)]
pub enum Quality {
    P480,
    P720,
    P1080,
    P1440,
    P2160,
    Best,
}

#[derive(Debug)]
pub enum MainAction {
    Download,
    Config,
    History,
    Exit,
}

pub struct UserInterface;

impl UserInterface {
    pub fn new() -> Self {
        Self
    }

    pub fn welcome(&self) {
        println!(
            "{}",
            style("🎥 Welcome to ryt - Your Media Downloader")
                .cyan()
                .bold()
        );
        println!("{}", style("Built on yt-dlp for reliable downloads").dim());
        println!();
    }

    pub fn goodbye(&self) {
        println!();
        println!("{}", style("Thanks for using ryt! 👋").green());
    }

    pub fn get_main_action(&self) -> Result<MainAction> {
        let options = vec![
            "📥 Download media",
            "⚙️  Configure settings",
            "📜 View history",
            "🚪 Exit",
        ];

        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()?;

        Ok(match selection {
            0 => MainAction::Download,
            1 => MainAction::Config,
            2 => MainAction::History,
            3 => MainAction::Exit,
            _ => MainAction::Exit,
        })
    }

    pub fn get_url(&self) -> Result<String> {
        let url: String = Input::new()
            .with_prompt("Enter the URL to download")
            .interact_text()?;
        Ok(url.trim().to_string())
    }

    pub fn get_content_type(&self) -> Result<ContentType> {
        let options = vec!["Single video/audio", "Entire playlist"];

        let selection = Select::new()
            .with_prompt("What would you like to download?")
            .items(&options)
            .default(0)
            .interact()?;

        Ok(match selection {
            0 => ContentType::Single,
            1 => ContentType::Playlist,
            _ => ContentType::Single,
        })
    }

    pub fn get_format(&self) -> Result<Format> {
        let options = vec!["🎬 Video (with audio)", "🎵 Audio only"];

        let selection = Select::new()
            .with_prompt("Choose format")
            .items(&options)
            .default(0)
            .interact()?;

        Ok(match selection {
            0 => Format::Video,
            1 => Format::Audio,
            _ => Format::Video,
        })
    }

    pub fn get_quality(&self) -> Result<Quality> {
        let options = vec![
            "480p",
            "720p",
            "1080p",
            "1440p (2K)",
            "2160p (4K)",
            "Best available",
        ];

        let selection = Select::new()
            .with_prompt("Select video quality")
            .items(&options)
            .default(2)
            .interact()?;

        Ok(match selection {
            0 => Quality::P480,
            1 => Quality::P720,
            2 => Quality::P1080,
            3 => Quality::P1440,
            4 => Quality::P2160,
            5 => Quality::Best,
            _ => Quality::P1080,
        })
    }

    pub fn continue_prompt(&self) -> Result<bool> {
        println!();
        Ok(Confirm::new()
            .with_prompt("Would you like to download something else?")
            .default(true)
            .interact()?)
    }

    pub fn info(&self, message: &str) {
        println!("{} {}", style("ℹ").blue(), message);
    }

    pub fn error(&self, message: &str) {
        println!("{} {}", style("✗").red(), message);
    }
}

impl Quality {
    pub fn to_height(&self) -> &str {
        match self {
            Quality::P480 => "480",
            Quality::P720 => "720",
            Quality::P1080 => "1080",
            Quality::P1440 => "1440",
            Quality::P2160 => "2160",
            Quality::Best => "best",
        }
    }
}
