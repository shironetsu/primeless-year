use primeless_year::integer::*;
use rug::{Complete, Integer};

fn main() {
    let f = Integer::factorial(1231).complete();
    let (sig, exp) = f.to_exp10();
    println!("{} * 10^{}", sig, exp);
}
