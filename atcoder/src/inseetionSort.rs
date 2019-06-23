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
    let mut v = read_vec::<i64>();
    let ans = insertionSort(v,v.len());
    println!("---------");
    println!("{:?}",ans);
}
fn insertionSort(a:mut Vec<i64>,n) -> Vec<i64>{
    for i in 1..n {
        let v = a[i];
        let j = i - 1;
        while (j >= 0 &&  a[j] > v) {
            a[j + 1] = a[j];
            j--;
        }
        a[j + 1] = v;
        println!("{:?}",a);
    }
}
