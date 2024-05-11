use std::cmp::Reverse;

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
    loop {
        let n = scan!(usize);
        if n == 0 {
            break;
        }
        let data = (0..n)
            .map(|_| (scan!(u32), (scan!(f64) * 100.).round() as i32))
            .collect::<Vec<_>>();
        let mut s = 0;
        let sum1 = data
            .iter()
            .map(|&(_, p)| {
                let tmp = s;
                s = tmp + p;
                tmp
            })
            .collect::<Vec<_>>();
        let sum2 = {
            s = 0;
            let mut tmp = data
                .iter()
                .rev()
                .map(|&(_, p)| {
                    let tmp = s;
                    s = tmp + p;
                    tmp
                })
                .collect::<Vec<_>>();
            tmp.reverse();
            tmp
        };
        let sum = s;
        let mut ans = (0, 0u32, 0u32);
        for o in 0..n {
            let p1 = sum - sum1[o];
            for c in o..n {
                let p = p1 - sum2[c] - 8 * (data[c].0 - data[o].0 + 1) as i32;
                ans = std::cmp::max_by_key(ans, (p, data[o].0, data[c].0), |&(p, o, c)| (p, Reverse(c - o), Reverse(o)));
            }
        }
        if ans.0 != 0 {
            println!("{:.2} {} {}", ans.0 as f64 / 100., ans.1, ans.2);
        } else {
            println!("no profit");
        }
    }
}
