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
    let t = scan!(u8);
    for _ in 0..t {
        let m = scan!(usize);
        let mut wh = (0..m).map(|_| scan!(u32, u32)).collect::<Vec<_>>();
        wh.sort_unstable();
        let mut last_set = BTreeMap::<u32, u32>::new();
        let mut will_entry = Vec::<u32>::new();
        let mut prev_w = 0;
        for (w, h) in wh {
            if w != prev_w {
                for &h in &will_entry {
                    last_set.entry(h).and_modify(|c| *c += 1).or_insert(1);
                }
                will_entry.clear();
            }

            if let Some((&last_h, count)) = last_set.range_mut(..h).last() {
                will_entry.push(h);
                *count -= 1;
                if *count == 0 {
                    last_set.remove(&last_h);
                }
            } else {
                will_entry.push(h);
            }

            prev_w = w;
        }
        for &h in &will_entry {
            last_set.entry(h).and_modify(|c| *c += 1).or_insert(1);
        }
        println!("{}", last_set.into_values().sum::<u32>());
    }
}
