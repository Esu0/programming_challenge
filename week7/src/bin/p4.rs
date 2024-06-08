use rand::Rng;

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

#[allow(dead_code)]
mod rolling_hash {
    use std::ops::RangeBounds;

    pub struct RollingHasher {
        modulo: u64,
        /// `exponents[i] = base^(i + 1) % modulo`
        exponents: Vec<u64>,
        hash: Vec<u64>,
    }

    const MODULO_DEFAULT: u64 = (1 << 61) - 1;

    impl RollingHasher {
        pub fn new(base: u64, data: impl IntoIterator<Item = u64>) -> Self {
            let mut e = 1u64;
            let modulo = MODULO_DEFAULT;
            let base = base as u128;
            let mut tmp = 0u64;
            let hash = data
                .into_iter()
                .map(|x| {
                    tmp = ((tmp as u128 * base + x as u128) % modulo as u128) as u64;
                    tmp
                })
                .collect::<Vec<_>>();
            let exponents = std::iter::repeat_with(|| {
                e = (e as u128 * base % modulo as u128) as u64;
                e
            })
            .take(hash.len() - 1)
            .collect();
            Self {
                modulo,
                exponents,
                hash,
            }
        }

        pub fn hash(&self, range: impl RangeBounds<usize>) -> u64 {
            let start = match range.start_bound() {
                std::ops::Bound::Included(&x) => x,
                std::ops::Bound::Excluded(&x) => x + 1,
                std::ops::Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                std::ops::Bound::Included(&x) => x + 1,
                std::ops::Bound::Excluded(&x) => x,
                std::ops::Bound::Unbounded => self.hash.len(),
            };
            let mut ret = self.hash[end - 1] as i128;
            let modulo = self.modulo as i128;
            if start > 0 {
                ret = (ret
                    - self.hash[start - 1] as i128 * self.exponents[end - start - 1] as i128)
                    .rem_euclid(modulo);
            }
            ret as _
        }
    }
}

fn main() {
    let s = scan!(String).into_bytes();
    let mut rng = rand::thread_rng();
    let rh = rolling_hash::RollingHasher::new(
        rng.gen_range(2..1_000_000_000_000_000),
        s.iter().map(|&x| x as u64),
    );

    let mut dp = vec![vec![(u32::MAX, u64::MAX); s.len()]; s.len()];
    dp[0].iter_mut().enumerate().for_each(|(i, x)| *x = (1, rh.hash(i..i + 1)));
    for l in 1..s.len() {
        for i in 0..s.len() - l {
            for j in 0..l {
                let (a, h1) = dp[j][i];
                let (b, h2) = dp[l - j - 1][i + j + 1];
                if h1 == h2 {
                    assert_eq!(a, b);
                    dp[l][i] = std::cmp::min_by_key(dp[l][i], (a, h1), |x| x.0);
                } else {
                    dp[l][i] = std::cmp::min_by_key(dp[l][i], (a + b, rh.hash(i..i + l + 1)), |x| x.0);
                }
            }
        }
    }
    println!("{}", dp[s.len() - 1][0].0);
}
