# Puccinia's Checkmate

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

Two example apps included - terminal CLI app (src/bin) and browser web application ([examples/spa](https://github.com/jesper-olsen/puccinia_s_checkmate/tree/main/examples/spa)).

Run CLI app like this to see options: 

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
% cargo run --release --bin main -- -k 1 -n 1000000000

Position  1; Searched:   3183375, Score:  9995, Move (black): d6 d1 =  Qd1; Expected: Qd1+
Position  2; Searched: 2161654527, Score:    93, Move (white): f2 g1 =  Kg1; Expected: d5
Position  3; Searched: 2207943441, Score:   -40, Move (black): f6 f5 =   f5; Expected: f5
Position  4; Searched: 4355981165, Score:     8, Move (white): d4 f3 =  Nf3; Expected: e6
Position  5; Searched: 3630096111, Score:   115, Move (white): h2 h3 =   h3; Expected: Nd5 a4
Position  6; Searched: 1442509670, Score:    76, Move (white): g5 g6 =   g6; Expected: g6
Position  7; Searched: 3131231064, Score:   115, Move (white): a3 b4 =  Bb4; Expected: Nf6
Position  8; Searched: 1284100369, Score:    35, Move (white): f4 f5 =   f5; Expected: f5
Position  9; Searched: 1722902889, Score:   107, Move (white): d1 e1 =  Re1; Expected: f5
Position 10; Searched: 3876726437, Score:   -23, Move (black): c6 e5 =  Ne5; Expected: Ne5
Position 11; Searched: 2256850482, Score:    11, Move (white): g3 f5 =  Nf5; Expected: f4
Position 12; Searched: 1331147461, Score:  -194, Move (black): d7 f5 =  Bf5; Expected: Bf5
Position 13; Searched: 3891088848, Score:   -96, Move (white): b2 b4 =   b4; Expected: b4
Position 14; Searched: 1547786839, Score:   290, Move (white): d1 d2 =  Qd2; Expected: Qd2 Qe1
Position 15; Searched: 1807840071, Score:    36, Move (white): g4 g7 = Qxg7; Expected: Qxg7+
Position 16; Searched: 2903337884, Score:    41, Move (white): d2 e4 =  Ne4; Expected: Ne4
Position 17; Searched: 4023258275, Score:   -83, Move (black): h7 h6 =   h6; Expected: h5
Position 18; Searched: 4458694238, Score:    38, Move (black): c5 b3 =  Nb3; Expected: Nb3
Position 19; Searched: 4486738199, Score:   113, Move (black): c7 c5 =   c5; Expected: Rxe4
Position 20; Searched: 2908439918, Score:   -26, Move (white): g3 g4 =   g4; Expected: g4
Position 21; Searched: 2010455296, Score:   162, Move (white): c4 c5 =   c5; Expected: Nh6
Position 22; Searched: 3187379107, Score:   -52, Move (black): f6 h5 =  Nh5; Expected: Bxe4
Position 23; Searched: 2637375138, Score:   -34, Move (black): b7 b6 =   b6; Expected: f6
Position 24; Searched: 1915385712, Score:    -4, Move (white): f2 f4 =   f4; Expected: f4

Correct: [1, 3, 6, 8, 10, 12, 13, 14, 15, 16, 18, 20, 24] 13/24
Points: 13.5
Time: 3638684 ms => 151611 ms/position
Search total: 63182106516; Time 3638684 ms => 17363 nodes/ms
```

### Kaufman [4]

```
% cargo run --release --bin main -- -k 2 -n 1000000000

Position  1; Searched: 1292167897, Score:   187, Move (white): e4 f6 =  Nf6; Expected: Nf6+
Position  2; Searched: 3471519038, Score:   -19, Move (black): a5 d5 =  Qd5; Expected: Nxd4
Position  3; Searched: 2380869329, Score:  -124, Move (white): h4 h5 =   h5; Expected: Rd1
Position  4; Searched: 2864200550, Score:   -26, Move (white): c2 e3 =  Ne3; Expected: Rxb2
Position  5; Searched: 3017581943, Score:    78, Move (black): g5 c1 = Qxc1; Expected: Qxc1
Position  6; Searched: 1664898702, Score:   -45, Move (black): h7 h5 =   h5; Expected: Rxa2
Position  7; Searched: 3187379107, Score:   -52, Move (black): f6 h5 =  Nh5; Expected: Bxe4
Position  8; Searched: 1808675776, Score:   -87, Move (black): e1 f2 =  Qf2; Expected: h6
Position  9; Searched: 1553562485, Score:   361, Move (white): f3 e2 =  Be2; Expected: Be2
Position 10; Searched: 3800164032, Score:   164, Move (black): d5 f4 =  Nf4; Expected: Nxc3
Position 11; Searched: 2877362773, Score:  9989, Move (white): g3 f5 =  Nf5; Expected: Nf5
Position 12; Searched: 1086094218, Score:  -185, Move (black): c6 d6 =  Rd6; Expected: Rd6
Position 13; Searched: 1386063150, Score:   200, Move (white): c3 d5 = Nxd5; Expected: Nxd5
Position 14; Searched: 3908627212, Score:   -26, Move (white): f1 d1 = R1d1; Expected: Rxb2
Position 15; Searched: 1117794524, Score:   266, Move (white): g3 f5 = Nxf5; Expected: Bxf5
Position 16; Searched: 1222229056, Score:   631, Move (white): b3 b4 =   b4; Expected: b4
Position 17; Searched: 1012373163, Score:    46, Move (white): e4 e5 =   e5; Expected: e5
Position 18; Searched: 1065633939, Score:    71, Move (black): a8 a7 =  Qa7; Expected: Qc8
Position 19; Searched: 1188458893, Score:   210, Move (white): a4 d7 = Qxd7; Expected: Qxd7
Position 20; Searched: 6262294233, Score:   257, Move (white): h3 g4 =  Bg4; Expected: Bg4
Position 21; Searched: 1228154720, Score:  -245, Move (white): c4 c5 = Kxc5; Expected: Kxb5
Position 22; Searched: 3136391000, Score:  -149, Move (white): h7 g6 =  Kg6; Expected: Ba7
Position 23; Searched: 2355457042, Score:   -20, Move (black): f7 f5 =   f5; Expected: Ba6
Position 24; Searched: 1637804457, Score:    58, Move (white): e4 c6 = Bxc6; Expected: Bxc6
Position 25; Searched: 3862443858, Score:   153, Move (white): e3 e4 =   e4; Expected: Bxd7

Correct: [1, 5, 9, 11, 12, 13, 16, 17, 19, 20, 24] 11/25
Points: 12
Time: 3405925 ms => 136237 ms/position
Search total: 58388201097; Time 3405925 ms => 17143 nodes/ms
```

### Lasker [5]

```
% cargo run --release --bin main -- -k 3
Position  1; Searched:   1721125, Score:   103, Move (white): a1 b1 =  Kb1; Expected: Kb1

Correct: [1] 1/1
Points: 1
Time: 211 ms => 211 ms/position
Search total: 1721125; Time 211 ms => 8156 nodes/ms
```
