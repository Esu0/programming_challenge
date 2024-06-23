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
    let n = scan!(u32);
    let cubic = (1..=(n as f64).cbrt().ceil() as usize + 1).map(|i| (i as u64).pow(3)).collect::<Vec<_>>();
    let mut set = HashSet::new();
    let mut ans = 0;
    for (i, &a) in cubic.iter().enumerate() {
        for &b in &cubic[i + 1..] {
            if !set.insert(a + b) && a + b <= n as u64 {
                ans = ans.max(a + b);
            }
        }
    }
    if ans != 0 {
        println!("{ans}");
    } else {
        println!("none");
    }
}
