use std::fmt::Display;

use anchor_lang::prelude::*;

use crate::errors::CushionError;

pub fn pow(x: u64, y: i16) -> u64 {
    let mut res = 1;
    for i in 0..y.abs() {
        res *= x;
    }

    if y >= 0 {
        res
    } else {
        1 / res
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BigNumber {
    pub value: u64,
    pub exp: u8,
}

impl BigNumber {
    pub fn new(value: u64, exp: u8) -> Self {
        BigNumber { value, exp }
    }

    pub fn new_from_string(text: &String) -> Result<Self> {
        let parts = text.split('.').collect::<Vec<&str>>();
        let (integer, decimals) = if parts.len() > 1 {
            (parts[0], parts[1])
        } else {
            (parts[0], "")
        };

        let parsed_decimals = match decimals.parse::<u64>() {
            Ok(res) => res,
            Err(err) => return err!(CushionError::NumberParsingFailed),
        };
        let parsed_integer = match integer.parse::<u64>() {
            Ok(res) => res * pow(10, decimals.len() as i16) + parsed_decimals,
            Err(err) => return err!(CushionError::NumberParsingFailed),
        };

        Ok(BigNumber {
            value: parsed_integer,
            exp: decimals.len() as u8,
        })
    }

    fn mul(&self, other: Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();

        let self_offset = if self.exp > other.exp {
            b.value *= pow(10, (self.exp - other.exp) as i16);
            0
        } else {
            a.value *= pow(10, (other.exp - self.exp) as i16);
            other.exp - self.exp
        };

        let result = (a.value as u128) * (b.value as u128);

        BigNumber {
            value: (result / pow(10, a.exp as i16) as u128) as u64,
            exp: a.exp - self_offset,
        }
    }
}

impl Display for BigNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut price_str = format!("{}", self.value);
        if self.exp > 0 {
            price_str.insert(price_str.len() - self.exp as usize, '.');
        }

        f.write_str(price_str.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::BigNumber;

    #[test]
    fn test_new() {
        assert_eq!(BigNumber::new(10, 0).value, 10);
        assert_eq!(BigNumber::new(10, 1).value, 10);
    }

    #[test]
    fn test_new_from_string() {
        assert_eq!(
            BigNumber::new_from_string(&"10".to_string()).unwrap(),
            BigNumber::new(10, 0)
        );
        assert_eq!(
            BigNumber::new_from_string(&"10.000".to_string()).unwrap(),
            BigNumber::new(10000, 3)
        );
        assert_eq!(
            BigNumber::new_from_string(&"99.9999".to_string()).unwrap(),
            BigNumber::new(999999, 4)
        );
    }
}
