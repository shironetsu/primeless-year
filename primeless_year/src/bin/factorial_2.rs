use primeless_year::integer::ToExp10;
use primeless_year::year::{GetMmdds, GregorianCalender};
use rug::Integer;

fn main() {
    let a = GregorianCalender::LeapYear
        .get_mmdds()
        .iter()
        .fold(Integer::from(1), |acc, &x| acc * Integer::from(x));
    let (sig, exp) = a.to_exp10();
    println!("{} * 10^{}", sig, exp);
}
