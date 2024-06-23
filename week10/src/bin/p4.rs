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

fn dijkstra(
    adj_list: &[Vec<(usize, u64)>],
    s: usize,
    t: usize,
    mut f: impl FnMut(usize, usize, u64, u64) -> Option<u64>,
) -> Option<u64> {
    let n = adj_list.len();
    let mut visited = vec![false; n];
    let mut queue = BTreeMap::from([(0u64, vec![s])]);
    // let mut count = 0;
    while let Some((dist, nodes)) = queue.pop_first() {
        for &node in &nodes {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            if node == t {
                // println!("{}", count);
                return Some(dist);
            }
            for &(to, d) in &adj_list[node] {
                // count += 1;
                if visited[to] {
                    continue;
                }
                let Some(dist) = f(node, t, dist, d) else {
                    continue;
                };
                queue
                    .entry(dist)
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(to);
            }
        }
    }
    None
}

fn solve(t_max: u64, adj_list: &[Vec<(usize, u64)>], l_max: u64) -> bool {
    let n = adj_list.len();
    dijkstra(adj_list, 0, n - 1, |_, _, dist, d| {
        if d <= t_max {
            Some(dist + d)
        } else {
            None
        }
    })
    .is_some_and(|dist| dist <= l_max)
}

// fn binary_search(map: &[Vec<u32>], l: u32, r: u32) -> u32 {
//     if l == r {
//         return l;
//     }
//     let m = (l + r) / 2;
//     if solve(m, map) {
//         binary_search(map, l, m)
//     } else {
//         binary_search(map, m + 1, r)
//     }
// }

fn main() {
    let (n, m, x) = scan!(usize, usize, u64);
    let roads = (0..m).map(|_| scan!(usize, usize, u64));
    let mut adj_list = vec![vec![]; n];

    for (c1, c2, t) in roads {
        adj_list[c1 - 1].push((c2 - 1, t));
        adj_list[c2 - 1].push((c1 - 1, t));
    }

    let shortest = dijkstra(&adj_list, 0, n - 1, |_, _, dist, d| Some(dist + d)).unwrap();
    let l_max = (shortest as f64 * ((x + 100) as f64 / 100.)).floor() as u64;

    let mut l = 1;
    let mut r = 1_000_000_000;
    // let mut count = 0;
    while l < r {
        let m = (l + r) / 2;
        if solve(m, &adj_list, l_max) {
            r = m;
        } else {
            l = m + 1;
        }
        // count += 1;
    }
    println!("{}", l);
}
