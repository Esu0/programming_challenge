use std::{cmp, iter};

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
    let mut case = 1;
    while let Some(n) = input::scan::<usize>() {
        let mut set = iter::repeat_with(|| scan!(i32)).take(n).collect::<Vec<_>>();
        set.sort_unstable();
        let m = scan!(u8);
        println!("Case {case}:");
        case += 1;
        for _ in 0..m {
            let query = scan!(i32);
            let mut ans = i32::MAX;
            for (i, &ai) in set.iter().enumerate() {
                let j = set.partition_point(|&x| ai + x <= query);
                if i != j {
                    if let Some(&aj) = set.get(j) {
                        ans = cmp::min_by_key(ans, ai + aj, |&x| x.abs_diff(query));
                    }
                }
                let j = j.wrapping_sub(1);
                if i != j {
                    if let Some(&aj) = set.get(j) {
                        ans = cmp::min_by_key(ans, ai + aj, |&x| x.abs_diff(query));
                    }
                }
            }
            println!("Closest sum to {} is {}.", query, ans);
        }
    }
}
