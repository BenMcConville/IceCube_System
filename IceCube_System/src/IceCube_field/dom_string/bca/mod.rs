pub mod dom;
// Breakout Cable Assembly (BCA)

pub struct BCA {
    bca_id: String,
    pub dom_list: Vec<dom::DOM>,
}

impl BCA {
    pub fn new_bca(new_bca_id: &String) -> BCA {
        BCA {
            bca_id: String::from(new_bca_id),
            dom_list: vec![],
        }
    }
    pub fn get_id(&self) -> &String {
        &self.bca_id
    }
    pub fn get_dom_ids(&self) -> Vec<&String> {
        let mut dom_id_list: Vec<&String> = vec![];
        for current_bca in self.dom_list.iter() {
            dom_id_list.push(current_bca.get_id());
        }
        dom_id_list
    }
}
