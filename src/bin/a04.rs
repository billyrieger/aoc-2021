const LEN: usize = 5;
type BingoBoard = Vec<i32>;

fn main() {
    let file = std::fs::read_to_string("input/04.txt").unwrap();
    let lines: Vec<_> = file.lines().collect();
    let called_numbers: Vec<i32> = lines[0].split(',').map(|n| n.parse().unwrap()).collect();

    let mut scores: Vec<(usize, i32)> = lines[2..]
        .chunks(LEN + 1)
        .map(|rows| {
            rows.into_iter()
                .flat_map(|row| row.split_whitespace().map(|n| n.parse().unwrap()))
                .collect::<BingoBoard>()
        })
        .filter_map(|board| check_board(&called_numbers, &board))
        .collect();

    scores.sort_by_key(|&(t, _)| t);

    let (_, winning_score) = scores.first().unwrap();
    let (_, losing_score) = scores.last().unwrap();

    println!("1: {}", winning_score);
    println!("2: {}", losing_score);
}

fn check_board(called_numbers: &[i32], board: &BingoBoard) -> Option<(usize, i32)> {
    let mut seen = [false; LEN * LEN];
    for (nth_called, &called) in called_numbers.iter().enumerate() {
        for (_, seen) in board.iter().zip(&mut seen).filter(|&(&v, _)| v == called) {
            *seen = true;
        }
        let is_bingo = (0..LEN)
            .any(|i| (0..LEN).all(|j| seen[i * LEN + j]) || (0..LEN).all(|j| seen[j * LEN + i]));
        if is_bingo {
            let unseen_total: i32 = seen
                .iter()
                .zip(board.iter())
                .filter_map(|(seen, value)| if !seen { Some(value) } else { None })
                .sum();
            return Some((nth_called, unseen_total * called));
        }
    }
    None
}
