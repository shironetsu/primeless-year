use primeless_year::*;
use rug::Integer;

fn main() {
    let x = GregorianCalender::LeapYear
        .get_mds()
        .iter()
        .fold(Integer::from(1), |prod, x| prod * x);
    println!("{}", x);
}
