use iced::Length;
use iced::alignment::Vertical;
use iced::widget::column;
use iced::widget::container;
use iced::widget::row;
use iced::widget::text;

use crate::state::Message;
use crate::ui::style;

pub(super) fn setting_row<'a>(
    label: &'static str,
    description: &'a str,
    description_role: style::ColorRole,
    control: impl Into<iced::Element<'a, Message>>,
) -> iced::Element<'a, Message> {
    let text_column = column(vec![
        text(label).size(style::TEXT_LABEL).into(),
        text(description)
            .size(style::TEXT_DESCRIPTION)
            .style(style::text_role(description_role))
            .into(),
    ])
    .spacing(style::LABEL_SPACING)
    .width(Length::Fill);

    container(row(vec![text_column.into(), control.into()]).align_y(Vertical::Center))
        .padding(style::SETTING_ROW_PADDING)
        .into()
}
