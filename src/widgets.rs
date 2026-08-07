use iced::Length;
use iced::widget::{Button, Container, Svg, Text};

use crate::app_state::Message;
use crate::icons;
use crate::styles;

/// Height of every icon SVG and its enclosing button.
const ICON_SIZE: f32 = 22.0;
/// Button height for control-row buttons (pill shape).
const BTN_HEIGHT: f32 = 32.0;
/// Horizontal padding inside control buttons.
const BTN_HORIZ_PAD: u16 = 8;

/// SVG icon sized to ICON_SIZE.
fn icon_btn(icon_data: &[u8]) -> Svg<'_> {
    Svg::new(icons::svg_handle(icon_data))
        .width(Length::Fixed(ICON_SIZE))
        .height(Length::Fixed(ICON_SIZE))
}

/// Width/height of the square (circular) utility buttons — loop, mute,
/// and fullscreen. Matches BTN_HEIGHT for visual consistency with the
/// pill-shaped transport buttons next to them.
const UTIL_BTN_SIZE: f32 = BTN_HEIGHT;

/// Build a `Container` whose child icon is vertically centered within
/// the parent's content area. Width stays at `Shrink` so the parent
/// button's own width remains the controlling dimension (see note on
/// `centered_icon` callers below) — without this, a previous version of
/// the helper used `Length::Fill` for both axes, which caused each
/// button's content to grab all available row width, turning every
/// circular icon button into a wide pill.
///
/// Why `height` Fill: Iced's `Container` only honors `align_x` /
/// `align_y` when the container is wider/taller than its child.
/// Without `height(Length::Fill)`, the container collapses to the
/// text's line-box height and `align_y(Center)` becomes a no-op,
/// leaving the icon pinned to the top of the button — the original bug
/// the helper exists to fix.
fn centered_icon<'a>(text: Text<'a>) -> Container<'a, Message> {
    Container::new(text)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
}

// ── Transport controls ──────────────────────────────────────────────────

pub(crate) fn skip_back_30_btn() -> Button<'static, Message> {
    Button::new(icon_btn(icons::SKIP_BACK_30))
        .padding([0, BTN_HORIZ_PAD])
        .height(Length::Fixed(BTN_HEIGHT))
        .on_press(Message::SkipBack(30))
        .style(styles::rewind_btn)
}

pub(crate) fn skip_back_5_btn() -> Button<'static, Message> {
    Button::new(icon_btn(icons::SKIP_BACK_5))
        .padding([0, BTN_HORIZ_PAD])
        .height(Length::Fixed(BTN_HEIGHT))
        .on_press(Message::SkipBack(5))
        .style(styles::rewind_btn)
}

/// Circular green play/pause button — the hero control.
pub(crate) fn pause_play_btn(is_paused: bool) -> Button<'static, Message> {
    let icon = if is_paused { icons::PLAY } else { icons::PAUSE };
    let size = BTN_HEIGHT + 4.0;
    Button::new(icon_btn(icon))
        .padding(0)
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
        .on_press(Message::TogglePause)
        .style(styles::main_btn)
}

pub(crate) fn skip_forward_5_btn() -> Button<'static, Message> {
    Button::new(icon_btn(icons::SKIP_FORWARD_5))
        .padding([0, BTN_HORIZ_PAD])
        .height(Length::Fixed(BTN_HEIGHT))
        .on_press(Message::SkipForward(5))
        .style(styles::forward_btn)
}

pub(crate) fn skip_forward_30_btn() -> Button<'static, Message> {
    Button::new(icon_btn(icons::SKIP_FORWARD_30))
        .padding([0, BTN_HORIZ_PAD])
        .height(Length::Fixed(BTN_HEIGHT))
        .on_press(Message::SkipForward(30))
        .style(styles::forward_btn)
}

pub(crate) fn frame_step_btn() -> Button<'static, Message> {
    Button::new(icon_btn(icons::FRAME_STEP))
        .padding([0, BTN_HORIZ_PAD])
        .height(Length::Fixed(BTN_HEIGHT))
        .on_press(Message::FrameStepForward)
        .style(styles::step_btn)
}

// ── Utility controls ────────────────────────────────────────────────────

pub(crate) fn loop_btn<'a>(is_looping: bool) -> Button<'a, Message> {
    Button::new(centered_icon(Text::new("\u{1F501}").size(14)))
        .padding(0)
        .width(Length::Fixed(UTIL_BTN_SIZE))
        .height(Length::Fixed(UTIL_BTN_SIZE))
        .on_press(Message::ToggleLoop)
        .style(if is_looping {
            styles::active_btn
        } else {
            styles::loop_btn_style
        })
}

pub(crate) fn mute_btn<'a>(muted: bool) -> Button<'a, Message> {
    let icon = if muted { "\u{1F507}" } else { "\u{1F50A}" };
    Button::new(centered_icon(Text::new(icon).size(14)))
        .padding(0)
        .width(Length::Fixed(UTIL_BTN_SIZE))
        .height(Length::Fixed(UTIL_BTN_SIZE))
        .on_press(Message::ToggleMute)
        .style(if muted {
            styles::muted_btn_style
        } else {
            styles::mute_btn_style
        })
}

pub(crate) fn content_fit_btn<'a>(cf: iced::ContentFit) -> Button<'a, Message> {
    let text = Text::new(format!("{:?}", cf)).size(10);
    Button::new(centered_icon(text))
        .padding([4, 8])
        .height(Length::Fixed(BTN_HEIGHT))
        .on_press(Message::CycleContentFit)
        .style(styles::fit_btn_style)
}

pub(crate) fn fullscreen_btn<'a>() -> Button<'a, Message> {
    Button::new(centered_icon(Text::new("\u{26F6}").size(14)))
        .padding(0)
        .width(Length::Fixed(UTIL_BTN_SIZE))
        .height(Length::Fixed(UTIL_BTN_SIZE))
        .on_press(Message::ToggleFullscreen)
        .style(styles::fullscreen_btn_style)
}
