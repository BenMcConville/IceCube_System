pub mod dom;
use super::bca_app::dom_app;
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
    pub fn init_bca_app(&self) -> Vec<dom_app::DomApp> {
        let mut temp_list: Vec<dom_app::DomApp> = vec![];
        for (index, current_dom) in self.dom_list.iter().enumerate() {
            if index == 0 {
                temp_list.push(dom_app::DomApp::new(current_dom.get_id(), false, true));
            } else {
                temp_list.push(dom_app::DomApp::new(current_dom.get_id(), false, false));
            }
        }
        temp_list
    }
    pub fn all_doms_operational(&self) -> bool {
        let mut all_doms_status = true;
        for current_dom in &self.dom_list {
            if !*current_dom.get_status() {
                all_doms_status = false;
            }
        }
        all_doms_status
    }
}
