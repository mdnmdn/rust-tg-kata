use itertools::sorted;


// https://www.hackerrank.com/challenges/electronics-shop/problem
pub fn calculate_best_deal(amount_owned: u32, keyboard_prices: &[u32], usb_prices: &[u32]) -> u32 {

    let usb_prices: Vec<u32> = sorted(usb_prices.iter().filter(|&&i| i < amount_owned)).copied().collect();

    // handle no keyboards
    if keyboard_prices.is_empty() {
        return usb_prices.into_iter().max().unwrap_or(0);
    }

    let keyboard_prices: Vec<u32> = sorted(keyboard_prices.iter().filter(|&&i| i < amount_owned)).copied().collect();

    // handle no usb
    if usb_prices.is_empty() {
        return keyboard_prices.into_iter().max().unwrap_or(0);
    }


    let mut actual_max = 0;
    let mut usb_pos = usb_prices.len() - 1;

    //'external_loop: for keyb_price in keyboard_prices.into_iter() {
    'external_loop: for keyb_price in keyboard_prices {
        loop {
            let usb_price = usb_prices[usb_pos];

            let combination_cost = usb_price + keyb_price;

            if combination_cost > amount_owned {
                break;
            }
            if combination_cost > actual_max {
                actual_max = combination_cost;
            } else {
                if usb_pos == 0 {
                    break 'external_loop;
                }
                usb_pos -= 1;
            }
        }
    }

    actual_max
}
