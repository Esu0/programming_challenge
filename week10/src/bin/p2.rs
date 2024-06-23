use std::collections::{BinaryHeap, HashSet};

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
    let (h, n, m) = scan!(u32, usize, usize);
    let cells = (0..n).map(|_| (0..m).map(|_| scan!(char) == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
    let to_ij = |idx: usize| (idx / m, idx % m);
    let from_ij = |i: usize, j: usize| i * m + j;
    let mut unvisited = (0..n * m).filter(|&idx| {
        let (i, j) = to_ij(idx);
        !cells[i][j]
    }).collect::<HashSet<_>>();

    let edge_even = [(0usize, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0), (1, usize::MAX), (usize::MAX, usize::MAX)];
    let edge_odd = [(0usize, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0), (1, 1), (usize::MAX, 1)];

    let mut counts = BinaryHeap::new();
    while let Some(s) = unvisited.iter().next().copied() {
        unvisited.remove(&s);
        let mut stack = vec![s];
        let mut count = 1u32;
        while let Some(idx) = stack.pop() {
            let (i, j) = to_ij(idx);
            let edges = if i % 2 == 0 {
                edge_even
            } else {
                edge_odd
            }.into_iter().map(|(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj))).filter(|&(i, j)| i < n && j < m);
            for (i, j) in edges {
                let idx = from_ij(i, j);
                if unvisited.remove(&idx) {
                    count += 1;
                    stack.push(idx);
                }
            }
        }
        counts.push(count);
    }

    eprintln!("{:?}", counts);
    let mut ans = 0u32;
    let mut remaining = h;
    while remaining > 0 {
        remaining = remaining.saturating_sub(counts.pop().unwrap());
        ans += 1;
    }
    println!("{ans}");
}
