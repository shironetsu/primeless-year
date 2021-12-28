use primeless_year::integer::ToExp10;
use primeless_year::year::{GetMmdds, GregorianCalender};
use primeless_year::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rug::{Complete, Integer};

const R: f32 = 0.08;

fn main() {
    let mut mmdds = GregorianCalender::CommonYear
        .get_mmdds()
        .iter()
        .filter(|&&d| d % 2 != 0 && d % 5 != 0)
        .cloned()
        .collect::<Vec<_>>();
    let mut rng = thread_rng();

    let a = loop {
        mmdds.shuffle(&mut rng);
        let a = mmdds
            .iter()
            .take((R * (mmdds.len() as f32).floor()) as usize)
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
        println!("{}", a.get_prime_mmdds().len());

        if a.get_prime_mmdds().len() == 0 {
            break a;
        }
    };

    let (sig, exp) = a.to_exp10();
    println!("{} * 10^{}", sig, exp);
}
