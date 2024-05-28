use std::collections::BTreeSet;

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
    let edges = (0..m).map(|_| scan!(usize, usize)).collect::<Vec<_>>();
    let mut adj_list = vec![vec![]; n];
    for (u, v) in edges {
        adj_list[u - 1].push(v - 1);
        adj_list[v - 1].push(u - 1);
    }
    let mut unvisited = (0..n).collect::<BTreeSet<_>>();
    let mut marker = vec![0i8; n];
    let mut is_bipartite = true;
    let mut count_cc = 0u32;
    while let Some(start) = unvisited.pop_first() {
        let mut stack = vec![start];
        marker[start] = 1;
        while let Some(i) = stack.pop() {
            for &j in &adj_list[i] {
                if unvisited.remove(&j) {
                    stack.push(j);
                    marker[j] = -marker[i];
                } else if is_bipartite && marker[j] == marker[i] {
                    is_bipartite = false;
                }
            }
        }
        count_cc += 1;
    }
    if is_bipartite {
        println!("{count_cc}");
    } else {
        println!("{}", count_cc.checked_sub(1).unwrap());
    }
}
