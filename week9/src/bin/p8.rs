use std::{cmp::Reverse, collections::HashSet};

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
    let c = scan!(usize);
    let points = (0..c).map(|_| scan!(i32, i32)).collect::<HashSet<_>>();
    let mut points = points.into_iter().collect::<Vec<_>>();
    let c = points.len();
    if c == 1 {
        println!("0");
        return;
    }
    if c == 2 {
        let &[(x1, y1), (x2, y2)] = &points[..] else {
            unreachable!()
        };
        println!("{}", (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt());
        return;
    }
    let (idx, &(pivot_x, pivot_y)) = points.iter().enumerate().min_by_key(|&(_, &(x, y))| (y, Reverse(x))).unwrap();
    points.remove(idx);
    points.sort_unstable_by(|&(x1, y1), &(x2, y2)| {
        let (dx1, dy1) = (x1 - pivot_x, y1 - pivot_y);
        let (dx2, dy2) = (x2 - pivot_x, y2 - pivot_y);

        // let gcd1 = gcd(dx1.unsigned_abs(), dy1.unsigned_abs()) as i32;
        // let slope1 = (dx1 / gcd1, dy1 / gcd1);
        // let gcd2 = gcd(dx2.unsigned_abs(), dy2.unsigned_abs()) as i32;
        // let slope2 = (dx2 / gcd2, dy2 / gcd2);
        let cross = dx1 * dy2 - dx2 * dy1;
        if cross == 0 {
            (dx1 * dx1 + dy1 * dy1).cmp(&(dx2 * dx2 + dy2 * dy2))
        } else {
            let angle1 = (dy1 as f64).atan2(dx1 as f64);
            let angle2 = (dy2 as f64).atan2(dx2 as f64);
            angle1.partial_cmp(&angle2).unwrap()
        }
    });
    // eprintln!("{:?}", points);

    let mut i = 1;
    let mut convex_hull = vec![(pivot_x, pivot_y), points[0]];
    while i < points.len() {
        let (x3, y3) = points[i];
        while convex_hull.len() > 1 {
            let (x1, y1) = convex_hull[convex_hull.len() - 2];
            let (x2, y2) = convex_hull[convex_hull.len() - 1];
            let cross = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);
            if cross <= 0 {
                convex_hull.pop();
            } else {
                break;
            }
        }
        convex_hull.push(points[i]);
        i += 1;
    }
    // convex_hull.push((pivot_x, pivot_y));
    // eprintln!("{:?}", convex_hull);
    let mut j = 2usize;
    let mut max = 0;
    let n = convex_hull.len();
    for i in 0..n {
        let (x1, y1) = convex_hull[i];
        j = (j + n - 1) % n;
        let (x2, y2) = convex_hull[j];
        let mut prev = (x2 - x1).pow(2) + (y2 - y1).pow(2);
        loop {
            let (x3, y3) = convex_hull[(j + 1) % n];
            let dist = (x3 - x1).pow(2) + (y3 - y1).pow(2);
            if dist <= prev {
                break;
            } else {
                prev = dist;
                j = (j + 1) % n;
            }
        }
        max = max.max(prev);
    }
    println!("{}", (max as f64).sqrt());
}
