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
    let n = scan!(usize);
    let mut lu = iter::repeat_with(|| scan!(u32, u32))
        .take(n)
        .collect::<Vec<_>>();
    lu.sort_unstable_by_key(|(_, u)| *u);
    let mut deleted = [false; 100];
    let mut ans = 0;
    for (i, &(_, u)) in lu.iter().enumerate() {
        if !deleted[i] {
            ans += 1;
            lu.iter()
                .enumerate()
                .filter(|&(_, &(l2, u2))| l2 <= u && u <= u2)
                .for_each(|(i, _)| deleted[i] = true);
        }
    }
    println!("{ans}");
}
