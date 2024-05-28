use std::collections::BTreeSet;

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
    let t = scan!(u16);
    let mut indegree = vec![0u32; 100_000];
    let mut edge_list = vec![vec![]; 100_000];
    let mut stacks = [BTreeSet::new(), BTreeSet::new()];
    for _ in 0..t {
        let (n, m) = scan!(usize, usize);
        let p = (0..n).map(|_| scan!(u8)).collect::<Vec<_>>();
        let deps = (0..m).map(|_| scan!(usize, usize));
        let edge_list = &mut edge_list[..n];
        let indegree = &mut indegree[..n];
        edge_list.fill_with(Vec::new);
        indegree.fill(0);
        for (u, v) in deps {
            edge_list[u - 1].push(v - 1);
            indegree[v - 1] += 1;
        }
        let stacks = &mut stacks;
        stacks[0].clear();
        stacks[1].clear();
        for i in 0..n {
            if indegree[i] == 0 {
                stacks[(p[i] - 1) as usize].insert(i);
            }
        }
        let tmp = (stacks.clone(), indegree.to_vec());

        // println!("{:?}", edge_list);
        // println!("{:?}", indegree);
        let mut current_lab = 0;
        let mut count1 = 0u32;
        loop {
            while let Some(i) = stacks[current_lab].pop_first() {
                // println!("i: {i}, stack: {:?}", stacks);
                for &j in &edge_list[i] {
                    // println!("\tj: {j}");
                    // println!("\tindegree: {}", indegree[j]);
                    indegree[j] -= 1;
                    if indegree[j] == 0 {
                        stacks[(p[j] - 1) as usize].insert(j);
                    }
                }
            }
            if stacks[0].is_empty() && stacks[1].is_empty() {
                break;
            }
            current_lab ^= 1;
            count1 += 1;
        }

        *stacks = tmp.0;
        indegree.copy_from_slice(&tmp.1);
        let mut current_lab = 1;
        let mut count2 = 0u32;
        loop {
            while let Some(i) = stacks[current_lab].pop_first() {
                // println!("i: {i}, stack: {:?}", stacks);
                for &j in &edge_list[i] {
                    // println!("\tj: {j}");
                    // println!("\tindegree: {}", indegree[j]);
                    indegree[j] -= 1;
                    if indegree[j] == 0 {
                        stacks[(p[j] - 1) as usize].insert(j);
                    }
                }
            }
            if stacks[0].is_empty() && stacks[1].is_empty() {
                break;
            }
            current_lab ^= 1;
            count2 += 1;
        }
        println!("{}", count1.min(count2));
    }
}
