pub mod chart;
pub mod info_box;
pub mod left_list;
use super::app;
use super::app::App;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    // Create the layout sections.
    //
    // When displaying list options:
    // match app.current_screen {
    //     CurrentScreen::Main => main_page(f, app),
    //     CurrentScreen::Editing => {
    //         main_page(f, app);
    //         popup(f, app);
    //     }
    //     _ => {
    //         println!("Nothing Here");
    //     }
    // }
    main_page(f, app);
}
fn main_page(f: &mut Frame, app: &mut App) {
    let screen = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());
    let screen_ui = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Rgb(41, 44, 50)));
    f.render_widget(screen_ui, screen[0]);

    let layer01 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(screen[0]);
    let layer01_ui = Block::default().borders(Borders::ALL);
    f.render_widget(
        left_list::create_list(app).block(Block::bordered()),
        layer01[0],
    );
    f.render_widget(&layer01_ui, layer01[1]);
    let layer02_ui = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layer01[1]);
    let layer03_ui = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(layer02_ui[0]);
    f.render_widget(
        info_box::create_info_box(app).block(Block::bordered()),
        layer03_ui[0],
    );
    f.render_widget(&layer01_ui, layer03_ui[1]);
    f.render_widget(chart::create_chart(app.get_graph_data()), layer02_ui[1]);
}

// fn popup(f: &mut Frame, app: &App) {
//     let popup_block = Block::default()
//         .title("Enter a new key-value pair")
//         .borders(Borders::ALL)
//         .style(Style::default().bg(Color::DarkGray));
//     let area = centered_rect(60, 25, f.size());
//     f.render_widget(popup_block, area);
// }
