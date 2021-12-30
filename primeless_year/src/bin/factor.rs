use primeless_year::year::*;

fn main() {
    let y = 13446204i64;
    for &mmdd in y.get_mmdds() {
        let n = 10000 * y + mmdd as i64;
        let mut factors = vec![];
        let mut m = n;
        for k in 2.. {
            while m % k == 0 {
                m /= k;
                factors.push(k);
            }
            if k * k > m {
                if m > 1 {
                    factors.push(m);
                }
                break;
            }
        }
        println!(
            "{}={}",
            n,
            factors
                .iter()
                .map(|k| k.to_string())
                .collect::<Vec<_>>()
                .join("*")
        );
    }
}
