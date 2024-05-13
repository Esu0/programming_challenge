use std::iter;

use segtree::{MaxQuery, MinQuery, SegTree};

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

mod segtree {
    use core::slice;
    use std::{
        alloc::Layout,
        cmp::Reverse,
        mem::MaybeUninit,
        ops::{Add, Bound, Deref, RangeBounds},
    };

    #[derive(Clone, Debug)]
    pub struct SegTree<T> {
        tree: Box<[T]>,
    }

    enum Cow<'a, T> {
        Owned(T),
        Borrowed(&'a T),
    }

    impl<T> Deref for Cow<'_, T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            match self {
                Self::Owned(value) => value,
                Self::Borrowed(value) => value,
            }
        }
    }

    impl<T> Cow<'_, T> {
        fn into_owned(self, f: impl FnOnce(&T) -> T) -> T {
            match self {
                Self::Owned(value) => value,
                Self::Borrowed(value_ref) => f(value_ref),
            }
        }
    }

    impl<T: Query> SegTree<T> {
        /// half_len != 0
        unsafe fn make_tree_ptr(half_len: usize, f: impl FnOnce(*mut T) -> usize) -> *mut [T] {
            let len = half_len * 2 - 1;
            let ptr = std::alloc::alloc(Layout::array::<T>(len).unwrap()) as *mut T;
            {
                let data_ptr = ptr.add(half_len - 1);
                let orig_len = f(data_ptr);
                for i in orig_len..half_len {
                    data_ptr.add(i).write(T::IDENT);
                }
            }

            Self::eval(ptr, half_len);

            std::ptr::slice_from_raw_parts_mut(ptr, len)
        }

        unsafe fn from_write_fn(half_len: usize, f: impl FnOnce(*mut T) -> usize) -> Self {
            Self {
                tree: Box::from_raw(Self::make_tree_ptr(half_len, f)),
            }
        }
        pub fn new(data: &[T]) -> Self {
            let orig_len = data.len();
            if orig_len == 0 {
                return Self { tree: Box::new([]) };
            }
            let half_len = orig_len.next_power_of_two();
            unsafe {
                Self::from_write_fn(half_len, |data_ptr| {
                    for (i, data_i) in data.iter().enumerate() {
                        data_ptr.add(i).write(data_i.query(&T::IDENT))
                    }
                    orig_len
                })
            }
        }

        unsafe fn eval(ptr: *mut T, half_len: usize) {
            for i in (0..(half_len - 1)).rev() {
                ptr.add(i)
                    .write((*ptr.add(i * 2 + 1)).query(&*ptr.add(i * 2 + 2)));
            }
        }

        fn get_lr(&self, range: impl RangeBounds<usize>) -> (usize, usize) {
            let size = self.tree.len() / 2 + 1;
            let l = match range.start_bound() {
                Bound::Excluded(s) => *s + 1,
                Bound::Included(s) => *s,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Excluded(e) => *e,
                Bound::Included(e) => *e + 1,
                Bound::Unbounded => size,
            };
            (l, r)
        }

        pub fn query(&self, range: impl RangeBounds<usize>) -> T {
            let (l, r) = self.get_lr(range);

            self.query_rec(l, r, 0, self.tree.len() / 2 + 1, 0)
                .into_owned(|v| v.query(&T::IDENT))
        }

        fn query_rec(&self, a: usize, b: usize, l: usize, r: usize, i: usize) -> Cow<'_, T> {
            if b <= l || r <= a {
                Cow::Owned(T::IDENT)
            } else if a <= l && r <= b {
                Cow::Borrowed(&self.tree[i])
            } else {
                let mid = (l + r) / 2;
                Cow::Owned(
                    self.query_rec(a, b, l, mid, i * 2 + 1)
                        .query(&self.query_rec(a, b, mid, r, i * 2 + 2)),
                )
            }
        }

        pub fn update(&mut self, i: usize, val: &T)
        where
            T: Clone,
        {
            self.update_rec(i, 0, self.tree.len() / 2 + 1, 0, val);
        }

        fn update_rec(&mut self, i: usize, l: usize, r: usize, j: usize, val: &T)
        where
            T: Clone,
        {
            if i < l || r <= i {
                return;
            }
            if l + 1 == r {
                self.tree[j] = val.clone();
            } else {
                let mid = (l + r) / 2;
                let ch1 = 2 * j + 1;
                let ch2 = ch1 + 1;
                self.update_rec(i, l, mid, ch1, val);
                self.update_rec(i, mid, r, ch2, val);
                self.tree[j] = self.tree[ch1].query(&self.tree[ch2]);
            }
        }
    }

    impl<I: Query> FromIterator<I> for SegTree<I> {
        fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
            let iter = iter.into_iter();
            let (size_min, size_max) = iter.size_hint();
            if size_max == Some(0) {
                Self { tree: Box::new([]) }
            } else {
                assert_ne!(size_min, 0);
                let half_len_min = size_min.next_power_of_two();
                let half_len_max = size_max.map(usize::next_power_of_two);
                if Some(half_len_min) == half_len_max {
                    let half_len = half_len_min;
                    unsafe {
                        Self::from_write_fn(half_len, move |data_ptr| {
                            let mut i = 0;
                            for item in iter {
                                data_ptr.add(i).write(item);
                                i += 1;
                            }
                            i
                        })
                    }
                } else {
                    let mut data = iter.collect::<Vec<_>>();
                    let orig_len = data.len();
                    unsafe {
                        Self::from_write_fn(orig_len.next_power_of_two(), move |data_ptr| {
                            let src = data.as_mut_ptr();
                            let cap = data.capacity();
                            std::mem::forget(data);
                            data_ptr.copy_from_nonoverlapping(src, orig_len);
                            // `I`のデストラクタは呼ばずにメモリの解放のみ行う
                            drop(Vec::from_raw_parts(
                                src as *mut MaybeUninit<I>,
                                orig_len,
                                cap,
                            ));
                            orig_len
                        })
                    }
                }
            }
        }
    }

    // impl<T> SegTree<MinQuery<T>>
    // where
    //     MinQuery<T>: Query,
    //     T: Copy,
    // {
    //     pub fn fill(&mut self, val: T) {
    //         self.tree.fill(MinQuery(val));
    //     }
    // }

    // impl<T> SegTree<MaxQuery<T>>
    // where
    //     MaxQuery<T>: Query,
    //     T: Copy,
    // {
    //     pub fn fill(&mut self, val: T) {
    //         self.tree.fill(MaxQuery(val));
    //     }
    // }

    impl<T> IntoIterator for SegTree<T> {
        type IntoIter = std::iter::Skip<std::vec::IntoIter<T>>;
        type Item = T;

        fn into_iter(self) -> Self::IntoIter {
            let len = self.tree.len();
            let skip = (len / 2).saturating_sub(1);
            self.tree.into_vec().into_iter().skip(skip)
        }
    }

    impl<'a, T> IntoIterator for &'a SegTree<T> {
        type IntoIter = std::iter::Skip<std::slice::Iter<'a, T>>;
        type Item = &'a T;

        #[allow(clippy::iter_skip_zero)]
        fn into_iter(self) -> Self::IntoIter {
            let skip = (self.tree.len() / 2).saturating_sub(1);
            self.tree.iter().skip(skip)
        }
    }

    trait HasAddIdent {
        const IDENT: Self;
    }

    macro_rules! has_ident_num_impl {
        ($t:ty) => {
            impl HasAddIdent for $t {
                const IDENT: Self = 0;
            }
        };
    }

    has_ident_num_impl! {u8}
    has_ident_num_impl! {u16}
    has_ident_num_impl! {u32}
    has_ident_num_impl! {u64}
    has_ident_num_impl! {u128}
    has_ident_num_impl! {i8}
    has_ident_num_impl! {i16}
    has_ident_num_impl! {i32}
    has_ident_num_impl! {i64}
    has_ident_num_impl! {i128}

    trait HasMin {
        const MIN: Self;
    }

    macro_rules! has_min_num_impl {
        ($t:ty) => {
            impl HasMin for $t {
                const MIN: Self = <$t>::MIN;
            }
        };
    }

    has_min_num_impl! {u8}
    has_min_num_impl! {u16}
    has_min_num_impl! {u32}
    has_min_num_impl! {u64}
    has_min_num_impl! {u128}
    has_min_num_impl! {i8}
    has_min_num_impl! {i16}
    has_min_num_impl! {i32}
    has_min_num_impl! {i64}
    has_min_num_impl! {i128}

    trait HasMax {
        const MAX: Self;
    }

    macro_rules! has_max_num_impl {
        ($t:ty) => {
            impl HasMax for $t {
                const MAX: Self = <$t>::MAX;
            }
        };
    }

    has_max_num_impl! {u8}
    has_max_num_impl! {u16}
    has_max_num_impl! {u32}
    has_max_num_impl! {u64}
    has_max_num_impl! {u128}
    has_max_num_impl! {i8}
    has_max_num_impl! {i16}
    has_max_num_impl! {i32}
    has_max_num_impl! {i64}
    has_max_num_impl! {i128}

    impl<T: HasMax> HasMin for Reverse<T> {
        const MIN: Self = Self(<T as HasMax>::MAX);
    }

    impl<T: HasMin> HasMax for Reverse<T> {
        const MAX: Self = Self(<T as HasMin>::MIN);
    }

    pub trait Query {
        const IDENT: Self;
        fn query(&self, other: &Self) -> Self;
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SumQuery<T>(pub T);

    impl<T> SumQuery<T> {
        pub fn slice_from(slice: &[T]) -> &[Self] {
            let data = slice.as_ptr();
            let len = slice.len();
            unsafe { slice::from_raw_parts(data as _, len) }
        }
    }

    impl<T: Add<Output = T> + Clone + HasAddIdent> Query for SumQuery<T> {
        const IDENT: Self = Self(<T as HasAddIdent>::IDENT);
        fn query(&self, other: &Self) -> Self {
            Self(self.0.clone() + other.0.clone())
        }
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MaxQuery<T>(pub T);

    impl<T> MaxQuery<T> {
        pub fn slice_from(slice: &[T]) -> &[Self] {
            let data = slice.as_ptr();
            let len = slice.len();
            unsafe { slice::from_raw_parts(data as _, len) }
        }
    }

    impl<T: Ord + Clone + HasMin> Query for MaxQuery<T> {
        const IDENT: Self = Self(<T as HasMin>::MIN);
        fn query(&self, other: &Self) -> Self {
            Self(std::cmp::max(self.0.clone(), other.0.clone()))
        }
    }

    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MinQuery<T>(pub T);

    impl<T> MinQuery<T> {
        pub fn slice_from(slice: &[T]) -> &[Self] {
            let data = slice.as_ptr();
            let len = slice.len();
            unsafe { slice::from_raw_parts(data as _, len) }
        }
    }

    impl<T: Ord + Clone + HasMax> Query for MinQuery<T> {
        const IDENT: Self = Self(<T as HasMax>::MAX);
        fn query(&self, other: &Self) -> Self {
            Self(std::cmp::min(self.0.clone(), other.0.clone()))
        }
    }
}

fn main() {
    let n = scan!(u16) as usize;
    if n == 0 {
        println!("0");
        return;
    }
    let w = (0..n).map(|_| scan!(u16) as u32).collect::<Vec<_>>();
    let mut dp1 = SegTree::from_iter(iter::repeat(MaxQuery(0u32)).take(10000));
    for &wi in w.iter().rev() {
        let max = dp1.query(..wi as usize).0 + 1;
        dp1.update(wi as usize, &MaxQuery(max));
    }

    let mut dp2 = SegTree::from_iter(iter::repeat(MaxQuery(0u32)).take(10000));
    for &wi in w.iter().rev() {
        let max = dp2.query((wi + 1) as usize..).0 + 1;
        dp2.update(wi as usize, &MaxQuery(max));
    }

    let ans = (&dp1)
        .into_iter()
        .zip(&dp2)
        .map(|(a, b)| a.0 + b.0)
        .max()
        .unwrap()
        - 1;
    println!("{}", ans);
}
