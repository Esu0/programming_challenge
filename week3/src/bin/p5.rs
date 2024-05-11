use std::iter;

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
    let (a, d) = scan!(usize, usize);
    let mut aht = iter::repeat_with(|| scan!(i32, i32))
        .take(a)
        .collect::<Vec<_>>();
    let mut dht = iter::repeat_with(|| scan!(i32, i32))
        .take(d)
        .collect::<Vec<_>>();
    let aht_d = aht.clone();
    dht.reverse();
    let mut sum_h = 0;
    let mut sum_t = 0;
    for (h, t) in &mut aht {
        let tmp_h = sum_h + *h;
        *h = std::mem::replace(&mut sum_h, tmp_h);
        let tmp_t = sum_t + *t;
        *t = std::mem::replace(&mut sum_t, tmp_t);
    }
    let total_time = sum_t;

    let mut next_y2 = dht[0].0;
    let mut next_x2 = total_time - dht[0].1;
    let mut di = 0;
    for ((y1, x1), (dy1, dx1)) in aht.into_iter().zip(aht_d) {
        let next_x1 = x1 + dx1;
        let next_y1 = y1 + dy1;
        let (mut dy2, mut dx2) = loop {
            if y1 <= next_y2 {
                break dht[di];
            }
            di += 1;
            let (dy2, dx2) = dht[di];
            next_y2 += dy2;
            next_x2 += dx2;
        };
        let mut di = di;
        let mut next_y2 = next_y2;
        let mut next_x2 = next_x2;
        loop {
            {
                let (x2, y2) = (next_x2 - dx2, next_y2);
                let next_y2 = next_y2 - dy2;
                let intersec = dx2 * (y2 - y1) + (x2 - x1) * dy2;
                println!("ascent time: {x1} -> {next_x1}, height: {y1} -> {next_y1}");
                println!("descent time: {x2} -> {next_x2}, height: {y2} -> {next_y2}");
                let div = dx2 * dy1 + dx1 * dy2;
                let intersec_t = intersec * dx1 + x1 * (dx2 * dy1 + dx1 * dy2);
                let del = intersec as f64 / div as f64;
        
                if ((x1 * div)..=(next_x1 * div)).contains(&intersec_t) && ((x2 * div)..=(next_x2 * div)).contains(&intersec_t) {
                    println!("intersec: {}, {} (answer)", intersec_t as f64 / div as f64, y1 as f64 + del * dy1 as f64);
                    // println!("{}", intersec_t as f64 / div as f64);
                    // return;
                } else {
                    println!("intersec: {}, {}", x1 as f64 + del * dx1 as f64, y1 as f64 + del * dy1 as f64);
                }
            }
            if di >= d - 1 {
                break;
            }
            if next_y2 < y1 {
                break;
            }
            di += 1;
            (dy2, dx2) = dht[di];
            next_y2 += dy2;
            next_x2 += dx2;
        }
    }
    panic!("end")
}
