use crate::api;

pub fn get_status() {
    let res = api::status::fetch_server_status();
    println!("{:?}", res)
}
