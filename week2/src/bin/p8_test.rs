use rand::Rng;

fn main() {
    let n = 10;
    let mut v = Vec::new();
    let m = 10;
    let mut rng = rand::thread_rng();
    for _ in 0..m {
        v.push((
            rng.gen_range(1..=2),
            rng.gen_range(1..=n),
            rng.gen_range(1..=n),
        ));
    }
    println!("{n} {}", m + n);
    v.iter().for_each(|vi| {
        println!("{} {} {}", vi.0, vi.1, vi.2);
    });
    for i in 1..=n {
        println!("3 {i}");
    }
}
