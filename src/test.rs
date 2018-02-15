fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
 
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}
 
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
fn main(){
let i1 = read_vec();
let mat = read_vec2(i1[0] as u32);
  
  for i in {0..h} {
    for j in {0..w} {
      let mut val = 0;
      for k in {-1..2} {
        for l in {-1..2} {
          let y = i + k;
          let x = j + l;
          if 0 <= y && y < h && 0 <= x && x < w {
            if mat[y as usize].chars().nth(x as usize).unwrap() == '#' {
              val += 1;
            }
          }
        }
      }
      if mat[i as usize].chars().nth(j as usize).unwrap() == '#' {
        print!("#");
      } else {
        print!("{}", val);
      }
    }
    println!("")
  }
} 