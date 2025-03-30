use criterion::{Criterion, black_box, criterion_group, criterion_main};
use puccinia_s_checkmate::benchmark::BRATKO_KOPEC;
use puccinia_s_checkmate::mgen::*;
use puccinia_s_checkmate::val::Colour;

fn bench_move_gen(c: &mut Criterion) {
    let fen = BRATKO_KOPEC[0].0;
    let board = Board::from_fen(fen);
    c.bench_function("mgen BK1", |b| {
        b.iter(|| black_box(board.moves(false, false, None)))
    });
}

fn bench_in_check(c: &mut Criterion) {
    let fen = BRATKO_KOPEC[0].0;
    let board = Board::from_fen(fen);
    c.bench_function("in_check BK1", |b| {
        b.iter(|| black_box(board.in_check(Colour::white())))
    });
}

fn bench_mobility(c: &mut Criterion) {
    let fen = BRATKO_KOPEC[0].0;
    let board = Board::from_fen(fen);
    c.bench_function("mobility BK1", |b| b.iter(|| black_box(board.mobility())));
}

fn bench_pawn_structure(c: &mut Criterion) {
    let fen = BRATKO_KOPEC[0].0;
    let board = Board::from_fen(fen);
    c.bench_function("pawn_structure BK1", |b| {
        b.iter(|| black_box(board.score_pawn_structure()))
    });
}

criterion_group!(
    benches,
    bench_move_gen,
    bench_in_check,
    bench_mobility,
    bench_pawn_structure
);
criterion_main!(benches);
