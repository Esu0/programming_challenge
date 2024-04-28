use std::iter;

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
    let (r, _) = scan!(usize, usize);
    let grid = iter::repeat_with(|| scan!(String).into_bytes()).take(r).collect::<Vec<_>>();
    let mut count = [0; 5];
    for rx2 in grid.windows(2) {
        let [r1, r2] = rx2 else {
            unreachable!()
        };
        for cellx4 in r1.windows(2).zip(r2.windows(2)) {
            let (&[ul, ur], &[dl, dr]) = cellx4 else {
                unreachable!()
            };
            let cells = [ul, ur, dl, dr];
            if !cells.contains(&b'#') {
                count[cells.iter().filter(|c| **c == b'X').count()] += 1;
            }
        }
    }
    for ans in count {
        println!("{ans}");
    }
}
