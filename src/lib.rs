use itertools::sorted;


fn get_max_with_limit(items: &[u32], limit: u32) -> u32 {
    items.iter().cloned().filter(|i| i < &limit).max().unwrap_or(0)
}

//fn get_max_with_limit(items: &[u32], limit: u32) -> u32 {
//    items.iter().fold(0, |max, val: &u32| -> u32 {
//        let val = *val;
//        if (val > max) && (val < limit) {
//            val
//        } else {
//            max
//        }
//    })
//}

// https://www.hackerrank.com/challenges/electronics-shop/problem
pub fn calculate_best_deal(amount_owned: u32, keyboard_prices: &[u32], usb_prices: &[u32]) -> u32 {

    // handle no keyboards
    if keyboard_prices.is_empty() {
        return get_max_with_limit(usb_prices, amount_owned);
    }
    // handle no usb
    if usb_prices.is_empty() {
        return get_max_with_limit(keyboard_prices, amount_owned);
    }

    let keyboard_prices: Vec<u32> = sorted(keyboard_prices.iter()).cloned().collect();
    let usb_prices: Vec<u32> = sorted(usb_prices.iter()).cloned().collect();

    let mut actual_max = 0;
    let mut usb_pos = usb_prices.len() - 1;

    'external_loop: for keyb_price in keyboard_prices.into_iter() {
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
