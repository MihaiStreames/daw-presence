use iced::Background;
use iced::Border;
use iced::Length;
use iced::padding;
use iced::widget::Space;
use iced::widget::container;
use iced::widget::container::Style;

use crate::state::Message;
use crate::ui::style;

pub(in crate::ui) fn status_icon(connected: bool) -> iced::Element<'static, Message> {
    let circle = container(Space::new())
        .width(Length::Fixed(style::STATUS_ICON_SIZE))
        .height(Length::Fixed(style::STATUS_ICON_SIZE))
        .style(move |theme: &iced::Theme| {
            let colors = style::colors(theme);
            let color = if connected { colors.accent } else { colors.danger };

            Style {
                background: Some(Background::Color(color)),
                border: Border {
                    radius: (style::STATUS_ICON_SIZE / 2.0).into(),
                    ..Border::default()
                },
                ..Default::default()
            }
        });

    // outer wrapper for vertical offset to align with heading text
    container(circle)
        .padding(padding::top(style::STATUS_ICON_TOP_OFFSET))
        .into()
}
