use std::io::stdin;

#[allow(unused_imports)]
#[macro_use]
extern crate dmoj;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut char_iter = s.trim().chars();
    let initial = char_iter.next().unwrap();
    let chars = char_iter.collect::<Vec<_>>();

    // policy 1
    let ans = if initial == 'D' {
        chars[1..].iter().filter(|&&c| c == 'D').count() * 2 + 1
    } else {
        chars.iter().filter(|&&c| c == 'D').count() * 2
    };
    println!("{ans}");

    // policy 2
    let ans = if initial == 'U' {
        chars[1..].iter().filter(|&&c| c == 'U').count() * 2 + 1
    } else {
        chars.iter().filter(|&&c| c == 'U').count() * 2
    };
    println!("{ans}");

    let mut ans = 0usize;
    let mut prev = initial;
    for &c in &chars {
        if prev != c {
            prev = c;
            ans += 1;
        }
    }
    println!("{ans}");
}
