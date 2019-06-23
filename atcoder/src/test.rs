fn run(an: Vec<i32>, cp: Vec<i32>, count: i32) -> i32 {
    let cons = cp.split_first();
    match cons {
        Some(x) => {
            let num: i32 = an.iter().filter(|m| m == &x.0).collect::<Vec<&i32>>().len() as i32;
            match x {
                num => run(an, x.1.to_vec(), count),
                _ => run(an, x.1.to_vec(), count + (x.0 - &num).abs()),
            }
        }
        None => count,
    }
}
