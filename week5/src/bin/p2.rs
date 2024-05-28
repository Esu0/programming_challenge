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
    let (w, h) = scan!(usize, usize);
    let map = (0..h)
        .map(|_| scan!(String).into_bytes())
        .collect::<Vec<_>>();
    let player_pos = map
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == b'P').map(move |j| (i, j)))
        .unwrap_or_else(|| unreachable!());

    let mut stack = Vec::new();
    let mut visited = vec![vec![false; w]; h];
    let mut gold = 0u32;
    stack.push(player_pos);
    while let Some((i, j)) = stack.pop() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        if map[i][j] == b'G' {
            gold += 1;
        }
        let flat = |(di, dj)| (i.wrapping_add(di), j.wrapping_add(dj));
        let it = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)]
            .into_iter()
            .map(&flat);
        if it.clone().any(|(ni, nj)| map[ni][nj] == b'T') {
            continue;
        }
        for (i, j) in it {
            if map[i][j] != b'#' {
                stack.push((i, j));
            }
        }
    }
    println!("{}", gold);
}
