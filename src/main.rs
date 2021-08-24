use std::env;

use electronic_shop::config;

fn main() {
    let params: Vec<String> = env::args().collect();
    let cfg = config::Config::new(&params);
    let data = config::read_data(cfg);

    println!("ok!: {:?}", data);
}
