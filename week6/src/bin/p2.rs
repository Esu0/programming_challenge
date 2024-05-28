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

fn main() {
    loop {
        let (h, w) = scan!(usize, usize);
        if h == 0 && w == 0 {
            break;
        }
        let mut map = (0..h)
            .map(|_| {
                scan!(String)
                    .into_bytes()
                    .into_iter()
                    .map(|x| (x - b'0') as u32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut queue = BTreeMap::new();
        for j in 0..w {
            queue
                .entry(map[0][j])
                .and_modify(|x: &mut Vec<(usize, usize, usize, usize)>| {
                    x.push((0, j, usize::MAX, usize::MAX))
                })
                .or_insert_with(|| vec![(0, j, usize::MAX, usize::MAX)]);
        }
        // println!("{:?}", queue);
        let mut visited = vec![vec![false; w]; h];
        let mut path = vec![vec![(usize::MAX, usize::MAX); w]; h];
        'outer: loop {
            let Some(mut entry) = queue.first_entry() else {
                break;
            };
            let d = *entry.key();
            let (i, j, prev_i, prev_j) = entry.get_mut().pop().unwrap();
            if entry.get().is_empty() {
                entry.remove();
            }
            if visited[i][j] {
                continue;
            }
            path[i][j] = (prev_i, prev_j);
            if i == h - 1 {
                let (mut i, mut j) = (i, j);
                map.iter_mut().flatten().for_each(|x| *x += b'0' as u32);
                map[i][j] = ' ' as u32;
                // println!("{:?}", path);
                while path[i][j] != (usize::MAX, usize::MAX) {
                    let (ni, nj) = path[i][j];
                    map[ni][nj] = ' ' as u32;
                    i = ni;
                    j = nj;
                }
                for v in map {
                    v.into_iter()
                        .map(|x| x as u8)
                        .for_each(|x| print!("{}", x as char));
                    println!();
                }
                // println!("{}", d);
                break 'outer;
            }
            visited[i][j] = true;
            for (di, dj) in [
                (0, 1),
                (0, usize::MAX),
                (1, 0),
                (usize::MAX, 0),
                (1, 1),
                (1, usize::MAX),
                (usize::MAX, 1),
                (usize::MAX, usize::MAX),
            ] {
                let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
                if ni < h && nj < w && !visited[ni][nj] {
                    let nd = d + map[ni][nj];
                    queue
                        .entry(nd)
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push((ni, nj, i, j));
                }
            }
        }
        println!();
    }
}
