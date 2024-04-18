#[allow(unused_imports)]
#[macro_use]
extern crate dmoj;

fn main() {
    let (n1, n2) = dmoj::scan!(i32, i32);
    let mut d = n2 - n1;
    if d > 180 {
        d -= 360;
    } else if d <= -180 {
        d += 360;
    }
    dmoj::print!("{d}");
}
