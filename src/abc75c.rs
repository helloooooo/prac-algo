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

fn main() {
    let nm: Vec<i32> = read_vec();
    let edge: Vec<Vec<i32>> = read_vec2(nm[1] as u32);
    //let ans = hoge(&node,&nm[0],&nm[1]);
    let mut graph = vec![vec![false; nm[0] as usize]; nm[0] as usize];
    let mut visited = vec![false; nm[0] as usize];
    for i in 0..edge.len() {
        graph[(edge[i][0] - 1) as usize][(edge[i][1] - 1) as usize] = true;
        graph[(edge[i][1] - 1) as usize][(edge[i][0] - 1) as usize] = true;
    }
    let mut ans = 0;
    for i in 0..nm[1] {
        graph[(edge[i as usize][0] - 1) as usize][(edge[i as usize][1] - 1) as usize] = false;
        graph[(edge[i as usize][1] - 1) as usize][(edge[i as usize][0] - 1) as usize] = false;

        for j in 0..nm[0] {
            visited[j as usize] = false
        }
        dfs(0, &mut visited, &graph);
        let mut bridge = false;
        for j in 0..nm[0] {
            if visited[j as usize] == false {
                bridge = true;
            }
        }
        if bridge {
            ans += 1
        }
        graph[(edge[i as usize][0] - 1) as usize][(edge[i as usize][1] - 1) as usize] = true;
        graph[(edge[i as usize][1] - 1) as usize][(edge[i as usize][0] - 1) as usize] = true;
    }
    print!("{}", ans);
}
fn dfs(v: i32, visited: &mut Vec<bool>, graph: &Vec<Vec<bool>>) {
    visited[v as usize] = true;
    for v2 in 0..visited.len() {
        if graph[v as usize][v2 as usize] == false {
            continue;
        }
        if visited[v2 as usize] == true {
            continue;
        }
        dfs(v2 as i32, visited, graph);
    }
}
