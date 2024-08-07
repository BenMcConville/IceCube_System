// Temp Imports-------------
#![allow(warnings)]

use serde_json::json;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::{borrow::BorrowMut, error::Error, io, time::Duration};

use crate::ui::ui;
use app::App;
use crossterm::{
    event::{
        self, poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
    },
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    let mut sys_time = SystemTime::now();
    let mut new_sys_time = SystemTime::now();
    loop {
        if poll(Duration::from_micros(5000))? {
            // Sampling Rate
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
            }
        } else {
            terminal.draw(|f| ui(f, app))?;
            match new_sys_time.duration_since(sys_time) {
                Ok(time) => {
                    write_json_file(app.temp_data_sync(15.0 * (time.as_secs_f64())));
                    //println!("current Time is: {}", time.as_secs());
                }
                Err(_) => println!("Error"),
            }
            new_sys_time = SystemTime::now();
        }
    }
}

fn write_json_file(uid: String) -> std::io::Result<()> {
    // println!("{:?}", read_json_file())
    let file = File::create("Sensor_Input.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(
        &mut writer,
        &json!({"UID": uid, "x": 0, "y": 0, "Data": 0, "Operational": false, "Updated": false}),
    )?;
    writer.flush()?;
    Ok(())
}
fn read_json_file() {
    let json: serde_json::Value =
        serde_json::from_str("Sensor_Input.json").expect("JSON was not well-formatted");
}
