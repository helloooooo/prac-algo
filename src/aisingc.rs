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
    let field = read_vec2::<char>(h as u32);
    for j in 0..h {
        for k in 0..w {
            if field[j as usize][k as usize] == '#' {

            } 
        }
    } 
}

fn move_increment(field:&Vec<Vec<char>>,j:usize,k:usize,before:char,max:usize) -> i64 {
    if before == '#'{
        if field[j][k] == '.' {
            1  
            + if j - 1 >=  0 { move_increment(field, j-1, k, '.',max) }
            + if j + 1 <= max { move_increment(field, j+1, k, '.' max)}
            + if 
        }
    }
}