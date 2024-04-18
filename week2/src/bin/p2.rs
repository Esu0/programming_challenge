use std::io::stdin;

#[allow(unused_imports)]
#[macro_use]
extern crate dmoj;

fn read_line() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    loop {
        let n = read_line().trim().parse::<usize>().unwrap();
        if n == 0 {
            return;
        }
        let mut names = (0..n)
            .map(|_| String::leak(read_line()).trim())
            .collect::<Vec<_>>();

        names.sort_by_key(|name| {
            let bytes = name.as_bytes();
            (bytes[0], bytes[1])
        });

        for name in names {
            println!("{name}");
        }
        println!();
    }
}
