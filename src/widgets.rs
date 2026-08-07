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

/// SVG icon sized to ICON_SIZE — used by the transport controls.
fn icon_btn(icon_data: &[u8]) -> Svg<'_> {
    sized_icon(icon_data, ICON_SIZE)
}

/// SVG icon sized to an explicit `size`. Use this when a single global
/// `ICON_SIZE` doesn't fit every caller — for example, the circular
/// utility buttons are 32×32 rather than the 36×36 play/pause button,
/// so the icon inside them needs to be smaller to leave enough visual
/// padding.
fn sized_icon(icon_data: &[u8], size: f32) -> Svg<'_> {
    Svg::new(icons::svg_handle(icon_data))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
}

/// Side length of the icons used by the utility-cluster buttons
/// (loop, mute, fullscreen). Sized to sit clearly inside the 32×32
/// circle with comfortable padding after the SVG is centered by
/// `centered_icon`. 12 leaves 10px of margin on every side, which
/// reads as "icon inside a circle" rather than "icon crammed against
/// the edge".
const UTIL_ICON_SIZE: f32 = 16.0;

/// Width/height of the square (circular) utility buttons — loop, mute,
/// and fullscreen. Matches BTN_HEIGHT for visual consistency with the
/// pill-shaped transport buttons next to them.
const UTIL_BTN_SIZE: f32 = BTN_HEIGHT;

/// Build a fixed-size [`Container`] whose inner child is centered on
/// both axes. Used to wrap the utility-cluster SVG glyphs so they
/// sit at the geometric center of a 32×32 button — needed because
/// `iced::widget::button::Button` uses `layout::padded` to position
/// its content at `(padding.left, padding.top)` (top-left), not the
/// center, so a smaller fixed-size child like `Sized::Fixed(14)` SVG
/// would otherwise land at the top edge of the button.
///
/// Using `Length::Fixed(UTIL_BTN_SIZE)` (not `Length::Fill`) for the
/// container's size is critical — `Length::Fill` propagates up to the
/// parent `Row` and turns the 32×32 buttons into wide pills. Fixed
/// sizes keep the button widths and put centering entirely under our
/// control.
fn centered_icon<'a>(icon: Svg<'a>) -> Container<'a, Message> {
    Container::new(icon).center(Length::Fixed(UTIL_BTN_SIZE))
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

/// Circular loop toggle. Uses an SVG icon (not the previously used 🔁
/// emoji) so the glyph renders at the exact size we set and centers
/// reliably inside the button on every platform. Sized via
/// `sized_icon` + `UTIL_ICON_SIZE` so it sits inside the 32×32 circle
/// with sensible padding rather than crowding the edge.
pub(crate) fn loop_btn<'a>(is_looping: bool) -> Button<'a, Message> {
    Button::new(centered_icon(sized_icon(icons::LOOP, UTIL_ICON_SIZE)))
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

/// Circular mute toggle. Switches between two SVG glyphs (sized to
/// `UTIL_ICON_SIZE`) so the icon stays aligned and proportionate
/// inside the 32×32 button regardless of OS emoji font behavior.
pub(crate) fn mute_btn<'a>(muted: bool) -> Button<'a, Message> {
    let icon = if muted { icons::MUTE } else { icons::VOLUME };
    Button::new(centered_icon(sized_icon(icon, UTIL_ICON_SIZE)))
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

/// Text-only "Contain / Cover / ..." pill. Not a circular button —
/// keeps its natural width based on the label.
pub(crate) fn content_fit_btn<'a>(cf: iced::ContentFit) -> Button<'a, Message> {
    let text = Text::new(format!("{:?}", cf)).size(10);
    Button::new(
        Container::new(text)
            .center_y(Length::Fixed(BTN_HEIGHT))
            .align_x(iced::alignment::Horizontal::Center),
    )
    .padding([4, 8])
    .height(Length::Fixed(BTN_HEIGHT))
    .on_press(Message::CycleContentFit)
    .style(styles::fit_btn_style)
}

/// Circular fullscreen toggle. SVG icon sized to `UTIL_ICON_SIZE` and
/// centered inside the 32×32 button (the construction matches
/// `pause_play_btn`, which is the canonical example of a centered
/// SVG inside a fixed-size button).
pub(crate) fn fullscreen_btn<'a>() -> Button<'a, Message> {
    Button::new(centered_icon(sized_icon(icons::FULLSCREEN, UTIL_ICON_SIZE)))
        .padding(0)
        .width(Length::Fixed(UTIL_BTN_SIZE))
        .height(Length::Fixed(UTIL_BTN_SIZE))
        .on_press(Message::ToggleFullscreen)
        .style(styles::fullscreen_btn_style)
}
