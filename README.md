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
% caffeinate cargo run --release --bin main -- -k 1 -n 100000000

Position  1; Searched:    158236, Score: 31995, Move (black): d6d1 = Qd1+; Expected: Qd1+
Position  2; Searched: 309575477, Score:    93, Move (white): f2g1 =  Kg1; Expected: d5
Position  3; Searched: 259246969, Score:   -40, Move (black): f6f5 =   f5; Expected: f5
Position  4; Searched: 503592398, Score:    -1, Move (white): d4b5 =  Nb5; Expected: e6
Position  5; Searched: 295457196, Score:   115, Move (white): h2h3 =   h3; Expected: Nd5,a4
Position  6; Searched: 460951135, Score:    76, Move (white): g5g6 =   g6; Expected: g6
Position  7; Searched: 375111842, Score:   115, Move (white): a3b4 =  Bb4; Expected: Nf6
Position  8; Searched: 124803410, Score:     7, Move (white): f4f5 =   f5; Expected: f5
Position  9; Searched: 150017110, Score:   107, Move (white): d1e1 =  Re1; Expected: f5
Position 10; Searched: 427847036, Score:   -23, Move (black): c6e5 =  Ne5; Expected: Ne5
Position 11; Searched: 225882100, Score:    11, Move (white): g3f5 =  Nf5; Expected: f4
Position 12; Searched: 124792801, Score:  -194, Move (black): d7f5 =  Bf5; Expected: Bf5
Position 13; Searched: 492356736, Score:   -96, Move (white): b2b4 =   b4; Expected: b4
Position 14; Searched: 180157761, Score:   290, Move (white): d1d2 =  Qd2; Expected: Qd2 Qe1
Position 15; Searched: 157728366, Score:    68, Move (white): g4g7 = Qxg7+; Expected: Qxg7+
Position 16; Searched: 474462600, Score:    41, Move (white): d2e4 =  Ne4; Expected: Ne4
Position 17; Searched: 283867290, Score:   -83, Move (black): h7h6 =   h6; Expected: h5
Position 18; Searched: 377738512, Score:    38, Move (black): c5b3 =  Nb3; Expected: Nb3
Position 19; Searched: 535188462, Score:    99, Move (black): c7c5 =   c5; Expected: Rxe4
Position 20; Searched: 347654839, Score:   -34, Move (white): e1g1 =  Rg1; Expected: g4
Position 21; Searched: 113014895, Score:   211, Move (white): f5h6 =  Nh6; Expected: Nh6
Position 22; Searched: 262725835, Score:   -52, Move (black): f6h5 =  Nh5; Expected: Bxe4
Position 23; Searched: 298719207, Score:   -34, Move (black): b7b6 =   b6; Expected: f6
Position 24; Searched: 155486306, Score:    -4, Move (white): f2f4 =   f4; Expected: f4

Correct: [1, 3, 6, 8, 10, 12, 13, 14, 15, 16, 18, 21, 24] 13/24
Points: 13.5
Time: 1048890 ms => 43703 ms/position
Search total: 6936536519; Time 1048890 ms => 6613 nodes/ms
```

### Kaufman [4]

```
% caffeinate cargo run --release --bin main -- -k 2 -n 100000000

Position  1; Searched: 140955085, Score:   187, Move (white): e4f6 = Nf6+; Expected: Nf6+
Position  2; Searched: 112711424, Score:   301, Move (black): f5d4 = Nxd4; Expected: Nxd4
Position  3; Searched: 411272839, Score:  -124, Move (white): h4h5 =   h5; Expected: Rd1
Position  4; Searched: 258301254, Score:   -82, Move (white): c2e3 =  Ne3; Expected: Rxb2
Position  5; Searched: 622299013, Score:    78, Move (black): g5c1 = Qxc1; Expected: Qxc1
Position  6; Searched: 481227040, Score:   -45, Move (black): h7h5 =   h5; Expected: Rxa2
Position  7; Searched: 262725835, Score:   -52, Move (black): f6h5 =  Nh5; Expected: Bxe4
Position  8; Searched: 136324337, Score:  -252, Move (black): e1f2 =  Qf2; Expected: h6
Position  9; Searched: 287946088, Score:   361, Move (white): f3e2 =  Be2; Expected: Be2
Position 10; Searched: 628990059, Score:   164, Move (black): d5f4 =  Nf4; Expected: Nxc3
Position 11; Searched:  69959118, Score: 31987, Move (white): g3f5 =  Nf5; Expected: Nf5
Position 12; Searched: 204052705, Score:    50, Move (black): c6d6 =  Rd6; Expected: Rd6
Position 13; Searched: 364252471, Score:   194, Move (white): c3d5 = Nxd5; Expected: Nxd5
Position 14; Searched: 105819718, Score:    94, Move (white): h6f4 =  Bf4; Expected: Rxb2
Position 15; Searched: 143607627, Score:   533, Move (white): g3f5 = Nxf5; Expected: Bxf5
Position 16; Searched: 319630190, Score:   501, Move (white): b3b4 =   b4; Expected: b4
Position 17; Searched: 138689586, Score:    37, Move (white): e4e5 =   e5; Expected: e5
Position 18; Searched: 313838431, Score:   120, Move (black): f8e8 =  Re8; Expected: Qc8
Position 19; Searched: 163483821, Score:   307, Move (white): a4d7 = Qxd7; Expected: Qxd7
Position 20; Searched: 151603490, Score:   222, Move (white): h3g4 =  Bg4; Expected: Bg4
Position 21; Searched: 322222597, Score:  -237, Move (white): c4c5 = Kxc5; Expected: Kxb5
Position 22; Searched: 345131172, Score:  -138, Move (white): h7g7 =  Kg7; Expected: Ba7
Position 23; Searched: 209708554, Score:   -20, Move (black): f7f6 =   f6; Expected: Ba6
Position 24; Searched: 156257733, Score:    65, Move (white): e4c6 = Bxc6; Expected: Bxc6
Position 25; Searched: 332607977, Score:   166, Move (white): f3f4 =   f4; Expected: Bxd7

Correct: [1, 2, 5, 9, 11, 12, 13, 16, 17, 19, 20, 24] 12/25
Points: 12.5
Time: 748888 ms => 29955 ms/position
Search total: 6683618164; Time 748888 ms => 8924 nodes/ms
```

### Lasker [5]

```
cargo run --release --bin main -- -k 3  | tee RES/OUTk3
Position  1; Searched:   1310590, Score:   103, Move (white): a1b1 =  Kb1; Expected: Kb1

Correct: [1] 1/1
Points: 1
Time: 241 ms => 241 ms/position
Search total: 1310590; Time 241 ms => 5438 nodes/ms
```
