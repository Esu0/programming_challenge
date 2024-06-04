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
mod z_algorithm {
    use std::ops::Deref;

    pub struct ZArray {
        data: Vec<usize>,
    }

    impl Deref for ZArray {
        type Target = [usize];

        fn deref(&self) -> &Self::Target {
            &self.data
        }
    }

    impl ZArray {
        pub fn new<T: Eq>(slice: &[T]) -> Self {
            let mut z = Vec::with_capacity(slice.len() - 1);
            let (mut l, mut r) = (0, 1);
            while z.len() < slice.len() - 1 {
                let i = z.len() + 1;
                if z.get(i - l - 1).is_some_and(|&x| x + i < r) {
                    z.push(z[i - l - 1]);
                } else {
                    l = i;
                    r = i.max(r);
                    while slice.get(r).is_some_and(|x| x == &slice[r - l]) {
                        r += 1;
                    }
                    z.push(r - l);
                }
            }
            Self { data: z }
        }

        pub fn into_vec(self) -> Vec<usize> {
            self.data
        }
    }
}

fn main() {
    'next_case: loop {
        let s = scan!(String);
        if s.starts_with('.') {
            break;
        }
        let s = s.into_bytes();
        let z = z_algorithm::ZArray::new(&s);
        for i in 1..s.len() {
            if s.len() % i == 0 && z[i - 1] + i == s.len() {
                println!("{}", s.len() / i);
                continue 'next_case;
            }
        }
        println!("1");
    }
}
