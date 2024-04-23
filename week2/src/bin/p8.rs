use std::{
    iter,
    ops::{Add, Bound, Range, RangeBounds},
};

use dmoj::scan;

struct UnionFindUndo<T> {
    stack: Vec<(ArrayUndo<usize>, ArrayUndo<usize>, ArrayUndo<T>)>,
    uf: Vec<usize>,
    size: Vec<usize>,
    query: Vec<T>,
}

struct ArrayUndo<T> {
    index: usize,
    value: T,
}

impl<T> ArrayUndo<T> {
    fn undo(self, slc: &mut [T]) {
        slc[self.index] = self.value;
    }

    fn assign(slc: &mut [T], index: usize, value: T) -> Self {
        Self {
            index,
            value: std::mem::replace(&mut slc[index], value),
        }
    }

    fn process_query(slc: &mut [T], dst: usize, index: usize) -> Self
    where
        T: Query,
    {
        let output = slc[dst].query(&slc[index]);
        Self::assign(slc, dst, output)
    }
}

trait Query {
    fn query(&self, other: &Self) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SumQuery<T>(T);

impl<T: Add<Output = T> + Clone> Query for SumQuery<T> {
    fn query(&self, other: &Self) -> Self {
        Self(self.0.clone() + other.0.clone())
    }
}

impl Query for () {
    fn query(&self, _other: &Self) -> Self {}
}

impl<T> UnionFindUndo<T> {
    fn new(data: Vec<T>) -> Self {
        let size = data.len();
        Self {
            stack: Vec::new(),
            uf: (0..size).collect(),
            size: vec![1; size],
            query: data,
        }
    }

    fn len(&self) -> usize {
        self.uf.len()
    }
}

impl<T: Query> UnionFindUndo<T> {
    fn unite(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            let size_i = self.size[root_i];
            let size_j = self.size[root_j];
            self.stack.push(if size_i > size_j {
                (
                    ArrayUndo::assign(&mut self.uf, root_j, root_i),
                    ArrayUndo::assign(&mut self.size, root_i, size_i + size_j),
                    ArrayUndo::process_query(&mut self.query, root_i, root_j),
                )
            } else {
                (
                    ArrayUndo::assign(&mut self.uf, root_i, root_j),
                    ArrayUndo::assign(&mut self.size, root_j, size_i + size_j),
                    ArrayUndo::process_query(&mut self.query, root_j, root_i),
                )
            });
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

    fn undo(&mut self) {
        let Some((p1, p2, p3)) = self.stack.pop() else {
            return;
        };
        p1.undo(&mut self.uf);
        p2.undo(&mut self.size);
        p3.undo(&mut self.query);
    }
}

struct SegTree<T> {
    data: Vec<T>,
}

impl<T> SegTree<T> {
    fn new_with(size: usize, init: impl FnMut() -> T) -> Self {
        Self {
            data: iter::repeat_with(init)
                .take(size.next_power_of_two() * 2)
                .collect(),
        }
    }

    fn dfs(
        &self,
        range: impl RangeBounds<usize>,
        mut query: impl FnMut(&T),
        mut undo: impl FnMut(&T),
    ) {
        let size = self.data.len() / 2;
        let l = match range.start_bound() {
            Bound::Excluded(l) => *l + 1,
            Bound::Included(l) => *l,
            Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            Bound::Excluded(r) => *r,
            Bound::Included(r) => *r + 1,
            Bound::Unbounded => size,
        };
        self.dfs_rec(l..r, &mut query, &mut undo, 0..size, 1);
    }

    fn dfs_rec(
        &self,
        range: Range<usize>,
        query: &mut impl FnMut(&T),
        undo: &mut impl FnMut(&T),
        current_range: Range<usize>,
        ind: usize,
    ) {
        if range.end <= current_range.start || current_range.end <= range.start {
            return;
        } else if current_range.end - current_range.start <= 1 {
            return;
        } else {
            let mid = (current_range.start + current_range.end) / 2;
            let ind1 = ind * 2;
            query(&self.data[ind1]);
            self.dfs_rec(
                range.clone(),
                query,
                undo,
                current_range.start..mid,
                ind * 2,
            );
            undo(&self.data[ind1]);
            let ind2 = ind1 + 1;
            query(&self.data[ind2]);
            self.dfs_rec(range, query, undo, mid..current_range.end, ind * 2 + 1);
            undo(&self.data[ind2]);
        }
    }
}

fn main() {}
