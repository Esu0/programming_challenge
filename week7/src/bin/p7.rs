use std::io::BufRead;

#[allow(dead_code)]
mod suffix_array {
    use std::ops::Deref;

    pub struct SuffixArray<'a, T> {
        array: Vec<&'a [T]>,
    }

    impl<'a, T> Deref for SuffixArray<'a, T> {
        type Target = [&'a [T]];

        fn deref(&self) -> &Self::Target {
            &self.array
        }
    }

    impl<'a, T> SuffixArray<'a, T> {
        pub fn new_simple(array: &'a [T]) -> Self
        where
            T: Ord,
        {
            let mut suffixes = (0..array.len()).map(|i| &array[i..]).collect::<Vec<_>>();
            suffixes.sort_unstable();
            Self { array: suffixes }
        }
    }
}

fn main() {
    let mut buf = String::new();
    let mut s = String::new();
    let mut stdin = std::io::BufReader::new(std::io::stdin());
    loop {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        s.clear();
        let mut split = buf.split_ascii_whitespace();
        let Some(first) = split.next() else {
            break;
        };
        s.push_str(first);
        for w in split {
            s.push_str(w);
        }
        let sa = suffix_array::SuffixArray::new_simple(s.as_bytes());
        for l in 1..s.len() {
            let mut max_count = 0usize;
            let mut count = 0usize;
            for w in sa.windows(2) {
                let &[a, b] = w else {
                    unreachable!();
                };
                if a.get(..l)
                    .is_some_and(|x| b.get(..l).is_some_and(|y| x == y))
                {
                    count += 1;
                } else {
                    max_count = max_count.max(count);
                    count = 0;
                }
            }
            max_count = max_count.max(count);
            if max_count == 0 {
                break;
            } else {
                println!("{}", max_count + 1);
            }
        }
        println!();
    }
}
