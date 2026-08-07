/// Inline SVG icons drawn on a uniform 24×24 viewBox.
/// Every shape is centered at viewBox center (12, 12) so that when each
/// SVG is rendered at the same size, the icon centers line up exactly.

pub const SKIP_BACK_30: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M5 12L11 4V20Z M13 12L19 4V20Z" fill="currentColor"/>
</svg>"#;

pub const SKIP_BACK_5: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M5 12L15 4V20Z" fill="currentColor"/>
</svg>"#;

pub const PLAY: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M8 4V20L21 12Z" fill="currentColor"/>
</svg>"#;

pub const PAUSE: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M7 4H11V20H7Z M13 4H17V20H13Z" fill="currentColor"/>
</svg>"#;

pub const SKIP_FORWARD_5: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M19 12L9 4V20Z" fill="currentColor"/>
</svg>"#;

pub const SKIP_FORWARD_30: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M11 12L5 4V20Z M19 12L13 4V20Z" fill="currentColor"/>
</svg>"#;

pub const FRAME_STEP: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M3 4H7V20H3Z M11 4V20L21 12Z" fill="currentColor"/>
</svg>"#;

/// Utility-cluster icons (loop / volume / fullscreen).
///
/// These replace what used to be Unicode emoji glyphs ("🔁", "🔊",
/// "⛶"). The problem with emoji is that on Windows the OS emoji font
/// renders them at much larger than `.size(14)` and with vertical
/// metrics that don't sit on the text baseline, so `Container::align_y`
/// can't actually center them — they always float toward the top of
/// their parent button. SVG renders at the exact `ICON_SIZE` we hand
/// it and lines up with the transport buttons above.

pub const LOOP: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v4z" fill="currentColor"/>
</svg>"#;

/// Speaker with sound waves (unmuted / volume_up).
pub const VOLUME: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 7.97v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06A9 9 0 0 1 21 12a9 9 0 0 1-7 8.71v2.06A11 11 0 0 0 21 12 11 11 0 0 0 14 3.23z" fill="currentColor"/>
</svg>"#;

/// Speaker with a slash through it (muted / volume_off).
pub const MUTE: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M16.5 12A4.5 4.5 0 0 0 14 7.97v2.21l2.45 2.45c.03-.2.05-.41.05-.63zM19 12c0 .94-.2 1.82-.54 2.64l1.51 1.51A9 9 0 0 0 21 12c0-4.28-2.99-7.86-7-8.77v2.06A7 7 0 0 1 19 12zM4.27 3 3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06a11 11 0 0 0 3.69-1.81L19.73 21 21 19.73l-9-9L4.27 3zM12 4 9.91 6.09 12 8.18V4z" fill="currentColor"/>
</svg>"#;

/// Four corner brackets — fullscreen.
pub const FULLSCREEN: &[u8] = br#"<svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
  <path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z" fill="currentColor"/>
</svg>"#;

/// Build an Iced SVG handle from inline data.
pub fn svg_handle(data: &[u8]) -> iced::widget::svg::Handle {
    iced::widget::svg::Handle::from_memory(Vec::from(data))
}
