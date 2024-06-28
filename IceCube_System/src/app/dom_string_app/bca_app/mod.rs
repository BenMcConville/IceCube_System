pub mod dom_app;

pub struct BcaApp {
    selected_bca: bool,
    pub current_list_index: usize,
    is_open: bool,
    bca_id: String,
    bca_status: bool,
    pub bca_doms: Vec<dom_app::DomApp>,
}
impl BcaApp {
    pub fn new(
        new_bca_id: &String,
        new_bca_status: bool,
        dom_list: Vec<dom_app::DomApp>,
    ) -> BcaApp {
        BcaApp {
            selected_bca: false,
            current_list_index: 0,
            is_open: false,
            bca_id: String::from(new_bca_id),
            bca_status: new_bca_status,
            bca_doms: dom_list,
        }
    }
    pub fn get_id(&self) -> String {
        String::from(&self.bca_id)
    }
    pub fn get_is_open(&self) -> &bool {
        &self.is_open
    }
    pub fn is_selected(&self) -> &bool {
        &self.selected_bca
    }
    pub fn toggle_selection(&mut self, setting: bool) {
        self.selected_bca = setting;
    }
    pub fn toggle_open_list(&mut self, setting: bool) {
        self.is_open = setting;
    }
    pub fn current_list_index_move_down(&mut self) {
        self.bca_doms[self.current_list_index].toggle_selection(false);
        self.current_list_index = (self.current_list_index + 1) % self.bca_doms.len();
        self.bca_doms[self.current_list_index].toggle_selection(true);
    }
    pub fn current_list_index_move_up(&mut self) {
        self.bca_doms[self.current_list_index].toggle_selection(false);
        self.current_list_index = if self.current_list_index == 0 {
            self.bca_doms.len() - 1
        } else {
            (self.current_list_index - 1) % self.bca_doms.len()
        };
        self.bca_doms[self.current_list_index].toggle_selection(true);
    }
    pub fn all_doms_operational(&self) -> bool {
        let mut all_doms_status = true;
        for current_dom in &self.bca_doms {
            if !*current_dom.get_status() {
                all_doms_status = false;
            }
        }
        all_doms_status
    }
}
