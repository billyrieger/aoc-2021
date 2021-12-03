fn main() {
    let file = std::fs::read_to_string("input/02").unwrap();
    let lines: Vec<_> = file.lines().map(parse_line).collect();
    part1(&lines);
    part2(&lines);
}

fn parse_line(line: &str) -> (&str, i32) {
    let (dir, num) = line.split_once(' ').unwrap();
    let num = num.parse().unwrap();
    (dir, num)
}

fn part1(lines: &[(&str, i32)]) {
    let mut pos = 0;
    let mut depth = 0;
    for &(dir, num) in lines {
        match dir {
            "forward" => pos += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => panic!(),
        }
    }
    println!("1: {:?}", pos * depth);
}

fn part2(lines: &[(&str, i32)]) {
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for &(dir, num) in lines {
        match dir {
            "forward" => {
                pos += num;
                depth += aim * num;
            }
            "down" => aim += num,
            "up" => aim -= num,
            _ => panic!(),
        }
    }
    println!("2: {:?}", pos * depth);
}
