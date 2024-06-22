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

fn solve_dp(
    dp: &mut [Vec<(f64, f64)>],
    i: usize,
    state: usize,
    n: u32,
    xys: &[(i32, i32, u32)],
    m: f64,
) -> (f64, f64) {
    if dp[i][state].0 >= 0.0 {
        return dp[i][state];
    }
    let mut mask = 1usize;
    let mut min = f64::INFINITY;
    let ones = state.count_ones();
    if ones == 1 {
        let (xi, yi, si) = xys[i];
        let d = ((xi.pow(2) + yi.pow(2)) as f64).sqrt();
        dp[i][state] = (d, d / si as f64);
        return dp[i][state];
    }
    let state2 = state & !(1 << i);

    for j in 0..n {
        if mask & state2 != 0 {
            let (l, e) = solve_dp(dp, j as usize, state2, n, xys, m);
            let (xi, yi, si) = xys[i];
            let (xj, yj, _sj) = xys[j as usize];
            let d = (((xi - xj).pow(2) + (yi - yj).pow(2)) as f64).sqrt();
            let next_l = l + d / m.powi((ones - 1) as _);

            let mut next_e = next_l / si as f64;
            // let next_j = j as u8;
            if e > next_e {
                next_e = e;
            }
            if next_e < min {
                min = next_e;
                dp[i][state] = (next_l, next_e);
            }
        }
        mask <<= 1;
    }
    dp[i][state]
}

fn main() {
    let n = scan!(u8) as u32;
    let xys = (0..n)
        .map(|_| (scan!(i32), scan!(i32), scan!(u32)))
        .collect::<Vec<_>>();
    let m = scan!(f64);

    let mut dp = vec![vec![(-1.0f64, 0.0f64); 1 << n]; n as usize];
    let ans = (0..n)
        .map(|i| solve_dp(&mut dp, i as usize, (1 << n) - 1, n, &xys, m as f64).1)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    println!("{}", ans);
    // eprintln!("{:?}", dp);
}
