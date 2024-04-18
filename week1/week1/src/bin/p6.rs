use std::{io::stdin, str::FromStr};

#[allow(unused_imports)]
#[macro_use]
extern crate dmoj;

#[derive(Clone, Copy, Debug)]
enum Hand {
    Rock,
    Scissors,
    Paper,
}

impl<'a> FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rock" => Ok(Self::Rock),
            "scissors" => Ok(Self::Scissors),
            "paper" => Ok(Self::Paper),
            _ => Err(()),
        }
    }
}

impl Hand {
    fn score(self, other: Self) -> i8 {
        use Hand::*;
        match (self, other) {
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => 0,
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 1,
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => -1,
        }
    }
}

fn main() {
    loop {
        let mut lines = String::new();
        stdin().read_line(&mut lines).unwrap();
        let mut splitted = lines.split_whitespace();
        let n = splitted.next().unwrap().parse::<usize>().unwrap();
        if n == 0 {
            break;
        }
        let k = splitted.next().unwrap().parse::<usize>().unwrap();
        let mut w = vec![0i64; n];
        let mut l = w.clone();
        let games = (k * n * (n - 1)) / 2;
        for _ in 0..games {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            let mut splitted = line.split_whitespace();
            let p1 = splitted.next().unwrap().parse::<usize>().unwrap() - 1;
            let m1 = splitted.next().unwrap().parse::<Hand>().unwrap();
            let p2 = splitted.next().unwrap().parse::<usize>().unwrap() - 1;
            let m2 = splitted.next().unwrap().parse::<Hand>().unwrap();
            let score = m1.score(m2);
            if score > 0 {
                // p1 won
                w[p1] += 1;
                l[p2] += 1;
            } else if score < 0 {
                // p2 won
                w[p2] += 1;
                l[p1] += 1;
            }
        }
        for (&wi, &li) in w.iter().zip(&l) {
            if wi == 0 && li == 0 {
                dmoj::println!("-");
            } else {
                dmoj::println!("{:.3}", wi as f64 / (wi as f64 + li as f64));
            }
        }
        println!();
    }
}
