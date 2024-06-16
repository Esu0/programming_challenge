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
        let m = scan!(usize);
        let points = (0..m).map(|_| scan!(i32, i32)).collect::<Vec<_>>();
        let mut s = points.windows(2).map(|w| {
            let &[(x1, y1), (x2, y2)] = w else {
                unreachable!()
            };
            (x1 - x2) * (y1 + y2)
        }).sum::<i32>();
        let &(first_x, first_y) = points.first().unwrap();
        let &(last_x, last_y) = points.last().unwrap();
        s += (last_x - first_x) * (last_y + first_y);
        println!("{}", s.abs() as f64 / 2.);
    }
}
