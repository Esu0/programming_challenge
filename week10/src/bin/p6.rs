use union_find::UnionFind;

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

#[allow(dead_code)]
mod union_find {
    use std::ops::{Add, Sub};

    pub struct UnionFind<T> {
        uf: Vec<usize>,
        size: Vec<usize>,
        query: Vec<T>,
    }

    pub trait Query {
        fn query(&self, other: &Self) -> Self;
    }

    pub trait RevQuery: Query {
        /// other.query(output) == self
        fn rev_query(&self, other: &Self) -> Self;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SumQuery<T>(T);

    impl<T: Add<Output = T> + Clone> Query for SumQuery<T> {
        fn query(&self, other: &Self) -> Self {
            Self(self.0.clone() + other.0.clone())
        }
    }

    impl<T: Add<Output = T> + Sub<Output = T> + Clone> RevQuery for SumQuery<T> {
        fn rev_query(&self, other: &Self) -> Self {
            Self(self.0.clone() - other.0.clone())
        }
    }

    impl<Q1: Query, Q2: Query> Query for (Q1, Q2) {
        fn query(&self, other: &Self) -> Self {
            (self.0.query(&other.0), self.1.query(&other.1))
        }
    }

    impl<Q1: RevQuery, Q2: RevQuery> RevQuery for (Q1, Q2) {
        fn rev_query(&self, other: &Self) -> Self {
            (self.0.rev_query(&other.0), self.1.rev_query(&other.1))
        }
    }

    impl Query for () {
        fn query(&self, _other: &Self) -> Self {}
    }

    impl<T> UnionFind<T> {
        pub fn new(data: Vec<T>) -> Self {
            let size = data.len();
            Self {
                uf: (0..size).collect(),
                size: vec![1; size],
                query: data,
            }
        }

        pub fn len(&self) -> usize {
            self.uf.len()
        }
    }

    impl<T: Query> UnionFind<T> {
        pub fn unite(&mut self, i: usize, j: usize) {
            let root_i = self.find(i);
            let root_j = self.find(j);
            if root_i != root_j {
                let size_i = self.size[root_i];
                let size_j = self.size[root_j];
                if size_i > size_j {
                    self.uf[root_j] = root_i;
                    self.size[root_i] = size_i + size_j;
                    let new_data = self.query[root_i].query(&self.query[root_j]);
                    self.query[root_i] = new_data;
                } else {
                    self.uf[root_i] = root_j;
                    self.size[root_j] = size_i + size_j;
                    let new_data = self.query[root_j].query(&self.query[root_i]);
                    self.query[root_j] = new_data;
                }
            }
        }

        pub fn find(&self, i: usize) -> usize {
            self.find_query(i).0
        }

        pub fn query(&self, i: usize) -> &T {
            self.find_query(i).1
        }

        pub fn find_query(&self, i: usize) -> (usize, &T) {
            let parent = self.uf[i];
            if parent == i {
                (i, &self.query[i])
            } else {
                self.find_query(parent)
            }
        }
    }
}

fn solve_a(a: f64, w: u32, xyr: &[(i32, i32, u32)]) -> bool {
    let n = xyr.len();
    let mut uf = UnionFind::new(vec![(); xyr.len() + 2]);
    for (i, &(x, _, r)) in xyr.iter().enumerate() {
        if ((x - r as i32) as f64) < a {
            uf.unite(i, n);
        }
        if ((w as i32 - x - r as i32) as f64) < a {
            uf.unite(i, n + 1);
        }
    }
    for (i, &(x1, y1, r1)) in xyr.iter().enumerate() {
        for (j, &(x2, y2, r2)) in xyr.iter().enumerate().skip(i + 1) {
            let dx = x1 as f64 - x2 as f64;
            let dy = y1 as f64 - y2 as f64;
            let d = dx.hypot(dy);
            if (d - (r1 + r2) as f64) < a {
                uf.unite(i, j);
            }
        }
    }
    uf.find(n) != uf.find(n + 1)
}

fn main() {
    let t = scan!(u8);
    for _ in 0..t {
        let w = scan!(u32);
        let n = scan!(u16) as usize;
        let xyr = (0..n).map(|_| scan!(i32, i32, u32)).collect::<Vec<_>>();
        let mut diff = f64::INFINITY;
        let mut l = 0.0;
        let mut r = w as f64;
        while diff > 10e-7 {
            let a = (l + r) / 2.0;
            if solve_a(a, w, &xyr) {
                l = a;
            } else {
                r = a;
            }
            diff = r - l;
        }
        println!("{}", r / 2.);
        // println!("{}", solve_a(2.8, w, &xyr));
    }
}
