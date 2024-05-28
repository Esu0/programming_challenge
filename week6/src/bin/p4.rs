use std::collections::{BTreeMap, BTreeSet};

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
    let p = scan!(u16) as usize;
    let t = scan!(u32) as usize;
    let pl = (0..t).map(|_| (scan!(u16) as usize, scan!(u16) as usize, scan!(u16) as u32));
    let mut edges = vec![vec![]; p];
    let mut w = vec![vec![]; p];
    for (i, j, l) in pl {
        if i == j {
            continue;
        }
        edges[i].push(j);
        edges[j].push(i);
        w[i].push(l);
        w[j].push(l);
    }
    let mut dist = vec![None; p];
    let mut prev = vec![vec![]; p];
    let mut queue = BTreeMap::new();
    queue.insert(0u32, vec![(0usize, usize::MAX, 0)]);
    while let Some(first_entry) = queue.first_entry() {
        let d = *first_entry.key();
        for (i, prev_i, l) in first_entry.remove() {
            if let Some(di) = dist[i] {
                if di == d {
                    prev[i].push((prev_i, l));
                }
                continue;
            }
            dist[i] = Some(d);
            prev[i] = vec![(prev_i, l)];
            // flower[i] = f;
            for (&j, &l) in edges[i].iter().zip(w[i].iter()) {
                let d = d + l;
                // let f = flower[i] + l;
                queue
                    .entry(d)
                    .and_modify(|entry| entry.push((j, i, l)))
                    .or_insert_with(|| vec![(j, i, l)]);
            }
        }
    }

    // eprintln!("{:?}", prev);
    let mut set = BTreeSet::from([p - 1]);
    let mut reached = vec![false; p];
    while let Some(i) = set.pop_first() {
        if i == 0 {
            continue;
        }
        reached[i] = true;
        for &(j, _) in &prev[i] {
            set.insert(j);
        }
    }
    let sum = prev
        .iter()
        .zip(&reached)
        .filter(|&(_, &reached)| reached)
        .map(|(v, _)| v)
        .flat_map(|v| v.iter().map(|&(_, l)| l))
        .sum::<u32>();
    println!("{}", sum * 2);
}
