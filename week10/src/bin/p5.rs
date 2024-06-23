use std::collections::BTreeMap;

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
    let (n, m, s) = scan!(usize, usize, u64);
    let uvtpd = (0..m).map(|_| scan!(usize, usize, u64, u64, u64));
    let mut adj_list = vec![vec![]; n];
    for (u, v, t, p, d) in uvtpd {
        adj_list[v].push((u, t, p, d));
    }
    let mut visited = vec![false; n];
    let mut queue = BTreeMap::from([(0u64, vec![n - 1])]);
    while let Some((time, nodes)) = queue.pop_first() {
        for &node in &nodes {
            visited[node] = true;
            if node == 0 {
                println!("{}", s.checked_sub(time).unwrap());
                return;
            }
            for &(from, t, p, d) in &adj_list[node] {
                if visited[from] {
                    continue;
                }
                if s < time + t + d {
                    continue;
                }
                let offset = (s - time - t - d) % p;
                let time_d = offset + d;
                queue
                    .entry(time + time_d)
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(from);
            }
        }
    }
    println!("impossible");
}
