// Temp Imports-------------
#![allow(warnings)]

use std::{borrow::BorrowMut, error::Error, io};

use crate::ui::ui;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
pub mod app;
pub mod ui;
//----------------

use std::time::SystemTime;

pub mod IceCube_field;
pub mod data_stream;

fn main() -> Result<(), Box<dyn Error>> {
    let IceCube = IceCube_field::IceCubeField::init_IceCube_field();
    // println!("IceCube Field Strings:");
    // println!("{:?}", IceCube.get_all_dom_strings_id());
    let mut App = App::new_app();
    App.reload_dom_strings(&IceCube);
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let _res = run_app(&mut terminal, &mut App);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
fn run() {
    //Test for loading current time into program, will be passed into data_stream module
    let sys_time = SystemTime::now();
    let mut new_sys_time = SystemTime::now();
    loop {
        match new_sys_time.duration_since(sys_time) {
            Ok(time) => {
                if (time.as_secs() > 5) {
                    break;
                }
                println!("current Time is: {}", time.as_secs());
            }
            Err(_) => {}
        }
        new_sys_time = SystemTime::now();
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(false);
            }
            if key.code == KeyCode::Down {
                app.current_list_index_move_down();
            }
            if key.code == KeyCode::Up {
                app.current_list_index_move_up();
            }
            if key.code == KeyCode::Right {
                app.open_current_index();
            }
            if key.code == KeyCode::Left {
                app.close_current_index();
            }
            if key.code == KeyCode::Char('a') {
                app.graph_data.update_data(5.0);
            }
            if key.code == KeyCode::Char('b') {
                app.graph_data.update_data(0.0);
            }
        }
    }
}
