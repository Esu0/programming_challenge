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
    let n = scan!(u8);
    for _ in 0..n {
        let (k, w) = scan!(u8, u8);
        let words = (0..w)
            .map(|_| scan!(String).into_bytes())
            .collect::<Vec<_>>();
        let mut ans = 0;
        for wx2 in words.windows(2) {
            let [w1, w2] = wx2 else {
                unreachable!();
            };
            let mut pos = 0;
            while !w2.starts_with(&w1[pos..]) {
                pos += 1;
            }
            ans += pos;
        }
        println!("{}", ans + k as usize);
    }
}
