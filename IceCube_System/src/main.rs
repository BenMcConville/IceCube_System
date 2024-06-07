pub mod dom_string;

fn main() {
    // let st = dom_string::DOMString::new_string(&String::from("test"));
    // println!("{:?}", st.get_bca_ids());
    let string_01 = init_string();
    println!("{:?}", string_01.get_all_dom());
    println!("{:?}", string_01.get_bca_ids());
}

fn init_string() -> dom_string::DOMString {
    let mut st = dom_string::DOMString::new_string(&String::from("String_01"));
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
