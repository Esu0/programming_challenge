use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
fn main() {
    let mut a = b"aabbccddeeffgghhiijjkkllmmnnooppqqrrssttuuv".to_owned();
    let mut rng = StdRng::seed_from_u64(38497981);
    a.shuffle(&mut rng);
    println!("{}", std::str::from_utf8(&a).unwrap());

    let mut char_count = std::array::from_fn::<_, 26, _>(|_| Vec::new());
    for (i, &c) in a.iter().enumerate() {
        char_count[(c - b'a') as usize].push(i);
    }
    if char_count
        .iter()
        .map(Vec::len)
        .filter(|&x| x % 2 == 1)
        .count()
        > 1
    {
        return;
    }
    println!("{:?}", char_count);
    let mut cost = vec![usize::MAX; a.len()];

    let mut cost_set = Vec::new();
    for count in &char_count {
        let mut i = 0;
        let mut j = count.len();
        while j - i > 1 {
            j -= 1;
            let x = count[i];
            let y = count[j];
            cost[x] = x;
            cost[y] = a.len() - y - 1;
            i += 1;
            cost_set.push((x, y, cost[x], cost[y]));
        }
    }
    println!("{:?}", cost);
    println!("{:?}", cost_set);

    println!(
        "left cost: {}",
        cost_set.iter().map(|&(_, _, cx, _)| cx).sum::<usize>()
    );
    println!(
        "right cost: {}",
        cost_set.iter().map(|&(_, _, _, cy)| cy).sum::<usize>()
    );

    let mut sum = 0;
    while !cost_set.is_empty() {
        let (min_x, min_y, min_cost_x, min_cost_y) = *cost_set
            .iter()
            .min_by_key(|&&(_, _, cx, cy)| cx + cy)
            .unwrap();
        sum += min_cost_x + min_cost_y;
        cost_set.retain(|&(x, y, _, _)| x != min_x && y != min_y);
        cost_set.iter_mut().for_each(|(x, y, cx, cy)| {
            if min_x < *x && *x < min_y {
                *cx -= 1;
            } else if *x > min_y {
                *cx -= 2;
            }
            if min_x < *y && *y < min_y {
                *cy -= 1;
            } else if *y < min_x {
                *cy -= 2;
            }
        });
        println!("{:?}", cost_set);
    }

    println!("{}", sum);
}
