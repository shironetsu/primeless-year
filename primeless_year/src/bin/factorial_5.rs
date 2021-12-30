use primeless_year::integer::*;
use primeless_year::year::*;
use primeless_year::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rug::{ops::Pow, Complete, Integer};

const R: f32 = 0.08;

fn main() {
    let mut mmdds = GregorianCalender::CommonYear
        .get_mmdds()
        .iter()
        .filter(|&&d| d % 2 != 0 && d % 5 != 0)
        .cloned()
        .collect::<Vec<_>>();
    let mut rng = thread_rng();

    let mut y_min = Integer::from(10).pow(30);

    for _ in 0..100 {
        let y = loop {
            mmdds.shuffle(&mut rng);
            let y = mmdds
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

            if y.is_probably_primeless_year() == IsPrimelessYear::Yes {
                break y;
            }
        };
        if y < y_min {
            y_min = y;
            println!("{}", y_min);
            println!("{}", format_exp10!(y_min));
        }
    }
}
