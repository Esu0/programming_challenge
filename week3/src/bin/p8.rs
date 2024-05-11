use std::{collections::BTreeMap, iter};

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
    let (n, k) = scan!(usize, usize);
    let mut sf = iter::repeat_with(|| scan!(u32, u32))
        .take(n)
        .collect::<Vec<_>>();

    // println!("{sf:?}");
    sf.sort_unstable_by_key(|&(s, f)| [f, s]);
    let mut set = BTreeMap::from([(0u32, k)]);
    let mut ans = 0;
    for (s, f) in sf {
        if let Some((&last, count)) = set.range_mut(..s).last() {
            ans += 1;
            *count -= 1;
            if *count == 0 {
                set.remove(&last);
            }
            set.entry(f).and_modify(|c| *c += 1).or_insert(1);
        }
    }
    println!("{ans}");
}
