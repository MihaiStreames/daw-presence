use iced::Background;
use iced::Color;
use iced::widget::toggler;

use super::setting_row;
use crate::state::Message;
use crate::ui::style;

pub(in crate::ui) fn setting_toggle(
    label: &'static str,
    description: &'static str,
    enabled: bool,
    message: Message,
) -> iced::Element<'static, Message> {
    let toggle = toggler(enabled)
        .on_toggle(move |_| message.clone())
        .style(toggler_style);

    setting_row(label, description, |colors| colors.text_muted, toggle)
}

fn toggler_style(theme: &iced::Theme, status: toggler::Status) -> toggler::Style {
    let colors = style::colors(theme);
    let (bg, fg) = match status {
        toggler::Status::Active { is_toggled } => {
            if is_toggled {
                (colors.accent, Color::WHITE)
            } else {
                (colors.control, colors.text_muted)
            }
        }
        toggler::Status::Hovered { is_toggled } => {
            if is_toggled {
                (colors.accent_hover, Color::WHITE)
            } else {
                (colors.control_hover, colors.text_muted)
            }
        }
        toggler::Status::Disabled { is_toggled } => {
            if is_toggled {
                (colors.accent_muted, colors.text_muted)
            } else {
                (colors.control, colors.text_muted)
            }
        }
    };

    toggler::Style {
        background: Background::Color(bg),
        foreground: Background::Color(fg),
        background_border_width: 0.0,
        background_border_color: Color::TRANSPARENT,
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: None,
        border_radius: None,
        padding_ratio: 0.25,
    }
}
