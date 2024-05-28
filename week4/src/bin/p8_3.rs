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
    if n == 0 {
        println!("0");
        return;
    }
    let w = (0..n).map(|_| scan!(u16)).collect::<Vec<_>>();
    let mut dp1 = [0u16; 10000];
    for &wi in w.iter().rev() {
        let update = dp1[..wi as usize].iter().copied().max().unwrap_or(0) + 1;
        dp1[wi as usize] = update;
    }

    let mut dp2 = [0u16; 10000];
    for &wi in w.iter().rev() {
        let update = dp2
            .get((wi + 1) as usize..)
            .map(|slc| slc.iter().copied().max().unwrap_or(0))
            .unwrap_or(0)
            + 1;
        dp2[wi as usize] = update;
    }

    let ans = dp1.into_iter().zip(dp2).map(|(a, b)| a + b).max().unwrap() - 1;
    println!("{}", ans);
}
