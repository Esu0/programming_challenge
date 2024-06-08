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

pub fn prime_factors<M: Extend<(u64, u32)>>(mut n: u64, map: &mut M) {
    let mut count2 = 0;
    while n % 2 == 0 {
        n /= 2;
        count2 += 1;
    }
    if count2 > 0 {
        map.extend(std::iter::once((2, count2)));
    }
    let mut i = 3;
    while i * i <= n {
        let mut count = 0;
        while n % i == 0 {
            n /= i;
            count += 1;
        }
        if count > 0 {
            map.extend(std::iter::once((i, count)));
        }
        i += 2;
    }
    if n > 1 {
        map.extend(std::iter::once((n, 1)));
    }
}

fn main() {
    let mut factors = Vec::new();
    while let Some(p) = input::scan::<u32>() {
        factors.clear();
        prime_factors(p as u64, &mut factors);
        let mut sum = 1;
        for &(prime, exp) in &factors {
            if exp == 1 {
                sum *= prime + 1;
            } else {
                sum *= (prime.pow(exp + 1) - 1) / (prime - 1);
            }
        }
        let sum = sum - p as u64;
        if sum == p as _ {
            println!("{} perfect", p);
        } else if sum.abs_diff(p as u64) <= 2 {
            println!("{} almost perfect", p);
        } else {
            println!("{} not perfect", p);
        }
    }
}
