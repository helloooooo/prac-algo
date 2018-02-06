//深さ優先探索
fn main() {
    let mut field:  [[&str ; 12];10] = 
      [["W",".",".",".",".",".",".",".",".","W","W","."],
            [".","W","W","W",".",".",".",".",".","W","W","W"],
            [".",".",".",".","W","W",".",".",".","W","W","."],
            [".",".",".",".",".",".",".",".",".","W","W","."],
            [".",".",".",".",".",".",".",".",".","W",".","."],
            [".",".","W",".",".",".",".",".",".","W",".","."],
            [".","W",".","W",".",".",".",".",".","W","W","."],
            ["W",".","W",".","W",".",".",".",".",".","W","."],
            [".","W",".","W",".",".",".",".",".",".","W","."],
            [".",".","W",".",".",".",".",".",".",".","W","."]
            ];
    let mut res = 0;
    for i in 0..10 {  
        for j in 0..12 {
            if field[i as usize][j as usize] == "W" {
                dfs(i,j,&mut field); 
                res += 1 ;
                }
        }
    }
    println!("{}",res);
}

fn dfs(x:i32, y:i32, f:&mut [[&str;12];10]) -> () {
    f[x as usize][y as usize] = ".";
    for i in -1..2 {
        for j in  -1..2{
            let dx = x + i;
            let dy = y + j;
            if 0 <= dx && dx < 10 && 0 <= dy && dy < 12 && f[dx as usize][dy as usize] == "W" { dfs(dx,dy,f);}
        }
    }
}
