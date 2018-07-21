fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
fn main() {
    let n:i32 = read();
    let an:Vec<Vec<i32>> = read_vec2(2);
    let ans = run(an,n,0,0);
    println!("{}",ans);
}

fn run(an:Vec<Vec<i32>>,n:i32,ans:i32,i:i32) -> i32 {
   
    if  i > n {
        return ans 
    }
    let a1_ans:i32 = an[0].iter().take((i + 1) as usize).sum::<i32>();
    let a2_ans:i32 = an[1].iter().skip(i as usize).sum::<i32>();
    if a1_ans + a2_ans < ans {
        run(an,n,ans,i+1)
    } else {
         run(an,n,a1_ans + a2_ans,i+1)
    }
}