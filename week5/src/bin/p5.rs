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
    let dataset_num = scan!(u8);
    for _ in 0..dataset_num {
        let (n, m, l, s) = scan!(usize, usize, usize, usize);
        let initial = (0..s).map(|_| scan!(usize) - 1).collect::<Vec<_>>();
        let ije = (0..m).map(|_| (scan!(usize) - 1, scan!(usize) - 1, scan!(usize)));
        let mut edge_list = vec![vec![]; n];
        for (i, j, e) in ije {
            edge_list[i].push((j, e));
            edge_list[j].push((i, e));
        }
        let mut queue = BTreeMap::from([(0, initial)]);
        let mut visited = vec![false; n];
        let mut ans = 0;
        // let mut visited_count = 0;
        while let Some(mut entry) = queue.first_entry() {
            let i = entry.get_mut().pop().unwrap();
            let e = if entry.get().is_empty() {
                entry.remove_entry().0
            } else {
                *entry.key()
            };
            if visited[i] {
                continue;
            }
            visited[i] = true;
            ans += e;
            // visited_count += 1;
            for &(j, e) in &edge_list[i] {
                queue
                    .entry(e)
                    .and_modify(|v| v.push(j))
                    .or_insert_with(|| vec![j]);
            }
        }
        println!("{}", ans + (n - s) * l);
    }
}
