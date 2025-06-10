# ryt (yt-dlp wrapper)

A simple, interactive terminal media downloader powered by yt-dlp.

## Features

- Interactive Mode: Step-by-step prompts to guide your download.
- Flexible: Download video, audio-only, or entire playlists.
- Quality Control: Choose video quality from 480p to 4K.
- User-Friendly: Clean UI with progress bars and auto-generated config file.

## Prerequisites

Requires `yt-dlp` to be installed and in your system's PATH.

## Installation

```bash
git clone https://github.com/alwalxed/ryt.git && cd ryt && cargo install --path .
```

## Usage

```bash
ryt
```

## Configuration

A config file is auto-generated on first run.

- Linux/macOS: `~/.config/ryt/config.toml`
- Windows: `%APPDATA%\ryt\config.toml`
