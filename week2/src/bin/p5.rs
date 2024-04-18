use dmoj::scan;

fn main() {
    let n = scan!(usize);
    let (a, b) = std::iter::repeat_with(|| scan!(u8, u8)).take(n).unzip::<_, _, Vec<_>, Vec<_>>();
    
}