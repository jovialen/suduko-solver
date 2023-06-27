use criterion::{criterion_group, criterion_main, Criterion};
use std::str::FromStr;
use sudoku_solver::{StandardSudoku, Sudoku};

fn solve_easy(c: &mut Criterion) {
    let sudoku = StandardSudoku::from_str(
        "  9  2  5538 64  9162    3   3 27    546  1    7 1534 3  8 19 67  3  85  91   47 ",
    )
    .unwrap();

    let mut easy = c.benchmark_group("easy");
    easy.bench_function("1", |b| b.iter(|| sudoku.clone().solve()));
    easy.finish();
}

fn solve_medium(c: &mut Criterion) {
    let sudoku = StandardSudoku::from_str(
        "1 6    7448 6 1      94   6     923      3   329716   8       367 3   9  35 64  2",
    )
    .unwrap();

    let mut medium = c.benchmark_group("medium");
    medium.bench_function("1", |b| b.iter(|| sudoku.clone().solve()));
    medium.finish();
}

fn solve_hard(c: &mut Criterion) {
    let sudoku = StandardSudoku::from_str(
        "   75       2 8 45 5      2 7   1   8 1 492 7  9 2 6  41      8  3   4    8  456 ",
    )
    .unwrap();

    let mut hard = c.benchmark_group("hard");
    hard.bench_function("1", |b| b.iter(|| sudoku.clone().solve()));
    hard.finish();
}

fn solve_evil(c: &mut Criterion) {
    let sudoku = StandardSudoku::from_str(
        "  3  7  2  15  79  9      4        9 1   436   5 8    3  4           2   6   317 ",
    )
    .unwrap();

    let mut evil = c.benchmark_group("evil");
    evil.sample_size(10);
    evil.bench_function("1", |b| b.iter(|| sudoku.clone().solve()));
    evil.finish();
}

criterion_group!(benches, solve_easy, solve_medium, solve_hard, solve_evil);
criterion_main!(benches);
