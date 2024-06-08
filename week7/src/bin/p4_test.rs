use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let s = (0..10)
            .map(|_| if rng.gen_bool(0.5) { b'A' } else { b'B' })
            .collect::<Vec<u8>>();
        println!("{}", std::str::from_utf8(&s).unwrap());
    }
}
