use std::sync::atomic::{AtomicUsize, Ordering::*};

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

static R: AtomicUsize = AtomicUsize::new(0);
static C: AtomicUsize = AtomicUsize::new(0);
fn solve(d: u32, map: &[Vec<u32>]) -> bool {
    let r = R.load(Relaxed);
    let c = C.load(Relaxed);
    // let mut reachable = vec![vec![false; c]; r];
    let mut stack = Vec::new();
    let mut visited = vec![vec![false; c]; r];
    for i in 0..r {
        if map[i][0] <= d {
            stack.push((i, 0usize));
        }
        while let Some((i, j)) = stack.pop() {
            if visited[i][j] {
                continue;
            }
            // println!("{}, {}", i, j);
            visited[i][j] = true;
            for (di, dj) in [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)] {
                let ni = i.wrapping_add(di);
                let nj = j.wrapping_add(dj);
                if ni < r && nj < c && map[ni][nj] <= d {
                    if nj == c - 1 {
                        return true;
                    }
                    stack.push((ni, nj));
                }
            }
        }
    }
    false
}

fn binary_search(map: &[Vec<u32>], l: u32, r: u32) -> u32 {
    if l == r {
        return l;
    }
    let m = (l + r) / 2;
    if solve(m, map) {
        binary_search(map, l, m)
    } else {
        binary_search(map, m + 1, r)
    }
}

fn main() {
    let (r, c) = scan!(usize, usize);
    R.store(r, Relaxed);
    C.store(c, Relaxed);
    let map = (0..r)
        .map(|_| (0..c).map(|_| scan!(u32)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", binary_search(&map, 0, 1_000_000));
}
