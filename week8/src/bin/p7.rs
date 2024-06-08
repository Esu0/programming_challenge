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

fn main() {
    let t = scan!(u8);
    for _ in 0..t {
        let (n, v1, v2, w) = scan!(u8, u8, u8, u8);
        if v2 > n / 2 || v2 == n / 2 && n % 2 == 0 {
            println!("RECOUNT!");
        } else if v1 > n / 2 {
            println!("GET A CRATE OF CHAMPAGNE FROM THE BASEMENT!");
        } else {
            let k = n / 2 + 1 - v1;
            let n = n - v1 - v2;
            let mut prob = 0u128;
            for i in k..=n {
                let mut c = 1u128;
                for j in 1..=i {
                    c = c * (n - j + 1) as u128 / j as u128;
                }
                prob += c;
            }
            // println!("{}", prob as f64 / (1 << n) as f64);
            if prob * 100 > ((w as u128) << n) {
                println!("GET A CRATE OF CHAMPAGNE FROM THE BASEMENT!");
            } else {
                println!("PATIENCE, EVERYONE!");
            }
        }
    }
}
