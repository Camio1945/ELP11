use crate::app_state::{App, Message, VideoState};
use iced::Task;
use iced::window;
use iced_video_player::Video;

pub fn load_window_icon() -> Option<window::Icon> {
    let img = image::load_from_memory_with_format(
        include_bytes!("../assets/icon.png"),
        image::ImageFormat::Png,
    )
    .ok()?
    .to_rgba8();
    let (w, h) = img.dimensions();
    window::icon::from_rgba(img.into_raw(), w, h).ok()
}

pub fn parse_cli_args(args: &[String]) -> (Option<String>, Option<String>) {
    match args.len() {
        0 => (None, None),
        1 => (Some(args[0].clone()), None),
        _ => (Some(args[0].clone()), Some(args[1].clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cli_args_empty() {
        let args: &[String] = &[];
        let (video, subtitle) = parse_cli_args(args);
        assert_eq!(video, None);
        assert_eq!(subtitle, None);
    }

    #[test]
    fn test_parse_cli_args_video_only() {
        let args = &["video.mp4".to_string()];
        let (video, subtitle) = parse_cli_args(args);
        assert_eq!(video, Some("video.mp4".to_string()));
        assert_eq!(subtitle, None);
    }

    #[test]
    fn test_parse_cli_args_video_and_subtitle() {
        let args = &["video.mp4".to_string(), "subtitle.srt".to_string()];
        let (video, subtitle) = parse_cli_args(args);
        assert_eq!(video, Some("video.mp4".to_string()));
        assert_eq!(subtitle, Some("subtitle.srt".to_string()));
    }

    #[test]
    fn test_parse_cli_args_extra_args() {
        let args = &[
            "video.mp4".to_string(),
            "subtitle.srt".to_string(),
            "extra".to_string(),
        ];
        let (video, subtitle) = parse_cli_args(args);
        assert_eq!(video, Some("video.mp4".to_string()));
        assert_eq!(subtitle, Some("subtitle.srt".to_string())); // only first two used
    }
}

/// Build a `file://` URL from a CLI argument string, emitting a warning
/// and falling back to `file:///` when the path is not a valid URL.
fn url_from_cli_path(path: &str) -> url::Url {
    url::Url::from_file_path(path).unwrap_or_else(|_| {
        url::Url::parse(&format!("file:///{}", path)).unwrap_or_else(|e| {
            eprintln!(
                "failed to construct file URL from CLI argument '{}': {}",
                path, e
            );
            url::Url::parse("file:///").unwrap()
        })
    })
}

pub fn create_boot_closure(
    video_arg: Option<String>,
    subtitle_arg: Option<String>,
) -> impl Fn() -> (App, Task<Message>) {
    move || {
        let mut app = App::default();
        let mut initial_task = Task::none();

        if let Some(ref path) = video_arg {
            let path_str = std::path::Path::new(path).display().to_string();
            app.video = VideoState::Loading(path_str.clone());
            app.current_file_path = Some(path_str);

            if let Some(ref sp) = subtitle_arg {
                let sub_path = std::path::Path::new(sp).to_path_buf();
                app.pending_subtitle = Some(sub_path);
            }

            let url = url_from_cli_path(path);
            let path_owned = path.clone();
            initial_task = Task::perform(
                async move {
                    let path_buf = std::path::PathBuf::from(&path_owned);
                    match Video::new(&url) {
                        Ok(_) => Ok(path_buf.display().to_string()),
                        Err(e) => Err(format!("Failed to open: {}", e)),
                    }
                },
                Message::FileOpened,
            );
        }
        (app, initial_task)
    }
}
