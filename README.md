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

Position  1; Searched:   2897549, Score:  9995, Move (black): d6d1 = Qd1+; Expected: Qd1+
Position  2; Searched: 2385104986, Score:    93, Move (white): f2g1 =  Kg1; Expected: d5
Position  3; Searched: 3095552897, Score:   -40, Move (black): f6f5 =   f5; Expected: f5
Position  4; Searched: 4405549624, Score:     4, Move (white): d4f3 =  Nf3; Expected: e6
Position  5; Searched: 3602789387, Score:   115, Move (white): h2h3 =   h3; Expected: Nd5 a4
Position  6; Searched: 1493427119, Score:    76, Move (white): g5g6 =   g6; Expected: g6
Position  7; Searched: 3192294619, Score:   115, Move (white): a3b4 =  Bb4; Expected: Nf6
Position  8; Searched: 1300546970, Score:    35, Move (white): f4f5 =   f5; Expected: f5
Position  9; Searched: 1668544773, Score:   107, Move (white): d1e1 =  Re1; Expected: f5
Position 10; Searched: 1176029337, Score:   184, Move (black): d8d6 =  Rd6; Expected: Ne5
Position 11; Searched: 2116219060, Score:    11, Move (white): g3f5 =  Nf5; Expected: f4
Position 12; Searched: 1093436589, Score:  -194, Move (black): d7f5 =  Bf5; Expected: Bf5
Position 13; Searched: 4741352698, Score:   -96, Move (white): b2b4 =   b4; Expected: b4
Position 14; Searched: 1520464566, Score:   290, Move (white): d1d2 =  Qd2; Expected: Qd2 Qe1
Position 15; Searched: 2084822804, Score:    36, Move (white): g4g7 = Qxg7+; Expected: Qxg7+
Position 16; Searched: 1095826068, Score:    77, Move (white): d2e4 =  Ne4; Expected: Ne4
Position 17; Searched: 4167046371, Score:   -83, Move (black): h7h6 =   h6; Expected: h5
Position 18; Searched: 5011148258, Score:    38, Move (black): c5b3 =  Nb3; Expected: Nb3
Position 19; Searched: 4935772059, Score:    95, Move (black): c7c5 =   c5; Expected: Rxe4
Position 20; Searched: 3334463949, Score:   -34, Move (white): e1g1 =  Rg1; Expected: g4
Position 21; Searched: 2558274298, Score:   162, Move (white): c4c5 =   c5; Expected: Nh6
Position 22; Searched: 3228789409, Score:   -52, Move (black): f6h5 =  Nh5; Expected: Bxe4
Position 23; Searched: 2959770665, Score:   -34, Move (black): b7b6 =   b6; Expected: f6
Position 24; Searched: 1911993857, Score:    -4, Move (white): f2f4 =   f4; Expected: f4

Correct: [1, 3, 6, 8, 12, 13, 14, 15, 16, 18, 24] 11/24
Points: 11.5
Time: 3844848 ms => 160202 ms/position
Search total: 63082088182; Time 3844848 ms => 16406 nodes/ms
```

### Kaufman [4]

```
% cargo run --release --bin main -- -k 2 -n 1000000000

Position  1; Searched: 1360871616, Score:   187, Move (white): e4f6 = Nf6+; Expected: Nf6+
Position  2; Searched: 3688551228, Score:   -27, Move (black): f5d6 =  Nd6; Expected: Nxd4
Position  3; Searched: 2682511071, Score:  -124, Move (white): h4h5 =   h5; Expected: Rd1
Position  4; Searched: 2947200857, Score:   -26, Move (white): c2e3 =  Ne3; Expected: Rxb2
Position  5; Searched: 1802267260, Score:   194, Move (black): g5c1 = Qxc1; Expected: Qxc1
Position  6; Searched: 2875813341, Score:   -45, Move (black): h7h5 =   h5; Expected: Rxa2
Position  7; Searched: 3228789409, Score:   -52, Move (black): f6h5 =  Nh5; Expected: Bxe4
Position  8; Searched: 1762483231, Score:   -86, Move (black): e1f2 =  Qf2; Expected: h6
Position  9; Searched: 1676113339, Score:   361, Move (white): f3e2 =  Be2; Expected: Be2
Position 10; Searched: 4643368365, Score:   164, Move (black): d5f4 =  Nf4; Expected: Nxc3
Position 11; Searched: 2989767685, Score:  9989, Move (white): g3f5 =  Nf5; Expected: Nf5
Position 12; Searched: 1182887301, Score:  -185, Move (black): c6d6 =  Rd6; Expected: Rd6
Position 13; Searched: 1439438417, Score:   200, Move (white): c3d5 = Nxd5; Expected: Nxd5
Position 14; Searched: 3504831448, Score:   -32, Move (white): f1d1 = Rfd1; Expected: Rxb2
Position 15; Searched: 1237571665, Score:   241, Move (white): g3f5 = Nxf5; Expected: Bxf5
Position 16; Searched: 1143944686, Score:   631, Move (white): b3b4 =   b4; Expected: b4
Position 17; Searched: 1290068280, Score:    37, Move (white): e4e5 =   e5; Expected: e5
Position 18; Searched: 1060019910, Score:    71, Move (black): a8a7 = Qa7+; Expected: Qc8
Position 19; Searched: 1290978487, Score:   210, Move (white): a4d7 = Qxd7; Expected: Qxd7
Position 20; Searched: 6548647496, Score:   257, Move (white): h3g4 =  Bg4; Expected: Bg4
Position 21; Searched: 1284817625, Score:  -245, Move (white): c4c5 = Kxc5; Expected: Kxb5
Position 22; Searched: 3221518809, Score:  -149, Move (white): h7g6 =  Kg6; Expected: Ba7
Position 23; Searched: 2259298330, Score:   -20, Move (black): f7f6 =   f6; Expected: Ba6
Position 24; Searched: 1660245235, Score:    58, Move (white): e4c6 = Bxc6; Expected: Bxc6
Position 25; Searched: 3911903594, Score:   153, Move (white): e3e4 =   e4; Expected: Bxd7

Correct: [1, 5, 9, 11, 12, 13, 16, 17, 19, 20, 24] 11/25
Points: 12.49
Time: 3852473 ms => 154098 ms/position
Search total: 60693879117; Time 3852473 ms => 15754 nodes/ms
```

### Lasker [5]

```
% cargo run --release --bin main -- -k 3

Position  1; Searched:   1721145, Score:   103, Move (white): a1b1 =  Kb1; Expected: Kb1

Correct: [1] 1/1
Points: 1
Time: 209 ms => 209 ms/position
Search total: 1721125; Time 209 ms => 8235 nodes/ms
```
