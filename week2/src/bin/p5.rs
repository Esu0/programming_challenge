use dmoj::scan;

fn solve(a: &[i32], b: &[i32]) -> usize {
    let mut ai = 1usize;
    let mut bi = 100usize;
    let mut max = 0;
    let mut am = a[ai - 1];
    let mut bm = b[bi - 1];
    'out: loop {
        while am == 0 {
            if ai == 100 {
                break 'out;
            }
            ai += 1;
            am = a[ai - 1];
        }
        while bm == 0 {
            if bi == 1 {
                break 'out;
            }
            bi -= 1;
            bm = b[bi - 1];
        }
        max = std::cmp::max(max, ai + bi);
        if am > bm {
            am -= bm;
            bm = 0;
        } else {
            bm -= am;
            am = 0;
        }
    }
    max
}

fn main() {
    let n = scan!(usize);
    let mut a = vec![0; 100];
    let mut b = vec![0; 100];
    for (ai, bi) in std::iter::repeat_with(|| scan!(u8, u8)).take(n) {
        a[ai as usize - 1] += 1;
        b[bi as usize - 1] += 1;
        println!("{}", solve(&a, &b));
    }
}
