//深さ優先探索
struct Input {
    n: i32,
    k: i32,
    v: Vec<i32>,
}

impl Input {
    fn dfs(&self, i: i32, sum: i32) -> bool {
        if i == self.n {
            return sum == self.k;
        };
        if self.dfs(i + 1, sum) {
            return true;
        };
        if self.dfs(i + 1, sum + self.v[i as usize]) {
            return true;
        };
        false
    }
}
fn main() {
    let input = Input {
        n: 4,
        k: 13,
        v: vec![1, 2, 4, 7],
    };
    if input.dfs(0, 0) {
        println!("Yes")
    } else {
        println!("NO")
    };
}
