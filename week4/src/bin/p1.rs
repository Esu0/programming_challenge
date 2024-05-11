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
    let n = scan!(usize);
    let a = (0..n).map(|_| scan!(u32)).collect::<Vec<_>>();

    let mut ans = u32::MAX;
    let mut next = iter::once(0)
        .chain(iter::repeat(u32::MAX))
        .take(n)
        .collect::<Vec<_>>();
    for i in 1..n {
        let l = cmp::min(n - i, i * (i - 1) / 2 + 1);
        let new_vec = (0..i)
            .map(|_| u32::MAX)
            .chain(next[..l].iter().copied().enumerate().map(|(j, x)| {
                let k = j + i;
                a[k].saturating_add(x)
            }))
            .chain(iter::repeat(u32::MAX))
            .take(n)
            .collect::<Vec<_>>();
        next = new_vec;
        // println!("{:?}", prev);
        for j in i..(i + l) {
            let mut k = j;
            let mut prev_value = next[j];
            while k >= i {
                k -= i;
                prev_value = cmp::min(next[k], prev_value.checked_add(a[k]).unwrap());
                next[k] = prev_value;
            }
        }
        // println!("{:?}", next);
        let tmp = cmp::min(ans, *next.last().unwrap());
        ans = tmp;
    }
    println!("{ans}");
}
