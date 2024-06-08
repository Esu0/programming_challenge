use std::ops::{Add, Div, Mul, Sub};

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
    
}
