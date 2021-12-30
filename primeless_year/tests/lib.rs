use primeless_year::*;
use rstest::*;
use rug::Integer;

const PRIME_MMDDS_2021: [i32; 21] = [
    0101, 0107, 0131, 0317, 0401, 0503, 0513, 0609, 0717, 0803, 0831, 0923, 0929, 1001, 1007, 1031,
    1109, 1127, 1203, 1209, 1221,
];

const PRIME_MMDDS_2022: [i32; 23] = [
    0103, 0119, 0121, 0127, 0217, 0307, 0311, 0323, 0331, 0407, 0517, 0601, 0619, 0713, 0817, 0821,
    0901, 0923, 1009, 1027, 1127, 1213, 1231,
];

const PRIME_MMDDS_2024: [i32; 21] = [
    0107, 0219, 0323, 0327, 0411, 0419, 0531, 0603, 0611, 0723, 0729, 0807, 0819, 0821, 0903, 1017,
    1029, 1119, 1121, 1211, 1229,
];

#[rstest]
#[case(2021, &PRIME_MMDDS_2021)]
#[case(2022, &PRIME_MMDDS_2022)]
#[case(2024, &PRIME_MMDDS_2024)]
fn get_prime_mmdds_test_i32(#[case] input: i32, #[case] expected: &[i32]) {
    let mmdds = input.get_prime_mmdds();
    assert_eq!(&mmdds, expected);
}

#[rstest]
#[case(Integer::from(2021), &PRIME_MMDDS_2021)]
#[case(Integer::from(2022), &PRIME_MMDDS_2022)]
#[case(Integer::from(2024), &PRIME_MMDDS_2024)]
fn get_prime_mmdds_test_rug_integer(#[case] input: Integer, #[case] expected: &[i32]) {
    let mmdds = input.get_prime_mmdds();
    assert_eq!(&mmdds, expected);
}
