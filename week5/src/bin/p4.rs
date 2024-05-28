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
    let mut map = (0..n)
        .map(|_| scan!(String).into_bytes())
        .collect::<Vec<_>>();
    if n == 1 && m == 1 {
        if map[0][0] == b'1' {
            println!("4");
        } else {
            println!("0");
        }
        return;
    }
    let mut stack = Vec::new();
    let mut count = 0u32;
    for j in [0, m - 1] {
        for i in 0..n {
            if map[i][j] == b'0' {
                // start dfs

                stack.push((i, j));
                while let Some((i, j)) = stack.pop() {
                    if map[i][j] == 0 {
                        continue;
                    }
                    map[i][j] = 0;
                    for (di, dj) in [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)] {
                        let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
                        if ni < n && nj < m {
                            if map[ni][nj] == b'1' {
                                count += 1;
                            } else if map[ni][nj] == b'0' {
                                stack.push((ni, nj));
                            }
                        }
                    }
                }
            }
        }
    }
    for i in [0, n - 1] {
        for j in 1..m - 1 {
            if map[i][j] == b'0' {
                // start dfs
                stack.push((i, j));
                while let Some((i, j)) = stack.pop() {
                    if map[i][j] == 0 {
                        continue;
                    }
                    map[i][j] = 0;
                    for (di, dj) in [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)] {
                        let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
                        if ni < n && nj < m {
                            if map[ni][nj] == b'1' {
                                count += 1;
                            } else if map[ni][nj] == b'0' {
                                stack.push((ni, nj));
                            }
                        }
                    }
                }
            }
        }
    }
    // println!("{map:?}");
    count += map.first().unwrap().iter().filter(|&&c| c == b'1').count() as u32;
    count += map.last().unwrap().iter().filter(|&&c| c == b'1').count() as u32;
    count += map
        .iter()
        .map(|row| (row.first().unwrap() == &b'1') as u32 + (row.last().unwrap() == &b'1') as u32)
        .sum::<u32>();
    println!("{count}");
}
