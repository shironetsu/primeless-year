use primeless_year::integer::ToExp10;
use primeless_year::year::{GetMmdds, GregorianCalender};
use rug::{Complete, Integer};

fn main() {
    let a = GregorianCalender::CommonYear
        .get_mmdds()
        .iter()
        .filter(|&d| d % 2 != 0 && d % 5 != 0)
        .fold(Integer::from(1), |acc, &x| {
            let g = acc.gcd_ref(&Integer::from(x)).complete();
            if g == 1 {
                let p = (3..).find(|&y| x % y == 0).unwrap();
                acc * Integer::from(p)
            } else {
                acc
            }
        });
    let (sig, exp) = a.to_exp10();
    println!("{} * 10^{}", sig, exp);
}
