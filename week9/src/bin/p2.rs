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
    let n = scan!(usize);
    let mut platforms = (0..n).map(|_| scan!(u32, u32, u32)).collect::<Vec<_>>();
    platforms.sort_unstable_by_key(|&(h, _, _)| h);
    let mut sum = 0;
    for i in 0..n {
        let (h, l, r) = platforms[i];
        let max_l = platforms[..i]
            .iter()
            .rev()
            .find_map(|&(h2, l2, r2)| {
                if (l2..r2).contains(&l) {
                    Some(h2)
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let max_r = platforms[..i]
            .iter()
            .rev()
            .find_map(|&(h2, l2, r2)| {
                if (l2 + 1..=r2).contains(&r) {
                    Some(h2)
                } else {
                    None
                }
            })
            .unwrap_or_default();
        // println!("{} {} {}", max_l, h, max_r);
        sum += h - max_l + h - max_r;
    }
    println!("{}", sum);
}
