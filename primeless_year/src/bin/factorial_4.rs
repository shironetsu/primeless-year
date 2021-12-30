use primeless_year::integer::*;
use primeless_year::year::*;
use primeless_year::*;
use rug::{Complete, Integer};

fn main() {
    let y = GregorianCalender::CommonYear
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
    println!("{}", format_exp10!(y));
}
