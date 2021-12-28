use primeless_year::*;
//extern crate primeless_year;

fn main() {
    let mds = GregorianCalender::CommonYear.get_mds();
    println!("{:?}", mds);
}
