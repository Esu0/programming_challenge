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
    loop {
        let n = scan!(usize);
        if n == 0 {
            return;
        }
        if n == 1 || n == 2 {
            panic!()
        }
        let points = (0..n).map(|_| scan!(u32, u32)).collect::<Vec<_>>();
        let mut edges = {
            let &(mut prev_x, mut prev_y) = points.last().unwrap();
            let mut edges = Vec::with_capacity(n);
            for &(x, y) in &points {
                edges.push(((x.abs_diff(prev_x).pow(2) + y.abs_diff(prev_y).pow(2)) as f64).sqrt());
                prev_x = x;
                prev_y = y;
            }
            edges
        };

        eprintln!("{:?}", edges);
        let mut angles = {
            let mut angles = Vec::with_capacity(n);
            let &(prev_x, prev_y) = points.last().unwrap();
            let mut prev_x = prev_x as f64;
            let mut prev_y = prev_y as f64;
            for i in 0..n {
                let &(x, y) = &points[i];
                let (x, y) = (x as f64, y as f64);
                let &(next_x, next_y) = &points[(i + 1) % n];
                let (next_x, next_y) = (next_x as f64, next_y as f64);
                let vec1 = (x - prev_x, y - prev_y);
                let vec2 = (x - next_x, y - next_y);
                let dot = vec1.0 * vec2.0 + vec1.1 * vec2.1;
                let vec1_len = edges[i];
                let vec2_len = edges[(i + 1) % n];
                let angle = (dot / (vec1_len * vec2_len)).acos();
                angles.push((i, angle));

                eprintln!("{dot}");
                prev_x = x;
                prev_y = y;
            }
            angles
        };
        eprintln!("{:?}", angles);
        eprintln!("{}", angles.iter().map(|&(_, a)| a).sum::<f64>());
        let mut n = n;
        while n > 3 {
            let min_angle = angles
                .iter()
                .enumerate()
                .min_by(|&(_, &(_, a1)), &(_, &(_, a2))| a1.partial_cmp(&a2).unwrap())
                .unwrap();
            let (idx, &(_, angle)) = min_angle;
            let idx_prev = (idx + n - 1) % n;
            let idx_next = (idx + 1) % n;
            let edge1 = edges[idx];
            let edge2 = edges[idx_next];

            let e3_2 = edge1 * edge1 + edge2 * edge2 - 2. * edge1 * edge2 * angle.cos();
            let e3 = e3_2.sqrt();
            let angle1 = ((e3_2 + edge1 * edge1 - edge2 * edge2) / (2. * e3 * edge1)).acos();
            let angle2 = ((e3_2 + edge2 * edge2 - edge1 * edge1) / (2. * e3 * edge2)).acos();
            let poly_angle1 = angles[idx_prev].1;
            let poly_angle2 = angles[idx_next].1;
            if poly_angle1 - angle1 > angle && poly_angle2 - angle2 > angle {
                angles[idx_prev].1 = poly_angle1 - angle1;
                angles[idx_next].1 = poly_angle2 - angle2;
                edges[idx_next] = e3;
                edges.remove(idx);
                angles.remove(idx);
                n -= 1;
            } else {
                break;
            }
        }
        print!("{}", n);
        for &(idx, _) in &angles {
            print!(" {} {}", points[idx].0, points[idx].1);
        }
        println!();
    }
}
