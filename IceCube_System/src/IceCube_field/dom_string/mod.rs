pub mod bca;
use super::dom_string_app::bca_app;

pub struct DOMString {
    string_id: String,
    pub bca_list: Vec<bca::BCA>,
}

impl DOMString {
    pub fn new_string(new_string_id: &String) -> DOMString {
        DOMString {
            string_id: String::from(new_string_id),
            bca_list: vec![],
        }
    }
    pub fn get_id(&self) -> &String {
        &self.string_id
    }
    pub fn get_bca_ids(&self) -> Vec<&String> {
        let mut bca_id_list: Vec<&String> = vec![];
        for current_bca in self.bca_list.iter() {
            bca_id_list.push(current_bca.get_id());
        }
        bca_id_list
    }
    pub fn get_all_dom(&self) -> Vec<Vec<&String>> {
        let mut dom_list: Vec<Vec<&String>> = vec![];
        for current_bca in self.bca_list.iter() {
            dom_list.push(current_bca.get_dom_ids());
        }
        dom_list
    }
    pub fn init_bca_app(&self) -> Vec<bca_app::BcaApp> {
        let mut temp_list: Vec<bca_app::BcaApp> = vec![];
        for current_bca in &self.bca_list {
            temp_list.push(bca_app::BcaApp::new(
                current_bca.get_id(),
                false,
                current_bca.init_bca_app(),
            ));
        }
        temp_list
    }
}
