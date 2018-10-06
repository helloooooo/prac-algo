use std::collections::HashMap;
use std::cmp::max;
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
    let n = read::<usize>();
    let an = read_vec2::<i64>(n as u32);
    let mut ans = vec![];
    let target = an.iter().filter(|x| x[2] >= 1).count();
    // let mut ans2 = vec![];
    for j in 0..n{
        for cx in 0..101{
            for cy in 0..101{
                let (x,y,h) = (an[j][0],an[j][1],an[j][2]);
                let diff = (x-cx).abs() +(y-cy).abs();
                if h >= 1{
                    ans.push((cx,cy,h+diff));
                }

            }
        }
    }
    let ans = ans.into_iter().fold(HashMap::new(),|mut map,x|{
        let y = map.get(&x).map(|i| i+1).unwrap_or(1);
        map.insert(x,y);
        map
    }).into_iter().collect::<Vec<((_,_,_),_)>>()
    .into_iter().filter(|z| z.1 == target).nth(0).unwrap();

    println!("{} {} {}",(ans.0).0,(ans.0).1,(ans.0).2 );
}