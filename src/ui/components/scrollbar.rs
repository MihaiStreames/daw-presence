use iced::Background;
use iced::Border;
use iced::widget::container::Style;
use iced::widget::scrollable;

use crate::ui::style;

pub(in crate::ui) fn scrollable_style(theme: &iced::Theme, status: scrollable::Status) -> scrollable::Style {
    let colors = style::colors(theme);
    let scroller_color = match status {
        scrollable::Status::Active { .. } => colors.control,
        scrollable::Status::Hovered { .. } => colors.accent_muted,
        scrollable::Status::Dragged { .. } => colors.accent,
    };

    let rail = scrollable::Rail {
        background: None,
        border: Border::default(),
        scroller: scrollable::Scroller {
            background: Background::Color(scroller_color),
            border: Border {
                radius: style::BORDER_RADIUS.into(),
                ..Border::default()
            },
        },
    };

    scrollable::Style {
        container: Style::default(),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: None,
        auto_scroll: scrollable::AutoScroll {
            background: Background::Color(colors.control),
            border: Border::default(),
            shadow: iced::Shadow::default(),
            icon: colors.accent,
        },
    }
}
