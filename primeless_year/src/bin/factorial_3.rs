use primeless_year::integer::ToExp10;
use primeless_year::year::{GetMmdds, GregorianCalender};
use rug::Integer;

fn main() {
    let a = GregorianCalender::CommonYear
        .get_mmdds()
        .iter()
        .filter(|&x| x % 2 != 0 && x % 5 != 0)
        .fold(Integer::from(1), |acc, &x| acc * Integer::from(x));
    let (sig, exp) = a.to_exp10();
    println!("{} * 10^{}", sig, exp);
}
