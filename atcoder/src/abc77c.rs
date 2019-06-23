use std::io::{stdin, Read, StdinLock};
use std::str::FromStr;

struct Scanner<'a> {
    cin: StdinLock<'a>,
}

impl<'a> Scanner<'a> {
    fn new(cin: StdinLock<'a>) -> Scanner<'a> {
        Scanner { cin: cin }
    }

    fn read1<T: FromStr>(&mut self) -> Option<T> {
        let token = self
            .cin
            .by_ref()
            .bytes()
            .map(|c| c.unwrap() as char)
            .skip_while(|c| c.is_whitespace())
            .take_while(|c| !c.is_whitespace())
            .collect::<String>();
        token.parse::<T>().ok()
    }

    fn read<T: FromStr>(&mut self) -> T {
        self.read1().unwrap()
    }
}
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
fn main() {
    // let cin = stdin();
    // let cin = cin.lock();
    // let mut sc = Scanner::new(cin);
    let r1: String = read();
    let r2: String = read();
    let r3: String = read();
    let r4: String = read();
    let mut v1: Vec<i32> = r2.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut v2: Vec<i32> = r3.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut v3: Vec<i32> = r4.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut counter: i32 = 0;
    v1.sort();
    v3.sort();
    for i in &v2 {
        for j in 0..v1.len() {
            let a1 = v1.binary_search(&i).unwrap();
            if a1 == 0 {
                break;
            }
            for k in 0..v1.len() {
                let a2 = v2.binary_search(&i).unwrap();
                if a2 == v2.len() {
                    break;
                }
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}
