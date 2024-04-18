#[allow(unused_imports)]
#[macro_use]
extern crate dmoj;

fn main() {
    let n = scan!(usize);
    let h = (0..n).map(|_| scan!(usize)).collect::<Vec<_>>();
    let mut arrows = (0..1000000).map(|_| 0).collect::<Vec<_>>();

    let mut arrow_count = 0usize;
    for hi in h {
        if arrows.get(hi).copied().unwrap_or_default() > 0 {
            arrows[hi] -= 1;
        } else {
            arrow_count += 1;
        }
        arrows[hi - 1] += 1;
    }
    println!("{arrow_count}");
}
