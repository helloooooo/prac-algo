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
    let p = read_vec::<i64>();
    let (x1,y1,x2,y2) = (p[0],p[1],p[2],p[3]);
    let xd = x2 -x1;
    let yd = y2-y1;
    let ax = xd.abs();
    let ay = yd.abs();
    let mut x3 = 0;
    let mut x4 = 0;
    let mut y3 = 0;
    let mut y4 = 0;
    if xd >= 0 && yd >= 0{
         x3 = x2 - ay;
         y3 = y2 + ax;
         x4 = x1 - ay;
         y4 = y1 + ax; 
    } else if xd <= 0 && yd >= 0 {
        x3 =x2 - ay;
        y3 = y2 - ax;
        x4 = x1 - ay;
        y4 = y1 -ax;
    } else if  xd >= 0 && yd <= 0 {
        x3 =x2 + ay;
        y3 = y2 + ax;
        x4 = x1  + ay;
        y4 = y1  +ax;
    } else if xd <= 0 && yd <= 0{
      x3 =x2 + ay;
        y3 = y2 - ax;
        x4 = x1  + ay;
        y4 = y1 -ax;
    }
    println!("{} {} {} {}",x3,y3,x4,y4 );
}