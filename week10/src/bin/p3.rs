mod input {
    use std::{
        cell::RefCell,
        fmt::Debug,
        io::Read,
        str::{FromStr, SplitWhitespace},
    };

    fn tokens_init() -> RefCell<SplitWhitespace<'static>> {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        RefCell::new(String::leak(buf).split_whitespace())
    }

    fn next_token() -> Option<&'static str> {
        thread_local! {
            static TOKENS: RefCell<SplitWhitespace<'static>> = tokens_init();
        }
        TOKENS.with_borrow_mut(|tokens| tokens.next())
    }

    #[allow(dead_code)]
    pub fn scan<T: FromStr>() -> Option<T>
    where
        T::Err: Debug,
    {
        next_token().map(|s| s.parse().unwrap())
    }

    #[macro_export]
    macro_rules! scan {
        ($t:ty $(,)?) => {
            $crate::input::scan::<$t>().unwrap()
        };
        ($($t:ty),+ $(,)?) => {
            ($($crate::input::scan::<$t>().unwrap()),*)
        };
    }
}

fn gcd_ext(a: u64, b: u64) -> (i64, i64, i64) {
    if b == 0 {
        (a as i64, 1, 0)
    } else {
        // b * x + (a % b) * y = gcd
        let (gcd, x, y) = gcd_ext(b, a % b);
        let d = a / b;
        // b * x - d * b * y + (a % b) * y + d * b * y = b * (x - d * y) + a * y = gcd
        (gcd, y, x - d as i64 * y)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    gcd_ext(a, b).0 as u64
}

fn main() {
    let n = scan!(usize);
    let xyr = (0..n).map(|_| scan!(i64, i64, u64)).collect::<Vec<_>>();
    let mut marker = vec![0i8; n];
    let mut stack = vec![0];
    let mut is_bipartite = true;
    let mut visited = vec![false; n];
    marker[0] = 1;
    while let Some(i) = stack.pop() {
        if visited[i] {
            continue;
        }
        visited[i] = true;
        let (xi, yi, ri) = xyr[i];
        for (j, &(xj, yj, rj)) in xyr.iter().enumerate() {
            if i == j {
                continue;
            }
            let dx = xj - xi;
            let dy = yj - yi;
            let d = (dx * dx + dy * dy) as u64;
            if d == (ri + rj).pow(2) {
                if !visited[j] {
                    stack.push(j);
                }
                if marker[j] == 0 {
                    marker[j] = -marker[i];
                } else if marker[j] != -marker[i] {
                    is_bipartite = false;
                }
            }
        }
    }

    if visited[n - 1] {
        if is_bipartite {
            let a0 = xyr[n - 1].2;
            let b0 = xyr[0].2;
            let g = gcd(a0, b0);
            let a = a0 / g;
            let b = if marker[n - 1] == 1 {
                (b0 / g) as i64
            } else if marker[n - 1] == -1 {
                -((b0 / g) as i64)
            } else {
                unreachable!();
            };
            println!("{} {}", a, b);
        } else {
            println!("-1");
        }
    } else {
        println!("0");
    }
}
