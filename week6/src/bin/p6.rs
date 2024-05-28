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

/// returns (gcd, x, y) such that a * x + b * y = gcd
fn gcd_ext(a: u64, b: u64) -> (i64, i64, i64) {
    if b == 0 {
        (a as i64, 1, 0)
    } else {
        // b * x + (a % b) * y = gcd
        let (gcd, x, y) = gcd_ext(b, a % b);
        let d = a / b;
        // b * x - d * b * y + (a % b) * y + d * b * y = b * (x - d * y) + a * y = gcd
        (gcd, y, x - d as i64 * y)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    gcd_ext(a, b).0 as u64
}

fn main() {
    let n = scan!(u16) as usize;
    let mut numbers = (0..n).map(|_| scan!(u32)).collect::<Vec<_>>();
    numbers.sort_unstable();
    let mut edges = vec![vec![]; n];
    let mut redundant = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let gcd = gcd(numbers[i] as _, numbers[j] as _) as u32;
            if gcd > 1 {
                edges[i].push(j);
                edges[j].push(i);
                redundant[i][j] = gcd;
                redundant[j][i] = gcd;
            }
        }
    }
    // eprintln!("{:?}", edges);

    // max flow from 0 to n - 1
    let mut flow = vec![vec![0u32; n]; n];
    let mut max_flow = 0;
    loop {
        let mut path = vec![0; n];
        let mut visited = vec![false; n];
        let mut stack = vec![(0, usize::MAX)];
        while let Some((node, prev)) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            path[node] = prev;
            if node == n - 1 {
                break;
            }
            for &next in &edges[node] {
                if !visited[next] && redundant[node][next] > 0 {
                    stack.push((next, node));
                }
            }
        }
        if !visited[n - 1] {
            break;
        }
        let mut min_flow = u32::MAX;
        let mut node = n - 1;
        while node != 0 {
            let prev = path[node];
            min_flow = min_flow.min(redundant[prev][node]);
            node = prev;
        }
        max_flow += min_flow;
        let mut node = n - 1;
        while node != 0 {
            let prev = path[node];
            redundant[prev][node] -= min_flow;
            redundant[node][prev] += min_flow;
            flow[prev][node] += min_flow;
            node = prev;
        }
    }
    println!("{}", max_flow);
}
