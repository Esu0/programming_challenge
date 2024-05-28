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
    let (n, m) = scan!(usize, usize);
    let mut grid = (0..n)
        .map(|_| scan!(String).into_bytes())
        .collect::<Vec<_>>();
    let mut queue = VecDeque::from([(0usize, 0usize, 0u32)]);
    // let mut answer = 0;
    while let Some((i, j, d)) = queue.pop_back() {
        if grid[i][j] == b'0' {
            continue;
        }
        let k = (grid[i][j] - b'0') as usize;
        grid[i][j] = b'0';
        for (di, dj) in [(0, k), (0, k.wrapping_neg()), (k, 0), (k.wrapping_neg(), 0)] {
            let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
            if ni == n - 1 && nj == m - 1 {
                // answer = d + 1;
                println!("{}", d + 1);
                return;
            }
            if ni < n && nj < m && grid[ni][nj] != b'0' {
                queue.push_front((ni, nj, d + 1));
            }
        }
    }
    println!("-1");
}
