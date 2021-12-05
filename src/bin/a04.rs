type BingoBoard = Vec<Vec<i32>>;

fn main() {
    let file = std::fs::read_to_string("input/04").unwrap();
    let lines: Vec<_> = file.lines().collect();
    let called_numbers: Vec<i32> = lines[0].split(',').map(|n| n.parse().unwrap()).collect();

    let bingo_boards: Vec<BingoBoard> = lines[2..]
        .chunks(6)
        .map(|rows| {
            rows.into_iter()
                .map(|row| row.split_whitespace().map(|n| n.parse().unwrap()).collect())
                .collect()
        })
        .collect();

    let scores: Vec<_> = bingo_boards
        .iter()
        .filter_map(|board| check_board(&called_numbers, board))
        .collect();
    println!(
        "1: {}",
        scores
            .iter()
            .min_by_key(|(nth_called, _score)| nth_called)
            .unwrap()
            .1
    );
    println!(
        "2: {}",
        scores
            .iter()
            .max_by_key(|(nth_called, _score)| nth_called)
            .unwrap()
            .1
    );
}

fn check_board(called_numbers: &[i32], board: &BingoBoard) -> Option<(usize, i32)> {
    let mut seen = [[false; 5]; 5];
    for (nth_called, &called) in called_numbers.iter().enumerate() {
        for (i, &value) in board.iter().flatten().enumerate() {
            let (row, col) = (i / 5, i % 5);
            if value == called {
                seen[row][col] = true;
            }
        }
        if (0..5).any(|i| (0..5).all(|j| seen[i][j]) || (0..5).all(|j| seen[j][i])) {
            let mut unseen_total = 0;
            for (i, seen) in seen.iter().flatten().enumerate() {
                let (row, col) = (i / 5, i % 5);
                if !seen {
                    unseen_total += board[row][col];
                }
            }
            return Some((nth_called, unseen_total * called));
        }
    }
    None
}
