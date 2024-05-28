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
    loop {
        let (n, m, a, k) = scan!(usize, usize, usize, u8);
        if n == 0 {
            break;
        }
        let roads = (0..m).map(|_| scan!(usize, usize, u8));
        let alien_bases = (0..a).map(|_| scan!(usize));
        let mut adj = vec![vec![]; n];
        let mut dist = vec![vec![]; n];
        for (u, v, w) in roads {
            adj[u].push(v);
            adj[v].push(u);
            dist[u].push(w);
            dist[v].push(w);
        }
    }
}
