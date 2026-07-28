//! # Iced Video Player
//!
//! A convenient video player widget for Iced.
//!
//! To get started, load a video from a URI (e.g., a file path prefixed with `file:///`) using [`Video::new`](crate::Video::new),
//!     then use it like any other Iced widget in your `view` function by creating a [`VideoPlayer`].
//!
//! Example:
//! ```rust,no_run
//! use iced_video_player::{Video, VideoPlayer};
//!
//! fn main() -> iced::Result {
//!     iced::run(App::update, App::view)
//! }
//!
//! #[derive(Clone, Debug)]
//! enum Message {
//!     NewFrame,
//! }
//!
//! struct App {
//!     video: Video,
//! }
//!
//! impl Default for App {
//!     fn default() -> Self {
//!         App {
//!             video: Video::new(&url::Url::parse("file:///C:/my_video.mp4").unwrap()).unwrap(),
//!         }
//!     }
//! }
//!
//! impl App {
//!     fn update(&mut self, _message: Message) {
//!     }
//!
//!     fn view(&self) -> iced::Element<Message> {
//!         VideoPlayer::new(&self.video)
//!             .on_new_frame(Message::NewFrame)
//!             .into()
//!     }
//! }
//! ```
//!
//! You can programmatically control the video (e.g., seek, pause, loop, grab thumbnails) by accessing various methods on [`Video`].

pub mod pgs;
mod pipeline;
mod pipeline_helpers;
mod primitive;
mod video;
mod video_player;

use gstreamer as gst;
use thiserror::Error;

pub use video::Position;
pub use video::SubtitleStreamInfo;
pub use video::Video;
pub use video_player::VideoPlayer;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Glib(#[from] glib::Error),
    #[error("{0}")]
    Bool(#[from] glib::BoolError),
    #[error("failed to get the gstreamer bus")]
    Bus,
    #[error("failed to get AppSink element with name='{0}' from gstreamer pipeline")]
    AppSink(String),
    #[error("{0}")]
    StateChange(#[from] gst::StateChangeError),
    #[error("failed to cast gstreamer element")]
    Cast,
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("invalid URI: {0}")]
    Uri(String),
    #[error("failed to get media capabilities")]
    Caps,
    #[error("failed to query media duration or position")]
    Duration,
    #[error("failed to sync with playback")]
    Sync,
    #[error("failed to lock internal sync primitive")]
    Lock,
    #[error("invalid framerate: {0}")]
    Framerate(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_bus() {
        assert_eq!(Error::Bus.to_string(), "failed to get the gstreamer bus");
    }

    #[test]
    fn test_error_display_app_sink() {
        let err = Error::AppSink("test_sink".to_string());
        assert!(err.to_string().contains("test_sink"));
    }

    #[test]
    fn test_error_display_cast() {
        assert_eq!(Error::Cast.to_string(), "failed to cast gstreamer element");
    }

    #[test]
    fn test_error_display_uri() {
        let err = Error::Uri("bad://uri".to_string());
        assert!(err.to_string().contains("bad://uri"));
    }

    #[test]
    fn test_error_display_caps() {
        assert_eq!(Error::Caps.to_string(), "failed to get media capabilities");
    }

    #[test]
    fn test_error_display_duration() {
        assert_eq!(
            Error::Duration.to_string(),
            "failed to query media duration or position"
        );
    }

    #[test]
    fn test_error_display_sync() {
        assert_eq!(Error::Sync.to_string(), "failed to sync with playback");
    }

    #[test]
    fn test_error_display_lock() {
        assert_eq!(
            Error::Lock.to_string(),
            "failed to lock internal sync primitive"
        );
    }

    #[test]
    fn test_error_display_framerate() {
        let err = Error::Framerate(0.0);
        assert_eq!(err.to_string(), "invalid framerate: 0");
    }

    #[test]
    fn test_error_display_framerate_negative() {
        let err = Error::Framerate(-1.5);
        assert_eq!(err.to_string(), "invalid framerate: -1.5");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
        assert!(err.to_string().contains("file not found"));
    }

    #[test]
    fn test_error_debug() {
        let err = Error::Bus;
        assert_eq!(format!("{:?}", err), "Bus");
    }
}
