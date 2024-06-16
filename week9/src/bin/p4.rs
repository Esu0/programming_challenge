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
    let (n, t) = scan!(usize, u64);
    let positions = (0..n).map(|_| scan!(i64, i64, u64)).collect::<Vec<_>>();
    let mut time = 0;
    let mut i = 1;
    let mut records = Vec::new();
    while i < positions.len() {
        let (x1, y1, s1) = positions[i - 1];
        let (x2, y2, s2) = positions[i];
        while (s1..=s2).contains(&time) {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let dt = s2 - s1;
            let rec_x = dx as f64 / dt as f64 * (time - s1) as f64 + x1 as f64;
            let rec_y = dy as f64 / dt as f64 * (time - s1) as f64 + y1 as f64;
            records.push((rec_x, rec_y));
            time += t;
        }
        i += 1;
    }
    records.push({
        let &(x, y, _) = positions.last().unwrap();
        (x as f64, y as f64)
    });
    let records_distance = records
        .windows(2)
        .map(|w| {
            let &[(x1, y1), (x2, y2)] = w else {
                unreachable!()
            };
            ((x1 - x2).powf(2.) + (y1 - y2).powf(2.)).sqrt()
        })
        .sum::<f64>();
    let actual_distance = positions
        .windows(2)
        .map(|w| {
            let &[(x1, y1, _), (x2, y2, _)] = w else {
                unreachable!()
            };
            (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
        })
        .sum::<f64>();
    println!(
        "{:.10}",
        (actual_distance - records_distance) / actual_distance * 100.
    );
}
