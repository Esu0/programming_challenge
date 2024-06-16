use std::str::FromStr;

mod input {
    use std::{
        cell::RefCell,
        fmt::Debug,
        io::Read,
        str::{FromStr, SplitWhitespace},
    };

    fn tokens_init() -> RefCell<SplitWhitespace<'static>> {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        RefCell::new(String::leak(buf).split_whitespace())
    }

    fn next_token() -> Option<&'static str> {
        thread_local! {
            static TOKENS: RefCell<SplitWhitespace<'static>> = tokens_init();
        }
        TOKENS.with_borrow_mut(|tokens| tokens.next())
    }

    #[allow(dead_code)]
    pub fn scan<T: FromStr>() -> Option<T>
    where
        T::Err: Debug,
    {
        next_token().map(|s| s.parse().unwrap())
    }

    #[macro_export]
    macro_rules! scan {
        ($t:ty $(,)?) => {
            $crate::input::scan::<$t>().unwrap()
        };
        ($($t:ty),+ $(,)?) => {
            ($($crate::input::scan::<$t>().unwrap()),*)
        };
    }
}

#[derive(Clone, Copy, Debug)]
struct OptionInt(Option<u32>);
impl FromStr for OptionInt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OptionInt(s.parse().ok()))
    }
}

#[derive(Clone, Copy, Debug)]
enum Command {
    Fd,
    Lt,
    Bk,
    Rt,
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fd" => Ok(Command::Fd),
            "lt" => Ok(Command::Lt),
            "rt" => Ok(Command::Rt),
            "bk" => Ok(Command::Bk),
            _ => Err(s.to_owned()),
        }
    }
}

impl Command {
    fn execute(self, arg: u32, state: (f64, f64, u32)) -> (f64, f64, u32) {
        match self {
            Command::Fd => {
                let (x, y, d) = state;
                let x = x + arg as f64 * (d as f64).to_radians().cos();
                let y = y + arg as f64 * (d as f64).to_radians().sin();
                (x, y, d)
            }
            Command::Lt => {
                let (x, y, d) = state;
                let d = (d + arg) % 360;
                (x, y, d)
            }
            Command::Bk => {
                let (x, y, d) = state;
                let x = x - arg as f64 * (d as f64).to_radians().cos();
                let y = y - arg as f64 * (d as f64).to_radians().sin();
                (x, y, d)
            }
            Command::Rt => {
                let (x, y, d) = state;
                let d = (d + 360 - arg) % 360;
                (x, y, d)
            }
        }
    }
}

fn main() {
    let cases = scan!(u8);
    for _ in 0..cases {
        let m = scan!(usize);
        let commands = (0..m)
            .map(|_| scan!(Command, OptionInt))
            .collect::<Vec<_>>();
        let mut s = commands.splitn(2, |(_, c)| c.0.is_none());
        let command1 = s.next().unwrap();
        let command2 = s.next();
        let prev_state = command1
            .iter()
            .fold((0.0, 0.0, 0), |state, &(command, arg)| {
                command.execute(arg.0.unwrap(), state)
            });

        let d_com = command2
            .map(|c| {
                c.iter().fold((0.0, 0.0, 0), |state, &(command, arg)| {
                    command.execute(arg.0.unwrap(), state)
                })
            })
            .unwrap_or((0., 0., 0));
        let mut i = 0u32;
        let unknown_arg_command = commands.iter().find(|(_, arg)| arg.0.is_none()).unwrap().0;
        loop {
            let mut state = prev_state;
            state = unknown_arg_command.execute(i, state);
            let d = state.2;
            let (dx, dy, _) = d_com;
            let (sind, cosd) = (d as f64).to_radians().sin_cos();
            let (dx, dy) = (dx * cosd - dy * sind, dx * sind + dy * cosd);
            let (x, y, _) = state;
            let (x, y) = (x + dx, y + dy);
            // println!("{i}, {:.10} {:.10}", x, y);
            if x.abs() < 1.0e-8 && y.abs() < 1.0e-8 {
                println!("{i}");
                break;
            }
            i += 1;
        }
    }
}
