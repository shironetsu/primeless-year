use primeless_year::integer::*;
use primeless_year::*;
use rug::{Complete, Integer};

fn main() {
    let y = Integer::factorial(1231).complete();
    println!("{}", format_exp10!(y));
}
