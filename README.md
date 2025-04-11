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
% cargo run --release --bin main -- -h

Usage: main [OPTIONS]

Options:
  -n, --n <N>    break off search threshold - positions generated [default: 1000000]
  -d, --d <D>    max depth of regular search [default: 30]
  -m, --m <M>    number of (half) moves before stopping [default: -1]
  -w, --w        play white (human-computer)
  -b, --b        play black (human-computer)
  -l, --l        no opening library
  -k, --k <K>    benchmark test positions - Bratko-Kopec (1) / Kaufman (2) / Lasker (3) [default: 0]
  -v, --v        verbose output
  -f, --f <F>    fen board - start position [default: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR]
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
Position  2; Searched:  10921735, Score:    34, Move (white): c3c4 =  Rc4; Expected: d5
Position  3; Searched:  27261011, Score:   -40, Move (black): f6f5 =   f5; Expected: f5
Position  4; Searched:  14747345, Score:   -53, Move (white): e2b5 = Qb5+; Expected: e6
Position  5; Searched:  41658618, Score:   115, Move (white): h2h3 =   h3; Expected: Nd5,a4
Position  6; Searched:  14145881, Score:    71, Move (white): g5g6 =   g6; Expected: g6
Position  7; Searched:  12256412, Score:   -87, Move (white): a3b4 =  Bb4; Expected: Nf6
Position  8; Searched:  11146407, Score:   -26, Move (white): f4f5 =   f5; Expected: f5
Position  9; Searched:  16360489, Score:   107, Move (white): d1e1 =  Re1; Expected: f5
Position 10; Searched:  38664458, Score:   -23, Move (black): c6e5 =  Ne5; Expected: Ne5
Position 11; Searched:  23989362, Score:    11, Move (white): g3f5 =  Nf5; Expected: f4
Position 12; Searched:  31788208, Score:  -194, Move (black): d7f5 =  Bf5; Expected: Bf5
Position 13; Searched:  61439239, Score:   -96, Move (white): b2b4 =   b4; Expected: b4
Position 14; Searched:  28823161, Score:   265, Move (white): d1e1 =  Qe1; Expected: Qd2 Qe1
Position 15; Searched:  20782920, Score:    67, Move (white): g4g7 = Qxg7+; Expected: Qxg7+
Position 16; Searched:  10751552, Score:   -89, Move (white): g5e7 = Bxe7; Expected: Ne4
Position 17; Searched:  25973635, Score:   -83, Move (black): h7h6 =   h6; Expected: h5
Position 18; Searched:  45740629, Score:    38, Move (black): c5b3 =  Nb3; Expected: Nb3
Position 19; Searched:  63988523, Score:   100, Move (black): c7c5 =   c5; Expected: Rxe4
Position 20; Searched:  50581899, Score:   -26, Move (white): g3g4 =   g4; Expected: g4
Position 21; Searched:  21865374, Score:   259, Move (white): f5h6 =  Nh6; Expected: Nh6
Position 22; Searched:  27971645, Score:   -52, Move (black): f6h5 =  Nh5; Expected: Bxe4
Position 23; Searched:  25130244, Score:   -11, Move (black): f7f6 =   f6; Expected: f6
Position 24; Searched:  18981846, Score:    -4, Move (white): f2f4 =   f4; Expected: f4

Points: 14.83
Time: 788029 ms => 32834 ms/position
Search total: 644975646; Time 788029 ms => 818 nodes/ms
```

### Kaufman [4]

```
% cargo run --release --bin main -- -k 2 -n 10000000

Position  1; Searched:  19866004, Score:   187, Move (white): e4f6 = Nf6+; Expected: Nf6+
Position  2; Searched:  42829298, Score:   -27, Move (black): f5d6 =  Nd6; Expected: Nxd4
Position  3; Searched:  56387842, Score:  -127, Move (white): h4h5 =   h5; Expected: Rd1
Position  4; Searched:  25219929, Score:   -82, Move (white): c2e3 =  Ne3; Expected: Rxb2
Position  5; Searched:  10846552, Score:   124, Move (black): g5c1 = Qxc1; Expected: Qxc1
Position  6; Searched:  19122777, Score:  -168, Move (black): a3a4 = R3a4; Expected: Rxa2
Position  7; Searched:  27971645, Score:   -52, Move (black): f6h5 =  Nh5; Expected: Bxe4
Position  8; Searched:  16123441, Score:  -252, Move (black): e1f2 =  Qf2; Expected: h6
Position  9; Searched:  36537420, Score:   288, Move (white): f3e2 =  Be2; Expected: Be2
Position 10; Searched:  89043516, Score:   164, Move (black): d5f4 =  Nf4; Expected: Nxc3
Position 11; Searched:  13190227, Score: 31987, Move (white): g3f5 =  Nf5; Expected: Nf5
Position 12; Searched:  44118030, Score:    50, Move (black): c6d6 =  Rd6; Expected: Rd6
Position 13; Searched:  20590570, Score:   218, Move (white): c3d5 = Nxd5; Expected: Nxd5
Position 14; Searched:  16466307, Score:    91, Move (white): h6f4 =  Bf4; Expected: Rxb2
Position 15; Searched:  31122448, Score:   533, Move (white): g3f5 = Nxf5; Expected: Bxf5
Position 16; Searched:  21654819, Score:   141, Move (white): e4e5 =   e5; Expected: b4
Position 17; Searched:  14048252, Score:    37, Move (white): e4e5 =   e5; Expected: e5
Position 18; Searched:  10040707, Score:    71, Move (black): a8a7 = Qa7+; Expected: Qc8
Position 19; Searched:  28671406, Score:   307, Move (white): a4d7 = Qxd7; Expected: Qxd7
Position 20; Searched:  12056114, Score:    34, Move (white): h3g4 =  Bg4; Expected: Bg4
Position 21; Searched: 131546163, Score:  -239, Move (white): c4c5 = Kxc5; Expected: Kxb5
Position 22; Searched:  28030353, Score:  -138, Move (white): h7g7 =  Kg7; Expected: Ba7
Position 23; Searched:  21839239, Score:   -20, Move (black): f7f5 =   f5; Expected: Ba6
Position 24; Searched:  48301155, Score:    65, Move (white): e4c6 = Bxc6; Expected: Bxc6
Position 25; Searched:  11097894, Score:   166, Move (white): f3f4 =   f4; Expected: Bxd7

Correct: [1, 5, 9, 11, 12, 13, 17, 19, 20, 24] 10/25
Points: 11
Time: 567161 ms => 22686 ms/position
Search total: 796722108; Time 567161 ms => 1404 nodes/ms
```

### Lasker [5]

```
cargo run --release --bin main -- -k 3  
Position  1; Searched:   2308030, Score:   103, Move (white): a1b1 =  Kb1; Expected: Kb1

Correct: [1] 1/1
Points: 1
Time: 767 ms => 767 ms/position
Search total: 2308030; Time 767 ms => 3009 nodes/ms
```
