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
    let test_cases = scan!(u8);
    for _ in 0..test_cases {
        let (n, m) = scan!(usize, usize);
        let edges = (0..m).map(|_| scan!(usize, usize));
        let mut adj_list = vec![vec![]; n];
        let mut adj_list_rev = vec![vec![]; n];
        for (u, v) in edges {
            adj_list[u - 1].push(v - 1);
            adj_list_rev[v - 1].push(u - 1);
        }

        let mut dfs_low = vec![-1i32; n];
        let mut dfs_num = vec![0i32; n];
        let mut stack = vec![0usize];
        while let Some(i) = stack.pop() {
            for &j in &adj_list[i] {
                if dfs_num[j] < 0 {
                    stack.push(j);
                    dfs_num[j] = dfs_num[i] + 1;
                    dfs_low[j] = dfs_num[j];
                }
            }
        }
    }
}
