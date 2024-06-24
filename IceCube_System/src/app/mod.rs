pub mod dom_string_app;
use super::IceCube_field;

enum AppPages {
    Main,
}

pub struct App {
    current_page: AppPages,
    dom_app_strings: Vec<dom_string_app::StringApp>,
}
impl App {
    pub fn new_app() -> App {
        App {
            current_page: AppPages::Main,
            dom_app_strings: vec![],
        }
    }
    pub fn reload_dom_strings(self, string_field: &IceCube_field::IceCubeField) {
        let mut updated_dom_strings: Vec<dom_string_app::StringApp> = vec![];
        println!("App Dom_strings:");
        for dom_string in string_field.get_all_dom_strings() {
            println!("{}", dom_string.get_id());
            // updated_dom_strings.push();
        }
    }
}
