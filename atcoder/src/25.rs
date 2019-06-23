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
    let hw = read_vec::<i32>();
    let mut field = Vec::new();
    for _ in 0..hw[0]{
        let inp = read::<String>();
        field.push(inp.chars().collect::<Vec<char>>());
    }
    let mut ans = true;
    'outer: for i in 0..hw[0]{
      'inner:  for j in 0..hw[1] {
            let mut v = Vec::new();
            if j != 0{v.push(field[i as usize][(j-1) as usize])};
            if j != hw[1]- 1 {v.push(field[i as usize][(j+1) as usize])};
            if i != hw[0] -1 {v.push(field[(i+1) as usize][j as usize])};
            if i != 0{v.push(field[(i-1) as usize][j as usize])};
            v.push(field[i as usize][j as usize]);
            let check = v.into_iter().filter(|&x| x == '#').count();
            if field[i as usize][j as usize] == '#'{
                if check < 2 {
                    ans = false;
                    break 'outer;
                }
            }
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}

