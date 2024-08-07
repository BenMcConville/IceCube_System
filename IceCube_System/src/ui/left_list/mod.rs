use std::fmt::format;

use crossterm::cursor;
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::List;

use super::App;

pub fn create_list<'a>(app: &mut App) -> List<'a> {
    let mut temp_list: Vec<Text> = vec![];
    temp_list.push(Text::from(""));
    for current_string in app.dom_app_strings.iter() {
        if *current_string.is_selected() {
            if *current_string.get_is_open() {
                temp_list.push(Text::styled(
                    format!(" {}", current_string.get_id()),
                    Style::default().fg(Color::White).bg(Color::Blue),
                ));
            } else {
                temp_list.push(Text::styled(
                    format!(" {}", current_string.get_id()),
                    Style::default().fg(Color::White).bg(Color::Blue),
                ));
            }
        } else {
            temp_list.push(Text::styled(
                format!(" {}", current_string.get_id()),
                Style::default().fg(Color::White),
            ));
        }
        if *current_string.get_is_open() {
            for current_bca in current_string.app_bca_list.iter() {
                if *current_bca.is_selected() {
                    if *current_bca.get_is_open() {
                        temp_list.push(Text::styled(
                            format!("  {}", current_bca.get_id()),
                            Style::default().bg(Color::Cyan),
                        ));
                    } else {
                        temp_list.push(Text::styled(
                            format!("  {}", current_bca.get_id()),
                            Style::default().bg(Color::Cyan),
                        ));
                    }
                } else {
                    temp_list.push(Text::styled(
                        format!("  {}", current_bca.get_id()),
                        Style::default(),
                    ));
                }
                if *current_bca.get_is_open() {
                    for current_dom in current_bca.bca_doms.iter() {
                        if *current_dom.is_selected() {
                            temp_list.push(Text::styled(
                                format!("   {}", current_dom.get_id()),
                                Style::default().bg(Color::LightBlue),
                            ));
                        } else {
                            temp_list.push(Text::styled(
                                format!("   {}", current_dom.get_id()),
                                Style::default(),
                            ));
                        }
                    }
                }
            }
        }
    }
    List::new(temp_list)
}
