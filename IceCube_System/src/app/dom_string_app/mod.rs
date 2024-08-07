pub mod bca_app;
use super::IceCube_field::{dom_string, dom_string::bca};

pub struct StringApp {
    selected_string: bool,
    pub current_list_index: usize,
    is_open: bool,
    string_id: String,
    pub app_bca_list: Vec<bca_app::BcaApp>,
}
impl StringApp {
    pub fn new(
        new_dom_string_id: &String,
        app_bca_list: Vec<bca_app::BcaApp>,
        is_selected: bool,
    ) -> StringApp {
        StringApp {
            selected_string: is_selected,
            current_list_index: 0,
            is_open: false,
            string_id: String::from(new_dom_string_id),
            app_bca_list: app_bca_list,
        }
    }
    pub fn toggle_open_list(&mut self) {
        if self.is_open {
            self.app_bca_list[self.current_list_index].toggle_open_list(true);
        } else {
            self.is_open = true;
        }
    }
    pub fn toggle_close_list(&mut self) {
        if *self.app_bca_list[self.current_list_index].get_is_open() {
            self.app_bca_list[self.current_list_index].toggle_open_list(false);
        } else {
            self.is_open = false;
        }
    }
    pub fn toggle_selection(&mut self, setting: bool) {
        self.selected_string = setting;
    }
    pub fn get_id(&self) -> String {
        String::from(&self.string_id)
    }
    pub fn get_is_open(&self) -> &bool {
        &self.is_open
    }
    pub fn is_selected(&self) -> &bool {
        &self.selected_string
    }

    pub fn current_list_index_move_down(&mut self) {
        if *self.app_bca_list[self.current_list_index].get_is_open() {
            self.app_bca_list[self.current_list_index].current_list_index_move_down();
        } else {
            self.app_bca_list[self.current_list_index].toggle_selection(false);
            self.current_list_index = (self.current_list_index + 1) % self.app_bca_list.len();
            self.app_bca_list[self.current_list_index].toggle_selection(true);
        }
    }
    pub fn current_list_index_move_up(&mut self) {
        if *self.app_bca_list[self.current_list_index].get_is_open() {
            self.app_bca_list[self.current_list_index].current_list_index_move_up();
        } else {
            self.app_bca_list[self.current_list_index].toggle_selection(false);
            self.current_list_index = if self.current_list_index == 0 {
                self.app_bca_list.len() - 1
            } else {
                (self.current_list_index - 1) % self.app_bca_list.len()
            };
            self.app_bca_list[self.current_list_index].toggle_selection(true);
        }
    }
}

fn init_bca_list(reloaded_bcas: Vec<&bca::BCA>) -> Vec<bca_app::BcaApp> {
    let bca_app_list: Vec<bca_app::BcaApp> = vec![];
    bca_app_list
}
