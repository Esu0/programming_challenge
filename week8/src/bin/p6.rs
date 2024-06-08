use std::{borrow::Borrow, collections::HashMap, hash::Hash, ops::{Deref, DerefMut}};

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

struct MultiHashSet<T>(HashMap<T, usize>);

impl<T: Eq + Hash> MultiHashSet<T> {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, value: T) {
        *self.0.entry(value).or_default() += 1;
    }

    fn remove<Q>(&mut self, value: &Q)
    where
        T: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        if let Some(count) = self.0.get_mut(value) {
            *count -= 1;
            if *count == 0 {
                self.0.remove(value);
            }
        }
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

impl<I: Eq + Hash> Extend<I> for MultiHashSet<I> {
    fn extend<T: IntoIterator<Item = I>>(&mut self, iter: T) {
        for value in iter {
            self.insert(value);
        }
    }
}

struct PrimeFactors(MultiHashSet<u64>);
impl Extend<(u64, u32)> for PrimeFactors {
    fn extend<T: IntoIterator<Item = (u64, u32)>>(&mut self, iter: T) {
        for (prime, exp) in iter {
            *self.0.0.entry(prime).or_default() += exp as usize;
        }
    }
}

impl PrimeFactors {
    fn borrow_as_div(&mut self) -> PrimeFactorsDiv {
        PrimeFactorsDiv(&mut self.0)
    }
}

impl Deref for PrimeFactors {
    type Target = MultiHashSet<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PrimeFactors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct PrimeFactorsDiv<'a>(&'a mut MultiHashSet<u64>);

impl<'a> Extend<(u64, u32)> for PrimeFactorsDiv<'a> {
    fn extend<T: IntoIterator<Item = (u64, u32)>>(&mut self, iter: T) {
        for (prime, exp) in iter {
            *self.0.0.get_mut(&prime).unwrap() -= exp as usize;
        }
    }
}

fn main() {
    let mut factors = PrimeFactors(MultiHashSet::new());
    while let Some(n) = input::scan::<u64>() {
        let m = scan!(u64);
        factors.0.0.clear();
        for i in 1..=m {
            prime_factors(n + 1 - i, &mut factors);
            if i > 1 {
                prime_factors(i, &mut factors.borrow_as_div());
            }
        }

        println!("{}", factors.0.0.values().copied().map(|x| x + 1).product::<usize>());
    }
}
