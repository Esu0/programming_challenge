use std::collections::VecDeque;

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
    let n = scan!(u8) as usize;
    let s = scan!(String).into_bytes();
    let mut queue = VecDeque::new();
    let find = b"ICPCASIASG";
    let neg1 = usize::MAX;
    let neg2 = neg1 - 1;
    let dxdy = [
        (1usize, 2usize),
        (1, neg2),
        (neg1, 2),
        (neg1, neg2),
        (2, 1),
        (2, neg1),
        (neg2, 1),
        (neg2, neg1),
    ];
    let mut marker = vec![0usize; n * n];
    for i in 0..n {
        for j in 0..n {
            if s[i * n + j] == b'I' {
                queue.push_back((i, j, 0));
                marker[i * n + j] = 1;
            }
        }
    }

    while let Some((i, j, k)) = queue.pop_front() {
        if k == 9 {
            println!("YES");
            return;
        }
        for (dx, dy) in dxdy {
            let (ni, nj) = (i.wrapping_add(dx), j.wrapping_add(dy));
            if ni < n && nj < n && s[ni * n + nj] == find[k + 1] {
                queue.push_back((ni, nj, k + 1));
                marker[ni * n + nj] = k + 1;
            }
        }
    }
    println!("NO");
}
