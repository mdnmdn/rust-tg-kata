use std::fs;
use std::io::{stdin, Error, Read};

pub struct Config {
    pub filename: Option<String>,
}

impl Config {
    pub fn new(params: &[String]) -> Config {
        Config {
            filename: if params.len() >= 2 {
                Some(params[1].clone())
            } else {
                None
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ShopData {
    pub amount: u32,
    pub usb_prices: Vec<u32>,
    pub keyboard_prices: Vec<u32>,
}

impl ShopData {
    pub fn from_string_config(config: &str) -> Result<Self, String> {
        let lines: Vec<&str> = config.trim().lines().collect();
        if lines.len() < 3 {
            return Err(format!("Expected 3 lines instead of {}", lines.len()));
        }

        let first_line = parse_u32_list(lines[0])?;

        if first_line.is_empty() {
            return Err(format!("No amount found in first line: '{}'", lines[0]));
        }

        let result = ShopData {
            amount: first_line[0],
            usb_prices: parse_u32_list(lines[1])?,
            keyboard_prices: parse_u32_list(lines[2])?,
        };

        Ok(result)
    }
}

pub fn read_data(config: Config) -> Result<String, Error> {
    if let Some(filename) = config.filename {
        fs::read_to_string(filename)
    } else {
        let mut data = String::new();
        let result = stdin().read_to_string(&mut data);
        match result {
            Ok(_len) => Ok(data),
            Err(err) => Err(err),
        }
    }
}

fn parse_u32_list(number_row: &str) -> Result<Vec<u32>, String> {
    let mut result: Vec<u32> = Vec::new();

    for str_number in number_row.split(' ') {
        let parsed_number = str_number.parse::<u32>();
        if let Ok(n) = parsed_number {
            result.push(n);
        } else if parsed_number.is_err() {
            return Err(format!("Cannot parse '{}' as int", str_number));
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_error_if_not_3_lines() {
        let txt = "r1
r2";
        let res = ShopData::from_string_config(txt);
        assert_eq!(Err("Expected 3 lines instead of 2".to_string()), res);
    }

    #[test]
    fn parse_ok_if_3_lines_or_more() {
        let txt = "123 1
2
23";

        let res = ShopData::from_string_config(txt);
        assert!(res.is_ok());

        let txt = "1
2 99 11 5
3 2 4 5
4";

        let res = ShopData::from_string_config(txt);
        dbg!(&res);
        assert!(res.is_ok());
    }

    #[test]
    fn correct_parse_number_list() {
        let data = "10 3 99 23 12 199";
        let result = parse_u32_list(&data);
        assert_eq!(Ok(vec![10_u32, 3, 99, 23, 12, 199]), result);
    }

    #[test]
    fn error_parsing_number_list() {
        let data = "10 3 99 aaaa 12 199";
        let result = parse_u32_list(&data);
        assert_eq!(Err("Cannot parse 'aaaa' as int".to_owned()), result);
    }
}
