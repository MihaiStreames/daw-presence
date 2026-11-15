use iced::Background;
use iced::Font;
use iced::Length;
use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::mouse_area;
use iced::widget::row;
use iced::widget::text;

use crate::state::Message;
use crate::ui::style;

const CAPTION_FONT: Font = Font::with_name("Segoe MDL2 Assets");
const GLYPH_MINIMIZE: char = '\u{E921}';
const GLYPH_CLOSE: char = '\u{E8BB}';

pub(in crate::ui) fn title_bar() -> iced::Element<'static, Message> {
    let drag_area = mouse_area(Space::new().width(Length::Fill).height(Length::Fill)).on_press(Message::DragWindow);

    row(vec![
        drag_area.into(),
        caption_button(GLYPH_MINIMIZE, Message::MinimizeWindow, |colors| {
            colors.background_selected
        }),
        caption_button(GLYPH_CLOSE, Message::CloseWindow, |colors| colors.danger),
    ])
    .height(Length::Fixed(style::TITLE_BAR_HEIGHT))
    .into()
}

fn caption_button(glyph: char, message: Message, hover: style::ColorRole) -> iced::Element<'static, Message> {
    let icon = text(glyph)
        .font(CAPTION_FONT)
        .size(style::TITLE_GLYPH_SIZE)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);

    button(icon)
        .width(Length::Fixed(style::TITLE_BUTTON_WIDTH))
        .height(Length::Fill)
        .padding(0)
        .on_press(message)
        .style(move |theme: &iced::Theme, status| {
            let colors = style::colors(theme);
            let is_active = matches!(status, button::Status::Hovered | button::Status::Pressed);

            button::Style {
                background: is_active.then_some(Background::Color(hover(colors))),
                text_color: if is_active {
                    theme.palette().text
                } else {
                    colors.text_muted
                },
                ..button::Style::default()
            }
        })
        .into()
}
