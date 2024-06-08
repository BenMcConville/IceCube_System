use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame) {
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
    main_page(f);
}
fn main_page(f: &mut Frame) {
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

    // let list = List::new(vec![]).block(Block::default().borders(Borders::ALL));

    // f.render_widget(list, layer01[0]);
    // f.render_widget(&layer01_ui, layer01[0]);
    f.render_widget(&layer01_ui, layer01[1]);
}
// fn popup(f: &mut Frame, app: &App) {
//     let popup_block = Block::default()
//         .title("Enter a new key-value pair")
//         .borders(Borders::ALL)
//         .style(Style::default().bg(Color::DarkGray));
//     let area = centered_rect(60, 25, f.size());
//     f.render_widget(popup_block, area);
// }
