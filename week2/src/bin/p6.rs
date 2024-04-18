#[allow(unused_imports)]
#[macro_use]
extern crate dmoj;

fn read_line() -> String {
    let mut s = String::new();
    ::std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let mut n = String::leak(read_line()).trim().to_owned();
    if n.as_str() == "0" {
        println!("0");
        return;
    }
    let m = String::leak(read_line()).trim();
    let l = n.len();
    let k = m.as_bytes().len() - 1;

    for _ in 0..k {
        if n.ends_with('0') {
            n.pop();
        } else {
            break;
        }
    }

    if l > k {
        let (n1, n2) = n.split_at(l - k);
        if n2.is_empty() {
            println!("{n1}");
        } else {
            println!("{n1}.{n2}");
        }
    } else {
        let zeros = k - l;
        println!("0.{}{n}", "0".repeat(zeros));
    }
}
