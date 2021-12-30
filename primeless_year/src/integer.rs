use rug::{Complete, Integer};

pub trait ToExp10 {
    fn to_exp10(&self) -> (f64, u32);
}

impl ToExp10 for Integer {
    fn to_exp10(&self) -> (f64, u32) {
        if self == &0 {
            (0.0, 0)
        } else {
            let sign = self.signum_ref().complete().to_f64();
            let mut s = self.abs_ref().complete().to_string_radix(10);
            let exponent = s.len() - 1;
            let significand = if exponent == 0 {
                self.to_f64()
            } else {
                s.insert(1, '.');
                sign * s.parse::<f64>().unwrap()
            };

            (significand, exponent as u32)
        }
    }
}

#[macro_export]
macro_rules! format_exp10 {
    ($i:expr) => {{
        let (sig, exp) = $i.to_exp10();
        format!("{} * 10^{}", sig, exp)
    }};
}
