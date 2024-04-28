use std::{collections::HashSet, io};


fn main() {
    let mut lines = io::stdin().lines().map(|r| r.unwrap());
    let line1 = lines.next().unwrap();
    let mut sp = line1.split_whitespace();
    let p = sp.next().unwrap().parse::<usize>().unwrap();
    let t = sp.next().unwrap().parse::<usize>().unwrap();
    let lines = lines.map(|s| {
        let mut splitted = s.split_whitespace();
        (splitted.next().unwrap().parse::<u8>().unwrap(), splitted.next().unwrap().parse::<u8>().unwrap())
    });
    let mut data = [[0u8; 100]; 100];
    for (i, j) in lines {
        data[i as usize - 1][j as usize - 1] = 1;
    }
    let s = data.iter().take(p).map(|r| &r[..t]).collect::<HashSet<_>>();
    println!("{}", s.len());
}
