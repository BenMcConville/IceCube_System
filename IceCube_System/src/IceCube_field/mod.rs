pub mod dom_string;

pub struct IceCubeField {
    name: String,
    dom_strings: Vec<dom_string::DOMString>,
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
