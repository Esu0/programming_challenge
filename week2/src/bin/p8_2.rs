use std::{
    collections::HashMap,
    ops::{Add, Sub},
};

struct UnionFind<T> {
    uf: Vec<usize>,
    size: Vec<usize>,
    query: Vec<T>,
}

trait Query {
    fn query(&self, other: &Self) -> Self;
}

trait RevQuery: Query {
    /// other.query(output) == self
    fn rev_query(&self, other: &Self) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SumQuery<T>(T);

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
    fn new(data: Vec<T>) -> Self {
        let size = data.len();
        Self {
            uf: (0..size).collect(),
            size: vec![1; size],
            query: data,
        }
    }

    fn len(&self) -> usize {
        self.uf.len()
    }
}

impl<T: Query> UnionFind<T> {
    fn unite(&mut self, i: usize, j: usize) {
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

    fn find(&self, i: usize) -> usize {
        self.find_query(i).0
    }

    fn query(&self, i: usize) -> &T {
        self.find_query(i).1
    }

    fn find_query(&self, i: usize) -> (usize, &T) {
        let parent = self.uf[i];
        if parent == i {
            (i, &self.query[i])
        } else {
            self.find_query(parent)
        }
    }
}

struct DeletableUnionFind<T> {
    base: UnionFind<T>,
    index_map: Vec<usize>,
    index_map_rev: HashMap<usize, usize>,
    query_init: Vec<T>,
}

impl<T: Clone + RevQuery> DeletableUnionFind<T> {
    fn new(data: Vec<T>) -> Self {
        let base = UnionFind::new(data.clone());
        let size = base.len();
        Self {
            base,
            index_map: (0..size).collect(),
            index_map_rev: (0..size).map(|i| (i, i)).collect(),
            query_init: data,
        }
    }

    fn unite(&mut self, i: usize, j: usize) {
        self.base.unite(self.index_map[i], self.index_map[j]);
    }

    fn find_query(&self, i: usize) -> (usize, &T) {
        self.base.find_query(self.index_map[i])
    }

    fn query(&self, i: usize) -> &T {
        self.find_query(i).1
    }

    fn cut(&mut self, i: usize) {
        let root = self.find_query(i).0;
        let new_data = self.base.query[root].rev_query(&self.query_init[i]);
        self.base.query[root] = new_data;

        let len = self.base.len();
        let old = std::mem::replace(&mut self.index_map[i], len);
        self.index_map_rev.remove(&old).unwrap();
        self.index_map_rev.insert(len, i);

        self.base.uf.push(len);
        self.base.size.push(1);
        self.base.query.push(self.query_init[i].clone());
    }

    fn is_same(&self, i: usize, j: usize) -> bool {
        self.find_query(i).0 == self.find_query(j).0
    }
}

fn get_line() -> Option<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok()?;
    Some(s)
}

fn main() {
    while let Some(line) = get_line() {
        let mut line = line.split_whitespace();
        let Some(n) = line.next() else {
            return;
        };
        let Ok(n) = n.parse::<usize>() else {
            return;
        };
        let m = line.next().unwrap().parse::<usize>().unwrap();
        let mut uf =
            DeletableUnionFind::new((0..=n).map(|i| (SumQuery(1u64), SumQuery(i as u64))).collect());
        for _ in 0..m {
            let line = get_line().unwrap();
            let mut iter = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            match iter.next().unwrap() {
                1 => uf.unite(iter.next().unwrap(), iter.next().unwrap()),
                2 => {
                    let (p, q) = (iter.next().unwrap(), iter.next().unwrap());
                    if !uf.is_same(p, q) {
                        uf.cut(p);
                        uf.unite(p, q);
                    }
                }
                3 => {
                    let (n, s) = uf.query(iter.next().unwrap());
                    println!("{} {}", n.0, s.0);
                }
                _ => unreachable!(),
            }
        }
    }
}
