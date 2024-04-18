#[allow(unused_imports)]
#[macro_use] extern crate dmoj;

fn main() {
    let (n, m) = scan!(usize, usize);
    let p = (0..n).map(|_| (0..m).map(|_| scan!(i64)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut dp = vec![vec![0i64; m]; n];
    {
        let mut sum = 0;
        dp[0] = p[0].iter().map(|&pi| { sum += pi; sum}).collect();
    }
    {
        let mut sum = 0;
        dp.iter_mut().zip(&p).for_each(|(dp_row, p_row)| { sum += p_row[0]; dp_row[0] = sum; });
    }
    for i in 1..n {
        for j in 1..m {
            dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]) + p[i][j];
        }
    }
    let mut output_iter = dp.into_iter().map(|row| row.last().copied().unwrap());
    if let Some(t1) = output_iter.next() {
        print!("{t1}");
    }
    for ti in output_iter {
        print!(" {ti}");
    }
    println!();
}
