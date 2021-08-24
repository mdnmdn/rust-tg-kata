use std::env;
use std::process;

use electronic_shop::{calculate_best_deal_from_shop_config, config};

fn main() {
    let params: Vec<String> = env::args().collect();
    let cfg = config::Config::new(&params);
    let data = config::read_data(cfg).unwrap_or_else(|err| {
        eprintln!("Error reading data: {}", err);
        process::exit(1);
    });

    let shop_data = config::ShopData::from_string_config(&data).unwrap_or_else(|err| {
        eprintln!("Error parsing data: {}", err);
        process::exit(1);
    });

    println!("Amount: {}", shop_data.amount);
    println!("USB prices:   {:?}", shop_data.usb_prices);
    println!("Keyb. prices: {:?}", shop_data.keyboard_prices);

    let result = calculate_best_deal_from_shop_config(&shop_data).unwrap_or_else(|err| {
        eprintln!("Error calculating best deal: {}", err);
        process::exit(1);
    });
    println!("Best deal: {}", result);
}
