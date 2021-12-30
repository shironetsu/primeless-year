use primeless_year::integer::*;
use primeless_year::year::*;
use primeless_year::*;
use rug::Integer;

fn main() {
    let y = GregorianCalender::CommonYear
        .get_mmdds()
        .iter()
        .filter(|&x| x % 2 != 0 && x % 5 != 0)
        .fold(Integer::from(1), |acc, &x| acc * Integer::from(x));
    println!("{}", format_exp10!(y));
}
