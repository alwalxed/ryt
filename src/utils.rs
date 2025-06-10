use url::Url;

pub fn is_valid_url(input: &str) -> bool {
    if let Ok(parsed_url) = Url::parse(input) {
        if let Some(host) = parsed_url.host_str() {
            return is_supported_domain(host);
        }
    }
    false
}

fn is_supported_domain(host: &str) -> bool {
    let supported_domains = [
        "youtube.com",
        "www.youtube.com",
        "youtu.be",
        "m.youtube.com",
        "vimeo.com",
        "www.vimeo.com",
        "dailymotion.com",
        "www.dailymotion.com",
        "twitch.tv",
        "www.twitch.tv",
        "soundcloud.com",
        "www.soundcloud.com",
        "tiktok.com",
        "www.tiktok.com",
    ];

    supported_domains
        .iter()
        .any(|&domain| host == domain || host.ends_with(&format!(".{}", domain)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_urls() {
        assert!(is_valid_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
        assert!(is_valid_url("https://youtu.be/dQw4w9WgXcQ"));
        assert!(is_valid_url("https://vimeo.com/123456"));
        assert!(is_valid_url(
            "https://music.youtube.com/watch?v=dQw4w9WgXcQ"
        ));
        assert!(!is_valid_url("not a url"));
        assert!(!is_valid_url(""));
        assert!(!is_valid_url("https://unsupported-site.com/video"));
        assert!(!is_valid_url("https://fake-youtube.com.evil.com/video"));
    }

    #[test]
    fn test_domain_validation() {
        assert!(super::is_supported_domain("youtube.com"));
        assert!(super::is_supported_domain("www.youtube.com"));
        assert!(super::is_supported_domain("youtu.be"));

        assert!(super::is_supported_domain("music.youtube.com"));
        assert!(super::is_supported_domain("gaming.youtube.com"));

        assert!(!super::is_supported_domain("fake-youtube.com"));
        assert!(!super::is_supported_domain("youtube.com.evil.com"));
        assert!(!super::is_supported_domain("notyoutube.com"));
    }
}
