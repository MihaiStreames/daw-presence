use iced::Background;
use iced::Length;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::row;
use iced::widget::text;

use crate::state::Message;
use crate::state::Page;
use crate::ui::style;

pub(in crate::ui) fn sidebar_item(label: &'static str, page: Page, active: Page) -> iced::Element<'static, Message> {
    let is_active = page == active;

    let accent_bar = container(Space::new())
        .width(Length::Fixed(style::SIDEBAR_ACCENT_WIDTH))
        .height(Length::Fill)
        .style(move |theme: &iced::Theme| {
            if is_active {
                Style::default().background(style::colors(theme).accent)
            } else {
                Style::default()
            }
        });

    let label = container(text(label).size(style::TEXT_LABEL))
        .padding(iced::Padding {
            top: style::SIDEBAR_ITEM_PADDING - style::SIDEBAR_TEXT_NUDGE,
            right: style::SIDEBAR_ITEM_PADDING,
            bottom: style::SIDEBAR_ITEM_PADDING + style::SIDEBAR_TEXT_NUDGE,
            left: style::SIDEBAR_ITEM_PADDING,
        })
        .width(Length::Fill);

    // the fill-height accent bar would otherwise stretch the row, and the item, to the whole sidebar
    let content = row(vec![accent_bar.into(), label.into()]).height(Length::Shrink);

    button(content)
        .padding(0)
        .on_press(Message::NavigateTo(page))
        .style(move |theme, status| {
            let colors = style::colors(theme);
            let is_hovered = matches!(status, button::Status::Hovered);

            let background = if is_active {
                Some(colors.background_selected)
            } else if matches!(status, button::Status::Pressed) {
                Some(colors.control)
            } else if is_hovered {
                Some(colors.control_hover)
            } else {
                None
            };

            button::Style {
                background: background.map(Background::Color),
                text_color: if is_active || is_hovered {
                    theme.palette().text
                } else {
                    colors.text_muted
                },
                ..button::Style::default()
            }
        })
        .into()
}
