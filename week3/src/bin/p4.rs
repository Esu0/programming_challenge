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
    let mut s = scan!(String).into_bytes();
    s.reverse();
    let len = s.len();
    let mut buf = vec![0u8; len];
    let mut min = vec![0xFFu8; len];
    for i in 1..(len - 1) {
        for j in 1..(len - i) {
            let (a, b) = s.split_at(i);
            let (b, c) = b.split_at(j);
            let (buf_b, buf_a) = buf.split_at_mut(len - i);
            let (buf_c, buf_b) = buf_b.split_at_mut(len - i - j);
            buf_c.copy_from_slice(c);
            buf_b.copy_from_slice(b);
            buf_a.copy_from_slice(a);

            if buf < min {
                min.copy_from_slice(&buf);
            }
        }
    }
    println!("{}", String::from_utf8(min).unwrap());
}
