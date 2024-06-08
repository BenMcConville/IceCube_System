pub mod bca;

pub struct DOMString {
    string_id: String,
    bca_list: Vec<bca::BCA>,
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
}
