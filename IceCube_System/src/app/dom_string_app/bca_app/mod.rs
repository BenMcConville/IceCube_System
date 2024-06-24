pub mod dom_app;

pub struct BcaApp {
    selected_bca: bool,
    bca_id: String,
    bca_status: bool,
    bca_doms: Vec<dom_app::DomApp>,
}
impl BcaApp {
    pub fn new(
        new_bca_id: &String,
        new_bca_status: bool,
        dom_list: Vec<dom_app::DomApp>,
    ) -> BcaApp {
        BcaApp {
            selected_bca: false,
            bca_id: String::from(new_bca_id),
            bca_status: new_bca_status,
            bca_doms: dom_list,
        }
    }
}
