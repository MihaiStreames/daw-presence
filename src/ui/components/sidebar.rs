use iced::Length;
use iced::padding;
use iced::widget::column;
use iced::widget::container;
use iced::widget::container::Style;

use super::sidebar_item;
use crate::state::AppState;
use crate::state::Message;
use crate::state::Page;
use crate::ui::strings;
use crate::ui::style;

/// Render the sidebar with page navigation
pub(in crate::ui) fn sidebar(state: &AppState) -> iced::Element<'_, Message> {
    let items = column(vec![
        sidebar_item(strings::NAV_HOME, Page::Home, state.active_page),
        sidebar_item(strings::NAV_SETTINGS, Page::Settings, state.active_page),
    ])
    .spacing(style::SIDEBAR_ITEM_SPACING);

    container(items)
        .width(Length::Fixed(style::SIDEBAR_WIDTH))
        .height(Length::Fill)
        .padding(padding::top(style::TITLE_BAR_HEIGHT))
        .style(sidebar_bg)
        .into()
}

fn sidebar_bg(theme: &iced::Theme) -> Style {
    Style::default().background(iced::Background::Color(style::colors(theme).background_sunken))
}
