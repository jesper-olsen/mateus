# Puccinia's Checkmate

A rusty chess library:
* Principle variation negamax search with alpha beta pruning (See Knuth).
* Transposition table to avoid re-searching cycles
* Evaluation based on material, pawn structure & mobility
* Checks draw by 3x repetition and 50 move rule
* Opening library

References:
* ["An Analysis of Alpha-Beta Pruning", Donald E. Knuth and Ronald W. Moore, Artificial Intelligence 6 (1975), 293-326](http://www-public.telecom-sudparis.eu/~gibson/Teaching/Teaching-ReadingMaterial/KnuthMoore75.pdf) 
* ["Computer Chess Methods", T.A. Marsland, ENCYCLOPEDIAOF ARTIFICIAL INTELLIGENCE, 1987](https://www.researchgate.net/publication/2404258_Computer_Chess_Methods)
* ["The Bratko-Kopec Experiment: A Comparison of Human and Computer Performance in Chess", D. Kopec and I Bratko](http://spider.sci.brooklyn.cuny.edu/~kopec)

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
  -k, --k <K>    benchmark test positions - Bratko-Kopec (1) / Kaufman (2) [default: 0]
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



Run CLI app like this to benchmark on Bratko-Kopec positions:
```
% cargo run --release --bin main -- -k 1 -n 1000000000

Position  1; Searched:    2800755, Score:  9995, Move (black): d6 d1 =  Qd1; Expected: Qd1+
Position  2; Searched: 2222449681, Score:    93, Move (white): f2 g1 =  Kg1; Expected: d5
Position  3; Searched: 2229840451, Score:   -40, Move (black): f6 f5 =   f5; Expected: f5
Position  4; Searched: 4046464097, Score:     8, Move (white): d4 f3 =  Nf3; Expected: e6
Position  5; Searched: 3261753681, Score:   115, Move (white): f1 f3 =  Rf3; Expected: Nd5 a4
Position  6; Searched: 1608344607, Score:    76, Move (white): g5 g6 =   g6; Expected: g6
Position  7; Searched: 3197946681, Score:   115, Move (white): a3 b4 =  Bb4; Expected: Nf6
Position  8; Searched: 1544205424, Score:    35, Move (white): f4 f5 =   f5; Expected: f5
Position  9; Searched: 1714701619, Score:   107, Move (white): d1 e1 =  Re1; Expected: f5
Position 10; Searched: 3274617770, Score:   -23, Move (black): c6 e5 =  Ne5; Expected: Ne5
Position 11; Searched: 2328521415, Score:    11, Move (white): g3 f5 =  Nf5; Expected: f4
Position 12; Searched: 1310715712, Score:  -194, Move (black): d7 f5 =  Bf5; Expected: Bf5
Position 13; Searched: 3805615314, Score:   -96, Move (white): b2 b4 =   b4; Expected: b4
Position 14; Searched: 1658642184, Score:   290, Move (white): d1 d2 =  Qd2; Expected: Qd2 Qe1
Position 15; Searched: 2270534500, Score:    36, Move (white): g4 g7 = Qxg7; Expected: Qxg7+
Position 16; Searched: 4184492274, Score:    41, Move (white): d2 e4 =  Ne4; Expected: Ne4
Position 17; Searched: 3927819570, Score:   -83, Move (black): h7 h6 =   h6; Expected: h5
Position 18; Searched: 4625663064, Score:    38, Move (black): c5 b3 =  Nb3; Expected: Nb3
Position 19; Searched: 4552127309, Score:    95, Move (black): c7 c5 =   c5; Expected: Rxe4
Position 20; Searched: 3168317191, Score:   -26, Move (white): g3 g4 =   g4; Expected: g4
Position 21; Searched: 1462346662, Score:   162, Move (white): c4 c5 =   c5; Expected: Nh6
Position 22; Searched: 3002123202, Score:   -47, Move (black): f6 h5 =  Nh5; Expected: Bxe4
Position 23; Searched: 2741002409, Score:   -34, Move (black): b7 b6 =   b6; Expected: f6
Position 24; Searched: 1952106425, Score:    -4, Move (white): f2 f4 =   f4; Expected: f4

Correct: [1, 3, 6, 8, 10, 12, 13, 14, 15, 16, 18, 20, 24] 13/24
Points: 14
Time: 3770684 ms => 157111 ms/position
Search total: 64093151997; Time 3770684 ms => 16997 nodes/ms
```

Run CLI app like this to benchmark on Kaufman positions:

```
% cargo run --release --bin main -- -k 2 -n 1000000000

Position  1; Searched: 1395431159, Score:   187, Move (white): e4 f6 =  Nf6; Expected: Nf6+
Position  2; Searched: 3444952992, Score:   -27, Move (black): f5 d6 =  Nd6; Expected: Nxd4
Position  3; Searched: 2531952375, Score:  -124, Move (white): h4 h5 =   h5; Expected: Rd1
Position  4; Searched: 2818344653, Score:   -26, Move (white): c2 e3 =  Ne3; Expected: Rxb2
Position  5; Searched: 4197810114, Score:    78, Move (black): g5 c1 = Qxc1; Expected: Qxc1
Position  6; Searched: 3260377981, Score:   -45, Move (black): h7 h5 =   h5; Expected: Rxa2
Position  7; Searched: 3002123202, Score:   -47, Move (black): f6 h5 =  Nh5; Expected: Bxe4
Position  8; Searched: 1906626805, Score:   -87, Move (black): e1 f2 =  Qf2; Expected: h6
Position  9; Searched: 2484627037, Score:   361, Move (white): f3 e2 =  Be2; Expected: Be2
Position 10; Searched: 4246892341, Score:   164, Move (black): d5 f4 =  Nf4; Expected: Nxc3
Position 11; Searched: 1236660820, Score:  9989, Move (white): g3 f5 =  Nf5; Expected: Nf5
Position 12; Searched: 2093813685, Score:  -300, Move (black): a7 a6 =   a6; Expected: Rd6
Position 13; Searched: 1579128939, Score:   206, Move (white): c3 d5 = Nxd5; Expected: Nxd5
Position 14; Searched: 1058834540, Score:    91, Move (white): h6 f4 =  Bf4; Expected: Rxb2
Position 15; Searched: 1471202188, Score:   533, Move (white): g3 f5 = Nxf5; Expected: Bxf5
Position 16; Searched: 1142702021, Score:   631, Move (white): b3 b4 =   b4; Expected: b4
Position 17; Searched: 1059733768, Score:    46, Move (white): e4 e5 =   e5; Expected: e5
Position 18; Searched: 1004542637, Score:    71, Move (black): a8 a7 =  Qa7; Expected: Qc8
Position 19; Searched: 1209889636, Score:   307, Move (white): a4 d7 = Qxd7; Expected: Qxd7
Position 20; Searched: 7008215735, Score:   257, Move (white): h3 g4 =  Bg4; Expected: Bg4
Position 21; Searched: 1773774170, Score:  -245, Move (white): c4 c5 = Kxc5; Expected: Kxb5
Position 22; Searched: 3381170517, Score:  -149, Move (white): h7 g6 =  Kg6; Expected: Ba7
Position 23; Searched: 2502075164, Score:   -20, Move (black): f7 f5 =   f5; Expected: Ba6
Position 24; Searched: 1899177410, Score:    59, Move (white): e4 c6 = Bxc6; Expected: Bxc6
Position 25; Searched: 1123406907, Score:   166, Move (white): f3 f4 =   f4; Expected: Bxd7

Correct: [1, 5, 9, 11, 13, 16, 17, 19, 20, 24] 10/25
Points: 11.5
Time: 3744815 ms => 149792 ms/position
Search total: 58833466796; Time 3744815 ms => 15710 nodes/ms
```
