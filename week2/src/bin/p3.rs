use std::{
    collections::BTreeMap,
    fmt::{self, Display},
    str::FromStr,
};

fn read_line() -> String {
    let mut s = String::new();
    ::std::io::stdin().read_line(&mut s).unwrap();
    s
}

enum OrderType {
    Buy,
    Sell,
}

impl FromStr for OrderType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err(()),
        }
    }
}

struct Printer<T>(Option<T>);

impl<T: Display> Display for Printer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(inner) = &self.0 {
            write!(f, "{inner}")
        } else {
            write!(f, "-")
        }
    }
}

fn main() {
    let case_num = read_line().trim().parse::<usize>().unwrap();
    for _ in 0..case_num {
        let n = read_line().trim().parse::<usize>().unwrap();
        let mut buy_queue = BTreeMap::<_, i32>::new();
        let mut sell_queue = BTreeMap::<_, i32>::new();

        let mut stock_price = None;

        for _ in 0..n {
            let s = read_line();
            let mut line = s.split_whitespace();
            let ty = line.next().unwrap().parse::<OrderType>().unwrap();
            let shares = line.next().unwrap().parse::<i32>().unwrap();
            line.next();
            line.next();
            let desired_price = line.next().unwrap().parse::<i32>().unwrap();

            if let OrderType::Buy = ty {
                buy_queue
                    .entry(desired_price)
                    .and_modify(|remainder| {
                        *remainder += shares;
                    })
                    .or_insert(shares);
            } else {
                sell_queue
                    .entry(desired_price)
                    .and_modify(|remainder| {
                        *remainder += shares;
                    })
                    .or_insert(shares);
            }

            // process orders
            while let Some(mut buy_entry) = buy_queue.last_entry() {
                if let Some(mut sell_entry) = sell_queue.first_entry() {
                    if buy_entry.key() >= sell_entry.key() {
                        // deal is established
                        stock_price = Some(*sell_entry.key());
                        let b = buy_entry.get_mut();
                        let s = sell_entry.get_mut();
                        if s > b {
                            *s -= *b;
                            buy_entry.remove();
                        } else {
                            *b -= *s;
                            sell_entry.remove();
                            if *b == 0 {
                                buy_entry.remove();
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            println!(
                "{} {} {}",
                Printer(sell_queue.first_key_value().map(|(&k, _)| k)),
                Printer(buy_queue.last_key_value().map(|(&k, _)| k)),
                Printer(stock_price)
            );
        }
    }
}
