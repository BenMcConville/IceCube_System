pub struct DomApp {
    selected_dom: bool,
    dom_id: String,
    dom_status: bool,
}
impl DomApp {
    pub fn new(new_dom_id: &String, new_dom_status: bool, is_selected: bool) -> DomApp {
        DomApp {
            selected_dom: is_selected,
            dom_id: String::from(new_dom_id),
            dom_status: new_dom_status,
        }
    }
    pub fn get_id(&self) -> String {
        String::from(&self.dom_id)
    }
    pub fn get_status(&self) -> &bool {
        &self.dom_status
    }
    pub fn toggle_selection(&mut self, setting: bool) {
        self.selected_dom = setting;
    }
    pub fn is_selected(&self) -> &bool {
        &self.selected_dom
    }
}
