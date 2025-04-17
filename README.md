<p align="center">
    <img src="Images/Mateus128x128.png" alt="Mateus Logo">
</p>

# Mateus

A rusty chess library:
* Principle variation negamax search with alpha beta pruning (See [1]).
* Transposition table to avoid re-searching cycles
* Evaluation based on material, pawn structure & mobility
* Checks draw by 3x repetition and 50 move rule
* Opening library

## References:

[1] ["An Analysis of Alpha-Beta Pruning", Donald E. Knuth and Ronald W. Moore, Artificial Intelligence 6 (1975), 293-326](http://www-public.telecom-sudparis.eu/~gibson/Teaching/Teaching-ReadingMaterial/KnuthMoore75.pdf) <br/>
[2] ["Computer Chess Methods", T.A. Marsland, ENCYCLOPEDIAOF ARTIFICIAL INTELLIGENCE, 1987](https://www.researchgate.net/publication/2404258_Computer_Chess_Methods) <br/>
[3] ["The Bratko-Kopec Experiment: A Comparison of Human and Computer Performance in Chess", D. Kopec and I Bratko](http://spider.sci.brooklyn.cuny.edu/~kopec) <br/>
[4] ["Kaufman Test"](https://www.chessprogramming.org/Kaufman_Test)<br/>
[5] ["Lasker Test"](https://www.chessprogramming.org/Lasker-Reichhelm_Position) <br/>

## Run:

```
% cargo run --release --bin main -- --help

Usage: main [OPTIONS]

Options:
  -n, --n <N>    break off search threshold - positions generated [default: 1000000]
  -m, --m <M>    number of moves before stopping [default: -1]
  -w, --w        play white (human-computer)
  -b, --b        play black (human-computer)
  -l, --l        no opening library
  -k, --k <K>    benchmark test sets - Bratko-Kopec (1) / Kaufman (2) / Lasker (3) / Nolot (4) / CCR (5) / ERET (6) / BT-2450 (7) / BT-2630 (8) [default: 0]
  -v, --v        verbose output
  -f, --f <F>    fen board - start position [default: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"]
  -h, --help     Print help
  -V, --version  Print version
```

Run CLI app like this to play white:
```
% cargo run --release --bin main -- -w 

```
![alt text](https://github.com/jesper-olsen/puccinia_s_checkmate/blob/main/Images/your_move.png "Game UI")




## Benchmarks

### Bratko-Kopec [3]

```
% cargo run --release --bin main -- -k 1 -n 10000000

Position  1; Searched:      5053, Score: 31995, Move (black): d6d1 = Qd1+; Expected: Qd1+
Position  2; Searched:  10857751, Score:    34, Move (white): c3c4 =  Rc4; Expected: d5
Position  3; Searched:  26330471, Score:   -40, Move (black): f6f5 =   f5; Expected: f5
Position  4; Searched:  14618456, Score:   -53, Move (white): e2b5 = Qb5+; Expected: e6
Position  5; Searched:  40539053, Score:   115, Move (white): h2h3 =   h3; Expected: Nd5,a4
Position  6; Searched:  12710119, Score:    71, Move (white): g5g6 =   g6; Expected: g6
Position  7; Searched:  12095574, Score:   -87, Move (white): a3b4 =  Bb4; Expected: Nf6
Position  8; Searched:  10853501, Score:   -26, Move (white): f4f5 =   f5; Expected: f5
Position  9; Searched:  16351054, Score:   107, Move (white): d1e1 =  Re1; Expected: f5
Position 10; Searched:  38598481, Score:   -23, Move (black): c6e5 =  Ne5; Expected: Ne5
Position 11; Searched:  24009460, Score:    11, Move (white): g3f5 =  Nf5; Expected: f4
Position 12; Searched:  29500433, Score:  -194, Move (black): d7f5 =  Bf5; Expected: Bf5
Position 13; Searched:  60198151, Score:   -96, Move (white): b2b4 =   b4; Expected: b4
Position 14; Searched:  28021723, Score:   265, Move (white): d1e1 =  Qe1; Expected: Qd2 Qe1
Position 15; Searched:  14378916, Score:    67, Move (white): g4g7 = Qxg7+; Expected: Qxg7+
Position 16; Searched:  10956309, Score:  -107, Move (white): d2e4 =  Ne4; Expected: Ne4
Position 17; Searched:  25544240, Score:   -83, Move (black): h7h6 =   h6; Expected: h5
Position 18; Searched:  45755108, Score:    38, Move (black): c5b3 =  Nb3; Expected: Nb3
Position 19; Searched:  57742411, Score:   100, Move (black): c7c5 =   c5; Expected: Rxe4
Position 20; Searched:  49364869, Score:   -26, Move (white): g3g4 =   g4; Expected: g4
Position 21; Searched:  19332797, Score:   259, Move (white): f5h6 =  Nh6; Expected: Nh6
Position 22; Searched:  27217536, Score:   -52, Move (black): f6h5 =  Nh5; Expected: Bxe4
Position 23; Searched:  25355535, Score:    -5, Move (black): b7b6 =   b6; Expected: f6
Position 24; Searched:  18752247, Score:    -4, Move (white): f2f4 =   f4; Expected: f4

Correct: [1, 3, 6, 8, 10, 12, 13, 14, 15, 16, 18, 20, 21, 24] 14/24
Points: 15.08
Time: 756878 ms => 31536 ms/position
Search total: 619089248; Time 756878 ms => 817 nodes/ms
```

### Kaufman [4]

```
% cargo run --release --bin main -- -k 2 -n 10000000

Position  1; Searched:  19413832, Score:   187, Move (white): e4f6 = Nf6+; Expected: Nf6+
Position  2; Searched:  43490588, Score:   -19, Move (black): a5d5 =  Qd5; Expected: Nxd4
Position  3; Searched:  51956742, Score:  -124, Move (white): h4h5 =   h5; Expected: Rd1
Position  4; Searched:  24503628, Score:   -82, Move (white): c2e3 =  Ne3; Expected: Rxb2
Position  5; Searched:  10725777, Score:   124, Move (black): g5c1 = Qxc1; Expected: Qxc1
Position  6; Searched:  15652975, Score:  -169, Move (black): b6b5 =   b5; Expected: Rxa2
Position  7; Searched:  27217536, Score:   -52, Move (black): f6h5 =  Nh5; Expected: Bxe4
Position  8; Searched:  15726110, Score:  -252, Move (black): e1f2 =  Qf2; Expected: h6
Position  9; Searched:  33684331, Score:   288, Move (white): f3e2 =  Be2; Expected: Be2
Position 10; Searched: 102418804, Score:   164, Move (black): d5f4 =  Nf4; Expected: Nxc3
Position 11; Searched:  12403849, Score: 31987, Move (white): g3f5 =  Nf5; Expected: Nf5
Position 12; Searched:  10511224, Score:    50, Move (black): c6d6 =  Rd6; Expected: Rd6
Position 13; Searched:  19858165, Score:   215, Move (white): c3d5 = Nxd5; Expected: Nxd5
Position 14; Searched:  16454811, Score:    91, Move (white): h6f4 =  Bf4; Expected: Rxb2
Position 15; Searched:  27974289, Score:   264, Move (white): g3f5 = Nxf5; Expected: Bxf5
Position 16; Searched:  15819502, Score:   141, Move (white): e4e5 =   e5; Expected: b4
Position 17; Searched:  13934208, Score:    37, Move (white): e4e5 =   e5; Expected: e5
Position 18; Searched:  49050364, Score:   140, Move (black): f8e8 =  Re8; Expected: Qc8
Position 19; Searched:  27404510, Score:   307, Move (white): a4d7 = Qxd7; Expected: Qxd7
Position 20; Searched:  11606727, Score:    34, Move (white): h3g4 =  Bg4; Expected: Bg4
Position 21; Searched:  65642425, Score:  -239, Move (white): c4c5 = Kxc5; Expected: Kxb5
Position 22; Searched:  23593303, Score:  -138, Move (white): h7g7 =  Kg7; Expected: Ba7
Position 23; Searched:  20329306, Score:   -20, Move (black): f7f5 =   f5; Expected: Ba6
Position 24; Searched:  27412465, Score:    65, Move (white): e4c6 = Bxc6; Expected: Bxc6
Position 25; Searched:  10818214, Score:   166, Move (white): f3f4 =   f4; Expected: Bxd7

Correct: [1, 5, 9, 11, 12, 13, 17, 19, 20, 24] 10/25
Points: 11.5
Time: 533142 ms => 21325 ms/position
Search total: 697603685; Time 533142 ms => 1308 nodes/ms
```

### Lasker [5]

```
cargo run --release --bin main -- -k 3  

Position  1; Searched:   2285900, Score:   103, Move (white): a1b1 =  Kb1; Expected: Kb1

Correct: [1] 1/1
Points: 1
Time: 777 ms => 777 ms/position
Search total: 2285900; Time 777 ms => 2941 nodes/ms
```
