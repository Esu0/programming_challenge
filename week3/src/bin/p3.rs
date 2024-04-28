use std::{cmp::Reverse, fmt::Debug, iter, str::FromStr};

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

fn scan_array<T: FromStr>(count: usize) -> Vec<T>
where
    T::Err: Debug,
{
    iter::repeat_with(|| scan!(T)).take(count).collect()
}

fn main() {
    let (n, m, k) = scan!(u8, u8, u8);
    let mut p = scan_array::<u32>(n as usize);
    let c = scan_array::<u32>(m as usize);
    let s = scan_array::<u32>(k as usize);
    let mut h = c.into_iter().map(|r| 2 * r * r).chain(s.into_iter().map(|l| l * l)).collect::<Vec<_>>();
    h.sort_unstable_by_key(|&x| Reverse(x));
    p.iter_mut().for_each(|pi| *pi = 2 * *pi * *pi);
    p.sort_unstable_by_key(|&x| Reverse(x));
    let mut count = 0;
    let mut j = 0;
    'outer: for pi in p {
        loop {
            if j >= h.len() {
                break 'outer;
            }
            if h[j] < pi {
                count += 1;
                j += 1;
                break;
            }
            j += 1;
        }
    }
    println!("{count}");
}
