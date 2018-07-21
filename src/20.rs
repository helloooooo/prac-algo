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

fn main(){
    let input = read_vec::<i32>();
    let mut sub = input.clone();
    let variable = sub.split_at(3);
    let mini = variable.1.iter().min().unwrap();
    let maxi = variable.1.iter().max().unwrap();
    let flag = if input[0] + input[1] <= input[2] * 2 {
            true
    } else {
        false
    };
    let ans  = if flag {
        input[0] * input[3] + input[1] * input[4] 
    } else {
        if input[3] > input[4] {
            if input[0] < input[2] * 2{
                (input[2] * mini) * 2+ (maxi - mini) * input[0]
            } else {
                2 * maxi * input[2]
            }
        } else {
            if input[1] < input[2] * 2 {
                (input[2] * mini) * 2 + (maxi - mini) * input[1]
            } else {
                2 * maxi * input[2]
            }
        }
    };
    println!("{}",ans);
}