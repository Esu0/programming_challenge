use std::cmp::Ordering;

#[allow(unused_imports)]
#[macro_use]
extern crate dmoj;

fn main() {
    let (n, t) = dmoj::scan!(usize, u8);
    let a = (0..n).map(|_| dmoj::scan!(u32)).collect::<Vec<_>>();
    match t {
        1 => println!("7"),
        2 => match a[0].cmp(&a[1]) {
            Ordering::Greater => println!("Bigger"),
            Ordering::Equal => println!("Equal"),
            Ordering::Less => println!("Smaller"),
        },
        3 => {
            let mut a = a;
            a.truncate(3);
            a.sort_unstable();
            println!("{}", a[1]);
        }
        4 => {
            println!("{}", a.into_iter().map(|ai| ai as i64).sum::<i64>());
        }
        5 => {
            println!(
                "{}",
                a.into_iter()
                    .filter(|ai| *ai % 2 == 0)
                    .map(|ai| ai as i64)
                    .sum::<i64>()
            );
        }
        6 => {
            println!(
                "{}",
                String::from_utf8(
                    a.into_iter()
                        .map(|ai| b'a' + (ai % 26) as u8)
                        .collect::<Vec<_>>()
                )
                .unwrap()
            )
        }
        7 => {
            let mut i = 0;
            for _ in 0..n {
                if let Some(j) = a.get(i).copied() {
                    i = j as _;
                    if i == n - 1 {
                        println!("Done");
                        return;
                    }
                } else {
                    println!("Out");
                    return;
                }
            }
            println!("Cyclic");
        }
        _ => unreachable!(),
    }
}
