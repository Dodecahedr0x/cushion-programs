use std::fmt::Display;

use anchor_lang::prelude::*;

#[error_code]
pub enum MathError {
    #[msg("Exponents should be equal")]
    ExponentsDontMatch,

    #[msg("Parsing an invalid number")]
    NumberParsingFailed,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct BigNumber {
    pub value: u64,
    pub exp: u8,
}

impl BigNumber {
    pub fn unit(exp: u8) -> Self {
        Self {
            value: 10_u64.pow(exp as u32),
            exp,
        }
    }

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
            Err(_) => 0,
        };
        let parsed_integer = match integer.parse::<u64>() {
            Ok(res) => res * 10_u64.pow(decimals.len() as u32) + parsed_decimals,
            Err(_) => 0,
        };

        Ok(BigNumber {
            value: parsed_integer,
            exp: decimals.len() as u8,
        })
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();

        let (self_offset, out_exp) = if self.exp > other.exp {
            b.value *= 10_u64.pow((self.exp - other.exp) as u32);
            (0, a.exp)
        } else {
            a.value *= 10_u64.pow((other.exp - self.exp) as u32);
            (other.exp - self.exp, other.exp)
        };

        let result = (a.value as u128) * (b.value as u128);

        BigNumber {
            value: (result / 10_u128.pow((self.exp + self_offset) as u32)) as u64,
            exp: out_exp,
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        let mut a = self.clone();
        let mut b = other.clone();

        let (self_offset, out_exp) = if self.exp > other.exp {
            b.value *= 10_u64.pow((self.exp - other.exp) as u32);
            (0, self.exp)
        } else {
            a.value *= 10_u64.pow((other.exp - self.exp) as u32);
            (other.exp - self.exp, other.exp)
        };

        let result =
            10_u128.pow((self.exp + self_offset) as u32) * (a.value as u128) / (b.value as u128);

        BigNumber {
            value: result as u64,
            exp: out_exp,
        }
    }

    pub fn pow(&self, exponent: i16) -> Self {
        let mut res = Self::unit(self.exp);
        for _ in 0..exponent.abs() {
            res = res.mul(self);
        }

        if exponent >= 0 {
            res
        } else {
            Self::unit(self.exp).div(&res)
        }
    }
}

impl Display for BigNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value_str = format!("{}", self.value);
        let add_leading_zero = value_str.len() <= self.exp as usize;
        let has_decimal_part = self.exp > 0;

        if has_decimal_part {
            if value_str.len().to_owned() < (self.exp as usize) {
                value_str.insert_str(
                    0,
                    format!(
                        ".{}",
                        '0'.to_string().repeat(self.exp as usize - value_str.len())
                    )
                    .as_str(),
                );
            } else {
                value_str.insert(value_str.len() - self.exp as usize, '.')
            };
        }
        if add_leading_zero {
            value_str.insert(0, '0');
        }

        f.write_str(value_str.as_str())
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
    fn test_unit() {
        assert_eq!(format!("{}", BigNumber::unit(0)), "1");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", BigNumber::new(10, 0)), "10");
        assert_eq!(format!("{}", BigNumber::new(10, 1)), "1.0");
        assert_eq!(format!("{}", BigNumber::new(10, 2)), "0.10");
        assert_eq!(format!("{}", BigNumber::new(10, 3)), "0.010");
        assert_eq!(format!("{}", BigNumber::new(10, 4)), "0.0010");
        assert_eq!(format!("{}", BigNumber::new(10, 5)), "0.00010");
        assert_eq!(format!("{}", BigNumber::new(10, 6)), "0.000010");

        assert_eq!(format!("{}", BigNumber::new(999999, 0)), "999999");
        assert_eq!(format!("{}", BigNumber::new(999999, 1)), "99999.9");
        assert_eq!(format!("{}", BigNumber::new(999999, 2)), "9999.99");
        assert_eq!(format!("{}", BigNumber::new(999999, 3)), "999.999");
        assert_eq!(format!("{}", BigNumber::new(999999, 4)), "99.9999");
        assert_eq!(format!("{}", BigNumber::new(999999, 5)), "9.99999");
        assert_eq!(format!("{}", BigNumber::new(999999, 6)), "0.999999");
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

    #[test]
    fn test_mul() {
        assert_eq!(
            format!("{}", BigNumber::new(1000, 3).mul(&BigNumber::new(1000, 3))),
            "1.000"
        );
        assert_eq!(
            format!("{}", BigNumber::new(10000, 3).mul(&BigNumber::new(1000, 3))),
            "10.000"
        );
        assert_eq!(
            format!("{}", BigNumber::new(3000, 3).mul(&BigNumber::new(3333, 3))),
            "9.999"
        );
        assert_eq!(
            format!("{}", BigNumber::unit(3).mul(&BigNumber::new(3000, 0))),
            "3000.000"
        );
        assert_eq!(
            format!("{}", BigNumber::unit(3).mul(&BigNumber::new(3000, 1))),
            "300.000"
        );
        assert_eq!(
            format!("{}", BigNumber::unit(3).mul(&BigNumber::new(3000, 2))),
            "30.000"
        );
        assert_eq!(
            format!("{}", BigNumber::unit(3).mul(&BigNumber::new(3000, 3))),
            "3.000"
        );
        assert_eq!(
            format!("{}", BigNumber::unit(3).mul(&BigNumber::new(3000, 4))),
            "0.3000"
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            format!("{}", BigNumber::new(1000, 3).div(&BigNumber::new(1000, 3))),
            "1.000"
        );
        assert_eq!(
            format!("{}", BigNumber::new(1000, 3).div(&BigNumber::new(10000, 3))),
            "0.100"
        );
        assert_eq!(
            format!("{}", BigNumber::new(10000, 3).div(&BigNumber::new(3000, 3))),
            "3.333"
        );
        println!("{} {}", BigNumber::new(100000, 4), BigNumber::new(3000, 3));
        assert_eq!(
            format!(
                "{}",
                BigNumber::new(100000, 4).div(&BigNumber::new(3000, 3))
            ),
            "3.3333"
        );
        assert_eq!(
            format!(
                "{}",
                BigNumber::new(10000, 3).div(&BigNumber::new(30000, 4))
            ),
            "3.3333"
        );
    }

    #[test]
    fn test_fraction() {
        assert_eq!(
            format!(
                "{}",
                BigNumber::unit(3)
                    .div(&BigNumber::new(2, 0))
                    .mul(&BigNumber::new(3000, 0))
            ),
            "1500.000"
        );
        assert_eq!(
            format!(
                "{}",
                BigNumber::unit(3)
                    .div(&BigNumber::new(3, 0))
                    .mul(&BigNumber::new(3000, 0))
            ),
            "999.000"
        );
        assert_eq!(
            format!(
                "{}",
                BigNumber::unit(3)
                    .div(&BigNumber::new(3, 0))
                    .mul(&BigNumber::new(3000000, 3))
            ),
            "999.000"
        );
        assert_eq!(
            format!(
                "{}",
                BigNumber::unit(3)
                    .mul(&BigNumber::new(1000000, 3))
                    .div(&BigNumber::new(3000000, 6))
            ),
            "333.333333"
        );
    }

    #[test]
    fn test_pow() {
        assert_eq!(format!("{}", BigNumber::new(1000, 3).pow(2)), "1.000");
        assert_eq!(format!("{}", BigNumber::new(2000, 3).pow(2)), "4.000");
        assert_eq!(format!("{}", BigNumber::new(2000, 3).pow(3)), "8.000");

        assert_eq!(format!("{}", BigNumber::new(1000, 3).pow(-2)), "1.000");
        assert_eq!(format!("{}", BigNumber::new(2000, 3).pow(-2)), "0.250");
        assert_eq!(format!("{}", BigNumber::new(2000, 3).pow(-3)), "0.125");

        assert_eq!(format!("{}", BigNumber::new(2000, 3).pow(-3)), "0.125");

        let x = 100;
        assert_eq!(
            format!(
                "{}",
                BigNumber::unit(6)
                    .mul(&BigNumber::new(x / 2, 0))
                    .div(&BigNumber::new(x, 0))
                    .mul(&BigNumber::new(19000000, 6)),
            ),
            "9.500000"
        );
    }
}
