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

fn comb(n: usize, k: usize) -> usize {
    let mut res = 1;
    for i in 0..k {
        res *= n - i;
        res /= i + 1;
    }
    res
}

fn main() {
    let g = scan!(u8);
    for game in 1..=g {
        let m = scan!(u8) as usize;
        let labels = (0..m).map(|_| scan!(u32)).collect::<Vec<_>>();
        let (n, t) = (scan!(u8) as usize, scan!(u16) as usize);
        let mut dp = std::array::from_fn::<_, 31, _>(|i| vec![vec![0u32; t + 1]; i + 1]);
        for dpi in &mut dp {
            dpi[0][0] = 1;
        }
        for i in 1..=m {
            for j in 1..=i.min(n) {
                for k in 1..=t {
                    let label = labels[i - 1] as usize;
                    if k >= label {
                        dp[i][j][k] = dp[i - 1][j - 1][k - label];
                    }
                    if i > j {
                        dp[i][j][k] += dp[i - 1][j][k];
                    }
                }
            }
        }
        let ans = dp[m][n][t] as usize;
        println!("Game {game} -- {ans} : {}", comb(m, n) - ans);
    }
}
