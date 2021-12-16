use aoc_2021::*;

const LEN: usize = 5;
type BingoBoard = Vec<i32>;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("input/04.txt")?;
    let lines: Vec<_> = file.lines().collect();
    let called_numbers: Vec<i32> = lines[0].split(',').map(str::parse).try_collect()?;

    let _boards: Vec<Vec<i32>> = lines[2..]
        .chunks(LEN + 1)
        .map(|rows| {
            rows.iter()
                .flat_map(|row| row.split_whitespace().map(str::parse))
                .try_collect()
        })
        .try_collect()?;

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
    Ok(())
}

fn check_board(called_numbers: &[i32], board: &[i32]) -> Option<(usize, i32)> {
    let mut seen = [false; LEN * LEN];
    for (nth_called, &called) in called_numbers.iter().enumerate() {
        itertools::zip(board, &mut seen)
            .filter(|&(&v, _)| v == called)
            .for_each(|(_, seen)| *seen = true);
        let horizontal_bingo = (0..LEN).any(|i| (0..LEN).all(|j| seen[i * LEN + j]));
        let vertical_bingo = (0..LEN).any(|i| (0..LEN).all(|j| seen[j * LEN + i]));
        if horizontal_bingo || vertical_bingo {
            let unseen_total: i32 = seen
                .iter()
                .zip(board)
                .filter_map(|(seen, value)| if !seen { Some(value) } else { None })
                .sum();
            return Some((nth_called, unseen_total * called));
        }
    }
    None
}
