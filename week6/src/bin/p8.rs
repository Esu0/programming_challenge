use std::collections::HashMap;

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
    let (r, c) = scan!(usize, usize);
    let grid = (0..r)
        .map(|_| (0..c).map(|_| scan!(u32)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let castle_pos = scan!(usize, usize);
    let s = castle_pos.0 * c + castle_pos.1;
    let t = 2 * c * r;
    let in_offset = c * r;
    let mut adj = vec![vec![]; 2 * c * r + 1];
    let mut weight = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if grid[i][j] != 0 {
                let u = i * c + j;
                adj[u].push(u + in_offset);
                // 逆向きの辺
                // if u != s {
                // }
                adj[u + in_offset].push(u);
                weight.insert((u, u + in_offset), cell as i64);
                // 残余辺
                weight.insert((u + in_offset, u), 0);
                for (di, dj) in [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)].into_iter() {
                    let ni = i.wrapping_add(di);
                    let nj = j.wrapping_add(dj);
                    if ni < r && nj < c {
                        if grid[ni][nj] != 0 {
                            let v = ni * c + nj;
                            adj[u + in_offset].push(v);
                            // 逆向きの辺
                            adj[v].push(u + in_offset);
                            // 残余辺
                            weight.insert((v, u + in_offset), 0);
                        }
                    } else {
                        adj[u + in_offset].push(t);
                        // adj[t].push(u);
                    }
                }
            }
        }
    }
    // println!("t_out: {:?}", adj[t]);
    // println!("{:?}", adj);
    // println!("{:?}", weight);
    let mut redundant = weight;
    let mut max_flow = 0;
    loop {
        let mut path = vec![0; 2 * c * r + 1];
        let mut stack = vec![(s, usize::MAX)];
        let mut visited = vec![false; 2 * c * r + 1];
        while let Some((node, prev)) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            path[node] = prev;
            if node == t {
                break;
            }
            for &next in &adj[node] {
                if !visited[next] && redundant.get(&(node, next)).map(|&v| v > 0).unwrap_or(true) {
                    stack.push((next, node));
                }
            }
        }
        if !visited[t] {
            break;
        }
        let mut min_flow = i64::MAX;
        let mut node = t;
        while node != s {
            let prev = path[node];
            if let Some(&v) = redundant.get(&(prev, node)) {
                min_flow = min_flow.min(v);
            }
            node = prev;
        }
        max_flow += min_flow;
        let mut node = t;
        while node != s {
            let prev = path[node];
            if let Some(v) = redundant.get_mut(&(prev, node)) {
                *v -= min_flow;
            }
            if let Some(v) = redundant.get_mut(&(node, prev)) {
                *v += min_flow;
            }
            node = prev;
        }
    }
    println!("{}", max_flow)
}
