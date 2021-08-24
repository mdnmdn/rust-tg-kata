#[cfg(test)]
mod tests {
    use electronic_shop::*;

    #[test]
    fn test_simple_deal() {
        let keyboard_prices = [3, 1];
        let usb_prices = [5, 2, 8];
        let max_amount = 10;

        let total = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices).unwrap();
        assert_eq!(9, total);
    }

    #[test]
    fn test_simple_deal_reverse_order() {
        let keyboard_prices = [8, 3, 2];
        let usb_prices = [9, 5, 2, 3, 1];
        let max_amount = 10;

        let total = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices).unwrap();
        assert_eq!(10, total);
    }

    #[test]
    fn no_usb() {
        let keyboard_prices = [5, 2, 8];
        let usb_prices = [];
        let max_amount = 10;

        let err = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices).unwrap_err();
        assert_eq!("Missing usbs", err);
    }

    #[test]
    fn no_keyboard() {
        let keyboard_prices = [];
        let usb_prices = [5, 2, 8];
        let max_amount = 10;

        let err = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices).unwrap_err();
        assert_eq!("Missing keyboards", err);
    }

    #[test]
    fn no_amount() {
        let keyboard_prices = [5, 1, 11, 99];
        let usb_prices = [5, 2, 8];
        let max_amount = 0;

        let err = calculate_best_deal(max_amount, &keyboard_prices, &usb_prices).unwrap_err();
        assert_eq!("No combination found".to_string(), err);
    }
}
