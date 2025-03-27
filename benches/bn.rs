use criterion::{Criterion, black_box, criterion_group, criterion_main};
use puccinia_s_checkmate::mgen::*;

fn move_gen() -> Vec<Move> {
    let fen = "1k1r4/pp1b1R2/3q2pp/4p3/2B5/4Q3/PPP2B2/2K5 b - -"; // Bratko-Kopec Pos 1
    let board = Board::from_fen(fen);
    board.moves(false, false, None)
}

fn bench_move_gen(c: &mut Criterion) {
    c.bench_function("mgen BK1", |b| b.iter(|| move_gen()));
}

criterion_group!(benches, bench_move_gen);
criterion_main!(benches);
