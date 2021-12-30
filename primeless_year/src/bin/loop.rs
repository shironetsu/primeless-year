use primeless_year::*;
use rug::Integer;

fn main() {
    for y in 0.. {
        let y = Integer::from(y);
        if y.is_probably_primeless_year() == IsPrimelessYear::Yes {
            println!("{}", y);
            break;
        }
    }
}
