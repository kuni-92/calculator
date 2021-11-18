mod sub;
use crate::sub::calc;

fn main() {
    let x = 10;
    let y = 20;
    let res: i32 = calc::add(&x, &y);
    println!("calc add result: {}",res);
}
