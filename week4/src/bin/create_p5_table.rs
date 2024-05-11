fn solve(n: u8, k: u8) -> f64 {
    let mut dp = [[0.0f64; 10]; 101];
    dp[1] = [1.; 10];
    for i in 2..=n {
        for j in 0..=k {
            for l in j.saturating_sub(1)..=(j + 1).min(k) {
                dp[i as usize][j as usize] += dp[i as usize - 1][l as usize] / (k + 1) as f64;
            }
        }
    }
    // println!("{:?}", dp);
    dp[n as usize][..=k as usize].iter().sum::<f64>() / (k + 1) as f64
}

fn main() {
    for k in 0..=9 {
        print!("[");
        for n in 1..=100 {
            print!("{:.9}, ", solve(n, k) * 100.);
        }
        println!("],");
    }
}
