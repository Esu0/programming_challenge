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
    'outer: loop {
        let c = scan!(u8);
        if c == 0 {
            break;
        }
        let codes = (0..c)
            .map(|_| scan!(String).into_bytes())
            .collect::<Vec<_>>();
        let code_map = codes
            .iter()
            .enumerate()
            .map(|(i, code)| (&code[..], i))
            .collect::<HashMap<_, _>>();
        let r = scan!(usize);
        let ex_rates = (0..r)
            .map(|_| {
                (
                    code_map[scan!(String).as_bytes()],
                    code_map[scan!(String).as_bytes()],
                    {
                        let tmp = scan!(String);
                        let (a, b) = tmp.split_once(':').unwrap();
                        (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
                    },
                )
            })
            .collect::<Vec<_>>();
        let mut edges = vec![vec![]; c as usize];
        let mut rates = vec![vec![]; c as usize];
        for &(i, j, (a, b)) in &ex_rates {
            edges[i].push(j);
            rates[i].push((a, b));
        }
        for i in 0..c as usize {
            let mut dist = vec![f64::NEG_INFINITY; c as usize];
            dist[i] = 0.0;
            for _ in 0..(c + 1) as usize {
                let mut updated = false;
                for &(i, j, (a, b)) in &ex_rates {
                    let rate = (b as f64 / a as f64).ln();
                    if dist[i] + rate > dist[j] {
                        dist[j] = dist[i] + rate;
                        updated = true;
                    }
                }
                if !updated {
                    break;
                }
            }
            // println!("{:?}", dist);
            if dist.iter().any(|x| x.is_nan()) {
                panic!()
            }
            if dist[i] > 0.0 {
                println!("Arbitrage");
                continue 'outer;
            }
            // for &(ii, jj, (a, b)) in &ex_rates {
            //     let rate = (b as f64 / a as f64).ln();
            //     if dist[ii] + rate > dist[jj] {
            //         println!("Arbitrage");
            //         continue 'outer;
            //     }
            // }
        }
        println!("Ok");
    }
}
