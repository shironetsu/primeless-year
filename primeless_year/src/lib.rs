use self::year::*;
use rug::{integer::IsPrime, Integer};
use std::cmp::PartialEq;

pub mod integer;
pub mod year;

// pub struct PrimeMmddIterator{
//     todo!();
// }

// impl Iterator for PrimeMmddIterator{
//     type Item = i32;
//     fn next(&mut self)->Option<Self::Item>{
//          todo!();
//     }
// }

pub trait IsProbablyPrimelessYear {
    fn is_probably_primeless_year(&self) -> IsPrimelessYear;
}

#[derive(PartialEq, Eq)]
pub enum IsPrimelessYear {
    Yes,
    No,
    ProbablyNo,
}

impl IsProbablyPrimelessYear for Integer {
    fn is_probably_primeless_year(&self) -> IsPrimelessYear {
        self.get_mmdds()
            .iter()
            .find_map(|&mmdd| {
                let n = Integer::from(self * 10000) + Integer::from(mmdd);
                match n.is_probably_prime(100) {
                    IsPrime::No => None,
                    IsPrime::Probably => Some(IsPrime::Probably),
                    IsPrime::Yes => Some(IsPrime::Yes),
                }
            })
            .map_or_else(
                || IsPrimelessYear::Yes,
                |is_prime| match is_prime {
                    IsPrime::Yes => IsPrimelessYear::No,
                    IsPrime::Probably => IsPrimelessYear::ProbablyNo,
                    _ => unreachable!(),
                },
            )
    }
}

macro_rules! impl_is_primeless_year {
    ($int: ty) => {
        impl IsProbablyPrimelessYear for $int {
            fn is_probably_primeless_year(&self) -> IsPrimelessYear {
                self.get_mmdds()
                    .iter()
                    .find_map(|&mmdd| {
                        let n = Integer::from(self * (10000 as $int) + (mmdd as $int));
                        match n.is_probably_prime(100) {
                            IsPrime::No => None,
                            IsPrime::Probably => Some(IsPrime::Probably),
                            IsPrime::Yes => Some(IsPrime::Yes),
                        }
                    })
                    .map_or_else(
                        || IsPrimelessYear::Yes,
                        |is_prime| match is_prime {
                            IsPrime::Yes => IsPrimelessYear::No,
                            IsPrime::Probably => IsPrimelessYear::ProbablyNo,
                            _ => unreachable!(),
                        },
                    )
            }
        }
    };
}

impl_is_primeless_year!(i128);
impl_is_primeless_year!(i64);
impl_is_primeless_year!(i32);

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
