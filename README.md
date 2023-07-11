# Puccinia's Checkmate

A rusty chess library:
* [Negamax search](https://en.wikipedia.org/wiki/Negamax) with alpha beta pruning
* Evaluation based on material, pawn structure & mobility

Two example apps included - terminal CLI app (src/bin) and browser web application (examples/spa).

Run CLI app like this: 

```
% cargo run --release -- -h 

Usage: main [OPTIONS]

Options:
  -n, --n <N>    break off search threshold - positions generated [default: 1000000]
  -d, --d <D>    max depth of regular search [default: 25]
  -m, --m <M>    number of (half) moves before stopping [default: -1]
  -w, --w        play white (human-computer)
  -b, --b        play black (computer-human)
  -v, --v        verbose output
  -h, --help     Print help
  -V, --version  Print version

```

Run CLI app like this (option -w or -b to take a side).
```
% cargo run --release -- -v 

8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ........
3 ........
2 PPPPPPPP
1 RNBQKBNR
  ABCDEFGH
Opening positions: 55 - Moves: 65
Opening: e2,e4 (picked)
Opening: d2,d4
Search total: 20 / 0 ms / 0 nodes/ms
hash size r 1 t 0 
0/20: e2 e4 14/0

8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ....P...
3 ........
2 PPPP.PPP
1 RNBQKBNR
  ABCDEFGH
1. e4
score: 0, material: 14, abs: 8042, pawns: 0, mobility: 10
Opening: e7,e5 (picked)
Opening: e7,e6
Opening: c7,c6
Opening: c7,c5
Search total: 60 / 0 ms / 0 nodes/ms
hash size r 1 t 0 
0/20: e7 e5 -12/0

8 rnbqkbnr
7 pppp.ppp
6 ........
5 ....p...
4 ....P...
3 ........
2 PPPP.PPP
1 RNBQKBNR
  ABCDEFGH
2. e5
score: 0, material: 2, abs: 8054, pawns: 0, mobility: 0
Opening: g1,f3 (picked)
Search total: 129 / 0 ms / 0 nodes/ms
hash size r 2 t 0 
0/29: g1 f3 10/0

8 rnbqkbnr
7 pppp.ppp
6 ........
5 ....p...
4 ....P...
3 .....N..
2 PPPP.PPP
1 RNBQKB.R
  ABCDEFGH
2. Nf3
score: 0, material: 12, abs: 8064, pawns: 0, mobility: -2
Opening: b8,c6
Opening: d7,d6 (picked)
Search total: 227 / 0 ms / 0 nodes/ms
hash size r 3 t 0 
0/29: d7 d6 -7/0

8 rnbqkbnr
7 ppp..ppp
6 ...p....
5 ....p...
4 ....P...
3 .....N..
2 PPPP.PPP
1 RNBQKB.R
  ABCDEFGH
3. d6
score: 0, material: 5, abs: 8071, pawns: 0, mobility: -5
Opening: d2,d4 (picked)
Search total: 352 / 0 ms / 0 nodes/ms
hash size r 2 t 0 
0/27: d2 d4 12/0

8 rnbqkbnr
7 ppp..ppp
6 ...p....
5 ....p...
4 ...PP...
3 .....N..
2 PPP..PPP
1 RNBQKB.R
  ABCDEFGH
3. d4
score: 0, material: 17, abs: 8083, pawns: 0, mobility: 3
Opening: e5,d4 (picked)
Opening: b8,d7
Search total: 510 / 0 ms / 0 nodes/ms
hash size r 2 t 0 
0/33: e5 d4 -121/0

8 rnbqkbnr
7 ppp..ppp
6 ...p....
5 ........
4 ...pP...
3 .....N..
2 PPP..PPP
1 RNBQKB.R
  ABCDEFGH
4. xd4
score: 0, material: -104, abs: 7980, pawns: 6, mobility: 4
Depth  1 #searched     2893 bmove: h2 h4 bscore: 713
Depth  2 #searched    69335 bmove: c2 c3 bscore: -87
Depth  3 #searched   735773 bmove: f3 g5 bscore: 151
Depth  4 #searched  3855679 bmove: f1 c4 bscore: 52
Search total: 3856189 / 327 ms / 11792 nodes/ms
hash size r 2 t 4848 
0/37: f1 c4 11/52
1/37: c1 f4 11/52
2/37: b2 b3 2/52
3/37: a2 a4 2/52
4/37: g2 g3 2/52
5/37: e1 e2 -6/52
6/37: a2 a3 1/52
7/37: d1 d2 0/52
8/37: c1 e3 11/52
9/37: b1 a3 0/52
10/37: b1 d2 5/52
11/37: h1 g1 0/52
12/37: c2 c4 8/52
13/37: f3 d4 121/52
14/37: d1 e2 0/52
15/37: c1 d2 11/52
16/37: e1 d2 -6/52
17/37: d1 d4 121/52
18/37: f1 d3 11/52
19/37: d1 d3 0/52
20/37: f1 b5 11/52
21/37: g2 g4 4/52
22/37: c1 h6 11/52
23/37: f3 e5 5/51
24/37: e4 e5 7/50
25/37: b2 b4 4/50
26/37: c2 c3 4/49
27/37: b1 c3 10/46
28/37: f3 d2 -5/39
29/37: f1 a6 11/-75
30/37: h2 h3 1/-91
31/37: h2 h4 2/-94
32/37: f1 e2 11/-96
33/37: f3 g5 -5/-97
34/37: f3 g1 -10/-98
35/37: f3 h4 -10/-100
36/37: c1 g5 11/-117

8 rnbqkbnr
7 ppp..ppp
6 ...p....
5 ........
4 ..BpP...
3 .....N..
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
4. Bc4
score: 52, material: -93, abs: 7991, pawns: 6, mobility: 9
Depth  1 #searched     4300 bmove: c7 c5 bscore: 166
Depth  2 #searched    49934 bmove: d4 d3 bscore: -58
Depth  3 #searched   232953 bmove: d4 d3 bscore: 134
Depth  4 #searched  4455486 bmove: c7 c5 bscore: -57
Search total: 8311675 / 703 ms / 11823 nodes/ms
hash size r 3 t 6451 
0/33: c7 c5 -8/-57
1/33: d8 e7 0/-59
2/33: d8 d7 0/-60
3/33: h7 h6 -1/-61
4/33: c8 g4 -11/-64
5/33: c8 d7 -11/-64
6/33: f8 e7 -11/-65
7/33: g8 e7 -5/-67
8/33: e8 e7 6/-67
9/33: f7 f5 -8/-68
10/33: b8 d7 -5/-68
11/33: g7 g6 -2/-68
12/33: b8 a6 0/-68
13/33: d8 f6 0/-68
14/33: d4 d3 -7/-69
15/33: b8 c6 -10/-69
16/33: c7 c6 -4/-69
17/33: b7 b6 -2/-69
18/33: h7 h5 -2/-69
19/33: a7 a5 -2/-69
20/33: a7 a6 -1/-69
21/33: g8 f6 -10/-71
22/33: g8 h6 0/-73
23/33: b7 b5 -4/-77
24/33: e8 d7 6/-77
25/33: d6 d5 -7/-78
26/33: c8 f5 -11/-116
27/33: f7 f6 -4/-117
28/33: c8 h3 -11/-123
29/33: c8 e6 -11/-127
30/33: g7 g5 -4/-176
31/33: d8 g5 0/-413
32/33: d8 h4 0/-615

8 rnbqkbnr
7 pp...ppp
6 ...p....
5 ..p.....
4 ..BpP...
3 .....N..
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
5. c5
score: -57, material: -101, abs: 7999, pawns: 6, mobility: 8
Depth  1 #searched     3245 bmove: h2 h4 bscore: 722
Depth  2 #searched    68902 bmove: c4 f7 bscore: -18
Depth  3 #searched   829309 bmove: f3 g5 bscore: -25
Depth  4 #searched  4370626 bmove: f3 g5 bscore: 486
Search total: 12682301 / 1052 ms / 12055 nodes/ms
hash size r 2 t 5535 
0/43: f3 g5 -5/486
1/43: c4 b5 0/486
2/43: e4 e5 7/486
3/43: b1 c3 10/486
4/43: b2 b4 4/486
5/43: c1 g5 11/486
6/43: f3 e5 5/485
7/43: c1 h6 11/484
8/43: b2 b3 2/483
9/43: a2 a4 2/482
10/43: f3 h4 -10/479
11/43: e1 f1 6/479
12/43: g2 g3 2/479
13/43: c1 f4 11/478
14/43: d1 d3 0/478
15/43: c1 e3 11/476
16/43: d1 e2 0/476
17/43: e1 g1 18/474
18/43: c4 f7 100/473
19/43: e1 e2 -6/471
20/43: c1 d2 11/469
21/43: b1 d2 5/457
22/43: f3 d2 -5/399
23/43: d1 d2 0/374
24/43: c4 e6 0/369
25/43: c2 c3 4/360
26/43: g2 g4 4/308
27/43: h2 h3 1/307
28/43: f3 d4 121/304
29/43: d1 d4 121/302
30/43: c4 a6 0/176
31/43: a2 a3 1/174
32/43: c4 d5 0/173
33/43: b1 a3 0/173
34/43: c4 b3 0/173
35/43: h1 g1 0/171
36/43: h1 f1 0/171
37/43: c4 d3 0/165
38/43: f3 g1 -10/165
39/43: e1 d2 -6/163
40/43: c4 e2 0/162
41/43: c4 f1 -11/152
42/43: h2 h4 2/71

8 rnbqkbnr
7 pp...ppp
6 ...p....
5 ..p...N.
4 ..BpP...
3 ........
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
5. Ng5
score: 486, material: -106, abs: 7994, pawns: 6, mobility: 12
Depth  1 #searched     5297 bmove: g8 h6 bscore: 310
Depth  2 #searched    51936 bmove: d8 a5 bscore: -32
Depth  3 #searched   403965 bmove: d4 d3 bscore: -158
Depth  4 #searched  5316140 bmove: d8 c7 bscore: -232
Search total: 17998441 / 1473 ms / 12218 nodes/ms
hash size r 3 t 6156 
0/32: d8 c7 0/-232
1/32: d8 a5 0/-232
2/32: b7 b5 -4/-233
3/32: d8 e7 0/-237
4/32: d8 d7 0/-244
5/32: c8 g4 -11/-244
6/32: f7 f5 -8/-246
7/32: d8 g5 -320/-247
8/32: f7 f6 -4/-249
9/32: g8 h6 0/-250
10/32: g7 g6 -2/-250
11/32: c8 e6 -11/-259
12/32: d6 d5 -7/-277
13/32: d4 d3 -7/-285
14/32: c8 h3 -11/-298
15/32: b8 c6 -10/-339
16/32: g8 f6 -10/-341
17/32: f8 e7 -11/-341
18/32: g8 e7 -5/-348
19/32: h7 h5 -2/-352
20/32: a7 a5 -2/-352
21/32: c8 d7 -11/-353
22/32: b7 b6 -2/-354
23/32: b8 a6 0/-354
24/32: c8 f5 -11/-355
25/32: a7 a6 -1/-355
26/32: h7 h6 -1/-355
27/32: d8 b6 0/-357
28/32: e8 e7 6/-361
29/32: b8 d7 -5/-364
30/32: e8 d7 6/-374
31/32: d8 f6 0/-456

8 rnb.kbnr
7 ppq..ppp
6 ...p....
5 ..p...N.
4 ..BpP...
3 ........
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
6. Qc7
score: -232, material: -106, abs: 7994, pawns: 6, mobility: 12
Depth  1 #searched     3057 bmove: g5 f7 bscore: 237
Depth  2 #searched    26630 bmove: g5 f7 bscore: 237
Depth  3 #searched   255875 bmove: g5 f7 bscore: 488
Depth  4 #searched  3180774 bmove: g5 f7 bscore: 484
Search total: 21179215 / 1727 ms / 12263 nodes/ms
hash size r 4 t 3809 
0/45: g5 f7 100/484
1/45: c4 b5 0/484
2/45: c4 b3 0/484
3/45: e1 d2 -6/484
4/45: g5 e6 10/480
5/45: c1 d2 11/477
6/45: a2 a4 2/477
7/45: c4 f1 -11/476
8/45: f2 f3 4/472
9/45: c4 f7 100/462
10/45: e4 e5 7/458
11/45: b2 b4 4/458
12/45: g5 h7 95/445
13/45: c1 f4 11/442
14/45: c2 c3 4/400
15/45: d1 d4 121/389
16/45: h2 h3 1/385
17/45: g2 g4 4/385
18/45: b1 c3 10/384
19/45: d1 g4 0/366
20/45: c4 a6 0/356
21/45: c4 d3 0/353
22/45: c4 e2 0/352
23/45: d1 h5 0/335
24/45: f2 f4 8/290
25/45: e1 f1 6/274
26/45: d1 d3 0/272
27/45: e1 g1 18/271
28/45: d1 e2 0/271
29/45: d1 f3 0/270
30/45: b1 d2 5/268
31/45: b2 b3 2/267
32/45: a2 a3 1/267
33/45: c4 d5 0/267
34/45: d1 d2 0/265
35/45: b1 a3 0/265
36/45: g2 g3 2/261
37/45: h1 g1 0/255
38/45: g5 f3 5/254
39/45: h1 f1 0/251
40/45: c4 e6 0/192
41/45: c1 e3 11/175
42/45: e1 e2 -6/162
43/45: h2 h4 2/64
44/45: g5 h3 -5/53

8 rnb.kbnr
7 ppq..Npp
6 ...p....
5 ..p.....
4 ..BpP...
3 ........
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
6. Nxf7
score: 484, material: -6, abs: 7894, pawns: 8, mobility: 13
Depth  1 #searched     4386 bmove: c7 f7 bscore: -237
Depth  2 #searched    38020 bmove: b7 b5 bscore: -365
Depth  3 #searched   464811 bmove: g8 f6 bscore: -196
Depth  4 #searched  2335764 bmove: g8 f6 bscore: -232
Search total: 23514979 / 1905 ms / 12343 nodes/ms
hash size r 2 t 3215 
0/31: g8 f6 -10/-232
1/31: a7 a6 -1/-232
2/31: c7 c6 0/-233
3/31: c7 f7 -320/-238
4/31: c7 a5 0/-248
5/31: h7 h6 -1/-256
6/31: d6 d5 -7/-258
7/31: h7 h5 -2/-258
8/31: g8 h6 0/-277
9/31: g7 g6 -2/-280
10/31: g8 e7 -5/-320
11/31: c8 d7 -11/-327
12/31: f8 e7 -11/-335
13/31: c8 g4 -11/-344
14/31: b7 b6 -2/-346
15/31: a7 a5 -2/-346
16/31: c7 b6 0/-372
17/31: c7 d7 0/-375
18/31: b8 c6 -10/-377
19/31: e8 e7 6/-379
20/31: b7 b5 -4/-383
21/31: e8 d7 6/-386
22/31: d4 d3 -7/-398
23/31: b8 d7 -5/-423
24/31: c7 e7 0/-450
25/31: c8 h3 -11/-452
26/31: g7 g5 -4/-458
27/31: b8 a6 0/-466
28/31: c8 f5 -11/-470
29/31: c8 e6 -11/-483
30/31: c7 d8 0/-613

8 rnb.kb.r
7 ppq..Npp
6 ...p.n..
5 ..p.....
4 ..BpP...
3 ........
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
7. Nf6
score: -232, material: -16, abs: 7904, pawns: 8, mobility: 9
Depth  1 #searched     5648 bmove: f7 h8 bscore: 357
Depth  2 #searched    71858 bmove: c2 c3 bscore: 143
Depth  3 #searched  1139857 bmove: f7 h8 bscore: 232
Search total: 24654836 / 1990 ms / 12389 nodes/ms
hash size r 3 t 1558 
0/47: f7 h8 495/232
1/47: f7 g5 0/228
2/47: f7 h6 -5/227
3/47: d1 d4 121/151
4/47: c1 d2 11/127
5/47: a2 a4 2/127
6/47: h2 h4 2/127
7/47: g2 g3 2/126
8/47: h2 h3 1/125
9/47: a2 a3 1/124
10/47: d1 e2 0/123
11/47: h1 g1 0/123
12/47: d1 d2 0/122
13/47: h1 f1 0/122
14/47: b1 d2 5/121
15/47: e1 e2 -6/121
16/47: c4 d5 0/110
17/47: c4 a6 0/109
18/47: c1 g5 11/105
19/47: f7 d6 117/104
20/47: g2 g4 4/103
21/47: d1 g4 0/102
22/47: d1 d3 0/88
23/47: d1 f3 0/81
24/47: c4 d3 0/78
25/47: e1 d2 -6/75
26/47: f7 e5 10/44
27/47: c1 e3 11/43
28/47: c1 h6 11/42
29/47: c2 c3 4/34
30/47: b2 b4 4/32
31/47: e4 e5 7/31
32/47: c4 e6 0/29
33/47: f7 d8 -5/28
34/47: f2 f4 8/26
35/47: f2 f3 4/25
36/47: c4 b5 0/24
37/47: e1 g1 18/21
38/47: c1 f4 11/18
39/47: d1 h5 0/16
40/47: c4 e2 0/12
41/47: e1 f1 6/10
42/47: c4 f1 -11/9
43/47: b2 b3 2/8
44/47: b1 a3 0/4
45/47: c4 b3 0/2
46/47: b1 c3 10/-80

8 rnb.kb.N
7 ppq...pp
6 ...p.n..
5 ..p.....
4 ..BpP...
3 ........
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
7. Nxh8
score: 232, material: 479, abs: 7399, pawns: 8, mobility: 8
Depth  1 #searched     4429 bmove: f6 e4 bscore: 438
Depth  2 #searched    57717 bmove: c7 a5 bscore: -508
Depth  3 #searched  1153252 bmove: d6 d5 bscore: -509
Search total: 25808088 / 2075 ms / 12437 nodes/ms
hash size r 2 t 1513 
0/35: d6 d5 -7/-509
1/35: d4 d3 -7/-517
2/35: e8 e7 6/-528
3/35: f6 d5 0/-549
4/35: f6 g8 10/-584
5/35: c7 e7 0/-586
6/35: f6 e4 -119/-593
7/35: b7 b5 -4/-595
8/35: c7 f7 0/-603
9/35: g7 g5 -4/-619
10/35: f6 d7 5/-622
11/35: c7 c6 0/-640
12/35: c7 d7 0/-655
13/35: c8 h3 -11/-666
14/35: c8 e6 -11/-672
15/35: c8 g4 -11/-681
16/35: f8 e7 -11/-688
17/35: c8 d7 -11/-695
18/35: b8 c6 -10/-699
19/35: c7 b6 0/-706
20/35: e8 d7 6/-706
21/35: h7 h6 -1/-707
22/35: g7 g6 -2/-708
23/35: b8 d7 -5/-708
24/35: b7 b6 -2/-710
25/35: a7 a5 -2/-710
26/35: h7 h5 -2/-711
27/35: c8 f5 -11/-711
28/35: a7 a6 -1/-712
29/35: c7 d8 0/-713
30/35: b8 a6 0/-713
31/35: e8 d8 0/-715
32/35: f6 h5 10/-721
33/35: f6 g4 5/-728
34/35: c7 a5 0/-729

8 rnb.kb.N
7 ppq...pp
6 .....n..
5 ..pp....
4 ..BpP...
3 ........
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
8. d5
score: -509, material: 472, abs: 7406, pawns: 8, mobility: 0
Depth  1 #searched     8294 bmove: e4 d5 bscore: 774
Depth  2 #searched   313963 bmove: d1 h5 bscore: 432
Depth  3 #searched  2162331 bmove: c4 d5 bscore: 426
Search total: 27970419 / 2226 ms / 12565 nodes/ms
hash size r 2 t 1287 
0/43: c4 d5 114/426
1/43: c1 f4 11/420
2/43: c1 g5 11/420
3/43: c1 h6 11/420
4/43: c1 e3 11/420
5/43: g2 g3 2/420
6/43: a2 a4 2/413
7/43: h2 h4 2/413
8/43: b2 b3 2/413
9/43: h2 h3 1/411
10/43: a2 a3 1/410
11/43: c4 b3 0/410
12/43: h1 g1 0/409
13/43: b1 a3 0/409
14/43: d1 d2 0/408
15/43: h1 f1 0/408
16/43: e1 e2 -6/405
17/43: c4 b5 0/404
18/43: f2 f4 8/394
19/43: c1 d2 11/384
20/43: g2 g4 4/384
21/43: h8 g6 5/383
22/43: f2 f3 4/382
23/43: e1 f1 6/381
24/43: e4 d5 118/378
25/43: b1 d2 5/373
26/43: c4 d3 0/373
27/43: c4 e2 0/372
28/43: c4 f1 -11/361
29/43: b1 c3 10/352
30/43: d1 e2 0/347
31/43: d1 d3 0/347
32/43: d1 f3 0/347
33/43: e4 e5 7/347
34/43: d1 h5 0/340
35/43: h8 f7 5/332
36/43: d1 g4 0/326
37/43: e1 g1 18/321
38/43: e1 d2 -6/303
39/43: c4 a6 0/269
40/43: c2 c3 4/234
41/43: b2 b4 4/171
42/43: d1 d4 121/-24

8 rnb.kb.N
7 ppq...pp
6 .....n..
5 ..pB....
4 ...pP...
3 ........
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
8. Bxd5
score: 426, material: 586, abs: 7292, pawns: -12, mobility: 0
Depth  1 #searched     7254 bmove: f8 d6 bscore: -302
Depth  2 #searched    85910 bmove: c7 a5 bscore: -476
Depth  3 #searched   916509 bmove: c7 a5 bscore: -323
Depth  4 #searched  3738901 bmove: c7 a5 bscore: -465
Search total: 31709320 / 2489 ms / 12739 nodes/ms
hash size r 2 t 3058 
0/41: c7 a5 0/-465
1/41: d4 d3 -7/-465
2/41: c7 c6 0/-470
3/41: h7 h5 -2/-471
4/41: b8 c6 -10/-473
5/41: f8 d6 -11/-473
6/41: c7 e5 0/-475
7/41: c7 b6 0/-477
8/41: h7 h6 -1/-478
9/41: f6 d5 -350/-482
10/41: g7 g6 -2/-487
11/41: e8 d8 0/-490
12/41: a7 a6 -1/-491
13/41: c7 d8 0/-491
14/41: f6 e4 -119/-496
15/41: c8 d7 -11/-498
16/41: e8 e7 6/-499
17/41: e8 d7 6/-502
18/41: f6 d7 5/-507
19/41: a7 a5 -2/-507
20/41: c7 d6 0/-510
21/41: b8 a6 0/-510
22/41: c7 d7 0/-514
23/41: f8 e7 -11/-518
24/41: c7 e7 0/-521
25/41: b8 d7 -5/-521
26/41: b7 b5 -4/-522
27/41: b7 b6 -2/-526
28/41: c8 g4 -11/-534
29/41: f6 g4 5/-543
30/41: c8 f5 -11/-543
31/41: c5 c4 -4/-566
32/41: c8 e6 -11/-571
33/41: c7 h2 -100/-573
34/41: g7 g5 -4/-596
35/41: c8 h3 -11/-622
36/41: c7 g3 0/-656
37/41: c7 f7 0/-656
38/41: c7 f4 0/-662
39/41: f6 g8 10/-666
40/41: f6 h5 10/-745

8 rnb.kb.N
7 pp....pp
6 .....n..
5 q.pB....
4 ...pP...
3 ........
2 PPP..PPP
1 RNBQK..R
  ABCDEFGH
9. Qa5+
score: -465, material: 586, abs: 7292, pawns: -12, mobility: 1
Depth  1 #searched     2677 bmove: e1 g1 bscore: 583
Depth  2 #searched    20743 bmove: b2 b4 bscore: 476
Depth  3 #searched   335455 bmove: e1 g1 bscore: 494
Depth  4 #searched  2609059 bmove: c2 c3 bscore: 561
Search total: 34318379 / 2674 ms / 12834 nodes/ms
hash size r 3 t 2503 
0/9: c2 c3 4/561
1/9: e1 g1 18/559
2/9: b1 d2 5/558
3/9: c1 d2 11/558
4/9: e1 f1 6/555
5/9: b2 b4 4/544
6/9: b1 c3 10/508
7/9: e1 e2 -6/495
8/9: d1 d2 0/414

8 rnb.kb.N
7 pp....pp
6 .....n..
5 q.pB....
4 ...pP...
3 ..P.....
2 PP...PPP
1 RNBQK..R
  ABCDEFGH
9. c3
score: 561, material: 590, abs: 7296, pawns: -12, mobility: 4
Depth  1 #searched     6466 bmove: f8 d6 bscore: -305
Depth  2 #searched   120482 bmove: c8 g4 bscore: -578
Depth  3 #searched   474335 bmove: c8 g4 bscore: -326
Depth  4 #searched  3651918 bmove: b8 d7 bscore: -532
Search total: 37970297 / 2926 ms / 12976 nodes/ms
hash size r 2 t 2963 
0/39: b8 d7 -5/-532
1/39: b8 c6 -10/-532
2/39: b7 b6 -2/-534
3/39: f8 e7 -11/-541
4/39: f6 e4 -119/-546
5/39: a5 c3 -104/-550
6/39: e8 e7 6/-555
7/39: f8 d6 -11/-559
8/39: g7 g6 -2/-563
9/39: f6 d5 -350/-565
10/39: h7 h5 -2/-566
11/39: b8 a6 0/-566
12/39: a7 a6 -1/-566
13/39: h7 h6 -1/-566
14/39: a5 b5 0/-566
15/39: a5 d8 0/-566
16/39: e8 d8 0/-567
17/39: a5 b6 0/-571
18/39: e8 d7 6/-577
19/39: b7 b5 -4/-583
20/39: a5 c7 0/-584
21/39: f6 d7 5/-593
22/39: c8 f5 -11/-593
23/39: d4 c3 -98/-606
24/39: a5 a2 -100/-608
25/39: f6 g4 5/-613
26/39: c8 d7 -11/-622
27/39: c8 e6 -11/-622
28/39: a5 b4 0/-638
29/39: c8 h3 -11/-644
30/39: g7 g5 -4/-661
31/39: f6 g8 10/-676
32/39: a5 a6 0/-684
33/39: c5 c4 -4/-712
34/39: d4 d3 -7/-722
35/39: f6 h5 10/-735
36/39: a5 a3 0/-865
37/39: c8 g4 -11/-878
38/39: a5 a4 0/-891

8 r.b.kb.N
7 pp.n..pp
6 .....n..
5 q.pB....
4 ...pP...
3 ..P.....
2 PP...PPP
1 RNBQK..R
  ABCDEFGH
10. N7d7
score: -532, material: 585, abs: 7301, pawns: -12, mobility: 10
Depth  1 #searched    20060 bmove: f2 f4 bscore: 594
Depth  2 #searched   106715 bmove: d5 f7 bscore: 1237
Depth  3 #searched  2666314 bmove: d5 b3 bscore: 632
Search total: 40636611 / 3110 ms / 13066 nodes/ms
hash size r 3 t 961 
0/43: d5 b3 0/632
1/43: d1 e2 0/628
2/43: d1 f3 0/628
3/43: d1 b3 0/628
4/43: h8 g6 5/625
5/43: d5 c4 0/625
6/43: f2 f3 4/621
7/43: e1 e2 -6/621
8/43: d1 d2 0/621
9/43: d1 c2 0/620
10/43: b1 a3 0/616
11/43: g2 g4 4/614
12/43: b1 d2 5/614
13/43: b2 b3 2/613
14/43: e1 g1 18/610
15/43: c1 h6 11/608
16/43: d1 g4 0/608
17/43: e1 f1 6/606
18/43: c1 f4 11/606
19/43: h8 f7 5/604
20/43: e1 d2 -6/604
21/43: h2 h4 2/598
22/43: a2 a4 2/598
23/43: g2 g3 2/596
24/43: h2 h3 1/596
25/43: a2 a3 1/596
26/43: d1 a4 0/595
27/43: f2 f4 8/595
28/43: h1 f1 0/595
29/43: h1 g1 0/594
30/43: d1 h5 0/589
31/43: c1 g5 11/589
32/43: d1 d3 0/589
33/43: d5 e6 0/582
34/43: d5 g8 0/581
35/43: d5 c6 0/578
36/43: b2 b4 4/577
37/43: c1 d2 11/577
38/43: d5 f7 0/552
39/43: e4 e5 7/545
40/43: d5 b7 100/544
41/43: c1 e3 11/448
42/43: d1 d4 121/90

8 r.b.kb.N
7 pp.n..pp
6 .....n..
5 q.p.....
4 ...pP...
3 .BP.....
2 PP...PPP
1 RNBQK..R
  ABCDEFGH
10. Bb3
score: 632, material: 585, abs: 7301, pawns: -12, mobility: 6
Depth  1 #searched     3901 bmove: f6 e4 bscore: 358
Depth  2 #searched   166411 bmove: a5 b6 bscore: -652
Depth  3 #searched   898213 bmove: c5 c4 bscore: -494
Depth  4 #searched  3194409 bmove: d7 e5 bscore: -293
Search total: 43831020 / 3332 ms / 13154 nodes/ms
hash size r 4 t 2628 
0/33: d7 e5 -5/-293
1/33: f6 h5 10/-310
2/33: b7 b5 -4/-314
3/33: f6 e4 -119/-339
4/33: e8 d8 0/-339
5/33: e8 e7 6/-344
6/33: a5 b6 0/-357
7/33: d4 d3 -7/-391
8/33: f8 d6 -11/-427
9/33: g7 g5 -4/-433
10/33: d7 b8 5/-435
11/33: h7 h5 -2/-464
12/33: g7 g6 -2/-466
13/33: a7 a6 -1/-471
14/33: h7 h6 -1/-476
15/33: a8 b8 0/-478
16/33: d7 b6 0/-492
17/33: b7 b6 -2/-495
18/33: a5 a6 0/-499
19/33: a5 b5 0/-499
20/33: a5 c7 0/-499
21/33: a5 d8 0/-519
22/33: a5 b4 0/-531
23/33: f6 g4 5/-602
24/33: f6 d5 0/-626
25/33: f6 g8 10/-664
26/33: a5 a4 0/-762
27/33: c5 c4 -4/-763
28/33: f8 e7 -11/-772
29/33: d4 c3 -98/-801
30/33: a5 a2 -100/-823
31/33: a5 c3 -104/-853
32/33: a5 a3 0/-905

8 r.b.kb.N
7 pp....pp
6 .....n..
5 q.p.n...
4 ...pP...
3 .BP.....
2 PP...PPP
1 RNBQK..R
  ABCDEFGH
11. Ne5
score: -293, material: 580, abs: 7306, pawns: -12, mobility: -7
Depth  1 #searched    17843 bmove: f2 f3 bscore: 785
Depth  2 #searched   171076 bmove: b3 a4 bscore: 525
Depth  3 #searched  1066829 bmove: b3 a4 bscore: 1350
Search total: 44897849 / 3401 ms / 13201 nodes/ms
hash size r 5 t 479 
0/38: b3 a4 0/1350
1/38: d1 c2 0/1350
2/38: b3 e6 0/1349
3/38: d1 h5 0/1254
4/38: d1 f3 0/1249
5/38: g2 g3 2/1239
6/38: h8 g6 5/1227
7/38: b3 f7 0/1116
8/38: h8 f7 5/1113
9/38: c1 h6 11/1089
10/38: a2 a4 2/1079
11/38: h2 h4 2/1078
12/38: a2 a3 1/1078
13/38: b3 g8 0/1077
14/38: h1 g1 0/1075
15/38: h1 f1 0/1074
16/38: d1 d4 121/1072
17/38: f2 f4 8/979
18/38: c1 f4 11/971
19/38: h2 h3 1/971
20/38: c1 g5 11/968
21/38: b3 d5 0/949
22/38: e1 g1 18/947
23/38: c1 e3 11/942
24/38: f2 f3 4/940
25/38: e1 f1 6/936
26/38: b1 a3 0/935
27/38: b3 c4 0/933
28/38: d1 d3 0/932
29/38: e1 e2 -6/929
30/38: d1 d2 0/923
31/38: d1 g4 0/922
32/38: c1 d2 11/921
33/38: g2 g4 4/915
34/38: d1 e2 0/907
35/38: b1 d2 5/813
36/38: e1 d2 -6/806
37/38: b3 c2 0/801

8 r.b.kb.N
7 pp....pp
6 .....n..
5 q.p.n...
4 B..pP...
3 ..P.....
2 PP...PPP
1 RNBQK..R
  ABCDEFGH
11. Ba4+
score: 1350, material: 580, abs: 7306, pawns: -12, mobility: -4
Depth  1 #searched     1270 bmove: e8 d8 bscore: 227
Depth  2 #searched    58418 bmove: a5 a4 bscore: -564
Depth  3 #searched   491552 bmove: e5 d7 bscore: -927
Depth  4 #searched  5844767 bmove: b7 b5 bscore: -459
Search total: 50742616 / 3799 ms / 13356 nodes/ms
hash size r 6 t 3319 
0/9: b7 b5 -4/-459
1/9: c8 d7 -11/-470
2/9: a5 a4 -350/-491
3/9: f6 d7 5/-517
4/9: e8 e7 6/-663
5/9: e8 d8 0/-696
6/9: e5 c6 0/-966
7/9: a5 b5 0/-1047
8/9: e5 d7 5/-1073

8 r.b.kb.N
7 p.....pp
6 .....n..
5 qpp.n...
4 B..pP...
3 ..P.....
2 PP...PPP
1 RNBQK..R
  ABCDEFGH
12. b5
score: -459, material: 576, abs: 7310, pawns: -12, mobility: -8
Depth  1 #searched    18809 bmove: a4 c2 bscore: 763
Depth  2 #searched   193146 bmove: d1 h5 bscore: 325
Depth  3 #searched  1852831 bmove: a4 c2 bscore: 569
Search total: 52595447 / 3924 ms / 13403 nodes/ms
hash size r 2 t 815 
0/36: a4 c2 0/569
1/36: d1 g4 0/569
2/36: c1 d2 11/569
3/36: d1 d4 121/568
4/36: b1 d2 5/568
5/36: e1 d2 -6/566
6/36: d1 e2 0/565
7/36: a4 b3 0/564
8/36: e1 e2 -6/551
9/36: d1 h5 0/534
10/36: a4 b5 104/532
11/36: f2 f4 8/500
12/36: h8 g6 5/498
13/36: c1 h6 11/494
14/36: c1 g5 11/491
15/36: f2 f3 4/489
16/36: h1 f1 0/487
17/36: h1 g1 0/486
18/36: h2 h4 2/484
19/36: d1 d3 0/482
20/36: b1 a3 0/481
21/36: b2 b3 2/481
22/36: c1 f4 11/479
23/36: c1 e3 11/478
24/36: g2 g3 2/478
25/36: h2 h3 1/478
26/36: a2 a3 1/477
27/36: d1 f3 0/477
28/36: e1 f1 6/475
29/36: h8 f7 5/469
30/36: d1 c2 0/462
31/36: d1 b3 0/453
32/36: b2 b4 4/394
33/36: e1 g1 18/345
34/36: g2 g4 4/309
35/36: d1 d2 0/267

8 r.b.kb.N
7 p.....pp
6 .....n..
5 qpp.n...
4 ...pP...
3 ..P.....
2 PPB..PPP
1 RNBQK..R
  ABCDEFGH
12. Bc2
score: 569, material: 576, abs: 7310, pawns: -12, mobility: -10
Depth  1 #searched     6431 bmove: c8 e6 bscore: 250
Depth  2 #searched   140173 bmove: a5 c3 bscore: -588
Depth  3 #searched  1952158 bmove: d4 c3 bscore: 17
Search total: 54547605 / 4060 ms / 13435 nodes/ms
hash size r 3 t 1844 
0/45: d4 c3 -98/17
1/45: f6 g8 10/11
2/45: g7 g5 -4/-9
3/45: c8 h3 -11/-61
4/45: c8 f5 -11/-82
5/45: f6 d5 0/-106
6/45: c5 c4 -4/-118
7/45: f6 d7 5/-375
8/45: f6 h5 10/-430
9/45: a7 a6 -1/-536
10/45: f6 g4 5/-537
11/45: e5 c6 0/-538
12/45: a8 b8 0/-542
13/45: h7 h5 -2/-542
14/45: h7 h6 -1/-542
15/45: e8 d7 6/-543
16/45: g7 g6 -2/-544
17/45: c8 b7 -11/-550
18/45: c8 a6 -11/-553
19/45: c8 d7 -11/-554
20/45: f8 d6 -11/-555
21/45: f8 e7 -11/-558
22/45: e5 c4 -5/-558
23/45: c8 g4 -11/-559
24/45: c8 e6 -11/-561
25/45: e5 g4 5/-561
26/45: e5 d7 5/-567
27/45: e8 e7 6/-571
28/45: d4 d3 -7/-576
29/45: e8 d8 0/-590
30/45: e5 f3 -5/-595
31/45: a5 a2 -100/-608
32/45: a5 b4 0/-631
33/45: b5 b4 -2/-651
34/45: a5 a3 0/-666
35/45: a5 c7 0/-688
36/45: a5 a6 0/-696
37/45: a5 a4 0/-706
38/45: f6 e4 -119/-787
39/45: a5 d8 0/-916
40/45: a5 b6 0/-918
41/45: e5 f7 5/-920
42/45: e5 d3 -5/-920
43/45: e5 g6 5/-927
44/45: a5 c3 -104/-1413

8 r.b.kb.N
7 p.....pp
6 .....n..
5 qpp.n...
4 ....P...
3 ..p.....
2 PPB..PPP
1 RNBQK..R
  ABCDEFGH
13. xc3+
score: 17, material: 478, abs: 7200, pawns: -10, mobility: -4
Depth  1 #searched     6230 bmove: b1 c3 bscore: 813
Depth  2 #searched   193135 bmove: c1 d2 bscore: 458
Depth  3 #searched  1995285 bmove: b2 c3 bscore: 583
Search total: 56542890 / 4199 ms / 13465 nodes/ms
hash size r 2 t 1736 
0/40: b2 c3 119/583
1/40: b2 b4 4/502
2/40: h8 g6 5/502
3/40: e1 g1 18/502
4/40: d1 h5 0/502
5/40: d1 d7 0/502
6/40: h8 f7 5/501
7/40: d1 d6 0/500
8/40: d1 d4 0/498
9/40: b2 b3 2/480
10/40: e1 e2 -6/453
11/40: d1 d8 0/440
12/40: d1 d5 0/414
13/40: b1 c3 125/402
14/40: e1 f1 6/281
15/40: c2 b3 0/280
16/40: g2 g4 4/279
17/40: c2 d3 0/275
18/40: d1 g4 0/266
19/40: d1 d2 0/261
20/40: h1 g1 0/233
21/40: h1 f1 0/223
22/40: g2 g3 2/168
23/40: h2 h4 2/141
24/40: h2 h3 1/139
25/40: d1 e2 0/137
26/40: a2 a4 2/134
27/40: c1 h6 11/-19
28/40: f2 f3 4/-19
29/40: c1 e3 11/-113
30/40: b1 d2 5/-124
31/40: b1 a3 0/-129
32/40: f2 f4 8/-217
33/40: c1 g5 11/-218
34/40: c1 f4 11/-219
35/40: d1 d3 0/-249
36/40: a2 a3 1/-251
37/40: d1 f3 0/-255
38/40: c2 a4 0/-293
39/40: c1 d2 11/-530

8 r.b.kb.N
7 p.....pp
6 .....n..
5 qpp.n...
4 ....P...
3 ..P.....
2 P.B..PPP
1 RNBQK..R
  ABCDEFGH
13. xc3
score: 583, material: 597, abs: 7089, pawns: -6, mobility: -5
Depth  1 #searched     4031 bmove: h7 h5 bscore: 202
Depth  2 #searched    82172 bmove: e5 f3 bscore: -597
Depth  3 #searched  1086918 bmove: a7 a6 bscore: -408
Search total: 57629808 / 4274 ms / 13483 nodes/ms
hash size r 2 t 1262 
0/41: a7 a6 -1/-408
1/41: c8 f5 -11/-434
2/41: f6 h5 10/-441
3/41: h7 h5 -2/-446
4/41: c8 h3 -11/-456
5/41: c8 d7 -11/-465
6/41: a5 a6 0/-467
7/41: c8 b7 -11/-470
8/41: f6 d5 0/-483
9/41: e5 c4 -5/-541
10/41: a5 c7 0/-544
11/41: f8 d6 -11/-551
12/41: e8 e7 6/-559
13/41: a8 b8 0/-566
14/41: a5 b6 0/-573
15/41: g7 g6 -2/-578
16/41: h7 h6 -1/-579
17/41: e5 c6 0/-580
18/41: e5 g4 5/-586
19/41: f6 g4 5/-587
20/41: f6 g8 10/-593
21/41: g7 g5 -4/-597
22/41: c8 a6 -11/-602
23/41: c5 c4 -4/-604
24/41: c8 g4 -11/-607
25/41: f8 e7 -11/-615
26/41: b5 b4 -2/-629
27/41: c8 e6 -11/-650
28/41: e5 g6 5/-733
29/41: e5 f7 5/-743
30/41: e5 d3 -5/-768
31/41: f6 e4 -119/-790
32/41: a5 c3 -104/-798
33/41: a5 d8 0/-805
34/41: a5 a2 -100/-808
35/41: e5 f3 -5/-814
36/41: f6 d7 5/-818
37/41: e5 d7 5/-833
38/41: a5 a4 0/-899
39/41: a5 a3 0/-906
40/41: a5 b4 0/-910

8 r.b.kb.N
7 ......pp
6 p....n..
5 qpp.n...
4 ....P...
3 ..P.....
2 P.B..PPP
1 RNBQK..R
  ABCDEFGH
14. a6
score: -408, material: 596, abs: 7090, pawns: -6, mobility: -3
Depth  1 #searched    10326 bmove: f2 f4 bscore: 818
Depth  2 #searched    83997 bmove: a2 a4 bscore: 351
Depth  3 #searched   792055 bmove: a2 a4 bscore: 821
Depth  4 #searched  4004616 bmove: a2 a4 bscore: 629
Search total: 61634424 / 4556 ms / 13528 nodes/ms
hash size r 2 t 4338 
0/39: a2 a4 2/629
1/39: c1 f4 11/629
2/39: h8 f7 5/629
3/39: c1 b2 11/629
4/39: d1 d6 0/629
5/39: g2 g3 2/629
6/39: d1 e2 0/629
7/39: c1 d2 11/629
8/39: e1 e2 -6/629
9/39: g2 g4 4/629
10/39: c1 h6 11/629
11/39: h8 g6 5/629
12/39: h2 h3 1/628
13/39: e1 g1 18/628
14/39: d1 g4 0/628
15/39: a2 a3 1/628
16/39: d1 d2 0/628
17/39: h1 f1 0/628
18/39: f2 f4 8/628
19/39: c2 b3 0/628
20/39: h1 g1 0/627
21/39: e1 f1 6/626
22/39: d1 d4 0/625
23/39: c2 a4 0/625
24/39: f2 f3 4/624
25/39: h2 h4 2/624
26/39: b1 d2 5/624
27/39: d1 d3 0/622
28/39: d1 f3 0/622
29/39: d1 d7 0/622
30/39: b1 a3 0/620
31/39: c1 e3 11/620
32/39: d1 h5 0/618
33/39: c2 d3 0/616
34/39: c1 g5 11/615
35/39: e1 d2 -6/613
36/39: d1 d5 0/604
37/39: c1 a3 11/600
38/39: d1 d8 0/520

8 r.b.kb.N
7 ......pp
6 p....n..
5 qpp.n...
4 P...P...
3 ..P.....
2 ..B..PPP
1 RNBQK..R
  ABCDEFGH
14. a4
score: 629, material: 598, abs: 7092, pawns: -6, mobility: -2
Depth  1 #searched     6738 bmove: c8 e6 bscore: -572
Depth  2 #searched   135081 bmove: e5 f7 bscore: -618
Depth  3 #searched   556235 bmove: e5 f7 bscore: -338
Depth  4 #searched  4245039 bmove: c8 g4 bscore: -496
Search total: 65879463 / 4839 ms / 13614 nodes/ms
hash size r 2 t 3712 
0/38: c8 g4 -11/-496
1/38: e5 c6 0/-497
2/38: a5 c7 0/-499
3/38: c8 h3 -11/-499
4/38: c8 f5 -11/-501
5/38: c8 e6 -11/-502
6/38: e8 e7 6/-503
7/38: a8 a7 0/-505
8/38: f6 h5 10/-505
9/38: e5 d7 5/-511
10/38: e5 d3 -5/-517
11/38: f6 d5 0/-544
12/38: a5 a4 -102/-556
13/38: b5 b4 -2/-559
14/38: g7 g6 -2/-560
15/38: c8 b7 -11/-573
16/38: a5 b6 0/-577
17/38: b5 a4 -102/-579
18/38: c5 c4 -4/-583
19/38: c8 d7 -11/-584
20/38: f8 e7 -11/-585
21/38: a5 d8 0/-586
22/38: e5 c4 -5/-590
23/38: a8 b8 0/-595
24/38: h7 h6 -1/-596
25/38: f6 g4 5/-596
26/38: f6 g8 10/-596
27/38: f6 d7 5/-596
28/38: h7 h5 -2/-596
29/38: a5 c3 -104/-599
30/38: e5 g4 5/-603
31/38: g7 g5 -4/-668
32/38: e5 g6 5/-699
33/38: f6 e4 -119/-705
34/38: a5 b4 0/-896
35/38: f8 d6 -11/-927
36/38: e5 f3 -5/-1147
37/38: e5 f7 5/-1171

8 r...kb.N
7 ......pp
6 p....n..
5 qpp.n...
4 P...P.b.
3 ..P.....
2 ..B..PPP
1 RNBQK..R
  ABCDEFGH
15. Bg4
score: -496, material: 587, abs: 7103, pawns: -6, mobility: -7
Depth  1 #searched    20795 bmove: f2 f3 bscore: 797
Depth  2 #searched   201254 bmove: f2 f3 bscore: 576
Depth  3 #searched  1304382 bmove: f2 f3 bscore: 789
Search total: 67183845 / 4924 ms / 13644 nodes/ms
hash size r 3 t 451 
0/36: f2 f3 4/789
1/36: h1 f1 0/789
2/36: e1 f1 6/789
3/36: g2 g3 2/789
4/36: d1 d2 0/789
5/36: c1 b2 11/789
6/36: a1 a2 0/789
7/36: h2 h3 1/789
8/36: a1 a3 0/789
9/36: c1 d2 11/789
10/36: d1 f3 0/789
11/36: h1 g1 0/789
12/36: d1 d3 0/789
13/36: d1 e2 0/788
14/36: e1 g1 18/787
15/36: e1 d2 -6/786
16/36: b1 d2 5/786
17/36: c2 b3 0/783
18/36: d1 d7 0/783
19/36: h2 h4 2/780
20/36: d1 d4 0/780
21/36: d1 d6 0/769
22/36: d1 d8 0/758
23/36: c1 a3 11/725
24/36: d1 d5 0/723
25/36: a4 b5 108/715
26/36: h8 g6 5/705
27/36: f2 f4 8/657
28/36: c1 e3 11/649
29/36: c1 g5 11/647
30/36: c1 f4 11/647
31/36: h8 f7 5/644
32/36: c1 h6 11/633
33/36: d1 g4 350/630
34/36: b1 a3 0/619
35/36: c2 d3 0/597

8 r...kb.N
7 ......pp
6 p....n..
5 qpp.n...
4 P...P.b.
3 ..P..P..
2 ..B...PP
1 RNBQK..R
  ABCDEFGH
15. f3
score: 789, material: 591, abs: 7107, pawns: 0, mobility: -6
Depth  1 #searched     9649 bmove: e8 b8 bscore: -561
Depth  2 #searched   138435 bmove: e5 f7 bscore: -612
Depth  3 #searched   679933 bmove: e8 b8 bscore: -407
Depth  4 #searched  2386818 bmove: e8 b8 bscore: -407
Search total: 69570663 / 5086 ms / 13678 nodes/ms
hash size r 2 t 2552 
0/40: e8 b8 -18/-407
1/40: f8 e7 -11/-407
2/40: a5 c7 0/-408
3/40: a5 b6 0/-409
4/40: g4 f3 -104/-412
5/40: c5 c4 -4/-412
6/40: g7 g6 -2/-412
7/40: g4 e6 0/-413
8/40: e5 f3 -109/-417
9/40: h7 h6 -1/-418
10/40: h7 h5 -2/-419
11/40: a8 b8 0/-419
12/40: a8 c8 0/-419
13/40: a8 d8 0/-420
14/40: e5 c4 -5/-422
15/40: e5 c6 0/-422
16/40: f6 g8 10/-422
17/40: f6 h5 10/-422
18/40: g4 h5 0/-422
19/40: f6 d7 5/-422
20/40: e5 d7 5/-422
21/40: f6 d5 0/-422
22/40: e5 f7 5/-422
23/40: g4 h3 0/-423
24/40: e8 e7 6/-424
25/40: a8 a7 0/-427
26/40: e5 d3 -5/-429
27/40: g4 d7 0/-444
28/40: f8 d6 -11/-456
29/40: g4 f5 0/-460
30/40: b5 b4 -2/-470
31/40: g7 g5 -4/-486
32/40: b5 a4 -102/-500
33/40: f6 e4 -119/-579
34/40: a5 a4 -102/-591
35/40: g4 c8 11/-602
36/40: a5 d8 0/-678
37/40: e5 g6 5/-740
38/40: a5 b4 0/-928
39/40: a5 c3 -104/-964
Castle!!

8 .k.r.b.N
7 ......pp
6 p....n..
5 qpp.n...
4 P...P.b.
3 ..P..P..
2 ..B...PP
1 RNBQK..R
  ABCDEFGH
16. 0-0-0 Kb8+
score: -407, material: 573, abs: 7125, pawns: 0, mobility: -12
Depth  1 #searched    29544 bmove: d1 e2 bscore: 776
Depth  2 #searched   452151 bmove: f3 g4 bscore: 403
Depth  3 #searched  3576235 bmove: f3 g4 bscore: 524
Search total: 73146898 / 5329 ms / 13726 nodes/ms
hash size r 3 t 1114 
0/35: f3 g4 350/524
1/35: e1 g1 18/524
2/35: d1 d8 500/523
3/35: c1 f4 11/521
4/35: h8 f7 5/513
5/35: c1 b2 11/511
6/35: h8 g6 5/509
7/35: d1 d3 0/508
8/35: e1 e2 -6/508
9/35: h2 h3 1/507
10/35: b1 a3 0/499
11/35: d1 d5 0/495
12/35: c1 h6 11/492
13/35: c2 b3 0/490
14/35: c1 g5 11/488
15/35: d1 d4 0/488
16/35: d1 e2 0/485
17/35: d1 d7 0/481
18/35: g2 g3 2/479
19/35: e1 f2 0/467
20/35: a4 b5 108/461
21/35: c1 a3 11/457
22/35: d1 d6 0/454
23/35: c2 d3 0/453
24/35: c1 e3 11/438
25/35: b1 d2 5/437
26/35: a1 a3 0/435
27/35: c1 d2 11/430
28/35: a1 a2 0/428
29/35: h2 h4 2/423
30/35: h1 g1 0/419
31/35: h1 f1 0/419
32/35: e1 f1 6/408
33/35: d1 d2 0/296
34/35: f3 f4 4/60

8 .k.r.b.N
7 ......pp
6 p....n..
5 qpp.n...
4 P...P.P.
3 ..P.....
2 ..B...PP
1 RNBQK..R
  ABCDEFGH
16. xg4
score: 524, material: 923, abs: 6775, pawns: -32, mobility: -7
Depth  1 #searched    11090 bmove: d8 d1 bscore: -406
Depth  2 #searched    59237 bmove: d8 d1 bscore: -406
Depth  3 #searched   262039 bmove: d8 d1 bscore: -415
Depth  4 #searched 11196282 bmove: f8 e7 bscore: -353
Search total: 84343180 / 6116 ms / 13790 nodes/ms
hash size r 2 t 10091 
0/43: f8 e7 -11/-353
1/43: f6 g8 10/-356
2/43: d8 d6 0/-357
3/43: f6 e8 10/-361
4/43: c5 c4 -4/-372
5/43: e5 f3 -5/-393
6/43: e5 c4 -5/-423
7/43: g7 g6 -2/-427
8/43: a5 c7 0/-429
9/43: a5 b6 0/-429
10/43: h7 h6 -1/-430
11/43: e5 d7 5/-436
12/43: d8 d7 0/-439
13/43: e5 d3 -5/-451
14/43: h7 h5 -2/-462
15/43: f6 e4 -119/-462
16/43: b5 b4 -2/-470
17/43: b8 a8 0/-476
18/43: g7 g5 -4/-486
19/43: b8 b7 12/-494
20/43: d8 d3 0/-512
21/43: e5 g6 5/-530
22/43: f6 d5 0/-537
23/43: b8 c7 18/-549
24/43: e5 g4 -99/-549
25/43: f8 d6 -11/-549
26/43: b8 a7 0/-568
27/43: f6 h5 10/-568
28/43: f6 d7 5/-583
29/43: b8 c8 12/-597
30/43: d8 d1 -900/-615
31/43: b5 a4 -102/-622
32/43: d8 d4 0/-625
33/43: d8 e8 0/-677
34/43: d8 d2 -22/-690
35/43: d8 d5 0/-710
36/43: e5 f7 5/-728
37/43: f6 g4 -99/-742
38/43: e5 c6 0/-774
39/43: d8 c8 0/-794
40/43: a5 a4 -102/-868
41/43: a5 c3 -104/-967
42/43: a5 b4 0/-1221

8 .k.r...N
7 ....b.pp
6 p....n..
5 qpp.n...
4 P...P.P.
3 ..P.....
2 ..B...PP
1 RNBQK..R
  ABCDEFGH
17. Be7+
score: -353, material: 912, abs: 6786, pawns: -32, mobility: -10
Depth  1 #searched    11791 bmove: c1 f4 bscore: 483
Depth  2 #searched   287259 bmove: c1 f4 bscore: 734
Depth  3 #searched  2557096 bmove: d1 e2 bscore: 489
Search total: 86900276 / 6296 ms / 13802 nodes/ms
hash size r 3 t 2160 
0/35: d1 e2 0/489
1/35: c1 d2 11/482
2/35: c1 a3 11/482
3/35: d1 d8 500/482
4/35: c1 e3 11/482
5/35: c1 g5 11/482
6/35: c1 b2 11/482
7/35: e1 f1 6/482
8/35: h1 f1 0/482
9/35: h2 h3 1/482
10/35: h1 g1 0/482
11/35: d1 d3 0/482
12/35: h2 h4 2/481
13/35: g2 g3 2/481
14/35: a1 a2 0/481
15/35: e1 e2 -6/481
16/35: c1 f4 11/480
17/35: h8 f7 5/480
18/35: a1 a3 0/480
19/35: d1 d4 0/480
20/35: c1 h6 11/479
21/35: g4 g5 2/479
22/35: d1 d7 0/478
23/35: e1 f2 0/477
24/35: b1 a3 0/476
25/35: b1 d2 5/474
26/35: a4 b5 108/469
27/35: c2 d3 0/467
28/35: c2 b3 0/451
29/35: d1 d5 0/439
30/35: e1 g1 18/415
31/35: d1 d2 0/400
32/35: d1 f3 0/364
33/35: d1 d6 0/252
34/35: h8 g6 5/212

8 .k.r...N
7 ....b.pp
6 p....n..
5 qpp.n...
4 P...P.P.
3 ..P.....
2 ..B.Q.PP
1 RNB.K..R
  ABCDEFGH
17. Qe2
score: 489, material: 912, abs: 6786, pawns: -32, mobility: -9
Depth  1 #searched    14359 bmove: d8 h8 bscore: 223
Depth  2 #searched   174115 bmove: b5 a4 bscore: -489
Depth  3 #searched  1825021 bmove: d8 h8 bscore: -700
Search total: 88725297 / 6416 ms / 13828 nodes/ms
hash size r 4 t 1016 
0/46: d8 h8 -315/-700
1/46: f6 h5 10/-718
2/46: d8 d3 0/-727
3/46: d8 d6 0/-738
4/46: d8 d7 0/-773
5/46: e7 f8 11/-789
6/46: d8 d2 -22/-790
7/46: f6 e4 -119/-791
8/46: h7 h5 -2/-797
9/46: f6 d7 5/-801
10/46: b8 b7 12/-804
11/46: a5 c7 0/-814
12/46: d8 d1 0/-817
13/46: b8 a7 0/-824
14/46: c5 c4 -4/-825
15/46: d8 d5 0/-827
16/46: f6 g4 -99/-833
17/46: b5 b4 -2/-835
18/46: e5 d3 -5/-835
19/46: h7 h6 -1/-849
20/46: g7 g6 -2/-849
21/46: a5 b6 0/-849
22/46: d8 f8 0/-853
23/46: d8 d4 0/-854
24/46: d8 c8 0/-855
25/46: e5 c6 0/-856
26/46: d8 g8 0/-857
27/46: d8 e8 0/-857
28/46: f6 g8 10/-863
29/46: f6 e8 10/-864
30/46: g7 g5 -4/-865
31/46: e5 f3 -5/-865
32/46: e7 d6 0/-868
33/46: b8 c7 18/-869
34/46: a5 c3 -104/-869
35/46: a5 a4 -102/-870
36/46: e5 c4 -5/-877
37/46: b8 a8 0/-877
38/46: e5 f7 5/-877
39/46: b8 c8 12/-883
40/46: e5 g6 5/-888
41/46: e5 d7 5/-896
42/46: b5 a4 -102/-912
43/46: f6 d5 0/-942
44/46: e5 g4 -99/-971
45/46: a5 b4 0/-989

8 .k.....r
7 ....b.pp
6 p....n..
5 qpp.n...
4 P...P.P.
3 ..P.....
2 ..B.Q.PP
1 RNB.K..R
  ABCDEFGH
18. Rxh8
score: -700, material: 597, abs: 6471, pawns: -32, mobility: -6
Depth  1 #searched     7636 bmove: c1 f4 bscore: 778
Depth  2 #searched    58436 bmove: c1 f4 bscore: 577
Depth  3 #searched   701148 bmove: c1 f4 bscore: 783
Depth  4 #searched  2662959 bmove: c1 f4 bscore: 586
Search total: 91388256 / 6595 ms / 13857 nodes/ms
hash size r 2 t 2388 
0/35: c1 f4 11/586
1/35: c2 d1 -11/586
2/35: e1 f2 0/586
3/35: g2 g3 2/586
4/35: e1 d1 0/586
5/35: e1 f1 6/586
6/35: h2 h4 2/586
7/35: e2 d1 0/586
8/35: e1 d2 -6/586
9/35: c1 e3 11/586
10/35: c1 d2 11/586
11/35: c1 b2 11/586
12/35: c2 b3 0/586
13/35: h1 g1 0/586
14/35: a1 a3 0/586
15/35: e2 e3 0/586
16/35: h1 f1 0/586
17/35: e2 f2 0/586
18/35: e2 f1 0/586
19/35: e2 d2 0/586
20/35: c1 g5 11/586
21/35: g4 g5 2/586
22/35: h2 h3 1/586
23/35: e2 d3 0/586
24/35: e2 c4 0/586
25/35: e1 g1 18/586
26/35: c1 h6 11/586
27/35: c1 a3 11/586
28/35: b1 d2 5/586
29/35: e2 f3 0/586
30/35: b1 a3 0/586
31/35: c2 d3 0/586
32/35: a1 a2 0/585
33/35: a4 b5 108/575
34/35: e2 b5 104/378

8 .k.....r
7 ....b.pp
6 p....n..
5 qpp.n...
4 P...PBP.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
18. Bf4
score: 586, material: 608, abs: 6482, pawns: -32, mobility: -6
Depth  1 #searched     4399 bmove: e7 d6 bscore: -577
Depth  2 #searched    44736 bmove: a5 c7 bscore: -675
Depth  3 #searched   256666 bmove: e7 d6 bscore: -610
Depth  4 #searched  6885305 bmove: e7 f8 bscore: -927
Search total: 98273561 / 7140 ms / 13763 nodes/ms
hash size r 3 t 6776 
0/33: e7 f8 11/-927
1/33: b8 c7 18/-928
2/33: e7 d8 11/-930
3/33: h8 g8 0/-931
4/33: h7 h6 -1/-932
5/33: h8 f8 0/-933
6/33: h8 c8 0/-934
7/33: g7 g5 -4/-935
8/33: h8 e8 0/-936
9/33: h8 d8 0/-937
10/33: a5 b6 0/-938
11/33: b5 b4 -2/-942
12/33: f6 g8 10/-942
13/33: f6 d7 5/-953
14/33: f6 g4 -99/-957
15/33: b8 c8 12/-962
16/33: f6 e8 10/-964
17/33: a5 c7 0/-966
18/33: f6 h5 10/-995
19/33: h7 h5 -2/-1007
20/33: g7 g6 -2/-1008
21/33: c5 c4 -4/-1014
22/33: a5 d8 0/-1020
23/33: b5 a4 -102/-1028
24/33: e7 d6 0/-1033
25/33: f6 d5 0/-1043
26/33: a5 a4 -102/-1043
27/33: b8 b7 12/-1049
28/33: b8 a8 0/-1052
29/33: f6 e4 -119/-1053
30/33: b8 a7 0/-1097
31/33: a5 c3 -104/-1260
32/33: a5 b4 0/-1370

8 .k...b.r
7 ......pp
6 p....n..
5 qpp.n...
4 P...PBP.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
19. Bf8
score: -927, material: 619, abs: 6471, pawns: -32, mobility: -1
Depth  1 #searched     5682 bmove: f4 e5 bscore: 920
Depth  2 #searched    41443 bmove: f4 e5 bscore: 1167
Depth  3 #searched   279499 bmove: f4 e5 bscore: 944
Depth  4 #searched  1389362 bmove: f4 e5 bscore: 1163
Search total: 99662923 / 7245 ms / 13756 nodes/ms
hash size r 4 t 1642 
0/35: f4 e5 325/1163
1/35: e1 g1 18/1163
2/35: c2 d1 -11/1163
3/35: h2 h4 2/1163
4/35: h1 g1 0/1163
5/35: e1 f2 0/1162
6/35: e1 d1 0/1162
7/35: e1 d2 -6/1162
8/35: e2 d2 0/1161
9/35: e1 f1 6/1161
10/35: h2 h3 1/1161
11/35: e2 e3 0/1160
12/35: c2 b3 0/1160
13/35: e2 c4 0/1160
14/35: g2 g3 2/1160
15/35: a1 a2 0/1160
16/35: a1 a3 0/1160
17/35: f4 g3 0/1160
18/35: b1 d2 5/1159
19/35: f4 d2 0/1158
20/35: g4 g5 2/1157
21/35: e2 d3 0/1149
22/35: e2 f3 0/1142
23/35: e2 b5 104/1137
24/35: b1 a3 0/1108
25/35: a4 b5 108/1103
26/35: e2 f2 0/1101
27/35: e2 d1 0/1097
28/35: e2 f1 0/1096
29/35: c2 d3 0/1082
30/35: h1 f1 0/1077
31/35: f4 h6 0/1068
32/35: f4 e3 0/1058
33/35: f4 g5 0/1004
34/35: f4 c1 -11/993

8 .k...b.r
7 ......pp
6 p....n..
5 qpp.B...
4 P...P.P.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
19. Bxe5+
score: 1163, material: 944, abs: 6146, pawns: -32, mobility: 7
Depth  1 #searched     1704 bmove: b8 b7 bscore: -918
Depth  2 #searched    41400 bmove: b8 a8 bscore: -1160
Depth  3 #searched   111790 bmove: b8 b7 bscore: -979
Depth  4 #searched  1187939 bmove: a5 c7 bscore: -1835
Search total: 100850862 / 7337 ms / 13745 nodes/ms
hash size r 2 t 1294 
0/6: a5 c7 0/-1835
1/6: f8 d6 -11/-1841
2/6: b8 c8 12/-1849
3/6: b8 a7 0/-1852
4/6: b8 a8 0/-1853
5/6: b8 b7 12/-1863

8 .k...b.r
7 ..q...pp
6 p....n..
5 .pp.B...
4 P...P.P.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
20. Qc7
score: -1835, material: 944, abs: 6146, pawns: -32, mobility: 1
Depth  1 #searched     4416 bmove: e5 c7 bscore: 1629
Depth  2 #searched    26742 bmove: e5 c7 bscore: 1703
Depth  3 #searched   224316 bmove: e5 c7 bscore: 1641
Depth  4 #searched  1075571 bmove: e5 c7 bscore: 1641
Search total: 101926433 / 7419 ms / 13738 nodes/ms
hash size r 3 t 1771 
0/36: e5 c7 900/1641
1/36: e2 b5 104/1641
2/36: g4 g5 2/1641
3/36: e1 f1 6/1641
4/36: a4 a5 2/1641
5/36: b1 a3 0/1641
6/36: b1 d2 5/1641
7/36: a4 b5 108/1641
8/36: c2 b3 0/1640
9/36: h1 f1 0/1640
10/36: h2 h4 2/1640
11/36: g2 g3 2/1639
12/36: h2 h3 1/1639
13/36: a1 a2 0/1639
14/36: c3 c4 4/1639
15/36: e5 d6 0/1638
16/36: e5 f4 0/1638
17/36: a1 a3 0/1638
18/36: e2 c4 0/1638
19/36: e2 f1 0/1638
20/36: e2 d3 0/1638
21/36: e5 g3 0/1637
22/36: h1 g1 0/1637
23/36: e1 d1 0/1637
24/36: e1 d2 -6/1635
25/36: e1 f2 0/1635
26/36: c2 d3 0/1635
27/36: e1 g1 18/1629
28/36: c2 d1 -11/1627
29/36: e5 f6 325/1552
30/36: e2 d2 0/1529
31/36: e5 d4 0/1524
32/36: e2 d1 0/1523
33/36: e2 e3 0/1511
34/36: e2 f2 0/1507
35/36: e2 f3 0/1506

8 .k...b.r
7 ..B...pp
6 p....n..
5 .pp.....
4 P...P.P.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
20. Bxc7
score: 1641, material: 1844, abs: 5246, pawns: -32, mobility: 15
Depth  1 #searched      708 bmove: b8 c7 bscore: -1629
Depth  2 #searched    17089 bmove: b8 c7 bscore: -1703
Depth  3 #searched    43135 bmove: b8 c7 bscore: -1641
Depth  4 #searched    78282 bmove: b8 c7 bscore: -1641
Depth  5 #searched   204277 bmove: b8 c7 bscore: -1641
Depth  6 #searched   643353 bmove: b8 c7 bscore: -1712
Depth  7 #searched  1804484 bmove: b8 c7 bscore: -1495
Search total: 103730917 / 7553 ms / 13733 nodes/ms
hash size r 2 t 4494 
0/5: b8 c7 -332/-1495
1/5: b8 b7 12/-1496
2/5: b8 a7 0/-1499
3/5: b8 c8 12/-1502
4/5: b8 a8 0/-1524

8 .....b.r
7 ..k...pp
6 p....n..
5 .pp.....
4 P...P.P.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
21. Kxc7
score: -1495, material: 1512, abs: 4878, pawns: -32, mobility: 4
Depth  1 #searched     4131 bmove: a4 a5 bscore: 1703
Depth  2 #searched    27897 bmove: g4 g5 bscore: 1641
Depth  3 #searched   217521 bmove: e4 e5 bscore: 1662
Depth  4 #searched   963801 bmove: a4 b5 bscore: 1657
Depth  5 #searched  4508780 bmove: a4 b5 bscore: 1805
Search total: 108239697 / 7905 ms / 13692 nodes/ms
hash size r 2 t 7822 
0/31: a4 b5 108/1805
1/31: e2 f2 0/1805
2/31: e1 f2 0/1805
3/31: e2 e3 0/1805
4/31: e2 f3 0/1805
5/31: e4 e5 7/1805
6/31: b1 d2 5/1805
7/31: g4 g5 2/1805
8/31: c2 b3 0/1805
9/31: e1 f1 6/1805
10/31: h2 h4 2/1805
11/31: e2 c4 0/1805
12/31: e2 f1 0/1805
13/31: a1 a3 0/1805
14/31: h1 f1 0/1805
15/31: g2 g3 2/1805
16/31: a1 a2 0/1804
17/31: h2 h3 1/1804
18/31: c3 c4 4/1804
19/31: e1 d1 0/1804
20/31: h1 g1 0/1804
21/31: e1 g1 18/1804
22/31: a4 a5 2/1804
23/31: b1 a3 0/1803
24/31: e2 d3 0/1800
25/31: e1 d2 -6/1799
26/31: e2 d2 0/1798
27/31: e2 d1 0/1793
28/31: c2 d3 0/1791
29/31: c2 d1 -11/1781
30/31: e2 b5 104/1231

8 .....b.r
7 ..k...pp
6 p....n..
5 .Pp.....
4 ....P.P.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
21. xb5
score: 1805, material: 1620, abs: 4778, pawns: 26, mobility: 8
Depth  1 #searched     1603 bmove: a6 b5 bscore: -1629
Depth  2 #searched    12817 bmove: h7 h5 bscore: -1774
Depth  3 #searched   218533 bmove: f8 d6 bscore: -1753
Depth  4 #searched  1984666 bmove: c7 b7 bscore: -1810
Search total: 110224363 / 8051 ms / 13690 nodes/ms
hash size r 2 t 4766 
0/24: c7 b7 -6/-1810
1/24: c7 b6 0/-1810
2/24: a6 a5 -1/-1812
3/24: g7 g6 -2/-1813
4/24: g7 g5 -4/-1814
5/24: c7 d8 0/-1814
6/24: f6 g8 10/-1815
7/24: c7 d7 6/-1815
8/24: c5 c4 -4/-1815
9/24: f6 e8 10/-1816
10/24: h7 h6 -1/-1817
11/24: c7 d6 12/-1817
12/24: c7 b8 -18/-1818
13/24: f6 d7 5/-1834
14/24: f6 e4 -119/-1838
15/24: c7 c8 -6/-1843
16/24: a6 b5 -109/-1853
17/24: h8 g8 0/-1854
18/24: f8 e7 -11/-1857
19/24: f6 d5 0/-1869
20/24: f8 d6 -11/-1871
21/24: h7 h5 -2/-1885
22/24: f6 g4 -99/-1916
23/24: f6 h5 10/-1963

8 .....b.r
7 .k....pp
6 p....n..
5 .Pp.....
4 ....P.P.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
22. Kb7
score: -1810, material: 1614, abs: 4784, pawns: 26, mobility: 9
Depth  1 #searched     2771 bmove: a1 a6 bscore: 1978
Depth  2 #searched    42562 bmove: b5 a6 bscore: 1993
Depth  3 #searched   174861 bmove: e4 e5 bscore: 1803
Depth  4 #searched   808291 bmove: b5 a6 bscore: 1805
Depth  5 #searched  6552296 bmove: b5 a6 bscore: 1810
Search total: 116776659 / 8533 ms / 13685 nodes/ms
hash size r 3 t 15777 
0/34: b5 a6 101/1810
1/34: e1 d2 -6/1810
2/34: g4 g5 2/1810
3/34: e2 e3 0/1810
4/34: e1 f1 6/1810
5/34: e2 c4 0/1810
6/34: h2 h4 2/1810
7/34: a1 a4 0/1810
8/34: c2 b3 0/1810
9/34: a1 a5 0/1810
10/34: g2 g3 2/1810
11/34: e4 e5 7/1810
12/34: e2 f3 0/1810
13/34: b5 b6 2/1810
14/34: c3 c4 4/1810
15/34: e1 g1 18/1810
16/34: b1 a3 0/1810
17/34: c2 a4 0/1810
18/34: b1 d2 5/1810
19/34: e2 f2 0/1810
20/34: a1 a2 0/1810
21/34: h2 h3 1/1810
22/34: e2 f1 0/1810
23/34: a1 a3 0/1810
24/34: c2 d3 0/1810
25/34: h1 f1 0/1810
26/34: h1 g1 0/1810
27/34: e1 d1 0/1810
28/34: c2 d1 -11/1809
29/34: e2 d3 0/1809
30/34: e2 d1 0/1809
31/34: e2 d2 0/1808
32/34: e1 f2 0/1808
33/34: a1 a6 101/1806

8 .....b.r
7 .k....pp
6 P....n..
5 ..p.....
4 ....P.P.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
22. xa6+
score: 1810, material: 1715, abs: 4683, pawns: 40, mobility: 10
Depth  1 #searched      230 bmove: b7 a7 bscore: -1754
Depth  2 #searched     3729 bmove: b7 a7 bscore: -2038
Depth  3 #searched    37498 bmove: b7 a7 bscore: -1768
Depth  4 #searched    55845 bmove: b7 a7 bscore: -1800
Depth  5 #searched   332146 bmove: b7 a7 bscore: -1767
Depth  6 #searched   970143 bmove: b7 a7 bscore: -1805
Depth  7 #searched  5772263 bmove: b7 a7 bscore: -1778
Search total: 122548922 / 8934 ms / 13717 nodes/ms
hash size r 2 t 13331 
0/7: b7 a7 -12/-1778
1/7: b7 c8 0/-1778
2/7: b7 c7 6/-1778
3/7: b7 b6 6/-1778
4/7: b7 c6 12/-1778
5/7: b7 a8 -12/-1778
6/7: b7 b8 -12/-1779

8 .....b.r
7 k.....pp
6 P....n..
5 ..p.....
4 ....P.P.
3 ..P.....
2 ..B.Q.PP
1 RN..K..R
  ABCDEFGH
23. Ka7
score: -1778, material: 1703, abs: 4695, pawns: 40, mobility: 11
Depth  1 #searched     2148 bmove: e4 e5 bscore: 2038
Depth  2 #searched    17899 bmove: g4 g5 bscore: 1768
Depth  3 #searched   281509 bmove: e2 b5 bscore: 9997
Search total: 122830431 / 8957 ms / 13713 nodes/ms
hash size r 3 t 1108 
0/32: e2 b5 0/9997
1/32: e4 e5 7/1810
2/32: b1 d2 5/1806
3/32: e2 c4 0/1802
4/32: c2 b3 0/1801
5/32: e2 e3 0/1801
6/32: e1 f2 0/1800
7/32: g4 g5 2/1799
8/32: e1 f1 6/1799
9/32: a1 a4 0/1799
10/32: e2 d2 0/1798
11/32: h1 f1 0/1798
12/32: h2 h4 2/1798
13/32: c3 c4 4/1798
14/32: a1 a5 0/1797
15/32: b1 a3 0/1797
16/32: c2 a4 0/1797
17/32: g2 g3 2/1797
18/32: h2 h3 1/1796
19/32: a1 a2 0/1796
20/32: a1 a3 0/1795
21/32: c2 d3 0/1794
22/32: e2 d1 0/1794
23/32: h1 g1 0/1794
24/32: e1 d1 0/1794
25/32: e1 d2 -6/1792
26/32: e2 d3 0/1789
27/32: c2 d1 -11/1781
28/32: e2 f2 0/1778
29/32: e1 g1 18/1777
30/32: e2 f3 0/1777
31/32: e2 f1 0/1774

8 .....b.r
7 k.....pp
6 P....n..
5 .Qp.....
4 ....P.P.
3 ..P.....
2 ..B...PP
1 RN..K..R
  ABCDEFGH
23. Qb5
score: 9997, material: 1703, abs: 4695, pawns: 40, mobility: 19
Depth  1 #searched     1324 bmove: f6 g4 bscore: -855
Depth  2 #searched    31126 bmove: f6 g4 bscore: -9998
Search total: 122861557 / 8960 ms / 13712 nodes/ms
hash size r 4 t 64 
0/16: f6 g4 -99/-9998
1/16: g7 g5 -4/-9998
2/16: h7 h6 -1/-9998
3/16: g7 g6 -2/-9998
4/16: f8 e7 -11/-9998
5/16: f8 d6 -11/-9998
6/16: h8 g8 0/-9998
7/16: a7 a8 0/-9998
8/16: f6 g8 10/-9998
9/16: h7 h5 -2/-9998
10/16: c5 c4 -4/-9998
11/16: f6 e4 -119/-9998
12/16: f6 e8 10/-9998
13/16: f6 h5 10/-9998
14/16: f6 d7 5/-9998
15/16: f6 d5 0/-9998

8 .....b.r
7 k.....pp
6 P.......
5 .Qp.....
4 ....P.n.
3 ..P.....
2 ..B...PP
1 RN..K..R
  ABCDEFGH
24. Nxg4
score: -9998, material: 1604, abs: 4586, pawns: 60, mobility: 19
Depth  1 #searched     2784 bmove: b5 b7 bscore: 9999
Search total: 122864341 / 8960 ms / 13712 nodes/ms
hash size r 2 t 4 
0/38: b5 b7 0/9999
1/38: b5 d7 0/2009
2/38: e1 g1 18/1927
3/38: b1 d2 5/1918
4/38: g2 g3 2/1911
5/38: e1 f1 6/1910
6/38: c2 b3 0/1909
7/38: b1 a3 0/1908
8/38: c3 c4 4/1907
9/38: a1 a2 0/1907
10/38: a1 a4 0/1907
11/38: b5 c6 0/1906
12/38: a1 a3 0/1906
13/38: b5 e8 0/1905
14/38: a1 a5 0/1905
15/38: e1 d1 0/1904
16/38: b5 d3 0/1904
17/38: b5 c4 0/1904
18/38: c2 d3 0/1903
19/38: c2 a4 0/1902
20/38: b5 b3 0/1902
21/38: b5 f1 0/1901
22/38: b5 b4 0/1901
23/38: e1 e2 -6/1901
24/38: b5 e2 0/1900
25/38: e1 d2 -6/1900
26/38: b5 b2 0/1899
27/38: b5 a4 0/1898
28/38: b5 a5 0/1898
29/38: c2 d1 -11/1897
30/38: h2 h4 2/1686
31/38: h2 h3 1/1685
32/38: h1 f1 0/1588
33/38: h1 g1 0/1583
34/38: e4 e5 7/1552
35/38: b5 b6 0/858
36/38: b5 c5 108/855
37/38: b5 b8 0/768

8 .....b.r
7 kQ....pp
6 P.......
5 ..p.....
4 ....P.n.
3 ..P.....
2 ..B...PP
1 RN..K..R
  ABCDEFGH
24. Qb7#
score: 9999, material: 1604, abs: 4586, pawns: 60, mobility: 19
1-0
```
