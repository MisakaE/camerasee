use cae::{get_file::{get_all, parameters}, ui};
fn main() {
    let para = parameters().unwrap();
    if !para.0{

    } else {
        let all = get_all(para.1);
        ui::mainpage::body(all);
    }
}