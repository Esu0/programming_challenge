use std::{cmp::Ordering, collections::BTreeMap, iter};

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
    let (sx, sy) = scan!(f64, f64);
    let (gx, gy) = scan!(f64, f64);
    let n = scan!(u8) as usize;
    let canons = (0..n).map(|_| scan!(f64, f64)).collect::<Vec<_>>();

    let mut adj_list = vec![canons
        .iter()
        .enumerate()
        .map(|(i, &(x, y))| (i + 1, (sx - x).hypot(sy - y) / 5.0))
        .chain(iter::once((n + 1, (sx - gx).hypot(sy - gy) / 5.0)))
        .collect::<Vec<_>>()];
    for (i, &(px1, py1)) in canons.iter().enumerate() {
        let edges = canons
            .iter()
            .enumerate()
            .filter(|&(j, _)| i != j)
            .map(|(j, &(px2, py2))| {
                let dx = px1 - px2;
                let dy = py1 - py2;
                let d = dx.hypot(dy);
                (j + 1, 2.0 + (d - 50.).abs() / 5.0)
            })
            .chain(iter::once((n + 1, {
                let d = (px1 - gx).hypot(py1 - gy);
                2.0 + (d - 50.).abs() / 5.0
            })))
            .collect::<Vec<_>>();
        adj_list.push(edges);
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    struct F64Ord(f64);

    impl Eq for F64Ord {}
    impl PartialOrd for F64Ord {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for F64Ord {
        fn cmp(&self, other: &Self) -> Ordering {
            self.0.partial_cmp(&other.0).unwrap()
        }
    }

    // println!("{:?}", adj_list);
    let mut queue = BTreeMap::new();
    queue.insert(F64Ord(0.), vec![0usize]);
    let mut visited = vec![false; n + 2];
    while let Some(entry) = queue.first_entry() {
        let (time, nodes) = entry.remove_entry();
        for node in nodes {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            if node == n + 1 {
                println!("{:.6}", time.0);
                return;
            }
            for &(next, t) in &adj_list[node] {
                let next_time = time.0 + t;
                if !visited[next] {
                    queue
                        .entry(F64Ord(next_time))
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push(next);
                }
            }
        }
    }
}
