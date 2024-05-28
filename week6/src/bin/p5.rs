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
    'next_case: loop {
        let n = scan!(i8);
        if n == -1 {
            break;
        }
        let n = n as usize;
        let rooms = (0..n).map(|_| {
            let e = scan!(i32);
            let m = scan!(u8);
            let links = (0..m).map(|_| scan!(u8) - 1).collect::<Vec<_>>();
            (e, links)
        });
        let (en, edges) = rooms.unzip::<_, _, Vec<_>, Vec<_>>();
        // eprintln!("{:?}", en);
        // eprintln!("{:?}", edges);
        let mut player_energy = vec![None; n];
        let mut reachable = vec![false; n];
        player_energy[0] = Some(100);
        reachable[n - 1] = true;
        for count in 0..=n {
            let mut updated = false;
            for (i, j) in edges
                .iter()
                .enumerate()
                .flat_map(|(i, v)| v.iter().map(move |&j| (i, j as usize)))
            {
                if reachable[j] {
                    reachable[i] = true;
                }
                if let Some(e) = player_energy[i] {
                    let e = e + en[j];
                    if e <= 0 {
                        continue;
                    }
                    if let Some(dst) = &mut player_energy[j] {
                        if e > *dst {
                            *dst = e;
                            updated = true;
                        }
                    } else {
                        player_energy[j] = Some(e);
                    }
                }
            }
            if count == n {
                if !updated {
                    break;
                }
                // room1から到達可能で、energyを無限に増やすことができるループが存在する
                eprintln!("{:?}", player_energy);
                eprintln!("{:?}", reachable);

                for _ in 0..n {
                    for (i, j) in edges
                        .iter()
                        .enumerate()
                        .flat_map(|(i, v)| v.iter().map(move |&j| (i, j as usize)))
                    {
                        if let Some(e) = player_energy[i] {
                            let e = e + en[j];
                            if e <= 0 {
                                continue;
                            }
                            if let Some(dst) = &mut player_energy[j] {
                                if e > *dst {
                                    *dst = e;
                                    if reachable[j] {
                                        // room1から到達可能で、energyを無限に増やすことができるループが存在し、そのループからroomNに到達可能
                                        println!("winnable");
                                        continue 'next_case;
                                    }
                                }
                            } else {
                                player_energy[j] = Some(e);
                            }
                        }
                    }
                }
                println!("hopeless");
                continue 'next_case;
            }
        }
        // room1から到達可能で、energyを無限に増やすことができるループが存在しない
        eprintln!("{:?}", player_energy);
        if player_energy[n - 1].is_some() {
            assert!(player_energy[n - 1].unwrap() > 0);
            println!("winnable");
        } else {
            println!("hopeless");
        }
    }
}
