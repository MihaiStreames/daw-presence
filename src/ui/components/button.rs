use iced::Background;
use iced::Border;
use iced::widget::button;

use crate::ui::style;

pub(in crate::ui) fn accent_button(theme: &iced::Theme, status: button::Status) -> button::Style {
    let colors = style::colors(theme);
    let text_color = theme.palette().background;

    let (bg, tc) = match status {
        button::Status::Active => (colors.accent, text_color),
        button::Status::Hovered => (colors.accent_hover, text_color),
        button::Status::Pressed => (colors.accent_muted, text_color),
        button::Status::Disabled => (colors.control, colors.text_muted),
    };

    button::Style {
        background: Some(Background::Color(bg)),
        text_color: tc,
        border: Border {
            radius: style::BORDER_RADIUS.into(),
            ..Border::default()
        },
        ..Default::default()
    }
}
