fn read_line() -> String {
    let mut s = String::new();
    ::std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let case_num = read_line().trim().parse::<usize>().unwrap();
    for _ in 0..case_num {
        let n = read_line().trim().parse::<usize>().unwrap();
        for _ in 0..n {
            let mut line = read_line().split_whitespace();
        }
    }
}