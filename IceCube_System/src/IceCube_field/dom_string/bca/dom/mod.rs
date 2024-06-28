pub struct DOM {
    dom_id: String,
    dom_status: bool,
}

impl DOM {
    pub fn new_dom(new_dom_id: &String) -> DOM {
        DOM {
            dom_id: String::from(new_dom_id),
            dom_status: false,
        }
    }
    pub fn get_id(&self) -> &String {
        &self.dom_id
    }
    pub fn get_status(&self) -> &bool {
        &self.dom_status
    }
}
