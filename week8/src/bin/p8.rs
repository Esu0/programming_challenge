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

fn mpow(mut base: usize, mut exp: usize, m: usize) -> usize {
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

fn main() {
    let mut dp = vec![(0u32, 1u32); 1000001];
    for i in 2..1000001 {
        let mut count5 = 0;
        let mut j = i;
        while j % 5 == 0 {
            j /= 5;
            count5 += 1;
        }
        dp[i].0 = dp[i - 1].0 - count5 + j.trailing_zeros();
        j = j >> j.trailing_zeros();
        dp[i].1 = dp[i - 1].1 * j as u32 % 10;
    }
    // println!("{:?}", &dp[..100]);
    loop {
        let n = scan!(usize);
        if n == 0 {
            break;
        }
        let ans = mpow(2, dp[n].0 as usize, 10) * dp[n].1 as usize % 10;
        println!("{}", ans);
    }
}
