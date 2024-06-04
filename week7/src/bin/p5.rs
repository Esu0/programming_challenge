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
    let n = scan!(String).into_bytes();
    let mut i = 1;
    while i < n.len() && n[i - 1] <= n[i] {
        i += 1;
    }
    let mut j = (i + 1).min(n.len());
    while j < n.len() && n[j - 1] >= n[j] {
        j += 1;
    }
    println!("{} {}", i, j);
    if j != n.len() {
        println!("-1");
    } else {
        let len = j;
        let mut dp1 = vec![[0u64; 10]; len];
        dp1[0] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 1..len {
            for j in 1..10 {
                dp1[i][j] = dp1[i - 1][j] + dp1[i][j - 1];
            }
        }
        eprintln!("{:?}", dp1);
        let mut sum = 0u64;
        for k in (i..len).rev() {
            sum += dp1[len - k][(n[k] - b'0') as usize];
        }
        println!("{}", sum);
    }
}
