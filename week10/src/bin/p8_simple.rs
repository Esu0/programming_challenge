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

fn permutation(v: &mut [u8], n: u8, f: &mut impl FnMut(&[u8])) {
    if n == 0 {
        f(v);
        return;
    }
    for i in 0..n {
        v.swap(i as usize, n as usize - 1);
        permutation(v, n - 1, f);
        v.swap(i as usize, n as usize - 1);
    }
}
fn main() {
    let n = scan!(u8) as u32;
    // if n != 4 {
    //     panic!();
    // }
    let xys = (0..n)
        .map(|_| (scan!(i32), scan!(i32), scan!(u32)))
        .collect::<Vec<_>>();
    let m = scan!(f64);

    let mut v = (0..n as u8).collect::<Vec<_>>();
    let mut min = f64::INFINITY;
    permutation(&mut v, n as u8, &mut |v| {
        let mut max = 0.0f64;
        let mut l = 0.;
        let mut prev_xi = 0;
        let mut prev_yi = 0;
        // let mut perv_si = 0;
        for (k, &i) in v.iter().enumerate() {
            let (xi, yi, si) = xys[i as usize];
            let d = ((xi - prev_xi).pow(2) + (yi - prev_yi).pow(2)) as f64;
            l += d.sqrt() / m.powi(k as i32);
            let update = l / si as f64;
            if update > min {
                return;
            }
            max = max.max(update);
            prev_xi = xi;
            prev_yi = yi;
        }
        min = min.min(max);
    });
    // if min.is_infinite() {
    //     panic!();
    // }
    println!("{}", min);
}
