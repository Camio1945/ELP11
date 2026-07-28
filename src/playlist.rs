//! Playlist management utilities.

use std::path::Path;

/// Supported video file extensions (lowercase).
const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg", "m2ts", "ts", "mts",
    "vob", "ogv", "3gp", "3g2", "f4v", "rm", "rmvb", "asf", "divx", "dv",
];

/// Check if a file has a supported video extension.
pub fn is_video_file(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| VIDEO_EXTENSIONS.contains(&e.to_lowercase().as_str()))
}

/// Scan a directory for video files (non-recursive, sorted by name).
/// Returns a vector of file paths as strings.
pub fn scan_directory_for_videos(dir: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut videos: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            if path.is_file() && is_video_file(&path) {
                Some(path.display().to_string())
            } else {
                None
            }
        })
        .collect();
    videos.sort();
    videos
}

/// Get the directory containing a file.
pub fn parent_directory(file_path: &str) -> Option<std::path::PathBuf> {
    std::path::Path::new(file_path)
        .parent()
        .map(|p| p.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_video_file() {
        assert!(is_video_file(Path::new("video.mp4")));
        assert!(is_video_file(Path::new("video.MKV")));
        assert!(is_video_file(Path::new("video.WebM")));
        assert!(!is_video_file(Path::new("video.txt")));
        assert!(!is_video_file(Path::new("video.jpg")));
    }

    #[test]
    fn test_is_video_file_all_extensions() {
        for ext in VIDEO_EXTENSIONS {
            let path = format!("video.{ext}");
            assert!(
                is_video_file(Path::new(&path)),
                "expected {ext} to be a video extension"
            );
        }
    }

    #[test]
    fn test_is_video_file_no_extension() {
        assert!(!is_video_file(Path::new("video")));
    }

    #[test]
    fn test_is_video_file_uppercase() {
        assert!(is_video_file(Path::new("video.MP4")));
        assert!(is_video_file(Path::new("video.Mkv")));
    }

    #[test]
    fn test_parent_directory_absolute() {
        #[cfg(target_os = "windows")]
        {
            let result = parent_directory("C:\\Users\\test\\video.mp4");
            assert_eq!(result, Some(std::path::PathBuf::from("C:\\Users\\test")));
        }
        #[cfg(not(target_os = "windows"))]
        {
            let result = parent_directory("/home/user/video.mp4");
            assert_eq!(result, Some(std::path::PathBuf::from("/home/user")));
        }
    }

    #[test]
    fn test_parent_directory_relative() {
        let result = parent_directory("videos/video.mp4");
        assert_eq!(result, Some(std::path::PathBuf::from("videos")));
    }

    #[test]
    fn test_parent_directory_root() {
        #[cfg(not(target_os = "windows"))]
        {
            let result = parent_directory("/video.mp4");
            assert_eq!(result, Some(std::path::PathBuf::from("/")));
        }
    }

    #[test]
    fn test_scan_directory_for_videos_empty() {
        // Scanning a non-existent directory returns empty
        let result = scan_directory_for_videos(Path::new("/nonexistent/dir/12345"));
        assert!(result.is_empty());
    }
}
