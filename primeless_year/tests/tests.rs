use chrono::prelude::*;
use primeless_year::*;
use rstest::*;
use rug::{Complete, Integer};

#[rstest]
#[case(2000, true)]
#[case(2020, true)]
#[case(2021, false)]
#[case(2100, false)]
fn leap_year_test_i32(#[case] input: i32, #[case] expected: bool) {
    assert_eq!(input.is_leap_year(), expected);
}

#[rstest]
#[case("1000000000000000", true)]
#[case("1000000000000001", false)]
#[case("1000000000000004", true)]
#[case("1000000000000100", false)]
fn leap_year_test_rug_integer(#[case] input: &str, #[case] expected: bool) {
    let n = Integer::parse(input).unwrap().complete();
    assert_eq!(n.is_leap_year(), expected);
}

#[rstest]
#[case(GregorianCalender::LeapYear, true)]
#[case(GregorianCalender::CommonYear, false)]
fn contains_leap_day(#[case] input: GregorianCalender, #[case] expected: bool) {
    let mds = input.get_mds();
    assert_eq!(mds.contains(&229), expected);
}

#[rstest]
#[case(GregorianCalender::LeapYear)]
#[case(GregorianCalender::CommonYear)]
fn calender_is_ok(#[case] input: GregorianCalender) {
    let mds = input.get_mds();

    let proxy_year = if input.is_leap_year() { 2020 } else { 2021 };
    let mds_from_chrono = NaiveDate::from_ymd(proxy_year, 1, 1)
        .iter_days()
        .take_while(|&d| d.year() == proxy_year)
        .map(|d| d.format("%m%d").to_string().parse().unwrap())
        .collect::<Vec<i32>>();

    assert_eq!(mds, mds_from_chrono);
}
