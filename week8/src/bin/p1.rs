use std::ops::{Add, Div, Mul, Sub};

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

#[derive(Clone, Copy, Debug)]
struct Fraction {
    numerator: u64,
    denominator: u64,
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl Fraction {
    fn new(numerator: u64, denominator: u64) -> Self {
        let mut ret = Self {
            numerator,
            denominator,
        };
        ret.reduce();
        ret
    }

    fn reduce(&mut self) {
        let gcd = gcd(self.numerator, self.denominator);
        self.numerator /= gcd;
        self.denominator /= gcd;
    }

    // 整数ならSome
    fn integer(&self) -> Option<u64> {
        if self.denominator == 1 {
            Some(self.numerator)
        } else {
            None
        }
    }
}

impl Add for Fraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl Sub for Fraction {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator - rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.numerator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator;
        let denominator = self.denominator * rhs.numerator;
        Self::new(numerator, denominator)
    }
}

impl From<u64> for Fraction {
    fn from(numerator: u64) -> Self {
        Self {
            numerator,
            denominator: 1,
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        let tmp = b % a;
        b = a;
        a = tmp;
    }
    b
}

fn main() {
    let n = scan!(usize);
    let r = (0..n).map(|_| scan!(u64)).collect::<Vec<_>>();
    let first = Fraction::from(r[0]);
    for &r in &r[1..] {
        let r = Fraction::from(r);
        let ans = first / r;
        println!("{}/{}", ans.numerator, ans.denominator);
    }
}
