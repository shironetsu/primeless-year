use primeless_year::year::*;
use std::fmt::Write;

fn main() {
    for &d in GregorianCalender::LeapYear.get_mmdds() {
        if d % 2 == 0 || d % 5 == 0 {
            continue;
        }
        let mut s = String::new();
        write!(&mut s, "{} :", d);
        let mut d = d;
        let mut factors = vec![];
        for k in 2.. {
            while d % k == 0 {
                d /= k;
                factors.push(k);
            }
            if k * k > d {
                break;
            }
        }
        if d != 1 {
            factors.push(d);
        }
        write!(&mut s, "{:?}", factors);
        if factors.len() > 1 && !factors.contains(&3) {
            println!("{}", s);
        }
    }
}
