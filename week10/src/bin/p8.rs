use std::{cell::Cell, iter};

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

thread_local! {
    static N: Cell<usize> = const { Cell::new(0) };
    static M: Cell<f64> = const { Cell::new(0.0) };
}

fn solve_v(v: f64, xys: &[(i32, i32, u32)], dxy: &[Vec<f64>]) -> bool {
    let n = N.get();
    let mut dp = vec![vec![None; 1 << n]; n];
    (1..=n).any(|i| solve_dp(&mut dp, i, (1 << n) - 1, v, xys, dxy, n).is_finite())
}

fn solve_dp(
    dp: &mut [Vec<Option<f64>>],
    i: usize,
    state: usize,
    v: f64,
    xys: &[(i32, i32, u32)],
    dxy: &[Vec<f64>],
    lv: usize,
) -> f64 {
    if let Some(d) = dp[i - 1][state] {
        return d;
    }
    let n = N.get();
    let m = M.get();
    let mut mask = 1;
    let state2 = state ^ (1 << (i - 1));
    if state2 == 0 {
        if dxy[0][i] / v <= xys[i - 1].2 as f64 {
            dp[i - 1][state] = Some(dxy[0][i]);
            return dxy[0][i];
        } else {
            dp[i - 1][state] = Some(f64::INFINITY);
            return f64::INFINITY;
        }
    }
    let mut min = f64::INFINITY;
    let mlv = 1. / m.powi(lv as i32 - 1);
    for j in 1..=n {
        if state2 & mask != 0 {
            let l = solve_dp(dp, j, state2, v, xys, dxy, lv - 1);
            if l.is_finite() {
                let (_, _, si) = xys[i - 1];
                let dij = dxy[i][j];
                let new_l = l + dij * mlv;
                if new_l / v <= si as f64 {
                    min = min.min(new_l);
                }
            }
        }
        mask <<= 1;
    }
    dp[i - 1][state] = Some(min);
    min
}

fn main() {
    let n = scan!(u8) as usize;
    let xys = (0..n)
        .map(|_| (scan!(i32), scan!(i32), scan!(u32)))
        .collect::<Vec<_>>();
    let m = scan!(f64);

    let dxy = iter::once_with(|| {
        iter::once(0.0f64)
            .chain(xys.iter().map(|&(xi, yi, _)| (xi as f64).hypot(yi as f64)))
            .collect::<Vec<_>>()
    })
    .chain((0..n).map(|i| {
        let (xi, yi, _) = xys[i];
        iter::once((xi as f64).hypot(yi as f64))
            .chain((0..n).map(|j| {
                let (xj, yj, _) = xys[j];
                (((xi - xj).pow(2) + (yi - yj).pow(2)) as f64).sqrt()
            }))
            .collect::<Vec<_>>()
    }))
    .collect::<Vec<_>>();
    N.set(n);
    M.set(m);

    let mut diff = f64::INFINITY;
    let mut l = 0.0;
    let mut r = 1e10;
    while diff > 10e-5 {
        let v = (l + r) / 2.0;
        if solve_v(v, &xys, &dxy) {
            r = v;
        } else {
            l = v;
        }
        diff = r - l;
    }
    println!("{}", r);
}
