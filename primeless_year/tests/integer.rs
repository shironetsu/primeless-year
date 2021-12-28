use primeless_year::integer::*;
use rstest::*;
use rug::{Complete, Integer};

#[rstest]
#[case(Integer::from(123456789), (1.23456789, 8))]
#[case(Integer::from(-123456789), (-1.23456789, 8))]
#[case(Integer::from(0), (0.0, 0))]
#[case(Integer::from(7), (7.0, 0))]
#[case(Integer::from(-2), (-2.0, 0))]
#[case(Integer::parse("314159265358979323846").unwrap().complete(), (3.14159265358979323846, 20))]
#[case(Integer::parse("-314159265358979323846").unwrap().complete(), (-3.14159265358979323846, 20))]
fn to_exp10_works(#[case] input: Integer, #[case] expected: (f64, u32)) {
    assert_eq!(input.to_exp10(), expected);
}
