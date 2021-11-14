use std::{env};

const BEAT:  char = 'x';
const EMPTY: char = '.';

fn multiple(ls: &[u32]) -> u32 {
    let mut res: u32 = 1;
    for x in ls {
        res *= x;
    }
    res
}

fn least_common_multiple(measures: &[u32]) -> u32 {
    let mut ms = measures.to_vec();
    for x in measures {
        for y in measures {
            if x > y && x % y == 0 && ms.contains(x) {
                ms.retain(|it| it != x);
                ms.push(x/y);
            }
        }
    }
    multiple(&ms[..])
}

fn mono(rhythm: u32, least_common_multiple: u32) -> String {
    let mut result = String::from("");
    for x in 0..least_common_multiple {
        if x % rhythm == 0 {
            result.push(BEAT);
        } else {
            result.push(EMPTY);
        }
    }
    result
}

fn poly(measures: &[u32], least_common_multiple: u32) -> String {
    let mut result = String::from("");

    for x in 0..least_common_multiple {
        if divides(x, measures) {
            result.push(BEAT);
        } else {
            result.push(EMPTY);
        }
    }
    result
}

fn print_rhythms(measures: &[u32]) {
    let lcm = least_common_multiple(measures);
    for measure in measures {
        println!("{}", mono(*measure, lcm));
    }
    println!("{}", poly(measures, lcm));
}

fn divides(x: u32, measures: &[u32]) -> bool {
    for measure in measures {
        if x % measure == 0 {
            return true;
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let current_exe = env::current_exe().unwrap().file_name().unwrap().to_string_lossy().into_owned();
        println!("
Prints out polyrhythms for a given combination of beats.
First rows are the individual beats, the last row is the combined rhythym.

Examples:

# Prints a 3:2 polyrhythm
{0} 3 2
x.x.x.
x..x..
x.xxx.

# Prints a 3:4:5 polyrhythm
{0} 3 4 5
x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..x..
x...x...x...x...x...x...x...x...x...x...x...x...x...x...x...
x....x....x....x....x....x....x....x....x....x....x....x....
x..xxxx.xxx.x..xx.x.xx..xx.xx.x.xx.xx..xx.x.xx..x.xxx.xxxx..
        ", current_exe)
    } else {
        let measures: Vec<u32> = args[1..].iter().map(|it|it.parse::<u32>().unwrap()).collect();
        print_rhythms(&measures[..]);
    }
}
