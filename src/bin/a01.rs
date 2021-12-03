#![feature(array_windows)]

fn main() {
    let file = std::fs::read_to_string("input/01").unwrap();
    let lines: Vec<i32> = file.lines().map(|line| line.parse().unwrap()).collect();

    let count_increasing = |lines: &[i32]| lines.array_windows().filter(|[a, b]| a < b).count();
    println!("1: {:?}", count_increasing(&lines));

    let summed: Vec<_> = lines.array_windows().map(|[a, b, c]| a + b + c).collect();
    println!("2: {:?}", count_increasing(&summed));
}
