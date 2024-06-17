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

pub fn p8_solve_simple() {
    let c = scan!(usize);
    let points = (0..c).map(|_| scan!(i64, i64)).collect::<Vec<_>>();
    let mut max_dist = 0;
    for (i, &(x1, y1)) in points.iter().enumerate() {
        for &(x2, y2) in &points[i + 1..] {
            let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2);
            max_dist = max_dist.max(dist);
            eprintln!("{}", (dist as f64).sqrt());
        }
    }
    println!("{}", (max_dist as f64).sqrt());
}
