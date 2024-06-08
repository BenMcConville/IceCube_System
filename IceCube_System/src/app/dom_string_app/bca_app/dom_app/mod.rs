pub struct DomApp {
    selected_dom: bool,
    dom_id: String,
    dom_status: bool,
}
impl DomApp {
    pub fn new(new_dom_id: &String, new_dom_status: &bool) -> DomApp {
        DomApp {
            selected_dom: false,
            dom_id: String::from(new_dom_id),
            dom_status: *new_dom_status,
        }
    }
}
