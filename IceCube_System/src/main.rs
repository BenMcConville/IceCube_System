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
    // let st = dom_string::DOMString::new_string(&String::from("test"));
    // println!("{:?}", st.get_bca_ids());
    // let string_01 = init_string_01();
    // println!("{:?}", string_01.get_all_dom());
    // println!("{:?}", string_01.get_bca_ids());
    // run();
    let IceCube = IceCube_field::IceCubeField::init_IceCube_field();
    println!("IceCube Field Strings:");
    println!("{:?}", IceCube.get_all_dom_strings_id());
    let App = App::new_app();
    App.reload_dom_strings(&IceCube);
    // println!("{:?}", IceCube.get_all_dom_strings()[0].get_bca_ids());
    // println!("{:?}", IceCube.get_all_dom_strings()[1].get_bca_ids());
    // enable_raw_mode()?;
    // let mut stderr = io::stderr();
    // execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    // let backend = CrosstermBackend::new(stderr);
    // let mut terminal = Terminal::new(backend)?;

    // // let mut app = app::App::new();
    // // let _res = run_app(&mut terminal, &mut app);
    // let _res = run_app(&mut terminal);

    // // restore terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                return Ok(false);
            }
        }
    }
}
