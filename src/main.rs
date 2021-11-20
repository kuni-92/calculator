mod sub;
use crate::sub::calc;

fn main() {
    let x = 10;
    let y = 20;
    let add_res: i32 = calc::add(&x, &y);
    let rem_res: i32 = calc::remove(&x, &y);
    println!("calc add result: {}", add_res);
    println!("calc remove result: {}", rem_res);

}
