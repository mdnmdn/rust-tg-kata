pub mod config;

use config::ShopData;

pub fn calculate_best_deal_from_shop_config(config: &ShopData) -> Result<u32, String> {
    calculate_best_deal(config.amount, &config.keyboard_prices, &config.usb_prices)
}

// https://www.hackerrank.com/challenges/electronics-shop/problem
pub fn calculate_best_deal(
    amount_owned: u32,
    keyboard_prices: &[u32],
    usb_prices: &[u32],
) -> Result<u32, String> {
    // handle no keyboards
    if keyboard_prices.is_empty() {
        return Err("Missing keyboards".to_string());
    }

    // handle no usb
    if usb_prices.is_empty() {
        return Err("Missing usbs".to_string());
    }

    let mut usb_prices: Vec<u32> = usb_prices
        .iter()
        .filter(|&&i| i < amount_owned)
        .copied()
        .collect();
    usb_prices.sort_unstable();

    let mut keyboard_prices: Vec<u32> = keyboard_prices
        .iter()
        .filter(|&&i| i < amount_owned)
        .copied()
        .collect();
    keyboard_prices.sort_unstable();

    let mut actual_max = 0;

    println!("k: {:?}", keyboard_prices);
    println!("u: {:?}", usb_prices);

    if !keyboard_prices.is_empty() && !usb_prices.is_empty() {
        let mut usb_pos = usb_prices.len() - 1;

        'external_loop: for keyb_price in keyboard_prices {
            
            loop {
                let usb_price = usb_prices[usb_pos];
                
                let combination_cost = usb_price + keyb_price;
                
                //println!("---\nk: {}, u: {}, max: {}, cost: {}", keyb_price, usb_price, actual_max, combination_cost);
              
                if combination_cost <= amount_owned {
                  if actual_max < combination_cost {
                    actual_max = combination_cost;
                    break;
                  }
                }

                if usb_pos == 0 {
                    break 'external_loop;
                }
                usb_pos -= 1;
                
            }
        }
    }

    if actual_max > 0 {
        Ok(actual_max)
    } else {
        Err("No combination found".to_string())
    }
}
