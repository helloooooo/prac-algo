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
    let (h,w) = {
        let t = read_vec::<i64>();
        (t[0],t[1])
    };
    let mut field:Vec<Vec<char>> = vec![];
    for _ in 0..h {
        field.push(read::<String>().chars().collect());
    }
    for j in 0..h {
        for k in 0..w {
            if 0 <= j - 1 && j + 1  < h &&  0 <= k-1 && k+1 < w{
                let mut v = vec![];
                v.push(field[(j-1) as usize][k as usize]);
                v.push(field[(j+1) as usize][k as usize]);
                v.push(field[j as usize][(k-1) as usize]);
                v.push(field[j as usize][(k+1) as usize]);
                if field[j as usize][k as usize] == '#'{
                    if v.into_iter().filter(|&x| x == '.').count() == 4 {
                        println!("No");
                        return;
                    }
                }
            } 
        }
    }
    println!("Yes");
}