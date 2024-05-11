use std::collections::HashSet;

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
    let (n, m) = scan!(usize, usize);
    let l = (0..n).map(|_| {let ki = scan!(usize); (0..ki).map(|_| scan!(u16)).collect::<HashSet<_>>()}).collect::<Vec<_>>();
    let notes = (0..m).map(|_| scan!(u16)).collect::<Vec<_>>();
    let mut flg = vec![true; n];
    let mut i = 0;
    let mut ans = 0;
    while i < m {
        for (keyboard, flg) in l.iter().zip(&mut flg) {
            if *flg && !keyboard.contains(&notes[i]) {
                *flg = false;
            }
        }
        if flg.iter().all(|&x| !x) {
            ans += 1;
            flg.iter_mut().for_each(|x| *x = true);
        } else {
            i += 1;
        }
    }
    println!("{ans}");
}
