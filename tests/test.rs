#[cfg(test)]
mod tests {
    use kata01_electronic_shop::*;

    #[test]
    fn test_simple_deal() {
        let keyboard_prices = [3, 1];
        let usb_prices = [5, 2, 8];
        let max_amount = 10;

        let total = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices);
        assert_eq!(9, total);
    }

    #[test]
    fn no_usb() {
        let keyboard_prices = [5, 2, 8];
        let usb_prices = [];
        let max_amount = 10;

        let total = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices);
        assert_eq!(8, total);
    }

    #[test]
    fn no_keyboard() {
        let keyboard_prices = [];
        let usb_prices = [5, 2, 8];
        let max_amount = 10;

        let total = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices);
        assert_eq!(8, total);
    }

    #[test]
    fn no_keyboard_usb() {
        let keyboard_prices = [];
        let usb_prices = [];
        let max_amount = 10;

        let total = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices);
        assert_eq!(0, total);
    }

    #[test]
    fn no_amount() {
        let keyboard_prices = [5, 1, 11, 99];
        let usb_prices = [5, 2, 8];
        let max_amount = 0;

        let total = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices);
        assert_eq!(0, total);
    }
}
