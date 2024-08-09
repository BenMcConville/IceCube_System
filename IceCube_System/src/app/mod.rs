pub mod dom_string_app;
pub mod graph_data;

use super::IceCube_field;

enum AppPages {
    Main,
}

pub struct App {
    current_page: AppPages,
    pub dom_app_strings: Vec<dom_string_app::StringApp>,
    pub current_list_index: usize,
    pub graph_data: graph_data::Graph,
}
impl App {
    pub fn new_app() -> App {
        App {
            current_page: AppPages::Main,
            dom_app_strings: vec![],
            current_list_index: 0,
            graph_data: graph_data::Graph::new(),
        }
    }
    pub fn get_selected_dom(&self) -> String {
        for i in self.dom_app_strings.iter() {
            if *i.get_is_open() {
                for j in i.app_bca_list.iter() {
                    if *j.get_is_open() {
                        for d in j.bca_doms.iter() {
                            if *d.is_selected() {
                                return d.get_id();
                            }
                        }
                        break;
                    }
                }
                break;
            }
        }
        return String::from("None");
    }
    pub fn get_graph_data(&self) -> &[(f64, f64); 200] {
        &self.graph_data.get_data_points()
    }
    pub fn temp_data_sync(&mut self, data: f64) -> String {
        if self.get_selected_dom().trim() == "None" {
            self.graph_data.update_data(0.0);
        } else {
            self.graph_data.update_data(data);
        }
        return self.get_selected_dom();
    }
    pub fn reload_dom_strings(&mut self, string_field: &IceCube_field::IceCubeField) {
        let mut updated_dom_strings: Vec<dom_string_app::StringApp> = vec![];
        for (index, dom_string) in string_field.get_all_dom_strings().iter().enumerate() {
            if index == 0 {
                updated_dom_strings.push(dom_string_app::StringApp::new(
                    dom_string.get_id(),
                    dom_string.init_bca_app(),
                    true,
                ));
            } else {
                updated_dom_strings.push(dom_string_app::StringApp::new(
                    dom_string.get_id(),
                    dom_string.init_bca_app(),
                    false,
                ));
            }
        }
        self.dom_app_strings = updated_dom_strings;
    }
    pub fn open_current_index(&mut self) {
        self.dom_app_strings[self.current_list_index].toggle_open_list();
    }
    pub fn close_current_index(&mut self) {
        self.dom_app_strings[self.current_list_index].toggle_close_list();
    }
    pub fn current_list_index_move_down(&mut self) {
        if *self.dom_app_strings[self.current_list_index].get_is_open() {
            self.dom_app_strings[self.current_list_index].current_list_index_move_down();
        } else {
            self.dom_app_strings[self.current_list_index].toggle_selection(false);
            self.current_list_index = (self.current_list_index + 1) % self.dom_app_strings.len();
            self.dom_app_strings[self.current_list_index].toggle_selection(true);
        }
    }
    pub fn current_list_index_move_up(&mut self) {
        if *self.dom_app_strings[self.current_list_index].get_is_open() {
            self.dom_app_strings[self.current_list_index].current_list_index_move_up();
        } else {
            self.dom_app_strings[self.current_list_index].toggle_selection(false);
            self.current_list_index = if self.current_list_index == 0 {
                self.dom_app_strings.len() - 1
            } else {
                (self.current_list_index - 1) % self.dom_app_strings.len()
            };
            self.dom_app_strings[self.current_list_index].toggle_selection(true);
        }
    }
    // pub fn get_list(&self) -> Vec<String> {
    //     let mut temp_list: Vec<String> = vec![];
    //     for current_string in &self.dom_app_strings {
    //         temp_list.push(current_string.get_id());
    //         if *current_string.get_is_open() {
    //             println!("Is Open");
    //         }
    //     }
    //     temp_list
    // }
}
