pub mod dom_app;

pub struct BcaApp {
    selected_bca: bool,
    bca_id: String,
    bca_status: bool,
}
impl BcaApp {
    pub fn new(new_bca_id: &String, new_bca_status: &bool) -> BcaApp {
        BcaApp {
            selected_bca: false,
            bca_id: String::from(new_bca_id),
            bca_status: *new_bca_status,
        }
    }
}
