use std::{collections::HashMap, vec};

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
    let out_offset = c * r;
    let mut adj = vec![vec![]; 2 * c * r + 1];
    let mut weight = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if grid[i][j] != 0 {
                let u = i * c + j;
                adj[u].push(u + out_offset);
                // 逆向きの辺
                // if u != s {
                // }
                adj[u + out_offset].push(u);
                weight.insert((u, u + out_offset), cell as i64);
                // 残余辺
                weight.insert((u + out_offset, u), 0);
                for (di, dj) in [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)].into_iter() {
                    let ni = i.wrapping_add(di);
                    let nj = j.wrapping_add(dj);
                    if ni < r && nj < c {
                        if grid[ni][nj] != 0 {
                            let v = ni * c + nj;
                            adj[u + out_offset].push(v);
                            // 逆向きの辺
                            adj[v].push(u + out_offset);
                            // 残余辺
                            weight.insert((v, u + out_offset), 0);
                        }
                    } else {
                        adj[u + out_offset].push(t);
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
    let mut queue = vec![0usize; 3 * (2 * c * r + 1)];
    let mut path = Vec::with_capacity(2 * c * r + 1);
    let mut visited = vec![false; 2 * c * r + 1];
    let mut current_edge = vec![0usize; 2 * c * r + 1];
    'not_found: loop {
        path.truncate(0);
        path.resize(2 * c * r + 1, (0, 0));
        visited.truncate(0);
        visited.resize(2 * c * r + 1, false);
        visited[s] = true;
        let mut queue_head = 0;
        let mut queue_tail = 1;
        queue[0] = s;
        let mut level = vec![0u32; 2 * c * r + 1];
        
        while let Some(node) = {
            if queue_head == queue_tail {
                None
            } else {
                let node = queue[queue_head];
                queue_head += 1;
                Some(node)
            }
        } {
            assert!(visited[node]);
            // if visited[node] {
            //     continue;
            // }
            for &next in &adj[node] {
                if !visited[next] && redundant.get(&(node, next)).map(|&v| v > 0).unwrap_or(true) {
                    visited[next] = true;
                    level[next] = level[node] + 1;
                    queue[queue_tail] = next;
                    queue_tail += 1;
                }
            }
        }
        eprintln!("{:?}", level);

        path[0] = (s, usize::MAX);
        let mut i = 0;
        'outer: loop {
            let (node, _) = path[i];
            let mut current_edge_i = current_edge[node];
            let current_edge_level = level[node];
            loop {
                if current_edge_i >= adj[node].len() {
                    if i == 0 {
                        break 'not_found;
                    }
                    i -= 1;
                    current_edge[path[i].0] += 1;
                    break;
                }
                let next = adj[node][current_edge_i];
                if current_edge_level + 1 == level[next] && redundant.get(&(node, next)).map(|&v| v > 0).unwrap_or(true) {
                    i += 1;
                    path[i] = (next, current_edge_i);
                    current_edge[node] = current_edge_i;
                    if next == t {
                        break 'outer;
                    }
                    break;
                }
                current_edge_i += 1;
            }
        }
        eprintln!("{:?}", current_edge);
        for (i, &e) in current_edge.iter().enumerate() {
            eprintln!("{i}: {:?}", adj[i].get(e));
        }
        eprintln!("{:?}", path);
        // while let Some((node, prev)) = stack.pop() {
        //     if visited[node] {
        //         continue;
        //     }
        //     visited[node] = true;
        //     path[node] = prev;
        //     if node == t {
        //         break;
        //     }
        //     for &next in &adj[node] {
        //         if !visited[next] && redundant.get(&(node, next)).map(|&v| v > 0).unwrap_or(true) {
        //             stack.push((next, node));
        //         }
        //     }
        // }

        let mut min_flow = i64::MAX;
        for e in path[..=i].windows(2) {
            let &[(u, _), (v, edge_i)] = e else {
                unreachable!();
            };
            assert_eq!(adj[u][edge_i], v);
            if let Some(&v) = redundant.get(&(u, v)) {
                min_flow = min_flow.min(v);
            }
        }

        max_flow += min_flow;
        for e in path[..=i].windows(2) {
            let &[(u, _), (v, edge_i)] = e else {
                unreachable!();
            };
            if let Some(v) = redundant.get_mut(&(u, v)) {
                *v -= min_flow;
                if *v == 0 {
                    current_edge[u] = edge_i + 1;
                }
            }
            if let Some(v) = redundant.get_mut(&(v, u)) {
                *v += min_flow;
            }
        }
    }
    println!("{}", max_flow)
}
