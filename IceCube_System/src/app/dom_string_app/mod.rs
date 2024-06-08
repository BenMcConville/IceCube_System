pub mod bca_app;
use super::IceCube_field::{dom_string, dom_string::bca};

pub struct StringApp {
    selected_string: bool,
    string_id: String,
    app_bca_list: Vec<bca_app::BcaApp>,
}
impl StringApp {
    pub fn new(reloaded_dom_string: &dom_string::DOMString) -> StringApp {
        StringApp {
            selected_string: false,
            string_id: String::from(reloaded_dom_string.get_id()),
            app_bca_list: init_bca_list(reloaded_dom_string.get_bca_ids()),
        }
    }
}

fn init_bca_list(reloaded_bcas: Vec<&bca::BCA>) -> Vec<bca_app::BcaApp> {
    let bca_app_list: Vec<bca_app::BcaApp> = vec![];
    bca_app_list
}
