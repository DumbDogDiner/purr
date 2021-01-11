use crate::cfg;

pub fn get_config() {
    let settings = cfg::get_config();
    println!("{:?}", settings);
}

pub fn update_config() {}
