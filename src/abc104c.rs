// fn read<T: std::str::FromStr>() -> T {
//     let mut s = String::new();
//     std::io::stdin().read_line(&mut s).ok();
//     s.trim().parse().ok().unwrap()
// }

// fn read_vec<T: std::str::FromStr>() -> Vec<T> {
//     read::<String>()
//         .split_whitespace()
//         .map(|e| e.parse().ok().unwrap())
//         .collect()
// }

// fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
//     (0..n).map(|_| read_vec()).collect()
// }
// fn main(){
//     let dg = read_vec::<i64>();
//     let (d,g) = (dg[0],dg[1]);
//     let mut score :Vec<Vec<i64>> = Vec::new();
//     let mut pcn = read_vec2::<i64>(d as u32);
//     let mut ans :Vec<i64> = Vec::new();
//     for j in 0..d {
//         score.push(Vec::new());
//         for k in 1..pcn[j as usize][0]+1{
//                 if pcn[j as usize][0] == k{
//                     score[j as usize].push(k*(j+1)*100+pcn[j as usize][1]);
//                 } else{
//                      score[j as usize].push(k*(j+1)*100);
//                 }
//         }
//     }

//     println!("{:?}",score );
//     score.reverse();
//     for j in 0..d{
        
//         let mut count = 0;
//         'outer: for k in 0..j {
//             for l in 0..pcn[j as usize][0]{
//                 sub = sub -score[k as usize][l as usize];
//                 count += 1;
//                 if sub <= 0{
//                     break 'outer;
//                 }
//             }
//         }
//         if sub <= 0{
//             ans.push(count);
//         }
//     }
//     println!("{:?}",ans.iter().min().unwrap() );
// }