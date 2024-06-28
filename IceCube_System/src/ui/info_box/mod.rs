use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::Paragraph;

use super::app::App;

pub fn create_info_box(app: &mut App) -> Paragraph {
    if *app.dom_app_strings[app.current_list_index].get_is_open() {
        if *app.dom_app_strings[app.current_list_index].app_bca_list
            [app.dom_app_strings[app.current_list_index].current_list_index]
            .get_is_open()
        {
            dom_info_box(app)
        } else {
            bca_info_box(app)
        }
    } else {
        iceCube_field_info_box(app)
    }
}

fn dom_info_box(app: &mut App) -> Paragraph {
    let status = if app.dom_app_strings[app.current_list_index].app_bca_list
        [app.dom_app_strings[app.current_list_index].current_list_index]
        .all_doms_operational()
    {
        String::from("Stable")
    } else {
        String::from("Offline")
    };
    Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::styled(
                "    Station Name: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                "South-Pole Neutrino Observatory",
                Style::default().fg(ratatui::style::Color::Green),
            ),
        ]),
        Line::from("    ---------------------------------------------"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "   Selected String: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                app.dom_app_strings[app.current_list_index].get_id(),
                Style::default().fg(ratatui::style::Color::LightGreen),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "   Selected Bca: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                app.dom_app_strings[app.current_list_index].app_bca_list
                    [app.dom_app_strings[app.current_list_index].current_list_index]
                    .get_id(),
                Style::default().fg(ratatui::style::Color::LightGreen),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                ">>   Selected Dom: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                app.dom_app_strings[app.current_list_index].app_bca_list
                    [app.dom_app_strings[app.current_list_index].current_list_index]
                    .bca_doms[app.dom_app_strings[app.current_list_index].app_bca_list
                    [app.dom_app_strings[app.current_list_index].current_list_index]
                    .current_list_index]
                    .get_id(),
                Style::default().fg(ratatui::style::Color::White),
            ),
        ]),
    ]))
}

fn bca_info_box(app: &mut App) -> Paragraph {
    Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::styled(
                "    Station Name: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                "South-Pole Neutrino Observatory",
                Style::default().fg(ratatui::style::Color::Green),
            ),
        ]),
        Line::from("    ---------------------------------------------"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "   Selected String: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                app.dom_app_strings[app.current_list_index].get_id(),
                Style::default().fg(ratatui::style::Color::LightGreen),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                ">>   Selected Bca: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                app.dom_app_strings[app.current_list_index].app_bca_list
                    [app.dom_app_strings[app.current_list_index].current_list_index]
                    .get_id(),
                Style::default().fg(ratatui::style::Color::White),
            ),
        ]),
    ]))
}

fn iceCube_field_info_box(app: &mut App) -> Paragraph {
    Paragraph::new(Text::from(vec![
        Line::from(vec![
            Span::styled(
                "    Station Name: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                "South-Pole Neutrino Observatory",
                Style::default().fg(ratatui::style::Color::Green),
            ),
        ]),
        Line::from("    ---------------------------------------------"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                ">>   Selected String: ",
                Style::default().fg(ratatui::style::Color::White),
            ),
            Span::styled(
                app.dom_app_strings[app.current_list_index].get_id(),
                Style::default().fg(ratatui::style::Color::White),
            ),
        ]),
    ]))
}
