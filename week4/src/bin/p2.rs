use std::iter;

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

#[derive(Debug, PartialEq, Eq)]
enum Output {
    Impossible,
    Ambiguous,
    Possible(Vec<u32>),
}

fn main() {
    let n = scan!(usize);
    let mut c = (0..n).map(|i| (i as u8, scan!(u32))).collect::<Vec<_>>();
    c.sort_unstable_by_key(|&(_, x)| x);
    let m = scan!(usize);
    let s = (0..m).map(|_| scan!(u32)).collect::<Vec<_>>();
    let max_order = s.iter().copied().max().unwrap() as usize;
    let mut dp = iter::once(Output::Possible(vec![0; n]))
        .chain((0..max_order).map(|_| Output::Impossible))
        .collect::<Vec<_>>();
    for i in 1..=max_order {
        for &(j, cj) in &c {
            if i < cj as usize {
                break;
            }
            let k = i - cj as usize;
            match &dp[k] {
                Output::Impossible => continue,
                Output::Ambiguous => {
                    dp[i] = Output::Ambiguous;
                    break;
                }
                Output::Possible(prev) => {
                    let mut new_vec = prev.clone();
                    new_vec[j as usize] += 1;
                    if let Output::Possible(order) = &dp[i] {
                        if order != &new_vec {
                            dp[i] = Output::Ambiguous;
                            break;
                        }
                    }
                    dp[i] = Output::Possible(new_vec);
                }
            }
        }
    }
    // println!("{:?}", dp);
    for si in s {
        match &dp[si as usize] {
            Output::Impossible => println!("Impossible"),
            Output::Ambiguous => println!("Ambiguous"),
            Output::Possible(ans) => {
                for (i, &count) in ans.iter().enumerate() {
                    for _ in 0..count {
                        print!("{} ", i + 1);
                    }
                }
                println!();
            }
        }
    }
}
