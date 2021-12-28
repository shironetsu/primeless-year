use primeless_year::year::*;

fn main() {
    let mds = GregorianCalender::CommonYear.get_mmdds();
    println!("{:?}", mds);
}
