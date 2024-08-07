pub mod dom_string;
use super::app::dom_string_app;

pub struct IceCubeField {
    name: String,
    pub dom_strings: Vec<dom_string::DOMString>,
}

impl IceCubeField {
    pub fn init_IceCube_field() -> IceCubeField {
        IceCubeField {
            name: String::from("South-Pole Neutrino Observatory"),
            dom_strings: init_all_strings(10),
        }
    }

    pub fn get_all_dom_strings(&self) -> &Vec<dom_string::DOMString> {
        &self.dom_strings
    }
    pub fn get_all_dom_strings_id(&self) -> Vec<&String> {
        let mut temp_list = vec![];
        for i in &self.dom_strings {
            temp_list.push(i.get_id());
        }
        temp_list
    }

    pub fn init_dom_strings_app(&self) -> Vec<dom_string_app::StringApp> {
        let mut temp_list: Vec<dom_string_app::StringApp> = vec![];
        for (index, current_dom_string) in self.dom_strings.iter().enumerate() {
            if index == 0 {
                temp_list.push(dom_string_app::StringApp::new(
                    current_dom_string.get_id(),
                    current_dom_string.init_bca_app(),
                    true,
                ));
            } else {
                temp_list.push(dom_string_app::StringApp::new(
                    current_dom_string.get_id(),
                    current_dom_string.init_bca_app(),
                    false,
                ));
            }
        }
        temp_list
    }
    pub fn number_of_fully_operational_strings(&self) -> usize {
        let mut fully_operational_strings: usize = 0;
        for current_string in &self.dom_strings {
            if current_string.all_bcas_operational() {
                fully_operational_strings += 1;
            }
        }
        fully_operational_strings
    }
}

fn init_all_strings(number_of_dom_strings: u32) -> Vec<dom_string::DOMString> {
    let mut init_dom_string: Vec<dom_string::DOMString> = vec![];
    for i in 0..number_of_dom_strings {
        init_dom_string.push(init_string_n(i))
    }
    init_dom_string
}

fn init_string_n(init_nth_dom_string: u32) -> dom_string::DOMString {
    let current_dom_string_id = String::from("String_") + &init_nth_dom_string.to_string();
    let mut st = dom_string::DOMString::new_string(&current_dom_string_id);
    for i in 0..21 {
        let bca_name = String::from("BCA-") + &(i + 1).to_string();
        let mut bca = dom_string::bca::BCA::new_bca(&String::from(&bca_name));
        for j in 1..5 {
            let dom_name = String::from("DOM-") + &((4 * i) + j).to_string();
            let dom = dom_string::bca::dom::DOM::new_dom(&dom_name);
            bca.dom_list.push(dom);
        }
        st.bca_list.push(bca);
    }
    st
}
