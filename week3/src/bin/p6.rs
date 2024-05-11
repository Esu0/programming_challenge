use std::cmp::Reverse;

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
    let (n, h) = scan!(usize, u32);
    let (mut f, mut c) = (0..(n / 2)).map(|_| scan!(u32, u32)).unzip::<_, _, Vec<_>, Vec<_>>();
    f.sort_unstable();
    c.sort_unstable_by_key(|&x| Reverse(x));
    // let mut ob = f.len();
    let mut i = 0;
    let mut j = 0;
    let ans = (1..=h).map(|l| {
        while i < f.len() && f[i] < l {
            i += 1;
        }
        while j < c.len() && h - c[j] < l {
            j += 1;
        }
        // println!("l: {l}, ob: {}", (j + f.len() - i));
        (j + f.len() - i) as u32
    }).collect::<Vec<_>>();
    let min = *ans.iter().min().unwrap();
    let cnt = ans.iter().filter(|&x| *x == min).count();
    println!("{} {}", min, cnt)
}
