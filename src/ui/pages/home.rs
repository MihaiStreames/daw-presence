use iced::alignment;
use iced::widget::column;
use iced::widget::row;
use iced::widget::text;

use crate::daw::Project;
use crate::state::AppState;
use crate::state::Message;
use crate::ui::components;
use crate::ui::strings;
use crate::ui::style;

/// Render the home page
pub(in crate::ui) fn home_view(state: &AppState) -> iced::Element<'_, Message> {
    let (daw_name, project_name, memory_usage, cpu_usage) = match &state.daw_status {
        Some(status) => (
            status.display_name.clone(),
            match &status.project {
                _ if state.settings.hide_project_name => strings::PROJECT_HIDDEN.to_owned(),
                Project::Named(name) => name.clone(),
                Project::Untitled => strings::PROJECT_UNTITLED.to_owned(),
                Project::Unknown => strings::NO_PROJECT_OPEN.to_owned(),
            },
            format!("{} MB", status.memory_mb),
            format!("{:.1}%", status.cpu_usage),
        ),
        None => (
            strings::NO_DAW_DETECTED.to_owned(),
            strings::NO_DAW_DETECTED.to_owned(),
            strings::NO_DAW_DETECTED.to_owned(),
            strings::NO_DAW_DETECTED.to_owned(),
        ),
    };

    column(vec![
        row(vec![
            text(strings::STATUS).size(style::TEXT_HEADING).into(),
            components::status_icon(state.discord_connected),
        ])
        .align_y(alignment::Vertical::Center)
        .spacing(style::SPACING)
        .into(),
        row(vec![
            components::stat_card(strings::CARD_DAW, daw_name),
            components::stat_card(strings::CARD_PROJECT, project_name),
        ])
        .spacing(style::SPACING)
        .into(),
        row(vec![
            components::stat_card(strings::CARD_MEMORY, memory_usage),
            components::stat_card(strings::CARD_CPU, cpu_usage),
        ])
        .spacing(style::SPACING)
        .into(),
    ])
    .spacing(style::SPACING)
    .into()
}
