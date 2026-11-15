use iced::Border;
use iced::Length;
use iced::widget::column;
use iced::widget::container;
use iced::widget::container::Style;
use iced::widget::text;

use crate::state::Message;
use crate::ui::style;

pub(in crate::ui) fn stat_card(title: &'static str, value: String) -> iced::Element<'static, Message> {
    let label = text(title)
        .size(style::TEXT_LABEL)
        .style(style::text_role(|colors| colors.accent));
    let value = text(value).size(style::TEXT_LABEL);
    let card = column(vec![label.into(), value.into()]).spacing(style::SPACING_TIGHT);

    container(card)
        .padding(style::PADDING_CARD)
        .width(Length::Fill)
        .style(card_style)
        .into()
}

fn card_style(theme: &iced::Theme) -> Style {
    let colors = style::colors(theme);
    Style::default()
        .background(iced::Background::Color(colors.background_raised))
        .border(Border {
            radius: style::BORDER_RADIUS_CARD.into(),
            ..Border::default()
        })
}
