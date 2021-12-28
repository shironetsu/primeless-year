use self::year::*;
use rug::{integer::IsPrime, Integer};

pub mod integer;
pub mod year;

pub trait GetPrimeMmdds {
    fn get_prime_mmdds(&self) -> Vec<i32>;
}

impl GetPrimeMmdds for Integer {
    fn get_prime_mmdds(&self) -> Vec<i32> {
        self.get_mmdds()
            .iter()
            .filter(|&&mmdd| {
                let n = Integer::from(self * 10000) + Integer::from(mmdd);
                n.is_probably_prime(100) != IsPrime::No
            })
            .cloned()
            .collect()
    }
}

macro_rules! impl_get_mmdds {
    ($int: ty) => {
        impl GetPrimeMmdds for $int {
            fn get_prime_mmdds(&self) -> Vec<i32> {
                self.get_mmdds()
                    .iter()
                    .filter(|&&mmdd| {
                        let n = self * (10000 as $int) + (mmdd as $int);
                        Integer::from(n).is_probably_prime(100) != IsPrime::No
                    })
                    .cloned()
                    .collect()
            }
        }
    };
}

impl_get_mmdds!(i128);
impl_get_mmdds!(i64);
impl_get_mmdds!(i32);
