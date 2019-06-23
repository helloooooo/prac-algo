// 幅優先探索のサンプル
const N: usize = 10;
const M: usize = 10;
const INF: i32 = 1000000;
struct Points {
    start: (i32, i32),
    goal: (i32, i32),
}

impl Points {
    fn bfs(&self, d: &mut Vec<Vec<i32>>, maze: &mut [[&str; 10]; 10]) -> i32 {
        let mut v = Vec::new();
        let mut init_d: Vec<Vec<i32>> =
            d.iter().map(|x| x.iter().map(|_y| INF).collect()).collect();
        v.push(self.start);
        init_d[self.start.0 as usize][self.start.1 as usize] = 0;
        let dx = vec![1, 0, -1, 0];
        let dy = vec![0, 1, 0, -1];
        while !v.is_empty() {
            let v_pop = v.pop();
            if let Some(x) = v_pop {
                if x.0 == self.goal.0 && x.1 == self.goal.1 {
                    break;
                }
                let vectors: Vec<_> = dx.iter().zip(dy.iter()).collect();
                for i in 0..4 {
                    let nx = x.0 + vectors[i].0;
                    let ny = x.1 + vectors[i].1;
                    if 0 <= nx
                        && nx < 10
                        && 0 <= ny
                        && ny < 10
                        && maze[nx as usize][ny as usize] != "#"
                        && init_d[nx as usize][ny as usize] == INF
                    {
                        v.push((nx, ny));
                        init_d[nx as usize][ny as usize] = init_d[x.0 as usize][x.1 as usize] + 1;
                    }
                }
            }
        }
        init_d[self.goal.0 as usize][self.goal.1 as usize]
    }
}

struct PointsBuilder {
    start: (i32, i32),
    goal: (i32, i32),
}

impl PointsBuilder {
    fn new() -> PointsBuilder {
        PointsBuilder {
            start: (0, 0),
            goal: (0, 0),
        }
    }
    fn start(&mut self, coordinate: (i32, i32)) -> &mut PointsBuilder {
        self.start = coordinate;
        self
    }
    fn goal(&mut self, coordinate: (i32, i32)) -> &mut PointsBuilder {
        self.goal = coordinate;
        self
    }
    fn finalize(&self) -> Points {
        Points {
            start: self.start,
            goal: self.goal,
        }
    }
}

fn main() {
    let mut field: [[&str; 10]; 10] = [
        ["#", "S", "#", "#", "#", "#", "#", "#", ".", "#"],
        [".", ".", ".", ".", ".", ".", "#", ".", ".", "#"],
        [".", "#", ".", "#", "#", ".", "#", "#", ".", "#"],
        [".", "#", ".", ".", ".", ".", ".", ".", ".", "."],
        ["#", "#", ".", "#", "#", ".", "#", "#", "#", "#"],
        [".", ".", ".", ".", "#", ".", ".", ".", ".", "#"],
        [".", "#", "#", "#", "#", "#", "#", "#", ".", "#"],
        [".", ".", ".", ".", "#", ".", ".", ".", ".", "."],
        [".", "#", "#", "#", "#", ".", "#", "#", "#", "#"],
        [".", ".", ".", ".", "#", ".", ".", ".", "G", "#"],
    ];
    let mut d = vec![vec![0; N]; M];
    let p = PointsBuilder::new().start((0, 1)).goal((9, 8)).finalize();
    let res = p.bfs(&mut d, &mut field);
    println!("{}", res)
}
