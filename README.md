# Puccinia's Checkmate

Puccinia's Checkmate - a rusty chess library.
* Negamax search with alpha beta pruning
* Evaluation based on material, pawn structure & mobility

Run as cli app or web app in a browser (examples/spa).

```
% cargo run --release -- -v -a

8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ........
3 ........
2 PPPPPPPP
1 RNBQKBNR
  ABCDEFGH
Depth  1 #searched      845 bmove: e2 e4 bscore: 24
Depth  2 #searched    21209 bmove: g1 f3 bscore: -10
Depth  3 #searched   128370 bmove: d2 d4 bscore: 28
Depth  4 #searched  1303396 bmove: f2 f4 bscore: -13
Search total: 1303396 / 159 ms / 8197 nodes/ms
hash size r 1 t 1545 
0/20: f2 f4 8/-13
1/20: b2 b4 4/-13
2/20: d2 d4 12/-18
3/20: d2 d3 6/-18
4/20: c2 c3 4/-18
5/20: h2 h4 2/-18
6/20: g2 g3 2/-18
7/20: b2 b3 2/-18
8/20: a2 a4 2/-18
9/20: g2 g4 4/-18
10/20: h2 h3 1/-18
11/20: g1 h3 0/-18
12/20: e2 e3 7/-19
13/20: b1 c3 10/-19
14/20: f2 f3 4/-19
15/20: a2 a3 1/-19
16/20: e2 e4 14/-20
17/20: c2 c4 8/-20
18/20: b1 a3 0/-22
19/20: g1 f3 10/-22

8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 .....P..
3 ........
2 PPPPP.PP
1 RNBQKBNR
  ABCDEFGH
1. f4
score: -13, material: 8, abs: 8036, pawns: 0, mobility: 0
Depth  1 #searched      945 bmove: d7 d5 bscore: 14
Depth  2 #searched     8938 bmove: g8 f6 bscore: -13
Depth  3 #searched   152560 bmove: e7 e6 bscore: 12
Depth  4 #searched   581726 bmove: f7 f5 bscore: -23
Depth  5 #searched 11426753 bmove: e7 e5 bscore: 6
Search total: 12730149 / 1218 ms / 10451 nodes/ms
hash size r 1 t 11936 
0/20: e7 e5 -12/6
1/20: b8 c6 -10/5
2/20: g8 f6 -10/4
3/20: c7 c5 -8/3
4/20: f7 f6 -4/3
5/20: e7 e6 -6/2
6/20: d7 d6 -7/1
7/20: b8 a6 0/1
8/20: h7 h6 -1/1
9/20: g8 h6 0/1
10/20: d7 d5 -14/0
11/20: a7 a5 -2/-1
12/20: a7 a6 -1/-1
13/20: b7 b6 -2/-2
14/20: g7 g6 -2/-3
15/20: h7 h5 -2/-4
16/20: c7 c6 -4/-5
17/20: b7 b5 -4/-6
18/20: f7 f5 -8/-7
19/20: g7 g5 -4/-55

8 rnbqkbnr
7 pppp.ppp
6 ........
5 ....p...
4 .....P..
3 ........
2 PPPPP.PP
1 RNBQKBNR
  ABCDEFGH
2. e5
score: 6, material: -4, abs: 8048, pawns: 0, mobility: -10
Depth  1 #searched     2345 bmove: f4 e5 bscore: 122
Depth  2 #searched    40566 bmove: f4 e5 bscore: 73
Depth  3 #searched   138443 bmove: f4 e5 bscore: 118
Depth  4 #searched   659375 bmove: f4 e5 bscore: 93
Depth  5 #searched  2307303 bmove: f4 e5 bscore: 112
Search total: 15037452 / 1410 ms / 10664 nodes/ms
hash size r 2 t 1635 
0/21: f4 e5 125/112
1/21: g1 f3 10/112
2/21: d2 d4 12/112
3/21: g2 g3 2/111
4/21: g1 h3 0/111
5/21: a2 a3 1/111
6/21: b1 a3 0/111
7/21: e1 f2 0/109
8/21: d2 d3 6/108
9/21: b1 c3 10/106
10/21: b2 b4 4/106
11/21: h2 h4 2/99
12/21: c2 c4 8/98
13/21: c2 c3 4/93
14/21: a2 a4 2/91
15/21: b2 b3 2/91
16/21: h2 h3 1/88
17/21: f4 f5 4/76
18/21: e2 e4 14/72
19/21: e2 e3 7/37
20/21: g2 g4 4/-9998

8 rnbqkbnr
7 pppp.ppp
6 ........
5 ....P...
4 ........
3 ........
2 PPPPP.PP
1 RNBQKBNR
  ABCDEFGH
2. xe5
score: 112, material: 121, abs: 7949, pawns: 10, mobility: -9
Depth  1 #searched     3163 bmove: d7 d5 bscore: -70
Depth  2 #searched    19845 bmove: d8 h4 bscore: -118
Depth  3 #searched   657031 bmove: d8 e7 bscore: -7
Depth  4 #searched  2231184 bmove: d8 h4 bscore: -114
Search total: 17268636 / 1602 ms / 10779 nodes/ms
hash size r 2 t 2264 
0/29: d8 h4 0/-114
1/29: f7 f6 -4/-116
2/29: d7 d6 -7/-118
3/29: b7 b6 -2/-118
4/29: g7 g6 -2/-118
5/29: g7 g5 -4/-119
6/29: f8 e7 -11/-119
7/29: d7 d5 -14/-120
8/29: f7 f5 -8/-120
9/29: a7 a6 -1/-121
10/29: h7 h6 -1/-121
11/29: c7 c5 -8/-121
12/29: f8 b4 -11/-125
13/29: f8 c5 -11/-125
14/29: b7 b5 -4/-125
15/29: d8 g5 0/-126
16/29: c7 c6 -4/-126
17/29: a7 a5 -2/-128
18/29: h7 h5 -2/-128
19/29: g8 e7 -5/-131
20/29: b8 a6 0/-131
21/29: g8 h6 0/-132
22/29: d8 e7 0/-138
23/29: e8 e7 6/-143
24/29: b8 c6 -10/-153
25/29: g8 f6 -10/-328
26/29: f8 a3 -11/-328
27/29: f8 d6 -11/-392
28/29: d8 f6 0/-779

8 rnb.kbnr
7 pppp.ppp
6 ........
5 ....P...
4 .......q
3 ........
2 PPPPP.PP
1 RNBQKBNR
  ABCDEFGH
3. Qh4+
score: -114, material: 121, abs: 7949, pawns: 10, mobility: -25
Depth  1 #searched      249 bmove: g2 g3 bscore: 926
Depth  2 #searched     5575 bmove: g2 g3 bscore: 108
Depth  3 #searched    24200 bmove: g2 g3 bscore: 526
Depth  4 #searched   103675 bmove: g2 g3 bscore: 148
Depth  5 #searched   169805 bmove: g2 g3 bscore: 933
Depth  6 #searched   322317 bmove: g2 g3 bscore: 81
Depth  7 #searched   428168 bmove: g2 g3 bscore: 921
Depth  8 #searched   970866 bmove: g2 g3 bscore: 790
Depth  9 #searched  1335889 bmove: g2 g3 bscore: 899
Search total: 18604525 / 1708 ms / 10892 nodes/ms
hash size r 3 t 2059 
0/1: g2 g3 2/899

8 rnb.kbnr
7 pppp.ppp
6 ........
5 ....P...
4 .......q
3 ......P.
2 PPPPP..P
1 RNBQKBNR
  ABCDEFGH
3. g3
score: 899, material: 123, abs: 7951, pawns: 10, mobility: -21
Depth  1 #searched     4424 bmove: h4 e4 bscore: -108
Depth  2 #searched    28061 bmove: h4 g3 bscore: -526
Depth  3 #searched   466185 bmove: h4 h6 bscore: -115
Depth  4 #searched  4210310 bmove: h4 g3 bscore: -703
Search total: 22814835 / 2055 ms / 11102 nodes/ms
hash size r 2 t 2869 
0/42: h4 g3 -102/-703
1/42: h4 e4 0/-773
2/42: h4 h3 0/-773
3/42: h4 f6 0/-782
4/42: h4 h2 -100/-821
5/42: d7 d5 -14/-860
6/42: f8 c5 -11/-860
7/42: f8 b4 -11/-860
8/42: f7 f5 -8/-866
9/42: g7 g6 -2/-867
10/42: b7 b5 -4/-870
11/42: c7 c5 -8/-872
12/42: h7 h5 -2/-873
13/42: b7 b6 -2/-873
14/42: a7 a5 -2/-873
15/42: c7 c6 -4/-874
16/42: g8 e7 -5/-874
17/42: d7 d6 -7/-875
18/42: h7 h6 -1/-876
19/42: a7 a6 -1/-876
20/42: e8 d8 0/-876
21/42: b8 c6 -10/-877
22/42: g8 h6 0/-877
23/42: b8 a6 0/-877
24/42: f7 f6 -4/-881
25/42: e8 e7 6/-885
26/42: g8 f6 -10/-894
27/42: h4 c4 0/-916
28/42: h4 g5 0/-918
29/42: f8 e7 -11/-921
30/42: h4 d4 0/-922
31/42: h4 a4 0/-923
32/42: h4 h5 0/-926
33/42: h4 e7 0/-927
34/42: f8 d6 -11/-927
35/42: f8 a3 -11/-927
36/42: h4 d8 0/-928
37/42: h4 g4 0/-929
38/42: h4 b4 0/-930
39/42: h4 h6 0/-931
40/42: g7 g5 -4/-931
41/42: h4 f4 0/-981

8 rnb.kbnr
7 pppp.ppp
6 ........
5 ....P...
4 ........
3 ......q.
2 PPPPP..P
1 RNBQKBNR
  ABCDEFGH
4. Qxg3+
score: -703, material: 21, abs: 7849, pawns: 4, mobility: -23
Depth  1 #searched       51 bmove: h2 g3 bscore: 926
Depth  2 #searched     4801 bmove: h2 g3 bscore: 501
Depth  3 #searched    47385 bmove: h2 g3 bscore: 929
Depth  4 #searched    98970 bmove: h2 g3 bscore: 863
Depth  5 #searched   144229 bmove: h2 g3 bscore: 817
Depth  6 #searched   302065 bmove: h2 g3 bscore: 866
Depth  7 #searched   368757 bmove: h2 g3 bscore: 898
Depth  8 #searched   533374 bmove: h2 g3 bscore: 880
Depth  9 #searched   719509 bmove: h2 g3 bscore: 908
Depth 10 #searched   956882 bmove: h2 g3 bscore: 945
Depth 11 #searched  1230575 bmove: h2 g3 bscore: 975
Search total: 24045410 / 2152 ms / 11173 nodes/ms
hash size r 2 t 1520 
0/1: h2 g3 902/975

8 rnb.kbnr
7 pppp.ppp
6 ........
5 ....P...
4 ........
3 ......P.
2 PPPPP...
1 RNBQKBNR
  ABCDEFGH
4. xg3
score: 975, material: 923, abs: 6951, pawns: 4, mobility: -1
Depth  1 #searched     4241 bmove: f8 c5 bscore: -501
Depth  2 #searched    42573 bmove: d7 d6 bscore: -929
Depth  3 #searched   704087 bmove: h7 h6 bscore: -703
Depth  4 #searched  1021145 bmove: h7 h6 bscore: -714
Search total: 25066555 / 2236 ms / 11210 nodes/ms
hash size r 2 t 1275 
0/26: h7 h6 -1/-714
1/26: f7 f6 -4/-745
2/26: g7 g5 -4/-749
3/26: f8 d6 -11/-750
4/26: f7 f5 -8/-765
5/26: d7 d6 -7/-779
6/26: d7 d5 -14/-794
7/26: b8 c6 -10/-795
8/26: h7 h5 -2/-795
9/26: f8 c5 -11/-798
10/26: f8 b4 -11/-798
11/26: f8 e7 -11/-800
12/26: f8 a3 -11/-808
13/26: b7 b5 -4/-808
14/26: c7 c5 -8/-810
15/26: b7 b6 -2/-811
16/26: a7 a5 -2/-811
17/26: g7 g6 -2/-812
18/26: g8 e7 -5/-812
19/26: a7 a6 -1/-813
20/26: e8 d8 0/-814
21/26: g8 h6 0/-815
22/26: g8 f6 -10/-815
23/26: e8 e7 6/-823
24/26: b8 a6 0/-829
25/26: c7 c6 -4/-870

8 rnb.kbnr
7 pppp.pp.
6 .......p
5 ....P...
4 ........
3 ......P.
2 PPPPP...
1 RNBQKBNR
  ABCDEFGH
5. h6
score: -714, material: 922, abs: 6952, pawns: -2, mobility: -1
Depth  1 #searched     1637 bmove: a2 a3 bscore: 1155
Depth  2 #searched    45557 bmove: g1 h3 bscore: 703
Depth  3 #searched   332895 bmove: e2 e4 bscore: 945
Depth  4 #searched  1349190 bmove: e2 e4 bscore: 893
Search total: 26415745 / 2345 ms / 11264 nodes/ms
hash size r 2 t 1102 
0/24: e2 e4 14/893
1/24: b1 c3 10/893
2/24: e2 e3 7/893
3/24: c2 c3 4/893
4/24: e1 f2 0/893
5/24: b2 b3 2/893
6/24: a2 a4 2/893
7/24: g3 g4 2/893
8/24: h1 h2 0/893
9/24: d2 d3 6/893
10/24: a2 a3 1/893
11/24: b1 a3 0/893
12/24: h1 h3 0/893
13/24: b2 b4 4/893
14/24: h1 h5 0/893
15/24: g1 h3 0/893
16/24: e5 e6 7/893
17/24: g1 f3 10/892
18/24: h1 h4 0/892
19/24: f1 g2 11/891
20/24: c2 c4 8/889
21/24: d2 d4 12/887
22/24: f1 h3 11/885
23/24: h1 h6 101/709

8 rnb.kbnr
7 pppp.pp.
6 .......p
5 ....P...
4 ....P...
3 ......P.
2 PPPP....
1 RNBQKBNR
  ABCDEFGH
5. e4
score: 893, material: 936, abs: 6966, pawns: -2, mobility: 8
Depth  1 #searched     5171 bmove: f8 c5 bscore: -518
Depth  2 #searched    44524 bmove: d7 d6 bscore: -947
Depth  3 #searched   683305 bmove: f8 b4 bscore: -673
Depth  4 #searched  4977120 bmove: d7 d6 bscore: -967
Search total: 31392865 / 2732 ms / 11490 nodes/ms
hash size r 2 t 2790 
0/25: d7 d6 -7/-967
1/25: f7 f6 -4/-972
2/25: g8 e7 -5/-973
3/25: b7 b6 -2/-975
4/25: g7 g6 -2/-976
5/25: c7 c5 -8/-978
6/25: c7 c6 -4/-979
7/25: e8 d8 0/-982
8/25: h8 h7 0/-985
9/25: b8 a6 0/-987
10/25: b7 b5 -4/-1003
11/25: d7 d5 -14/-1007
12/25: f8 a3 -11/-1012
13/25: g8 f6 -10/-1015
14/25: f8 d6 -11/-1027
15/25: f7 f5 -8/-1030
16/25: h6 h5 -1/-1047
17/25: a7 a6 -1/-1060
18/25: e8 e7 6/-1078
19/25: g7 g5 -4/-1129
20/25: a7 a5 -2/-1159
21/25: b8 c6 -10/-1199
22/25: f8 e7 -11/-1200
23/25: f8 c5 -11/-1201
24/25: f8 b4 -11/-1205

8 rnb.kbnr
7 ppp..pp.
6 ...p...p
5 ....P...
4 ....P...
3 ......P.
2 PPPP....
1 RNBQKBNR
  ABCDEFGH
6. d6
score: -967, material: 929, abs: 6973, pawns: -2, mobility: 6
Depth  1 #searched     5457 bmove: d2 d4 bscore: 945
Depth  2 #searched    66525 bmove: d2 d4 bscore: 931
Depth  3 #searched   486427 bmove: d2 d4 bscore: 1083
Depth  4 #searched 19349963 bmove: f1 b5 bscore: 841
Search total: 50742828 / 4162 ms / 12191 nodes/ms
hash size r 2 t 5297 
0/34: f1 b5 11/841
1/34: b1 c3 10/840
2/34: f1 d3 11/838
3/34: f1 a6 11/837
4/34: g1 f3 10/835
5/34: g1 h3 0/835
6/34: d2 d4 12/834
7/34: e5 d6 110/834
8/34: d1 f3 0/831
9/34: h1 h3 0/830
10/34: h1 h4 0/829
11/34: e5 e6 7/828
12/34: f1 c4 11/827
13/34: h1 h5 0/824
14/34: h1 h6 101/823
15/34: f1 g2 11/822
16/34: f1 e2 11/821
17/34: c2 c4 8/821
18/34: f1 h3 11/821
19/34: b2 b4 4/820
20/34: c2 c3 4/819
21/34: d1 g4 0/818
22/34: h1 h2 0/818
23/34: d1 h5 0/817
24/34: e1 f2 0/817
25/34: b2 b3 2/817
26/34: a2 a4 2/817
27/34: g3 g4 2/815
28/34: d2 d3 6/814
29/34: a2 a3 1/814
30/34: d1 e2 0/813
31/34: b1 a3 0/813
32/34: g1 e2 5/811
33/34: e1 e2 -6/802

8 rnb.kbnr
7 ppp..pp.
6 ...p...p
5 .B..P...
4 ....P...
3 ......P.
2 PPPP....
1 RNBQK.NR
  ABCDEFGH
6. Bb5+
score: 841, material: 940, abs: 6984, pawns: -2, mobility: 10
Depth  1 #searched     1841 bmove: c7 c6 bscore: -918
Depth  2 #searched    20090 bmove: c7 c6 bscore: -804
Depth  3 #searched   235968 bmove: c7 c6 bscore: -645
Depth  4 #searched  3226428 bmove: b8 d7 bscore: -1043
Search total: 53969256 / 4400 ms / 12265 nodes/ms
hash size r 3 t 1431 
0/6: b8 d7 -5/-1043
1/6: b8 c6 -10/-1046
2/6: e8 e7 6/-1048
3/6: e8 d8 0/-1052
4/6: c8 d7 -11/-1053
5/6: c7 c6 -4/-1054

8 r.b.kbnr
7 pppn.pp.
6 ...p...p
5 .B..P...
4 ....P...
3 ......P.
2 PPPP....
1 RNBQK.NR
  ABCDEFGH
7. Nd7
score: -1043, material: 935, abs: 6989, pawns: -2, mobility: 12
Depth  1 #searched     7443 bmove: g1 f3 bscore: 957
Depth  2 #searched    80931 bmove: d2 d4 bscore: 932
Depth  3 #searched   388786 bmove: d2 d4 bscore: 1118
Depth  4 #searched  1883464 bmove: d2 d4 bscore: 1068
Search total: 55852720 / 4543 ms / 12294 nodes/ms
hash size r 4 t 1420 
0/36: d2 d4 12/1068
1/36: d1 f3 0/1068
2/36: d2 d3 6/1068
3/36: d1 e2 0/1068
4/36: d1 h5 0/1067
5/36: e1 f1 6/1067
6/36: b5 c4 0/1066
7/36: h1 h5 0/1066
8/36: h1 h6 101/1065
9/36: e1 f2 0/1064
10/36: c2 c3 4/1064
11/36: b2 b4 4/1064
12/36: b1 c3 10/1062
13/36: b2 b3 2/1062
14/36: h1 h2 0/1062
15/36: a2 a4 2/1062
16/36: g1 e2 5/1062
17/36: h1 h4 0/1061
18/36: c2 c4 8/1059
19/36: a2 a3 1/1059
20/36: b1 a3 0/1059
21/36: g3 g4 2/1059
22/36: b5 c6 0/1059
23/36: b5 a4 0/1059
24/36: g1 f3 10/1058
25/36: h1 h3 0/1058
26/36: g1 h3 0/1058
27/36: e1 e2 -6/1053
28/36: d1 g4 0/1046
29/36: b5 a6 0/1041
30/36: b5 e2 0/1041
31/36: b5 d3 0/1041
32/36: b5 d7 320/1036
33/36: e5 d6 110/992
34/36: b5 f1 -11/970
35/36: e5 e6 7/947

8 r.b.kbnr
7 pppn.pp.
6 ...p...p
5 .B..P...
4 ...PP...
3 ......P.
2 PPP.....
1 RNBQK.NR
  ABCDEFGH
7. d4
score: 1068, material: 947, abs: 7001, pawns: -2, mobility: 20
Depth  1 #searched    13594 bmove: f8 e7 bscore: -943
Depth  2 #searched   117417 bmove: c7 c6 bscore: -949
Depth  3 #searched   443418 bmove: d6 e5 bscore: -859
Depth  4 #searched  2241144 bmove: d6 e5 bscore: -789
Search total: 58093864 / 4702 ms / 12355 nodes/ms
hash size r 2 t 1010 
0/19: d6 e5 -126/-789
1/19: g7 g5 -4/-789
2/19: f7 f5 -8/-794
3/19: f7 f6 -4/-802
4/19: e8 e7 6/-803
5/19: c7 c5 -8/-806
6/19: h8 h7 0/-811
7/19: a7 a6 -1/-817
8/19: g8 f6 -10/-819
9/19: g7 g6 -2/-820
10/19: f8 e7 -11/-822
11/19: d6 d5 -7/-837
12/19: c7 c6 -4/-844
13/19: g8 e7 -5/-846
14/19: a7 a5 -2/-864
15/19: b7 b6 -2/-866
16/19: a8 b8 0/-867
17/19: e8 d8 0/-901
18/19: h6 h5 -1/-1006

8 r.b.kbnr
7 pppn.pp.
6 .......p
5 .B..p...
4 ...PP...
3 ......P.
2 PPP.....
1 RNBQK.NR
  ABCDEFGH
8. xe5
score: -789, material: 821, abs: 6885, pawns: 4, mobility: 17
Depth  1 #searched    12613 bmove: d4 e5 bscore: 956
Depth  2 #searched   304296 bmove: e1 f2 bscore: 822
Depth  3 #searched  1627197 bmove: e1 f2 bscore: 867
Search total: 59721061 / 4818 ms / 12395 nodes/ms
hash size r 2 t 666 
0/43: e1 f2 0/867
1/43: d1 g4 0/867
2/43: b5 d3 0/867
3/43: b1 c3 10/867
4/43: c1 e3 11/867
5/43: c2 c3 4/867
6/43: b5 c4 0/866
7/43: e1 f1 6/866
8/43: h1 h5 0/866
9/43: d1 h5 0/866
10/43: g1 f3 10/866
11/43: h1 h4 0/865
12/43: h1 h2 0/865
13/43: b5 e2 0/864
14/43: b5 a6 0/864
15/43: d1 e2 0/864
16/43: a2 a4 2/864
17/43: b2 b3 2/864
18/43: d1 d3 0/863
19/43: b1 d2 5/863
20/43: g3 g4 2/863
21/43: g1 e2 5/863
22/43: d1 d2 0/862
23/43: g1 h3 0/862
24/43: h1 h3 0/862
25/43: b1 a3 0/862
26/43: e1 d2 -6/861
27/43: a2 a3 1/861
28/43: c2 c4 8/861
29/43: b5 c6 0/860
30/43: c1 d2 11/860
31/43: h1 h6 101/859
32/43: c1 h6 112/859
33/43: d4 d5 6/859
34/43: b5 f1 -11/857
35/43: b5 a4 0/857
36/43: b2 b4 4/856
37/43: e1 e2 -6/855
38/43: d1 f3 0/844
39/43: d4 e5 121/841
40/43: c1 g5 11/833
41/43: b5 d7 320/810
42/43: c1 f4 11/639

8 r.b.kbnr
7 pppn.pp.
6 .......p
5 .B..p...
4 ...PP...
3 ......P.
2 PPP..K..
1 RNBQ..NR
  ABCDEFGH
8. Kf2
score: 867, material: 821, abs: 6885, pawns: 4, mobility: 21
Depth  1 #searched     7633 bmove: f7 f6 bscore: -822
Depth  2 #searched   103720 bmove: g8 f6 bscore: -847
Depth  3 #searched   518664 bmove: c7 c6 bscore: -823
Depth  4 #searched  3728300 bmove: c7 c6 bscore: -806
Search total: 63449361 / 5077 ms / 12497 nodes/ms
hash size r 3 t 1630 
0/22: c7 c6 -4/-806
1/22: e8 e7 6/-806
2/22: g8 e7 -5/-809
3/22: a7 a5 -2/-810
4/22: b7 b6 -2/-811
5/22: g7 g6 -2/-812
6/22: a7 a6 -1/-812
7/22: h8 h7 0/-813
8/22: a8 b8 0/-813
9/22: f8 b4 -11/-819
10/22: g8 f6 -10/-820
11/22: c7 c5 -8/-820
12/22: g7 g5 -4/-821
13/22: f8 d6 -11/-821
14/22: f8 e7 -11/-822
15/22: e8 d8 0/-824
16/22: f8 c5 -11/-825
17/22: f7 f5 -8/-834
18/22: e5 d4 -121/-838
19/22: f7 f6 -4/-866
20/22: h6 h5 -1/-903
21/22: f8 a3 -11/-915

8 r.b.kbnr
7 pp.n.pp.
6 ..p....p
5 .B..p...
4 ...PP...
3 ......P.
2 PPP..K..
1 RNBQ..NR
  ABCDEFGH
9. c6
score: -806, material: 817, abs: 6889, pawns: 4, mobility: 20
Depth  1 #searched    13548 bmove: b5 e2 bscore: 845
Depth  2 #searched   303778 bmove: b5 c4 bscore: 808
Depth  3 #searched   860775 bmove: b5 c4 bscore: 935
Depth  4 #searched 23883704 bmove: b5 c4 bscore: 782
Search total: 87333065 / 6788 ms / 12865 nodes/ms
hash size r 2 t 6276 
0/46: b5 c4 0/782
1/46: f2 e1 0/782
2/46: f2 e3 -12/782
3/46: c1 f4 11/782
4/46: d1 e2 0/782
5/46: h1 h6 101/778
6/46: c1 d2 11/775
7/46: f2 e2 -6/771
8/46: b5 a4 0/770
9/46: c1 h6 112/768
10/46: d1 f3 0/766
11/46: d1 h5 0/766
12/46: h1 h5 0/764
13/46: b1 d2 5/763
14/46: d1 f1 0/762
15/46: d1 e1 0/761
16/46: f2 f3 -6/760
17/46: d4 e5 121/753
18/46: b5 c6 104/751
19/46: g1 f3 10/749
20/46: b2 b4 4/746
21/46: d1 g4 0/745
22/46: b5 e2 0/740
23/46: b5 f1 -11/733
24/46: b1 c3 10/730
25/46: b1 a3 0/730
26/46: d1 d3 0/727
27/46: d4 d5 6/726
28/46: b5 d3 0/715
29/46: a2 a4 2/709
30/46: c2 c4 8/706
31/46: c1 g5 11/679
32/46: c1 e3 11/674
33/46: c2 c3 4/674
34/46: b5 a6 0/673
35/46: f2 g2 6/668
36/46: b2 b3 2/666
37/46: g3 g4 2/664
38/46: a2 a3 1/663
39/46: f2 f1 6/663
40/46: h1 h4 0/662
41/46: h1 h2 0/661
42/46: h1 h3 0/660
43/46: d1 d2 0/660
44/46: g1 e2 5/657
45/46: g1 h3 0/652

8 r.b.kbnr
7 pp.n.pp.
6 ..p....p
5 ....p...
4 ..BPP...
3 ......P.
2 PPP..K..
1 RNBQ..NR
  ABCDEFGH
9. Bc4
score: 782, material: 817, abs: 6889, pawns: 4, mobility: 21
Depth  1 #searched    12030 bmove: f8 b4 bscore: -812
Depth  2 #searched   136750 bmove: b7 b5 bscore: -840
Depth  3 #searched   945811 bmove: e5 d4 bscore: -791
Depth  4 #searched 21577942 bmove: h6 h5 bscore: -864
Search total: 108911007 / 8299 ms / 13123 nodes/ms
hash size r 3 t 8007 
0/26: h6 h5 -1/-864
1/26: d7 f6 -5/-865
2/26: h8 h7 0/-866
3/26: g8 e7 -5/-867
4/26: g8 f6 -10/-868
5/26: d7 b6 0/-869
6/26: g7 g6 -2/-870
7/26: f8 d6 -11/-871
8/26: e8 e7 6/-872
9/26: g7 g5 -4/-873
10/26: f8 b4 -11/-873
11/26: b7 b5 -4/-876
12/26: d7 c5 -5/-881
13/26: f7 f6 -4/-882
14/26: e5 d4 -121/-883
15/26: c6 c5 -4/-884
16/26: f8 c5 -11/-886
17/26: e8 d8 0/-889
18/26: f8 a3 -11/-890
19/26: f8 e7 -11/-892
20/26: f7 f5 -8/-903
21/26: a7 a5 -2/-904
22/26: b7 b6 -2/-905
23/26: a7 a6 -1/-906
24/26: a8 b8 0/-908
25/26: d7 b8 5/-941

8 r.b.kbnr
7 pp.n.pp.
6 ..p.....
5 ....p..p
4 ..BPP...
3 ......P.
2 PPP..K..
1 RNBQ..NR
  ABCDEFGH
10. h5
score: -864, material: 816, abs: 6890, pawns: -6, mobility: 18
Depth  1 #searched    12669 bmove: h1 h5 bscore: 955
Depth  2 #searched   154322 bmove: h1 h5 bscore: 955
Depth  3 #searched  2235510 bmove: g1 f3 bscore: 842
Search total: 111146517 / 8455 ms / 13145 nodes/ms
hash size r 2 t 991 
0/46: g1 f3 10/842
1/46: c2 c3 4/835
2/46: c1 f4 11/835
3/46: c1 g5 11/835
4/46: g1 e2 5/835
5/46: c1 h6 11/835
6/46: c1 e3 11/835
7/46: g1 h3 0/835
8/46: b1 c3 10/835
9/46: c1 d2 11/835
10/46: f2 g2 6/835
11/46: f2 e2 -6/835
12/46: g3 g4 2/835
13/46: c4 d5 0/834
14/46: c4 f1 -11/834
15/46: d1 d3 0/834
16/46: f2 f3 -6/834
17/46: f2 e3 -12/833
18/46: c4 e2 0/833
19/46: d1 d2 0/833
20/46: b2 b4 4/833
21/46: d1 e1 0/833
22/46: c4 b3 0/830
23/46: d4 d5 6/830
24/46: d1 f1 0/829
25/46: a2 a3 1/825
26/46: f2 e1 0/825
27/46: d4 e5 121/825
28/46: c4 d3 0/824
29/46: c4 f7 100/821
30/46: f2 f1 6/816
31/46: c4 e6 0/816
32/46: c4 b5 0/816
33/46: c4 a6 0/816
34/46: d1 f3 0/814
35/46: h1 h2 0/813
36/46: b1 d2 5/813
37/46: b1 a3 0/812
38/46: d1 e2 0/811
39/46: h1 h4 0/808
40/46: h1 h3 0/808
41/46: d1 g4 0/807
42/46: b2 b3 2/805
43/46: a2 a4 2/802
44/46: h1 h5 102/800
45/46: d1 h5 102/668

8 r.b.kbnr
7 pp.n.pp.
6 ..p.....
5 ....p..p
4 ..BPP...
3 .....NP.
2 PPP..K..
1 RNBQ...R
  ABCDEFGH
10. Nf3
score: 842, material: 826, abs: 6900, pawns: -6, mobility: 23
Depth  1 #searched    10150 bmove: f7 f6 bscore: -811
Depth  2 #searched    71307 bmove: f8 d6 bscore: -849
Depth  3 #searched   553390 bmove: f8 d6 bscore: -823
Depth  4 #searched  3166332 bmove: f8 d6 bscore: -883
Search total: 114312849 / 8667 ms / 13189 nodes/ms
hash size r 3 t 1661 
0/28: f8 d6 -11/-883
1/28: e5 d4 -121/-884
2/28: b7 b5 -4/-885
3/28: d7 b6 0/-886
4/28: e8 e7 6/-886
5/28: f7 f6 -4/-890
6/28: f8 a3 -11/-891
7/28: g8 f6 -10/-895
8/28: f8 b4 -11/-900
9/28: f8 e7 -11/-902
10/28: f7 f5 -8/-905
11/28: g8 h6 0/-905
12/28: g7 g5 -4/-911
13/28: g7 g6 -2/-915
14/28: a7 a5 -2/-916
15/28: b7 b6 -2/-916
16/28: c6 c5 -4/-917
17/28: a7 a6 -1/-918
18/28: e8 d8 0/-919
19/28: a8 b8 0/-919
20/28: h8 h7 0/-920
21/28: d7 f6 -5/-924
22/28: f8 c5 -11/-925
23/28: d7 b8 5/-925
24/28: g8 e7 -5/-931
25/28: h5 h4 -2/-975
26/28: d7 c5 -5/-991
27/28: h8 h6 0/-1001

8 r.b.k.nr
7 pp.n.pp.
6 ..pb....
5 ....p..p
4 ..BPP...
3 .....NP.
2 PPP..K..
1 RNBQ...R
  ABCDEFGH
11. Bd6
score: -883, material: 815, abs: 6911, pawns: -6, mobility: 19
Depth  1 #searched    18930 bmove: c2 c3 bscore: 849
Depth  2 #searched   298501 bmove: c2 c3 bscore: 823
Depth  3 #searched  1358914 bmove: f3 g5 bscore: 886
Search total: 115671763 / 8761 ms / 13203 nodes/ms
hash size r 4 t 296 
0/51: f3 g5 -5/886
1/51: c2 c3 4/883
2/51: f3 e5 117/883
3/51: d1 g1 0/873
4/51: f2 g2 6/872
5/51: b2 b4 4/872
6/51: a2 a4 2/870
7/51: a2 a3 1/868
8/51: b1 a3 0/868
9/51: b2 b3 2/867
10/51: f2 f1 6/867
11/51: h1 h4 0/866
12/51: c4 b5 0/866
13/51: c4 a6 0/866
14/51: d1 d3 0/865
15/51: b1 d2 5/865
16/51: h1 h2 0/864
17/51: d1 d2 0/864
18/51: h1 h3 0/864
19/51: c4 e2 0/864
20/51: d1 e1 0/864
21/51: h1 g1 0/863
22/51: d1 e2 0/863
23/51: c4 d3 0/863
24/51: c4 b3 0/862
25/51: f2 e1 0/861
26/51: c4 d5 0/861
27/51: d1 f1 0/861
28/51: c1 h6 11/861
29/51: c1 d2 11/860
30/51: c4 e6 0/859
31/51: h1 e1 0/859
32/51: f2 e2 -6/859
33/51: f3 h4 -10/858
34/51: h1 f1 0/858
35/51: f3 h2 -10/857
36/51: c1 f4 11/857
37/51: c1 g5 11/855
38/51: b1 c3 10/853
39/51: f2 g1 18/853
40/51: c1 e3 11/853
41/51: f2 e3 -12/850
42/51: c4 f1 -11/850
43/51: g3 g4 2/847
44/51: d4 d5 6/842
45/51: c4 f7 100/842
46/51: f3 d2 -5/836
47/51: f3 e1 -10/834
48/51: f3 g1 -10/833
49/51: d4 e5 121/715
50/51: h1 h5 102/698

8 r.b.k.nr
7 pp.n.pp.
6 ..pb....
5 ....p.Np
4 ..BPP...
3 ......P.
2 PPP..K..
1 RNBQ...R
  ABCDEFGH
11. Ng5
score: 886, material: 810, abs: 6906, pawns: -6, mobility: 20
Depth  1 #searched    17674 bmove: e5 d4 bscore: -821
Depth  2 #searched   173178 bmove: g8 h6 bscore: -938
Depth  3 #searched   549509 bmove: g8 h6 bscore: -915
Depth  4 #searched  3129410 bmove: g8 h6 bscore: -934
Search total: 118801173 / 8974 ms / 13238 nodes/ms
hash size r 5 t 1504 
0/31: g8 h6 0/-934
1/31: f7 f6 -4/-934
2/31: f7 f5 -8/-934
3/31: d7 f8 5/-936
4/31: d6 e7 0/-938
5/31: d6 b8 11/-938
6/31: h5 h4 -2/-940
7/31: d7 b8 5/-941
8/31: a8 b8 0/-942
9/31: d6 c7 0/-942
10/31: d6 f8 11/-944
11/31: e8 e7 6/-966
12/31: e5 d4 -121/-969
13/31: d7 c5 -5/-977
14/31: d6 b4 0/-981
15/31: d7 b6 0/-981
16/31: e8 f8 -6/-981
17/31: d7 f6 -5/-999
18/31: c6 c5 -4/-1000
19/31: g7 g6 -2/-1000
20/31: g8 f6 -10/-1002
21/31: h8 h6 0/-1012
22/31: a7 a5 -2/-1016
23/31: b7 b6 -2/-1017
24/31: a7 a6 -1/-1018
25/31: b7 b5 -4/-1022
26/31: g8 e7 -5/-1030
27/31: h8 h7 0/-1057
28/31: d6 c5 0/-1062
29/31: d6 a3 0/-1089
30/31: e8 d8 0/-1368

8 r.b.k..r
7 pp.n.pp.
6 ..pb...n
5 ....p.Np
4 ..BPP...
3 ......P.
2 PPP..K..
1 RNBQ...R
  ABCDEFGH
12. Nh6
score: -934, material: 810, abs: 6906, pawns: -6, mobility: 19
Depth  1 #searched    15960 bmove: h1 h5 bscore: 938
Depth  2 #searched   457917 bmove: h1 h5 bscore: 915
Depth  3 #searched  1583269 bmove: h1 h5 bscore: 934
Search total: 120384442 / 9085 ms / 13250 nodes/ms
hash size r 6 t 292 
0/51: h1 h5 102/934
1/51: c4 d5 0/934
2/51: c1 e3 11/933
3/51: g5 h7 -5/933
4/51: f2 e2 -6/932
5/51: d1 f3 0/932
6/51: f2 f3 -6/931
7/51: f2 g2 6/930
8/51: c4 a6 0/930
9/51: a2 a4 2/927
10/51: c4 e2 0/927
11/51: c4 f1 -11/927
12/51: b2 b3 2/926
13/51: c4 b5 0/926
14/51: c4 e6 0/926
15/51: f2 f1 6/925
16/51: a2 a3 1/924
17/51: d1 d3 0/923
18/51: d1 d2 0/923
19/51: h1 h4 0/922
20/51: b1 a3 0/921
21/51: c4 b3 0/921
22/51: f2 e3 -12/921
23/51: h1 h2 0/920
24/51: h1 h3 0/919
25/51: h1 e1 0/919
26/51: f2 e1 0/918
27/51: h1 g1 0/918
28/51: h1 f1 0/916
29/51: d1 h5 102/916
30/51: c4 f7 100/912
31/51: g5 f7 100/864
32/51: c1 f4 11/858
33/51: c2 c3 4/858
34/51: c1 d2 11/856
35/51: f2 g1 18/854
36/51: b1 c3 10/853
37/51: g5 e6 10/853
38/51: g5 f3 5/849
39/51: b2 b4 4/849
40/51: b1 d2 5/845
41/51: d4 d5 6/844
42/51: d1 g4 0/840
43/51: g5 h3 -5/840
44/51: d1 e2 0/839
45/51: c4 d3 0/839
46/51: d1 e1 0/838
47/51: d4 e5 121/836
48/51: d1 f1 0/834
49/51: d1 g1 0/832
50/51: g3 g4 2/832

8 r.b.k..r
7 pp.n.pp.
6 ..pb...n
5 ....p.NR
4 ..BPP...
3 ......P.
2 PPP..K..
1 RNBQ....
  ABCDEFGH
12. Rxh5
score: 934, material: 912, abs: 6804, pawns: 12, mobility: 17
Depth  1 #searched    17316 bmove: f7 f5 bscore: -908
Depth  2 #searched   187918 bmove: d7 f6 bscore: -932
Depth  3 #searched  1554515 bmove: f7 f6 bscore: -760
Search total: 121938957 / 9190 ms / 13268 nodes/ms
hash size r 2 t 353 
0/32: f7 f6 -4/-760
1/32: b7 b6 -2/-760
2/32: a7 a5 -2/-760
3/32: e8 f8 -6/-760
4/32: a7 a6 -1/-760
5/32: d6 b4 0/-760
6/32: a8 b8 0/-760
7/32: h8 g8 0/-760
8/32: d6 e7 0/-760
9/32: e8 e7 6/-760
10/32: d7 f6 -5/-761
11/32: e8 g8 -18/-761
12/32: d7 b8 5/-762
13/32: d6 c7 0/-762
14/32: d6 f8 11/-763
15/32: h8 f8 0/-763
16/32: g7 g6 -2/-768
17/32: h6 g4 -5/-776
18/32: h6 f5 -10/-857
19/32: c6 c5 -4/-861
20/32: d7 f8 5/-862
21/32: f7 f5 -8/-877
22/32: d6 b8 11/-877
23/32: d7 c5 -5/-882
24/32: b7 b5 -4/-894
25/32: d7 b6 0/-910
26/32: d6 a3 0/-914
27/32: e5 d4 -121/-934
28/32: d6 c5 0/-962
29/32: e8 d8 0/-1043
30/32: h6 g8 0/-1240
31/32: h8 h7 0/-1251

8 r.b.k..r
7 pp.n..p.
6 ..pb.p.n
5 ....p.NR
4 ..BPP...
3 ......P.
2 PPP..K..
1 RNBQ....
  ABCDEFGH
13. f6
score: -760, material: 908, abs: 6808, pawns: 6, mobility: 17
Depth  1 #searched    13989 bmove: g5 e6 bscore: 945
Depth  2 #searched   275724 bmove: g5 e6 bscore: 918
Depth  3 #searched  1080755 bmove: g5 e6 bscore: 1278
Search total: 123019712 / 9263 ms / 13280 nodes/ms
hash size r 2 t 259 
0/49: g5 e6 10/1278
1/49: g5 h3 -5/1274
2/49: g5 f3 5/1274
3/49: c2 c3 4/1272
4/49: g3 g4 2/1264
5/49: f2 e2 -6/1262
6/49: f2 f1 6/1257
7/49: c4 f1 -11/1257
8/49: c4 e2 0/1255
9/49: c4 b5 0/1254
10/49: c4 a6 0/1254
11/49: c4 e6 0/1254
12/49: c4 f7 0/1254
13/49: f2 g2 6/1253
14/49: a2 a4 2/1253
15/49: a2 a3 1/1253
16/49: d1 d3 0/1253
17/49: d1 d2 0/1253
18/49: f2 e1 0/1253
19/49: f2 e3 -12/1252
20/49: h5 h3 0/1252
21/49: b1 a3 0/1251
22/49: b2 b3 2/1251
23/49: c4 b3 0/1251
24/49: c4 g8 0/1250
25/49: c4 d5 0/1250
26/49: h5 h1 0/1249
27/49: d4 d5 6/1249
28/49: h5 h4 0/1248
29/49: h5 h2 0/1248
30/49: g5 h7 -5/1248
31/49: f2 f3 -6/1247
32/49: c1 f4 11/1207
33/49: c1 d2 11/1201
34/49: b1 d2 5/1199
35/49: d1 f3 0/1198
36/49: d1 e1 0/1197
37/49: d1 g4 0/1197
38/49: b2 b4 4/1194
39/49: d1 f1 0/1194
40/49: d1 e2 0/1194
41/49: d1 g1 0/1193
42/49: c4 d3 0/1188
43/49: g5 f7 0/1118
44/49: d4 e5 121/1073
45/49: h5 h6 315/1005
46/49: f2 g1 18/976
47/49: b1 c3 10/971
48/49: c1 e3 11/971

8 r.b.k..r
7 pp.n..p.
6 ..pbNp.n
5 ....p..R
4 ..BPP...
3 ......P.
2 PPP..K..
1 RNBQ....
  ABCDEFGH
13. Ne6
score: 1278, material: 918, abs: 6818, pawns: 6, mobility: 21
Depth  1 #searched    34347 bmove: e8 f7 bscore: -918
Depth  2 #searched   279485 bmove: e8 e7 bscore: -1053
Depth  3 #searched  1138052 bmove: g7 g5 bscore: -1044
Search total: 124157764 / 9338 ms / 13295 nodes/ms
hash size r 3 t 413 
0/31: g7 g5 -4/-1044
1/31: d6 a3 0/-1044
2/31: h6 g4 -5/-1046
3/31: e8 e7 6/-1050
4/31: h8 g8 0/-1050
5/31: h8 h7 0/-1050
6/31: d7 b6 0/-1054
7/31: h8 f8 0/-1075
8/31: h6 f7 -5/-1079
9/31: d6 c5 0/-1109
10/31: h6 f5 -10/-1185
11/31: e8 g8 -18/-1197
12/31: b7 b5 -4/-1219
13/31: c6 c5 -4/-1227
14/31: d7 c5 -5/-1228
15/31: f6 f5 -4/-1240
16/31: h6 g8 0/-1242
17/31: g7 g6 -2/-1252
18/31: e5 d4 -121/-1364
19/31: a7 a5 -2/-1369
20/31: b7 b6 -2/-1369
21/31: a7 a6 -1/-1371
22/31: d6 b4 0/-1373
23/31: a8 b8 0/-1374
24/31: d7 b8 5/-1376
25/31: d6 e7 0/-1378
26/31: d7 f8 5/-1379
27/31: d6 b8 11/-1390
28/31: e8 f7 0/-1426
29/31: d6 f8 11/-1449
30/31: d6 c7 0/-1786

8 r.b.k..r
7 pp.n....
6 ..pbNp.n
5 ....p.pR
4 ..BPP...
3 ......P.
2 PPP..K..
1 RNBQ....
  ABCDEFGH
14. g5
score: -1044, material: 914, abs: 6822, pawns: 6, mobility: 19
Depth  1 #searched    19285 bmove: c2 c3 bscore: 960
Depth  2 #searched   232388 bmove: c2 c3 bscore: 936
Depth  3 #searched  1496740 bmove: c2 c3 bscore: 1272
Search total: 125654504 / 9439 ms / 13312 nodes/ms
hash size r 2 t 338 
0/50: c2 c3 4/1272
1/50: d1 g4 0/1271
2/50: c4 d5 0/1271
3/50: b1 d2 5/1270
4/50: f2 e2 -6/1270
5/50: e6 f4 -5/1270
6/50: f2 f3 -6/1270
7/50: d1 e1 0/1268
8/50: g3 g4 2/1267
9/50: c4 f1 -11/1267
10/50: e6 f8 -15/1267
11/50: b1 c3 10/1266
12/50: f2 g1 18/1266
13/50: f2 f1 6/1266
14/50: h5 h6 315/1266
15/50: a2 a4 2/1265
16/50: c4 b5 0/1265
17/50: c4 a6 0/1265
18/50: d1 f1 0/1265
19/50: d1 e2 0/1265
20/50: b2 b3 2/1264
21/50: e6 c5 0/1264
22/50: c4 d3 0/1264
23/50: d1 g1 0/1264
24/50: c1 d2 11/1262
25/50: a2 a3 1/1262
26/50: f2 g2 6/1262
27/50: c1 e3 11/1261
28/50: h5 h1 0/1261
29/50: e6 g7 -10/1261
30/50: h5 h2 0/1261
31/50: h5 h4 0/1261
32/50: h5 h3 0/1261
33/50: d4 d5 6/1260
34/50: d1 f3 0/1259
35/50: e6 d8 -15/1259
36/50: f2 e3 -12/1258
37/50: e6 c7 -10/1217
38/50: d4 e5 121/1213
39/50: c1 g5 115/1205
40/50: b2 b4 4/1204
41/50: c1 f4 11/1174
42/50: b1 a3 0/1162
43/50: f2 e1 0/1161
44/50: c4 b3 0/1159
45/50: c4 e2 0/1158
46/50: h5 g5 104/1098
47/50: d1 d3 0/1082
48/50: d1 d2 0/1073
49/50: e6 g5 94/1042

8 r.b.k..r
7 pp.n....
6 ..pbNp.n
5 ....p.pR
4 ..BPP...
3 ..P...P.
2 PP...K..
1 RNBQ....
  ABCDEFGH
14. c3
score: 1272, material: 918, abs: 6826, pawns: 6, mobility: 20
Depth  1 #searched    19048 bmove: b7 b5 bscore: -925
Depth  2 #searched   164887 bmove: d7 f8 bscore: -950
Depth  3 #searched   616609 bmove: d7 f8 bscore: -757
Depth  4 #searched 31150571 bmove: d7 b6 bscore: -1100
Search total: 156805075 / 11545 ms / 13582 nodes/ms
hash size r 2 t 7886 
0/30: d7 b6 0/-1100
1/30: d7 c5 -5/-1101
2/30: d6 b8 11/-1102
3/30: b7 b5 -4/-1103
4/30: c6 c5 -4/-1104
5/30: a8 b8 0/-1106
6/30: a7 a6 -1/-1107
7/30: d6 c5 0/-1107
8/30: h8 h7 0/-1108
9/30: b7 b6 -2/-1109
10/30: e5 d4 -121/-1110
11/30: d7 b8 5/-1111
12/30: h8 f8 0/-1112
13/30: a7 a5 -2/-1115
14/30: e8 e7 6/-1117
15/30: g5 g4 -2/-1120
16/30: d7 f8 5/-1122
17/30: h6 g4 -5/-1132
18/30: f6 f5 -4/-1170
19/30: e8 f7 0/-1200
20/30: e8 g8 -18/-1222
21/30: d6 e7 0/-1234
22/30: h8 g8 0/-1240
23/30: h6 f7 -5/-1242
24/30: h6 g8 0/-1242
25/30: d6 f8 11/-1243
26/30: d6 a3 0/-1246
27/30: d6 b4 0/-1250
28/30: d6 c7 0/-1430
29/30: h6 f5 -10/-1437

8 r.b.k..r
7 pp......
6 .npbNp.n
5 ....p.pR
4 ..BPP...
3 ..P...P.
2 PP...K..
1 RNBQ....
  ABCDEFGH
15. Nb6
score: -1100, material: 918, abs: 6826, pawns: 6, mobility: 19
Depth  1 #searched    31318 bmove: c4 b3 bscore: 948
Depth  2 #searched   480818 bmove: e6 g7 bscore: 929
Depth  3 #searched  2589907 bmove: e6 g7 bscore: 959
Search total: 159394982 / 11718 ms / 13602 nodes/ms
hash size r 3 t 413 
0/51: e6 g7 -10/959
1/51: b1 d2 5/959
2/51: d1 e2 0/959
3/51: c4 b5 0/959
4/51: d1 g4 0/959
5/51: d1 d2 0/959
6/51: d1 f1 0/959
7/51: d1 d3 0/959
8/51: d1 e1 0/959
9/51: d1 b3 0/959
10/51: e6 f4 -5/959
11/51: c4 e2 0/959
12/51: f2 g2 6/959
13/51: d1 f3 0/959
14/51: d1 g1 0/959
15/51: c4 d3 0/959
16/51: e6 f8 -15/959
17/51: d4 d5 6/959
18/51: e6 c5 0/959
19/51: c4 f1 -11/959
20/51: e6 d8 -15/959
21/51: f2 e1 0/959
22/51: g3 g4 2/959
23/51: b2 b3 2/959
24/51: f2 e2 -6/959
25/51: c1 d2 11/959
26/51: a2 a4 2/959
27/51: a2 a3 1/959
28/51: h5 h4 0/959
29/51: h5 h3 0/959
30/51: d4 e5 121/959
31/51: c4 a6 0/959
32/51: d1 c2 0/958
33/51: d1 a4 0/958
34/51: c1 e3 11/958
35/51: b2 b4 4/958
36/51: h5 h1 0/958
37/51: c4 d5 0/957
38/51: f2 f3 -6/957
39/51: b1 a3 0/957
40/51: c4 b3 0/956
41/51: f2 e3 -12/956
42/51: f2 f1 6/955
43/51: h5 h2 0/952
44/51: e6 c7 -10/949
45/51: c1 g5 115/947
46/51: f2 g1 18/939
47/51: e6 g5 94/938
48/51: h5 h6 315/928
49/51: h5 g5 104/885
50/51: c1 f4 11/869

8 r.b.k..r
7 pp....N.
6 .npb.p.n
5 ....p.pR
4 ..BPP...
3 ..P...P.
2 PP...K..
1 RNBQ....
  ABCDEFGH
15. Ng7+
score: 959, material: 908, abs: 6816, pawns: 6, mobility: 15
Depth  1 #searched     3322 bmove: e8 f8 bscore: -914
Depth  2 #searched    27583 bmove: e8 f8 bscore: -734
Depth  3 #searched   385916 bmove: e8 f8 bscore: -734
Depth  4 #searched  2500636 bmove: e8 e7 bscore: -1020
Search total: 161895618 / 11882 ms / 13625 nodes/ms
hash size r 4 t 797 
0/4: e8 e7 6/-1020
1/4: e8 f8 -6/-1023
2/4: e8 d8 0/-1025
3/4: e8 d7 6/-1048

8 r.b....r
7 pp..k.N.
6 .npb.p.n
5 ....p.pR
4 ..BPP...
3 ..P...P.
2 PP...K..
1 RNBQ....
  ABCDEFGH
16. Ke7
score: -1020, material: 914, abs: 6810, pawns: 6, mobility: 14
Depth  1 #searched    21714 bmove: c4 d3 bscore: 946
Depth  2 #searched   594178 bmove: c4 d3 bscore: 900
Depth  3 #searched  3751598 bmove: d1 d3 bscore: 906
Search total: 165647216 / 12131 ms / 13654 nodes/ms
hash size r 5 t 932 
0/50: d1 d3 0/906
1/50: h5 h2 0/906
2/50: d1 c2 0/906
3/50: d1 g4 0/906
4/50: f2 e2 -6/905
5/50: g7 f5 10/905
6/50: d4 d5 6/905
7/50: f2 g2 6/905
8/50: h5 h4 0/904
9/50: h5 h3 0/904
10/50: d1 g1 0/904
11/50: d1 f3 0/903
12/50: h5 h1 0/901
13/50: d4 e5 121/901
14/50: h5 g5 104/898
15/50: g7 e8 -5/896
16/50: g7 e6 10/895
17/50: c1 d2 11/895
18/50: h5 h6 315/894
19/50: c1 g5 115/893
20/50: c4 f7 0/891
21/50: b2 b4 4/891
22/50: c4 e6 0/890
23/50: a2 a4 2/887
24/50: a2 a3 1/885
25/50: d1 b3 0/882
26/50: c4 g8 0/882
27/50: d1 e1 0/878
28/50: f2 e1 0/876
29/50: d1 d2 0/876
30/50: c4 b3 0/875
31/50: f2 f3 -6/875
32/50: c4 d5 0/870
33/50: f2 e3 -12/869
34/50: g3 g4 2/849
35/50: c1 f4 11/836
36/50: f2 g1 18/834
37/50: b1 d2 5/808
38/50: b2 b3 2/805
39/50: c4 a6 0/803
40/50: c4 b5 0/802
41/50: d1 e2 0/800
42/50: f2 f1 6/800
43/50: c4 e2 0/799
44/50: d1 f1 0/799
45/50: c1 e3 11/799
46/50: c4 d3 0/761
47/50: b1 a3 0/756
48/50: c4 f1 -11/748
49/50: d1 a4 0/719

8 r.b....r
7 pp..k.N.
6 .npb.p.n
5 ....p.pR
4 ..BPP...
3 ..PQ..P.
2 PP...K..
1 RNB.....
  ABCDEFGH
16. Qd3
score: 906, material: 914, abs: 6810, pawns: 6, mobility: 7
Depth  1 #searched    15570 bmove: c8 d7 bscore: -897
Depth  2 #searched   153053 bmove: h6 g4 bscore: -916
Depth  3 #searched  1688485 bmove: e5 d4 bscore: -863
Search total: 167335701 / 12244 ms / 13666 nodes/ms
hash size r 6 t 439 
0/33: e5 d4 -121/-863
1/33: c6 c5 -4/-866
2/33: c8 g4 -11/-882
3/33: h6 f5 -10/-884
4/33: h6 f7 -5/-891
5/33: c8 d7 -11/-895
6/33: a7 a5 -2/-902
7/33: a7 a6 -1/-904
8/33: d6 a3 0/-904
9/33: b6 d5 -5/-907
10/33: h8 h7 0/-908
11/33: b6 c4 -360/-912
12/33: d6 c7 0/-912
13/33: e7 f8 -12/-912
14/33: d6 b4 0/-912
15/33: c8 h3 -11/-916
16/33: d6 b8 11/-918
17/33: b6 d7 0/-920
18/33: c8 f5 -11/-922
19/33: c8 e6 -11/-922
20/33: e7 d8 -6/-926
21/33: a8 b8 0/-928
22/33: h6 g4 -5/-929
23/33: e7 d7 0/-931
24/33: b6 a4 5/-952
25/33: f6 f5 -4/-1049
26/33: d6 c5 0/-1057
27/33: h8 g8 0/-1071
28/33: h8 d8 0/-1081
29/33: h8 f8 0/-1090
30/33: h8 e8 0/-1100
31/33: g5 g4 -2/-1216
32/33: h6 g8 0/-1263

8 r.b....r
7 pp..k.N.
6 .npb.p.n
5 ......pR
4 ..BpP...
3 ..PQ..P.
2 PP...K..
1 RNB.....
  ABCDEFGH
17. xd4
score: -863, material: 793, abs: 6707, pawns: -30, mobility: 5
Depth  1 #searched    20609 bmove: g7 f5 bscore: 950
Depth  2 #searched   569146 bmove: c4 b3 bscore: 773
Depth  3 #searched  2380283 bmove: c4 b3 bscore: 936
Search total: 169715984 / 12402 ms / 13684 nodes/ms
hash size r 2 t 537 
0/43: c4 b3 0/936
1/43: b1 a3 0/936
2/43: h5 h2 0/936
3/43: h5 h4 0/936
4/43: h5 h1 0/936
5/43: g3 g4 2/936
6/43: c1 f4 11/936
7/43: e4 e5 7/936
8/43: c4 b5 0/936
9/43: g7 e8 -5/936
10/43: c4 a6 0/936
11/43: h5 h3 0/935
12/43: h5 g5 104/935
13/43: d3 d2 0/935
14/43: g7 e6 10/935
15/43: c3 d4 129/934
16/43: c1 e3 11/934
17/43: g7 f5 10/934
18/43: f2 e2 -6/933
19/43: d3 e3 0/933
20/43: d3 f1 0/931
21/43: d3 f3 0/931
22/43: c4 d5 0/931
23/43: d3 d4 121/930
24/43: c4 f7 0/930
25/43: b2 b4 4/929
26/43: a2 a4 2/927
27/43: c4 e6 0/927
28/43: d3 c2 0/926
29/43: b1 d2 5/925
30/43: f2 g1 18/925
31/43: b2 b3 2/925
32/43: d3 e2 0/924
33/43: a2 a3 1/923
34/43: d3 d1 0/923
35/43: c1 d2 11/918
36/43: f2 g2 6/915
37/43: f2 f1 6/912
38/43: c1 g5 115/903
39/43: f2 f3 -6/888
40/43: f2 e1 0/848
41/43: c4 g8 0/848
42/43: h5 h6 315/805

8 r.b....r
7 pp..k.N.
6 .npb.p.n
5 ......pR
4 ...pP...
3 .BPQ..P.
2 PP...K..
1 RNB.....
  ABCDEFGH
17. Bb3
score: 936, material: 793, abs: 6707, pawns: -30, mobility: 8
Depth  1 #searched    20753 bmove: d4 c3 bscore: -829
Depth  2 #searched   175223 bmove: h6 f7 bscore: -958
Depth  3 #searched  3264321 bmove: d6 e5 bscore: -833
Search total: 172980305 / 12617 ms / 13710 nodes/ms
hash size r 3 t 817 
0/36: d6 e5 0/-833
1/36: d6 c5 0/-833
2/36: a7 a5 -2/-842
3/36: a7 a6 -1/-844
4/36: a8 b8 0/-845
5/36: d6 c7 0/-848
6/36: b6 d7 0/-848
7/36: e7 d7 0/-850
8/36: c8 d7 -11/-852
9/36: d6 b8 11/-861
10/36: d4 c3 -98/-883
11/36: b6 c4 -10/-883
12/36: b6 a4 5/-885
13/36: d6 a3 0/-887
14/36: d6 b4 0/-891
15/36: c8 h3 -11/-902
16/36: c8 e6 -11/-903
17/36: b6 d5 -5/-904
18/36: c8 f5 -11/-906
19/36: d6 f4 0/-923
20/36: h6 f5 -10/-931
21/36: e7 d8 -6/-934
22/36: h8 f8 0/-936
23/36: g5 g4 -2/-938
24/36: h8 d8 0/-939
25/36: c8 g4 -11/-949
26/36: c6 c5 -4/-950
27/36: h8 h7 0/-953
28/36: h6 g4 -5/-955
29/36: e7 f8 -12/-964
30/36: h6 f7 -5/-969
31/36: f6 f5 -4/-980
32/36: d6 g3 -102/-982
33/36: h8 g8 0/-1103
34/36: h6 g8 0/-1107
35/36: h8 e8 0/-1174

8 r.b....r
7 pp..k.N.
6 .np..p.n
5 ....b.pR
4 ...pP...
3 .BPQ..P.
2 PP...K..
1 RNB.....
  ABCDEFGH
18. Be5
score: -833, material: 793, abs: 6707, pawns: -30, mobility: 9
Depth  1 #searched    46148 bmove: c3 d4 bscore: 1174
Depth  2 #searched   415710 bmove: c3 d4 bscore: 833
Depth  3 #searched  2335485 bmove: c3 d4 bscore: 1209
Search total: 175315790 / 12771 ms / 13727 nodes/ms
hash size r 4 t 289 
0/45: c3 d4 129/1209
1/45: h5 h1 0/1209
2/45: f2 g2 6/1208
3/45: c1 g5 115/1207
4/45: b3 e6 0/1206
5/45: h5 g5 104/1206
6/45: d3 a6 0/1205
7/45: d3 e2 0/1205
8/45: c1 f4 11/1205
9/45: a2 a3 1/1205
10/45: g7 e6 10/1205
11/45: h5 h2 0/1204
12/45: h5 h4 0/1204
13/45: d3 b5 0/1203
14/45: d3 f3 0/1203
15/45: f2 g1 18/1203
16/45: f2 f1 6/1203
17/45: b3 g8 0/1203
18/45: h5 h3 0/1203
19/45: d3 f1 0/1202
20/45: c1 d2 11/1202
21/45: c1 e3 11/1201
22/45: g3 g4 2/1201
23/45: b3 f7 0/1201
24/45: b3 a4 0/1199
25/45: g7 e8 -5/1199
26/45: b3 c4 0/1199
27/45: d3 c2 0/1197
28/45: f2 e1 0/1197
29/45: b3 d5 0/1196
30/45: d3 d1 0/1196
31/45: b3 c2 0/1196
32/45: f2 f3 -6/1194
33/45: f2 e2 -6/1193
34/45: d3 c4 0/1191
35/45: b1 a3 0/1187
36/45: d3 d4 121/1185
37/45: b3 d1 -11/1183
38/45: b1 d2 5/1181
39/45: d3 d2 0/1181
40/45: d3 e3 0/1180
41/45: g7 f5 10/1175
42/45: c3 c4 4/1157
43/45: a2 a4 2/1156
44/45: h5 h6 315/1052

8 r.b....r
7 pp..k.N.
6 .np..p.n
5 ....b.pR
4 ...PP...
3 .B.Q..P.
2 PP...K..
1 RNB.....
  ABCDEFGH
18. xd4
score: 1209, material: 922, abs: 6594, pawns: 16, mobility: 10
Depth  1 #searched     9984 bmove: e5 d6 bscore: -833
Depth  2 #searched   243520 bmove: e5 g3 bscore: -1091
Depth  3 #searched  2887859 bmove: e5 b8 bscore: -966
Search total: 178203649 / 12956 ms / 13754 nodes/ms
hash size r 2 t 1091 
0/34: e5 b8 11/-966
1/34: f6 f5 -4/-972
2/34: e5 f4 0/-1010
3/34: c8 e6 -11/-1036
4/34: h8 f8 0/-1041
5/34: e5 c7 0/-1043
6/34: b6 d5 -5/-1054
7/34: c6 c5 -4/-1061
8/34: b6 c4 -10/-1078
9/34: b6 a4 5/-1079
10/34: e5 d6 0/-1080
11/34: c8 h3 -11/-1095
12/34: h8 d8 0/-1134
13/34: b6 d7 0/-1139
14/34: h6 g4 -5/-1140
15/34: h6 f7 -5/-1165
16/34: e7 d6 6/-1168
17/34: a7 a5 -2/-1173
18/34: a8 b8 0/-1174
19/34: a7 a6 -1/-1174
20/34: c8 g4 -11/-1266
21/34: c8 d7 -11/-1283
22/34: e7 f8 -12/-1288
23/34: h8 h7 0/-1291
24/34: g5 g4 -2/-1292
25/34: h8 g8 0/-1318
26/34: e5 d4 -112/-1322
27/34: c8 f5 -11/-1323
28/34: h6 f5 -10/-1375
29/34: e5 g3 -102/-1403
30/34: e7 d8 -6/-1412
31/34: e7 d7 0/-1412
32/34: h8 e8 0/-1427
33/34: h6 g8 0/-1602

8 rbb....r
7 pp..k.N.
6 .np..p.n
5 ......pR
4 ...PP...
3 .B.Q..P.
2 PP...K..
1 RNB.....
  ABCDEFGH
19. Bb8
score: -966, material: 933, abs: 6583, pawns: 16, mobility: 12
Depth  1 #searched    16606 bmove: f2 g1 bscore: 1209
Depth  2 #searched   250358 bmove: b1 c3 bscore: 849
Depth  3 #searched  1643778 bmove: b1 c3 bscore: 1185
Search total: 179847427 / 13063 ms / 13767 nodes/ms
hash size r 3 t 507 
0/47: b1 c3 10/1185
1/47: b3 c4 0/1185
2/47: d3 d1 0/1185
3/47: b3 d1 -11/1185
4/47: b3 g8 0/1185
5/47: b3 c2 0/1185
6/47: a2 a4 2/1185
7/47: d3 f3 0/1185
8/47: a2 a3 1/1185
9/47: g7 e8 -5/1185
10/47: d3 c3 0/1185
11/47: d3 f1 0/1184
12/47: f2 e1 0/1184
13/47: d3 c2 0/1184
14/47: d3 d2 0/1184
15/47: c1 e3 11/1183
16/47: b1 d2 5/1183
17/47: b3 f7 0/1182
18/47: b1 a3 0/1182
19/47: g7 f5 10/1182
20/47: f2 g2 6/1181
21/47: f2 f3 -6/1181
22/47: h5 h4 0/1181
23/47: d3 e2 0/1180
24/47: b3 d5 0/1179
25/47: b3 a4 0/1179
26/47: f2 e2 -6/1179
27/47: d3 e3 0/1179
28/47: f2 f1 6/1178
29/47: h5 h3 0/1177
30/47: g3 g4 2/1176
31/47: f2 e3 -12/1174
32/47: d4 d5 6/1171
33/47: b3 e6 0/1169
34/47: f2 g1 18/1155
35/47: d3 b5 0/1155
36/47: d3 a6 0/1153
37/47: d3 c4 0/1150
38/47: e4 e5 7/1148
39/47: h5 h6 315/1142
40/47: c1 f4 11/1131
41/47: h5 g5 104/1116
42/47: c1 g5 115/1087
43/47: c1 d2 11/984
44/47: g7 e6 10/955
45/47: h5 h1 0/788
46/47: h5 h2 0/788

8 rbb....r
7 pp..k.N.
6 .np..p.n
5 ......pR
4 ...PP...
3 .BNQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
19. Nc3
score: 1185, material: 943, abs: 6593, pawns: 16, mobility: 16
Depth  1 #searched    13021 bmove: c8 g4 bscore: -849
Depth  2 #searched   153625 bmove: h6 g4 bscore: -986
Depth  3 #searched  1857236 bmove: b6 d7 bscore: -947
Search total: 181704663 / 13182 ms / 13784 nodes/ms
hash size r 4 t 778 
0/32: b6 d7 0/-947
1/32: h8 f8 0/-947
2/32: b8 c7 -11/-957
3/32: b8 d6 -11/-960
4/32: e7 d7 0/-960
5/32: a7 a6 -1/-964
6/32: a7 a5 -2/-973
7/32: c8 d7 -11/-977
8/32: c8 g4 -11/-979
9/32: e7 d8 -6/-982
10/32: h6 g4 -5/-986
11/32: b8 g3 -113/-986
12/32: c6 c5 -4/-993
13/32: g5 g4 -2/-994
14/32: h6 f5 -10/-995
15/32: e7 f8 -12/-998
16/32: h8 g8 0/-1017
17/32: b6 a4 5/-1090
18/32: b6 c4 -10/-1090
19/32: b8 e5 -11/-1092
20/32: b6 d5 -5/-1100
21/32: c8 h3 -11/-1109
22/32: c8 e6 -11/-1112
23/32: b8 f4 -11/-1116
24/32: c8 f5 -11/-1137
25/32: h6 f7 -5/-1159
26/32: h8 e8 0/-1167
27/32: h8 d8 0/-1182
28/32: h8 h7 0/-1189
29/32: e7 d6 6/-1197
30/32: f6 f5 -4/-1304
31/32: h6 g8 0/-1324

8 rbb....r
7 pp.nk.N.
6 ..p..p.n
5 ......pR
4 ...PP...
3 .BNQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
20. Nd7
score: -947, material: 943, abs: 6593, pawns: 16, mobility: 20
Depth  1 #searched    18021 bmove: f2 g1 bscore: 1225
Depth  2 #searched   274137 bmove: b3 e6 bscore: 908
Depth  3 #searched  1726974 bmove: b3 e6 bscore: 1196
Search total: 183431637 / 13294 ms / 13798 nodes/ms
hash size r 5 t 599 
0/51: b3 e6 0/1196
1/51: g3 g4 2/1196
2/51: b3 d1 -11/1196
3/51: d3 f3 0/1195
4/51: d3 e2 0/1195
5/51: d3 d1 0/1195
6/51: g7 f5 10/1193
7/51: c3 b5 -5/1192
8/51: h5 h4 0/1190
9/51: f2 f3 -6/1188
10/51: c3 d5 5/1187
11/51: f2 g1 18/1182
12/51: d4 d5 6/1181
13/51: c1 d2 11/1181
14/51: c1 e3 11/1179
15/51: f2 g2 6/1172
16/51: d3 b5 0/1169
17/51: a2 a4 2/1169
18/51: a2 a3 1/1169
19/51: f2 f1 6/1169
20/51: b3 f7 0/1168
21/51: d3 a6 0/1168
22/51: h5 h1 0/1166
23/51: h5 h2 0/1166
24/51: h5 h3 0/1166
25/51: d3 f1 0/1164
26/51: a1 b1 0/1164
27/51: f2 e1 0/1164
28/51: b3 c4 0/1164
29/51: b3 a4 0/1163
30/51: d3 c4 0/1163
31/51: b3 d5 0/1163
32/51: g7 e8 -5/1163
33/51: b3 c2 0/1162
34/51: b3 g8 0/1161
35/51: d3 d2 0/1159
36/51: d3 c2 0/1158
37/51: d3 e3 0/1158
38/51: f2 e2 -6/1157
39/51: c3 e2 -5/1156
40/51: d3 b1 0/1155
41/51: c1 f4 11/1155
42/51: e4 e5 7/1154
43/51: c3 a4 -10/1152
44/51: c3 b1 -10/1152
45/51: c3 d1 -10/1151
46/51: h5 g5 104/1114
47/51: f2 e3 -12/1095
48/51: c1 g5 115/1078
49/51: h5 h6 315/1025
50/51: g7 e6 10/914

8 rbb....r
7 pp.nk.N.
6 ..p.Bp.n
5 ......pR
4 ...PP...
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
20. Be6
score: 1196, material: 943, abs: 6593, pawns: 16, mobility: 22
Depth  1 #searched    13057 bmove: b8 c7 bscore: -908
Depth  2 #searched   251364 bmove: h6 f7 bscore: -1039
Depth  3 #searched  3457800 bmove: b8 d6 bscore: -960
Search total: 186889437 / 13515 ms / 13828 nodes/ms
hash size r 6 t 1212 
0/28: b8 d6 -11/-960
1/28: a7 a6 -1/-961
2/28: b7 b6 -2/-962
3/28: e7 d8 -6/-965
4/28: b8 c7 -11/-968
5/28: a7 a5 -2/-970
6/28: b7 b5 -4/-972
7/28: e7 f8 -12/-975
8/28: d7 b6 0/-1011
9/28: e7 d6 6/-1048
10/28: c6 c5 -4/-1053
11/28: d7 f8 5/-1114
12/28: g5 g4 -2/-1127
13/28: h8 h7 0/-1133
14/28: d7 c5 -5/-1147
15/28: d7 e5 -5/-1158
16/28: b8 e5 -11/-1159
17/28: h6 f7 -5/-1161
18/28: b8 g3 -113/-1163
19/28: b8 f4 -11/-1167
20/28: f6 f5 -4/-1182
21/28: h8 g8 0/-1189
22/28: h6 g4 -5/-1231
23/28: h8 f8 0/-1234
24/28: h8 d8 0/-1237
25/28: h8 e8 0/-1356
26/28: h6 g8 0/-1422
27/28: h6 f5 -10/-1642

8 r.b....r
7 pp.nk.N.
6 ..pbBp.n
5 ......pR
4 ...PP...
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
21. Bd6
score: -960, material: 932, abs: 6604, pawns: 16, mobility: 18
Depth  1 #searched    21536 bmove: f2 g1 bscore: 1227
Depth  2 #searched   290208 bmove: e4 e5 bscore: 936
Depth  3 #searched  5704876 bmove: e6 d7 bscore: 1156
Search total: 192594313 / 13879 ms / 13876 nodes/ms
hash size r 7 t 2023 
0/53: e6 d7 320/1156
1/53: h5 g5 104/1143
2/53: h5 h6 315/1038
3/53: c1 g5 115/1037
4/53: c3 b5 -5/984
5/53: d3 f3 0/981
6/53: d3 d2 0/981
7/53: f2 e1 0/980
8/53: a1 b1 0/980
9/53: d3 e3 0/980
10/53: f2 f3 -6/980
11/53: c3 e2 -5/980
12/53: f2 e2 -6/979
13/53: g7 f5 10/979
14/53: d3 c2 0/978
15/53: h5 h3 0/976
16/53: e6 h3 0/976
17/53: e6 b3 0/975
18/53: d3 f1 0/974
19/53: e6 c4 0/973
20/53: g7 e8 -5/971
21/53: h5 h2 0/968
22/53: c1 d2 11/967
23/53: c3 a4 -10/967
24/53: d4 d5 6/967
25/53: c3 d1 -10/966
26/53: c3 b1 -10/966
27/53: e6 g4 0/966
28/53: e6 f5 0/966
29/53: d3 b1 0/965
30/53: c3 d5 5/964
31/53: d3 b5 0/962
32/53: d3 a6 0/962
33/53: h5 h4 0/960
34/53: f2 g2 6/958
35/53: b2 b4 4/958
36/53: f2 f1 6/957
37/53: a2 a4 2/957
38/53: b2 b3 2/957
39/53: g3 g4 2/956
40/53: a2 a3 1/956
41/53: c1 e3 11/955
42/53: d3 c4 0/955
43/53: f2 g1 18/953
44/53: d3 e2 0/951
45/53: d3 d1 0/951
46/53: h5 h1 0/946
47/53: e4 e5 7/945
48/53: e6 g8 0/937
49/53: f2 e3 -12/933
50/53: e6 f7 0/922
51/53: c1 f4 11/894
52/53: e6 d5 0/862

8 r.b....r
7 pp.Bk.N.
6 ..pb.p.n
5 ......pR
4 ...PP...
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
21. Bxd7
score: 1156, material: 1252, abs: 6284, pawns: 16, mobility: 20
Depth  1 #searched    21473 bmove: c8 d7 bscore: -800
Depth  2 #searched    94344 bmove: d6 g3 bscore: -1167
Depth  3 #searched   561882 bmove: e7 d7 bscore: -1078
Depth  4 #searched  9808999 bmove: h6 g4 bscore: -1094
Search total: 202403312 / 14497 ms / 13961 nodes/ms
hash size r 2 t 4474 
0/30: h6 g4 -5/-1094
1/30: c8 d7 -361/-1099
2/30: e7 d7 -350/-1100
3/30: d6 f4 0/-1120
4/30: d6 a3 0/-1129
5/30: d6 g3 -102/-1130
6/30: c6 c5 -4/-1142
7/30: e7 d8 -6/-1151
8/30: b7 b5 -4/-1151
9/30: a7 a5 -2/-1155
10/30: b7 b6 -2/-1155
11/30: d6 c7 0/-1156
12/30: a8 b8 0/-1156
13/30: a7 a6 -1/-1156
14/30: d6 b4 0/-1157
15/30: h8 g8 0/-1164
16/30: h6 f5 -10/-1187
17/30: h6 f7 -5/-1199
18/30: h6 g8 0/-1203
19/30: e7 f7 -6/-1206
20/30: e7 f8 -12/-1244
21/30: h8 f8 0/-1246
22/30: h8 d8 0/-1249
23/30: f6 f5 -4/-1252
24/30: d6 e5 0/-1254
25/30: d6 c5 0/-1254
26/30: h8 h7 0/-1264
27/30: d6 b8 11/-1316
28/30: g5 g4 -2/-1583
29/30: h8 e8 0/-1675

8 r.b....r
7 pp.Bk.N.
6 ..pb.p..
5 ......pR
4 ...PP.n.
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
22. Ng4+
score: -1094, material: 1247, abs: 6289, pawns: 16, mobility: 18
Depth  1 #searched     2510 bmove: d7 g4 bscore: 1224
Depth  2 #searched    52853 bmove: d7 g4 bscore: 1218
Depth  3 #searched   312739 bmove: d7 g4 bscore: 1218
Depth  4 #searched  2467511 bmove: d7 g4 bscore: 996
Search total: 204870823 / 14652 ms / 13982 nodes/ms
hash size r 3 t 1005 
0/7: d7 g4 320/996
1/7: f2 e1 0/996
2/7: f2 f3 -6/991
3/7: f2 f1 6/991
4/7: f2 e2 -6/990
5/7: f2 g1 18/986
6/7: f2 g2 6/976

8 r.b....r
7 pp..k.N.
6 ..pb.p..
5 ......pR
4 ...PP.B.
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
22. Bxg4
score: 996, material: 1567, abs: 5969, pawns: 16, mobility: 22
Depth  1 #searched     8831 bmove: c8 g4 bscore: -1218
Depth  2 #searched    59903 bmove: c8 g4 bscore: -1218
Depth  3 #searched   626482 bmove: c8 g4 bscore: -996
Depth  4 #searched  1152499 bmove: c8 g4 bscore: -1479
Search total: 206023322 / 14722 ms / 13994 nodes/ms
hash size r 2 t 739 
0/29: c8 g4 -361/-1479
1/29: h8 g8 0/-1479
2/29: h8 f8 0/-1479
3/29: h8 d8 0/-1479
4/29: c6 c5 -4/-1485
5/29: h8 h5 -500/-1499
6/29: h8 h6 0/-1514
7/29: h8 h7 0/-1532
8/29: c8 f5 -11/-1550
9/29: c8 d7 -11/-1575
10/29: d6 b8 11/-1582
11/29: h8 e8 0/-1597
12/29: d6 g3 -102/-1604
13/29: f6 f5 -4/-1613
14/29: b7 b6 -2/-1688
15/29: c8 e6 -11/-1689
16/29: a7 a5 -2/-1737
17/29: a8 b8 0/-1745
18/29: d6 f4 0/-1759
19/29: d6 b4 0/-1772
20/29: e7 f7 -6/-1901
21/29: b7 b5 -4/-1919
22/29: d6 c7 0/-1923
23/29: a7 a6 -1/-1923
24/29: e7 f8 -12/-1964
25/29: d6 a3 0/-2002
26/29: d6 e5 0/-2067
27/29: d6 c5 0/-2069
28/29: e7 d8 -6/-2079

8 r......r
7 pp..k.N.
6 ..pb.p..
5 ......pR
4 ...PP.b.
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
23. Bxg4
score: -1479, material: 1206, abs: 5630, pawns: 16, mobility: 2
Depth  1 #searched    27965 bmove: g7 f5 bscore: 1225
Depth  2 #searched   152663 bmove: g7 f5 bscore: 1576
Depth  3 #searched  1196200 bmove: g7 f5 bscore: 1576
Search total: 207219522 / 14795 ms / 14006 nodes/ms
hash size r 2 t 405 
0/44: g7 f5 10/1576
1/44: c3 d5 5/1575
2/44: d3 a6 0/1570
3/44: c1 f4 11/1566
4/44: c3 b5 -5/1564
5/44: f2 g1 18/1563
6/44: c1 g5 115/1562
7/44: c1 d2 11/1562
8/44: h5 h7 22/1562
9/44: e4 e5 7/1561
10/44: c1 e3 11/1560
11/44: d3 b5 0/1553
12/44: f2 g2 6/1553
13/44: a2 a4 2/1550
14/44: d3 c4 0/1550
15/44: f2 f1 6/1550
16/44: b2 b3 2/1550
17/44: a2 a3 1/1549
18/44: d3 f1 0/1545
19/44: a1 b1 0/1545
20/44: f2 e1 0/1545
21/44: d3 c2 0/1543
22/44: d3 d2 0/1540
23/44: d3 e3 0/1539
24/44: d3 b1 0/1536
25/44: c3 a4 -10/1536
26/44: c3 b1 -10/1535
27/44: h5 g5 104/1518
28/44: d4 d5 6/1512
29/44: g7 e6 10/1506
30/44: b2 b4 4/1498
31/44: d3 e2 0/1493
32/44: h5 h6 0/1493
33/44: h5 h3 0/1493
34/44: h5 h4 0/1492
35/44: d3 f3 0/1492
36/44: g7 e8 -5/1489
37/44: d3 d1 0/1489
38/44: c3 e2 -5/1488
39/44: c3 d1 -10/1482
40/44: h5 h8 500/1481
41/44: h5 h1 0/1480
42/44: h5 h2 0/1478
43/44: f2 e3 -12/1471

8 r......r
7 pp..k...
6 ..pb.p..
5 .....NpR
4 ...PP.b.
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
23. Nf5+
score: 1576, material: 1216, abs: 5640, pawns: 16, mobility: 9
Depth  1 #searched     2920 bmove: g4 f5 bscore: -1250
Depth  2 #searched    39574 bmove: g4 f5 bscore: -908
Depth  3 #searched   125922 bmove: g4 f5 bscore: -1138
Depth  4 #searched  3149135 bmove: e7 d7 bscore: -1477
Search total: 210368657 / 14990 ms / 14033 nodes/ms
hash size r 3 t 1013 
0/7: e7 d7 0/-1477
1/7: g4 f5 -330/-1478
2/7: e7 f7 -6/-1531
3/7: e7 e6 6/-1555
4/7: e7 e8 -6/-1616
5/7: e7 f8 -12/-2095
6/7: e7 d8 -6/-2260

8 r......r
7 pp.k....
6 ..pb.p..
5 .....NpR
4 ...PP.b.
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
24. Kd7
score: -1477, material: 1216, abs: 5640, pawns: 16, mobility: 7
Depth  1 #searched    27965 bmove: h5 h4 bscore: 1233
Depth  2 #searched   407516 bmove: h5 h8 bscore: 1257
Depth  3 #searched  1608958 bmove: h5 h8 bscore: 1466
Search total: 211977615 / 15092 ms / 14045 nodes/ms
hash size r 4 t 311 
0/47: h5 h8 500/1466
1/47: c3 e2 -5/1466
2/47: d3 b1 0/1466
3/47: c3 d1 -10/1466
4/47: a2 a3 1/1466
5/47: f5 e3 -5/1466
6/47: f5 h4 -15/1466
7/47: d3 f3 0/1466
8/47: d3 e2 0/1466
9/47: d3 d1 0/1466
10/47: f5 h6 -15/1466
11/47: d3 d2 0/1465
12/47: c3 a4 -10/1465
13/47: f2 g2 6/1465
14/47: b2 b4 4/1465
15/47: f2 e3 -12/1464
16/47: d3 c4 0/1464
17/47: h5 h4 0/1464
18/47: d3 b5 0/1464
19/47: c1 d2 11/1463
20/47: c3 d5 5/1463
21/47: b2 b3 2/1463
22/47: a2 a4 2/1463
23/47: f5 g7 -10/1462
24/47: d3 a6 0/1461
25/47: c3 b1 -10/1460
26/47: e4 e5 7/1459
27/47: c1 f4 11/1431
28/47: f2 g1 18/1428
29/47: h5 h1 0/1427
30/47: h5 h3 0/1427
31/47: h5 h2 0/1427
32/47: f5 e7 -10/1426
33/47: c1 e3 11/1425
34/47: f2 e1 0/1412
35/47: d3 f1 0/1412
36/47: a1 b1 0/1412
37/47: d3 c2 0/1410
38/47: d3 e3 0/1409
39/47: c3 b5 -5/1409
40/47: c1 g5 115/1385
41/47: h5 g5 104/1383
42/47: d4 d5 6/1379
43/47: h5 h6 0/1295
44/47: f5 d6 350/1292
45/47: f2 f1 6/1280
46/47: h5 h7 22/1098

8 r......R
7 pp.k....
6 ..pb.p..
5 .....Np.
4 ...PP.b.
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
24. Rxh8
score: 1466, material: 1716, abs: 5140, pawns: 16, mobility: 21
Depth  1 #searched     9039 bmove: a8 h8 bscore: -1257
Depth  2 #searched    43761 bmove: a8 h8 bscore: -1466
Depth  3 #searched   264817 bmove: a8 h8 bscore: -1220
Depth  4 #searched   673790 bmove: a8 h8 bscore: -1479
Depth  5 #searched  2540378 bmove: a8 h8 bscore: -1209
Search total: 214517993 / 15246 ms / 14070 nodes/ms
hash size r 2 t 2000 
0/30: a8 h8 -500/-1209
1/30: d6 g3 -102/-1226
2/30: a7 a6 -1/-1333
3/30: d6 b4 0/-1333
4/30: d6 c7 0/-1334
5/30: d6 f8 11/-1373
6/30: d6 b8 11/-1376
7/30: g4 e2 0/-1395
8/30: a8 c8 0/-1479
9/30: a8 e8 0/-1480
10/30: a7 a5 -2/-1480
11/30: a8 f8 0/-1483
12/30: a8 b8 0/-1539
13/30: g4 f5 -330/-1561
14/30: d6 f4 0/-1576
15/30: a8 g8 0/-1591
16/30: d6 c5 0/-1633
17/30: d6 e5 0/-1679
18/30: g4 f3 0/-1686
19/30: c6 c5 -4/-1766
20/30: d6 a3 0/-1802
21/30: d7 c7 -6/-1819
22/30: g4 h5 0/-1824
23/30: g4 d1 0/-1824
24/30: b7 b5 -4/-1825
25/30: b7 b6 -2/-1825
26/30: d7 e6 6/-1830
27/30: a8 d8 0/-1863
28/30: d6 e7 0/-1879
29/30: g4 h3 0/-1916

8 .......r
7 pp.k....
6 ..pb.p..
5 .....Np.
4 ...PP.b.
3 ..NQ..P.
2 PP...K..
1 R.B.....
  ABCDEFGH
25. Rxh8
score: -1209, material: 1216, abs: 4640, pawns: 16, mobility: 0
Depth  1 #searched    14772 bmove: f5 e3 bscore: 1466
Depth  2 #searched   148201 bmove: e4 e5 bscore: 1238
Depth  3 #searched   732948 bmove: f5 e3 bscore: 1458
Depth  4 #searched  3093485 bmove: f5 e3 bscore: 1372
Search total: 217611478 / 15439 ms / 14094 nodes/ms
hash size r 2 t 1970 
0/39: f5 e3 -5/1372
1/39: f5 h6 -15/1372
2/39: e4 e5 7/1372
3/39: a2 a3 1/1372
4/39: f2 g1 18/1372
5/39: c1 e3 11/1372
6/39: c1 f4 11/1372
7/39: d3 e3 0/1372
8/39: d3 d2 0/1372
9/39: f2 e1 0/1372
10/39: f5 g7 -10/1372
11/39: f2 g2 6/1371
12/39: c1 d2 11/1371
13/39: d4 d5 6/1371
14/39: c1 g5 115/1370
15/39: a2 a4 2/1370
16/39: c3 b5 -5/1368
17/39: b2 b3 2/1368
18/39: d3 c4 0/1367
19/39: b2 b4 4/1367
20/39: c3 e2 -5/1366
21/39: f5 d6 350/1366
22/39: c3 a4 -10/1364
23/39: c3 d5 5/1364
24/39: f5 e7 -10/1363
25/39: d3 b5 0/1360
26/39: c3 d1 -10/1360
27/39: d3 f1 0/1360
28/39: d3 c2 0/1360
29/39: c3 b1 -10/1360
30/39: d3 a6 0/1360
31/39: f2 f1 6/1359
32/39: a1 b1 0/1359
33/39: f5 h4 -15/1358
34/39: f2 e3 -12/1349
35/39: d3 f3 0/1346
36/39: d3 b1 0/1345
37/39: d3 e2 0/1310
38/39: d3 d1 0/1268

8 .......r
7 pp.k....
6 ..pb.p..
5 ......p.
4 ...PP.b.
3 ..NQN.P.
2 PP...K..
1 R.B.....
  ABCDEFGH
25. Ne3
score: 1372, material: 1211, abs: 4635, pawns: 16, mobility: -6
Depth  1 #searched     8350 bmove: g4 e6 bscore: -1220
Depth  2 #searched    76288 bmove: d6 g3 bscore: -1464
Depth  3 #searched   762718 bmove: g4 e2 bscore: -1341
Depth  4 #searched  2715794 bmove: g4 e2 bscore: -1123
Search total: 220327272 / 15608 ms / 14116 nodes/ms
hash size r 3 t 2304 
0/43: g4 e2 0/-1123
1/43: g4 h3 0/-1128
2/43: g4 h5 0/-1129
3/43: h8 h2 -22/-1169
4/43: g4 d1 0/-1198
5/43: c6 c5 -4/-1199
6/43: h8 h1 0/-1199
7/43: f6 f5 -4/-1199
8/43: d7 c8 -12/-1201
9/43: g4 e6 0/-1203
10/43: d7 e8 -6/-1206
11/43: d7 d8 -6/-1206
12/43: g4 f3 0/-1206
13/43: b7 b5 -4/-1207
14/43: d7 c7 -6/-1208
15/43: b7 b6 -2/-1210
16/43: a7 a5 -2/-1210
17/43: h8 h5 0/-1211
18/43: h8 h7 0/-1211
19/43: h8 h6 0/-1211
20/43: h8 h3 0/-1211
21/43: a7 a6 -1/-1211
22/43: d6 b4 0/-1212
23/43: d6 c7 0/-1213
24/43: d7 e7 0/-1213
25/43: d6 e7 0/-1215
26/43: d7 e6 6/-1217
27/43: d6 f8 11/-1224
28/43: d6 b8 11/-1226
29/43: d6 g3 -102/-1230
30/43: h8 e8 0/-1287
31/43: h8 f8 0/-1345
32/43: h8 g8 0/-1347
33/43: h8 c8 0/-1349
34/43: h8 a8 0/-1350
35/43: h8 b8 0/-1351
36/43: h8 d8 0/-1351
37/43: g4 f5 0/-1367
38/43: d6 e5 0/-1513
39/43: d6 c5 0/-1520
40/43: d6 a3 0/-1523
41/43: d6 f4 0/-1534
42/43: h8 h4 0/-1734

8 .......r
7 pp.k....
6 ..pb.p..
5 ......p.
4 ...PP...
3 ..NQN.P.
2 PP..bK..
1 R.B.....
  ABCDEFGH
26. Be2
score: -1123, material: 1211, abs: 4635, pawns: 16, mobility: -6
Depth  1 #searched     8520 bmove: d3 e2 bscore: 1827
Depth  2 #searched   276976 bmove: d3 d2 bscore: 1220
Depth  3 #searched   860925 bmove: d3 d2 bscore: 1474
Depth  4 #searched 14913010 bmove: f2 e2 bscore: 1295
Search total: 235240282 / 16553 ms / 14211 nodes/ms
hash size r 4 t 7037 
0/35: f2 e2 344/1295
1/35: e3 g4 -5/1270
2/35: d3 e2 350/1210
3/35: d3 c2 0/1172
4/35: a2 a4 2/1172
5/35: b2 b3 2/1172
6/35: c3 b5 -5/1172
7/35: a2 a3 1/1171
8/35: d3 b1 0/1171
9/35: c3 d5 5/1169
10/35: g3 g4 2/1169
11/35: a1 b1 0/1168
12/35: c3 d1 -10/1162
13/35: c3 b1 -10/1156
14/35: c3 a4 -10/1156
15/35: e4 e5 7/1156
16/35: d4 d5 6/1148
17/35: c1 d2 11/1145
18/35: b2 b4 4/1136
19/35: d3 b5 0/1135
20/35: d3 a6 0/1131
21/35: d3 d1 0/1130
22/35: c3 e2 345/1129
23/35: f2 g1 18/1128
24/35: f2 e1 0/1124
25/35: d3 c4 0/1120
26/35: e3 c4 0/1093
27/35: e3 f5 5/1093
28/35: e3 d1 -10/1090
29/35: e3 d5 5/1075
30/35: e3 c2 -5/1062
31/35: d3 d2 0/1021
32/35: e3 g2 -5/689
33/35: e3 f1 -10/663
34/35: f2 g2 6/484

8 .......r
7 pp.k....
6 ..pb.p..
5 ......p.
4 ...PP...
3 ..NQN.P.
2 PP..K...
1 R.B.....
  ABCDEFGH
26. Kxe2
score: 1295, material: 1555, abs: 4279, pawns: 16, mobility: -2
Depth  1 #searched     3846 bmove: a7 a6 bscore: -775
Depth  2 #searched    75510 bmove: d6 b4 bscore: -1618
Depth  3 #searched  1389373 bmove: h8 e8 bscore: -1550
Search total: 236629655 / 16643 ms / 14217 nodes/ms
hash size r 2 t 738 
0/37: h8 e8 0/-1550
1/37: d7 e6 6/-1551
2/37: b7 b5 -4/-1554
3/37: h8 h2 -22/-1555
4/37: d6 e7 0/-1558
5/37: d6 f8 11/-1558
6/37: h8 h6 0/-1558
7/37: h8 f8 0/-1558
8/37: a7 a6 -1/-1558
9/37: d7 e8 -6/-1562
10/37: h8 g8 0/-1562
11/37: d6 c7 0/-1563
12/37: h8 h3 0/-1563
13/37: h8 h5 0/-1564
14/37: h8 c8 0/-1564
15/37: h8 a8 0/-1564
16/37: h8 b8 0/-1564
17/37: d7 d8 -6/-1565
18/37: a7 a5 -2/-1566
19/37: h8 d8 0/-1566
20/37: b7 b6 -2/-1567
21/37: d7 c8 -12/-1568
22/37: h8 h1 0/-1569
23/37: d6 b4 0/-1571
24/37: d6 b8 11/-1571
25/37: g5 g4 -2/-1572
26/37: d7 c7 -6/-1574
27/37: h8 h7 0/-1574
28/37: d6 g3 -102/-1580
29/37: d6 a3 0/-1581
30/37: d7 e7 0/-1607
31/37: f6 f5 -4/-1710
32/37: d6 f4 0/-1811
33/37: d6 e5 0/-1915
34/37: d6 c5 0/-1925
35/37: h8 h4 0/-1959
36/37: c6 c5 -4/-2071

8 ....r...
7 pp.k....
6 ..pb.p..
5 ......p.
4 ...PP...
3 ..NQN.P.
2 PP..K...
1 R.B.....
  ABCDEFGH
27. Re8
score: -1550, material: 1555, abs: 4279, pawns: 16, mobility: 2
Depth  1 #searched     6633 bmove: g3 g4 bscore: 1956
Depth  2 #searched   121689 bmove: d4 d5 bscore: 1438
Depth  3 #searched  1389087 bmove: e2 f1 bscore: 1836
Search total: 238018742 / 16735 ms / 14222 nodes/ms
hash size r 3 t 856 
0/35: e2 f1 12/1836
1/35: d3 b5 0/1836
2/35: a2 a3 1/1836
3/35: e2 f2 6/1835
4/35: e2 e1 6/1835
5/35: b2 b3 2/1833
6/35: a2 a4 2/1833
7/35: e2 f3 0/1833
8/35: e2 d1 6/1832
9/35: c3 a4 -10/1832
10/35: c3 b1 -10/1832
11/35: c3 d1 -10/1832
12/35: a1 b1 0/1829
13/35: g3 g4 2/1828
14/35: e2 d2 0/1825
15/35: e3 g2 -5/1740
16/35: e3 g4 -5/1737
17/35: e3 f1 -10/1737
18/35: e3 d1 -10/1733
19/35: e3 c4 0/1725
20/35: d3 c4 0/1703
21/35: e3 f5 5/1700
22/35: c3 d5 5/1694
23/35: d3 c2 0/1675
24/35: d3 b1 0/1655
25/35: c1 d2 11/1634
26/35: d3 d1 0/1621
27/35: d3 d2 0/1620
28/35: e3 c2 -5/1614
29/35: c3 b5 -5/1601
30/35: b2 b4 4/1569
31/35: e3 d5 5/1568
32/35: e4 e5 7/1544
33/35: d4 d5 6/1458
34/35: d3 a6 0/1075

8 ....r...
7 pp.k....
6 ..pb.p..
5 ......p.
4 ...PP...
3 ..NQN.P.
2 PP......
1 R.B..K..
  ABCDEFGH
27. Kf1
score: 1836, material: 1567, abs: 4291, pawns: 16, mobility: 2
Depth  1 #searched     3210 bmove: a7 a6 bscore: -789
Depth  2 #searched    82992 bmove: d6 f4 bscore: -1839
Depth  3 #searched   950732 bmove: a7 a6 bscore: -1562
Depth  4 #searched  2576149 bmove: a7 a6 bscore: -1570
Search total: 240594891 / 16905 ms / 14232 nodes/ms
hash size r 4 t 2570 
0/33: a7 a6 -1/-1570
1/33: a7 a5 -2/-1570
2/33: e8 h8 0/-1570
3/33: d6 f8 11/-1572
4/33: d7 e7 0/-1573
5/33: e8 g8 0/-1574
6/33: c6 c5 -4/-1576
7/33: d7 c8 -12/-1576
8/33: e8 a8 0/-1576
9/33: e8 c8 0/-1576
10/33: e8 d8 0/-1578
11/33: b7 b6 -2/-1578
12/33: b7 b5 -4/-1580
13/33: d7 e6 6/-1587
14/33: d6 c7 0/-1588
15/33: d6 g3 -102/-1589
16/33: d7 d8 -6/-1590
17/33: e8 b8 0/-1595
18/33: e8 e6 0/-1598
19/33: g5 g4 -2/-1607
20/33: e8 f8 0/-1611
21/33: d6 b8 11/-1612
22/33: d6 e7 0/-1618
23/33: d6 a3 0/-1621
24/33: e8 e7 0/-1621
25/33: d6 b4 0/-1634
26/33: d6 f4 0/-1648
27/33: d7 c7 -6/-1665
28/33: f6 f5 -4/-1725
29/33: e8 e4 -114/-1754
30/33: e8 e5 0/-1771
31/33: d6 c5 0/-1796
32/33: d6 e5 0/-1813

8 ....r...
7 .p.k....
6 p.pb.p..
5 ......p.
4 ...PP...
3 ..NQN.P.
2 PP......
1 R.B..K..
  ABCDEFGH
28. a6
score: -1570, material: 1566, abs: 4292, pawns: 16, mobility: 3
Depth  1 #searched     6719 bmove: g3 g4 bscore: 1966
Depth  2 #searched    83834 bmove: e4 e5 bscore: 1449
Depth  3 #searched  1081159 bmove: f1 g1 bscore: 1842
Search total: 241676050 / 16976 ms / 14236 nodes/ms
hash size r 2 t 627 
0/35: f1 g1 12/1842
1/35: a2 a4 2/1842
2/35: a2 a3 1/1842
3/35: d3 e2 0/1842
4/35: f1 e2 -12/1842
5/35: c3 d1 -10/1842
6/35: f1 g2 0/1841
7/35: f1 f2 -6/1841
8/35: c3 a4 -10/1841
9/35: c3 b1 -10/1841
10/35: d3 c4 0/1840
11/35: b2 b3 2/1839
12/35: a1 b1 0/1839
13/35: c3 e2 -5/1839
14/35: g3 g4 2/1836
15/35: d3 b5 0/1835
16/35: f1 e1 -6/1833
17/35: e3 g2 -5/1825
18/35: d3 b1 0/1825
19/35: e3 d1 -10/1819
20/35: e3 c4 0/1792
21/35: d3 a6 101/1781
22/35: c1 d2 11/1740
23/35: e3 f5 5/1715
24/35: d4 d5 6/1708
25/35: c3 d5 5/1705
26/35: e3 c2 -5/1701
27/35: d3 c2 0/1674
28/35: c3 b5 -5/1664
29/35: d3 d1 0/1635
30/35: e3 g4 -5/1634
31/35: d3 d2 0/1633
32/35: b2 b4 4/1581
33/35: e3 d5 5/1501
34/35: e4 e5 7/1462

8 ....r...
7 .p.k....
6 p.pb.p..
5 ......p.
4 ...PP...
3 ..NQN.P.
2 PP......
1 R.B...K.
  ABCDEFGH
28. Kg1
score: 1842, material: 1578, abs: 4304, pawns: 16, mobility: 5
Depth  1 #searched     5665 bmove: b7 b5 bscore: -576
Depth  2 #searched    96802 bmove: d6 c5 bscore: -1842
Depth  3 #searched   886483 bmove: b7 b5 bscore: -1513
Depth  4 #searched  2660302 bmove: b7 b5 bscore: -1589
Search total: 244336352 / 17147 ms / 14249 nodes/ms
hash size r 3 t 2212 
0/32: b7 b5 -4/-1589
1/32: d7 c8 -12/-1589
2/32: e8 a8 0/-1589
3/32: d6 b4 0/-1590
4/32: e8 b8 0/-1590
5/32: e8 c8 0/-1590
6/32: d6 c7 0/-1590
7/32: d7 e6 6/-1592
8/32: e8 d8 0/-1592
9/32: d6 f8 11/-1594
10/32: d7 e7 0/-1596
11/32: e8 f8 0/-1597
12/32: d6 e7 0/-1598
13/32: d6 g3 -102/-1601
14/32: c6 c5 -4/-1602
15/32: e8 h8 0/-1602
16/32: e8 g8 0/-1604
17/32: d6 b8 11/-1605
18/32: d7 d8 -6/-1605
19/32: d6 e5 0/-1606
20/32: a6 a5 -1/-1609
21/32: e8 e6 0/-1611
22/32: e8 e4 -114/-1612
23/32: d6 c5 0/-1617
24/32: g5 g4 -2/-1620
25/32: b7 b6 -2/-1620
26/32: d6 f4 0/-1627
27/32: d6 a3 0/-1632
28/32: e8 e7 0/-1634
29/32: e8 e5 0/-1635
30/32: d7 c7 -6/-1678
31/32: f6 f5 -4/-1738

8 ....r...
7 ...k....
6 p.pb.p..
5 .p....p.
4 ...PP...
3 ..NQN.P.
2 PP......
1 R.B...K.
  ABCDEFGH
29. b5
score: -1589, material: 1574, abs: 4308, pawns: 16, mobility: 5
Depth  1 #searched     7567 bmove: g3 g4 bscore: 1976
Depth  2 #searched    98033 bmove: e4 e5 bscore: 1461
Depth  3 #searched  1423016 bmove: c3 e2 bscore: 1837
Search total: 245759368 / 17240 ms / 14255 nodes/ms
hash size r 2 t 802 
0/36: c3 e2 -5/1837
1/36: d3 f1 0/1837
2/36: c3 b1 -10/1837
3/36: b2 b4 4/1837
4/36: b2 b3 2/1837
5/36: d3 e2 0/1837
6/36: g1 g2 -12/1837
7/36: g1 f1 -12/1837
8/36: g1 f2 -18/1837
9/36: c3 d1 -10/1836
10/36: a2 a3 1/1836
11/36: a1 b1 0/1835
12/36: c3 a4 -10/1834
13/36: e3 d1 -10/1824
14/36: e3 f1 -10/1821
15/36: d3 b5 104/1818
16/36: c1 d2 11/1769
17/36: c3 b5 99/1759
18/36: d3 c4 0/1739
19/36: e3 g4 -5/1728
20/36: c3 d5 5/1715
21/36: e3 c2 -5/1710
22/36: a2 a4 2/1630
23/36: e3 f5 5/1628
24/36: e3 g2 -5/1620
25/36: e3 c4 0/1614
26/36: e3 d5 5/1511
27/36: d3 b1 0/1492
28/36: d3 c2 0/1491
29/36: d4 d5 6/1490
30/36: d3 d1 0/1488
31/36: d3 d2 0/1480
32/36: e4 e5 7/1473
33/36: g3 g4 2/945
34/36: g1 h2 0/591
35/36: g1 h1 0/591

8 ....r...
7 ...k....
6 p.pb.p..
5 .p....p.
4 ...PP...
3 ...QN.P.
2 PP..N...
1 R.B...K.
  ABCDEFGH
29. Ne2
score: 1837, material: 1569, abs: 4303, pawns: 16, mobility: 2
Depth  1 #searched     5879 bmove: d7 c8 bscore: -773
Depth  2 #searched    82790 bmove: d6 e7 bscore: -1837
Depth  3 #searched   761959 bmove: e8 e4 bscore: -1496
Depth  4 #searched  4150407 bmove: d6 e7 bscore: -1839
Search total: 249909775 / 17512 ms / 14270 nodes/ms
hash size r 3 t 2856 
0/31: d6 e7 0/-1839
1/31: d6 b8 11/-1839
2/31: e8 e4 -114/-1840
3/31: d7 c7 -6/-1840
4/31: d7 d8 -6/-1840
5/31: d6 b4 0/-1840
6/31: d6 f8 11/-1840
7/31: e8 b8 0/-1840
8/31: e8 g8 0/-1840
9/31: d7 c8 -12/-1841
10/31: e8 a8 0/-1841
11/31: e8 f8 0/-1841
12/31: e8 h8 0/-1841
13/31: e8 c8 0/-1842
14/31: b5 b4 -2/-1842
15/31: d7 e7 0/-1842
16/31: e8 d8 0/-1843
17/31: a6 a5 -1/-1844
18/31: d7 e6 6/-1844
19/31: e8 e7 0/-1844
20/31: e8 e6 0/-1845
21/31: d6 g3 -102/-1847
22/31: d6 c7 0/-1849
23/31: d6 e5 0/-1853
24/31: d6 a3 0/-1854
25/31: f6 f5 -4/-1864
26/31: g5 g4 -2/-1867
27/31: d6 c5 0/-1920
28/31: d6 f4 0/-1939
29/31: c6 c5 -4/-1961
30/31: e8 e5 0/-1977

8 ....r...
7 ...kb...
6 p.p..p..
5 .p....p.
4 ...PP...
3 ...QN.P.
2 PP..N...
1 R.B...K.
  ABCDEFGH
30. Be7
score: -1839, material: 1569, abs: 4303, pawns: 16, mobility: 10
Depth  1 #searched     3100 bmove: a2 a3 bscore: 1823
Depth  2 #searched    53900 bmove: d3 d1 bscore: 1583
Depth  3 #searched  1029165 bmove: d3 b3 bscore: 1692
Search total: 250938940 / 17581 ms / 14273 nodes/ms
hash size r 4 t 655 
0/33: d3 b3 0/1692
1/33: d3 c3 0/1692
2/33: g1 h2 0/1692
3/33: a1 b1 0/1692
4/33: g1 f1 -12/1692
5/33: g1 h1 0/1691
6/33: g1 g2 -12/1690
7/33: g1 f2 -18/1688
8/33: e3 f5 5/1641
9/33: e2 c3 5/1635
10/33: a2 a4 2/1632
11/33: b2 b4 4/1632
12/33: e4 e5 7/1632
13/33: b2 b3 2/1628
14/33: c1 d2 11/1627
15/33: a2 a3 1/1627
16/33: d3 c2 0/1624
17/33: d3 d1 0/1623
18/33: d3 c4 0/1623
19/33: d4 d5 6/1622
20/33: d3 d2 0/1622
21/33: d3 b1 0/1617
22/33: e3 d5 5/1614
23/33: e3 c4 0/1614
24/33: g3 g4 2/1611
25/33: e2 f4 5/1608
26/33: e3 g4 -5/1607
27/33: e3 g2 -5/1605
28/33: e3 c2 -5/1603
29/33: e3 d1 -10/1599
30/33: e3 f1 -10/1598
31/33: d3 a3 0/1594
32/33: d3 b5 104/1042

8 ....r...
7 ...kb...
6 p.p..p..
5 .p....p.
4 ...PP...
3 .Q..N.P.
2 PP..N...
1 R.B...K.
  ABCDEFGH
30. Qb3
score: 1692, material: 1569, abs: 4303, pawns: 16, mobility: 12
Depth  1 #searched     3843 bmove: d7 c8 bscore: -781
Depth  2 #searched    57954 bmove: d7 d6 bscore: -1705
Depth  3 #searched   883198 bmove: e8 d8 bscore: -1581
Depth  4 #searched  3997364 bmove: c6 c5 bscore: -1605
Search total: 254936304 / 17845 ms / 14286 nodes/ms
hash size r 5 t 2898 
0/22: c6 c5 -4/-1605
1/22: e8 d8 0/-1606
2/22: b5 b4 -2/-1606
3/22: e8 b8 0/-1606
4/22: e8 a8 0/-1606
5/22: e8 f8 0/-1606
6/22: e8 c8 0/-1606
7/22: e8 h8 0/-1606
8/22: d7 c7 -6/-1606
9/22: d7 d8 -6/-1607
10/22: e7 d8 11/-1607
11/22: e7 d6 0/-1607
12/22: e7 f8 11/-1607
13/22: e7 a3 0/-1607
14/22: f6 f5 -4/-1608
15/22: d7 d6 6/-1608
16/22: a6 a5 -1/-1611
17/22: e8 g8 0/-1616
18/22: g5 g4 -2/-1694
19/22: d7 c8 -12/-1724
20/22: e7 c5 0/-1814
21/22: e7 b4 0/-1820

8 ....r...
7 ...kb...
6 p....p..
5 .pp...p.
4 ...PP...
3 .Q..N.P.
2 PP..N...
1 R.B...K.
  ABCDEFGH
31. c5
score: -1605, material: 1565, abs: 4307, pawns: 6, mobility: 14
Depth  1 #searched    11129 bmove: b3 d5 bscore: 1719
Depth  2 #searched    72044 bmove: b3 d5 bscore: 2082
Depth  3 #searched  1728572 bmove: a2 a3 bscore: 1596
Search total: 256664876 / 17958 ms / 14292 nodes/ms
hash size r 2 t 1045 
0/36: a2 a3 1/1596
1/36: e3 c4 0/1595
2/36: e3 g2 -5/1595
3/36: e3 c2 -5/1595
4/36: d4 c5 108/1593
5/36: g1 g2 -12/1589
6/36: e3 f1 -10/1589
7/36: e3 d1 -10/1589
8/36: d4 d5 6/1589
9/36: b3 c2 0/1588
10/36: a2 a4 2/1588
11/36: c1 d2 11/1588
12/36: e2 f4 5/1588
13/36: b3 g8 0/1588
14/36: b3 b4 0/1587
15/36: e3 d5 5/1587
16/36: b3 f7 0/1587
17/36: e3 f5 5/1586
18/36: e2 c3 5/1586
19/36: g1 f1 -12/1586
20/36: b3 c3 0/1584
21/36: e4 e5 7/1582
22/36: b3 d1 0/1582
23/36: g1 f2 -18/1582
24/36: b3 a4 0/1580
25/36: b3 a3 0/1578
26/36: b3 d3 0/1571
27/36: g3 g4 2/1571
28/36: e3 g4 -5/1567
29/36: a1 b1 0/1567
30/36: g1 h2 0/1563
31/36: g1 h1 0/1563
32/36: b3 d5 0/1487
33/36: b3 b5 104/1142
34/36: b3 e6 0/1043
35/36: b3 c4 0/787

8 ....r...
7 ...kb...
6 p....p..
5 .pp...p.
4 ...PP...
3 PQ..N.P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
31. a3
score: 1596, material: 1566, abs: 4308, pawns: 6, mobility: 14
Depth  1 #searched     4178 bmove: c5 c4 bscore: -1497
Depth  2 #searched   116557 bmove: e8 c8 bscore: -1600
Depth  3 #searched   435336 bmove: c5 c4 bscore: -1355
Depth  4 #searched  1025088 bmove: c5 c4 bscore: -1325
Search total: 257689964 / 18025 ms / 14296 nodes/ms
hash size r 2 t 753 
0/21: c5 c4 -4/-1325
1/21: e8 d8 0/-1352
2/21: d7 c7 -6/-1355
3/21: e8 h8 0/-1362
4/21: d7 c8 -12/-1365
5/21: e8 b8 0/-1369
6/21: e7 d8 11/-1372
7/21: e7 f8 11/-1376
8/21: e8 c8 0/-1383
9/21: c5 d4 -125/-1400
10/21: a6 a5 -1/-1410
11/21: e8 g8 0/-1417
12/21: e7 d6 0/-1435
13/21: g5 g4 -2/-1437
14/21: d7 d6 6/-1462
15/21: e8 f8 0/-1463
16/21: e8 a8 0/-1468
17/21: f6 f5 -4/-1471
18/21: d7 c6 0/-1482
19/21: d7 d8 -6/-1520
20/21: b5 b4 -2/-1684

8 ....r...
7 ...kb...
6 p....p..
5 .p....p.
4 ..pPP...
3 PQ..N.P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
32. c4
score: -1325, material: 1562, abs: 4312, pawns: -8, mobility: 6
Depth  1 #searched     9055 bmove: b3 c3 bscore: 1803
Depth  2 #searched   125008 bmove: b3 d1 bscore: 1338
Depth  3 #searched   797150 bmove: b3 d1 bscore: 1600
Depth  4 #searched  4092331 bmove: e3 c4 bscore: 1470
Search total: 261782295 / 18299 ms / 14305 nodes/ms
hash size r 2 t 2149 
0/31: e3 c4 112/1470
1/31: g1 h2 0/1468
2/31: g1 h1 0/1468
3/31: b3 c2 0/1457
4/31: b3 a2 0/1453
5/31: b3 c3 0/1451
6/31: e3 g2 -5/1450
7/31: e3 c2 -5/1448
8/31: e3 f1 -10/1443
9/31: c1 d2 11/1441
10/31: e3 d1 -10/1441
11/31: b3 b4 0/1428
12/31: g3 g4 2/1423
13/31: a1 b1 0/1420
14/31: a1 a2 0/1419
15/31: g1 g2 -12/1411
16/31: g1 f1 -12/1408
17/31: g1 f2 -18/1404
18/31: d4 d5 6/1397
19/31: e2 f4 5/1393
20/31: b3 b5 104/1391
21/31: b3 d1 0/1386
22/31: b3 d3 0/1381
23/31: e3 d5 5/1345
24/31: e3 g4 -5/1345
25/31: e3 f5 5/967
26/31: a3 a4 1/965
27/31: b3 c4 112/941
28/31: e4 e5 7/885
29/31: e2 c3 5/876
30/31: b3 a4 0/856

8 ....r...
7 ...kb...
6 p....p..
5 .p....p.
4 ..NPP...
3 PQ....P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
32. Nxc4
score: 1470, material: 1674, abs: 4200, pawns: 24, mobility: 9
Depth  1 #searched     4177 bmove: g5 g4 bscore: -912
Depth  2 #searched   143770 bmove: d7 e6 bscore: -1719
Depth  3 #searched   702406 bmove: d7 c7 bscore: -1495
Depth  4 #searched  2815549 bmove: d7 c7 bscore: -1499
Search total: 264597844 / 18477 ms / 14320 nodes/ms
hash size r 2 t 1873 
0/23: d7 c7 -6/-1499
1/23: e8 b8 0/-1499
2/23: e8 h8 0/-1499
3/23: e7 d8 11/-1499
4/23: e8 c8 0/-1499
5/23: e8 g8 0/-1499
6/23: d7 d8 -6/-1500
7/23: e8 d8 0/-1501
8/23: e7 f8 11/-1502
9/23: d7 e6 6/-1504
10/23: d7 c6 0/-1506
11/23: e7 d6 0/-1517
12/23: a6 a5 -1/-1547
13/23: e7 a3 -101/-1563
14/23: b5 b4 -2/-1567
15/23: d7 c8 -12/-1575
16/23: e8 f8 0/-1578
17/23: e8 a8 0/-1584
18/23: e7 c5 0/-1588
19/23: b5 c4 -333/-1590
20/23: f6 f5 -4/-1593
21/23: e7 b4 0/-1675
22/23: g5 g4 -2/-1681

8 ....r...
7 ..k.b...
6 p....p..
5 .p....p.
4 ..NPP...
3 PQ....P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
33. Kc7
score: -1499, material: 1668, abs: 4206, pawns: 24, mobility: 7
Depth  1 #searched    11628 bmove: c4 e3 bscore: 1935
Depth  2 #searched   151458 bmove: c4 e5 bscore: 1470
Depth  3 #searched  1763881 bmove: c4 e3 bscore: 1739
Search total: 266361725 / 18592 ms / 14326 nodes/ms
hash size r 3 t 740 
0/33: c4 e3 0/1739
1/33: c4 e5 5/1735
2/33: c4 b6 -5/1735
3/33: e2 c3 5/1731
4/33: c1 d2 11/1731
5/33: c1 g5 115/1731
6/33: e2 f4 5/1730
7/33: c4 a5 -10/1729
8/33: a3 a4 1/1728
9/33: b3 c3 0/1727
10/33: b3 c2 0/1727
11/33: c4 d2 -5/1727
12/33: e4 e5 7/1724
13/33: b3 f3 0/1723
14/33: b3 d3 0/1709
15/33: b3 e3 0/1709
16/33: b3 b4 0/1705
17/33: c1 e3 11/1705
18/33: b3 d1 0/1691
19/33: a1 a2 0/1690
20/33: b3 a2 0/1690
21/33: a1 b1 0/1688
22/33: g1 g2 -12/1681
23/33: g1 f1 -12/1678
24/33: g1 f2 -18/1674
25/33: d4 d5 6/1628
26/33: g3 g4 2/1618
27/33: c1 f4 11/1601
28/33: c4 d6 5/1487
29/33: g1 h2 0/1487
30/33: g1 h1 0/1487
31/33: b3 b5 104/1284
32/33: b3 a4 0/1198

8 ....r...
7 ..k.b...
6 p....p..
5 .p....p.
4 ...PP...
3 PQ..N.P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
33. Ne3
score: 1739, material: 1668, abs: 4206, pawns: 24, mobility: 10
Depth  1 #searched     3755 bmove: c7 b8 bscore: -892
Depth  2 #searched    47881 bmove: c7 d6 bscore: -1817
Depth  3 #searched   437920 bmove: c7 d6 bscore: -1710
Depth  4 #searched  2008939 bmove: c7 d6 bscore: -1732
Search total: 268370664 / 18721 ms / 14335 nodes/ms
hash size r 4 t 1687 
0/25: c7 d6 12/-1732
1/25: e8 b8 0/-1732
2/25: c7 d7 6/-1732
3/25: e8 h8 0/-1733
4/25: e8 c8 0/-1733
5/25: e8 a8 0/-1733
6/25: c7 c6 6/-1733
7/25: c7 b6 0/-1735
8/25: c7 d8 0/-1735
9/25: e8 d8 0/-1735
10/25: a6 a5 -1/-1736
11/25: e8 f8 0/-1739
12/25: e7 d8 11/-1739
13/25: e7 b4 0/-1786
14/25: c7 c8 -6/-1792
15/25: e7 a3 -101/-1804
16/25: g5 g4 -2/-1819
17/25: b5 b4 -2/-1823
18/25: f6 f5 -4/-1834
19/25: e7 c5 0/-1834
20/25: e7 d6 0/-1883
21/25: e7 f8 11/-1890
22/25: c7 b8 -18/-1924
23/25: e8 g8 0/-1957
24/25: c7 b7 -6/-1957

8 ....r...
7 ....b...
6 p..k.p..
5 .p....p.
4 ...PP...
3 PQ..N.P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
34. Kd6
score: -1732, material: 1680, abs: 4206, pawns: 24, mobility: 15
Depth  1 #searched     7798 bmove: c1 d2 bscore: 1739
Depth  2 #searched    83137 bmove: b3 d5 bscore: 1954
Depth  3 #searched  1140189 bmove: b3 f7 bscore: 1940
Search total: 269510853 / 18795 ms / 14339 nodes/ms
hash size r 5 t 972 
0/35: b3 f7 0/1940
1/35: b3 b5 104/1936
2/35: e3 f5 5/1829
3/35: e2 c3 5/1817
4/35: e3 g2 -5/1779
5/35: e3 d5 5/1756
6/35: c1 d2 11/1750
7/35: d4 d5 6/1747
8/35: e2 f4 5/1742
9/35: e3 g4 -5/1737
10/35: a3 a4 1/1737
11/35: b3 g8 0/1737
12/35: g3 g4 2/1735
13/35: b3 c3 0/1733
14/35: b3 c2 0/1732
15/35: b3 c4 0/1732
16/35: a1 b1 0/1731
17/35: b3 d3 0/1729
18/35: a1 a2 0/1728
19/35: b3 d1 0/1727
20/35: b3 a2 0/1727
21/35: e3 f1 -10/1726
22/35: e3 c2 -5/1725
23/35: g1 g2 -12/1723
24/35: e3 d1 -10/1718
25/35: g1 f1 -12/1718
26/35: g1 f2 -18/1716
27/35: e4 e5 7/1698
28/35: b3 b4 0/1693
29/35: b3 d5 0/1684
30/35: e3 c4 0/1673
31/35: b3 e6 0/1092
32/35: g1 h2 0/930
33/35: g1 h1 0/930
34/35: b3 a4 0/856

8 ....r...
7 ....bQ..
6 p..k.p..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
34. Qf7
score: 1940, material: 1680, abs: 4206, pawns: 24, mobility: 18
Depth  1 #searched     2194 bmove: e8 h8 bscore: -1147
Depth  2 #searched    52862 bmove: d6 d7 bscore: -1956
Depth  3 #searched   222394 bmove: d6 d7 bscore: -1697
Depth  4 #searched   499059 bmove: d6 d7 bscore: -1817
Depth  5 #searched  1152667 bmove: d6 d7 bscore: -1717
Search total: 270663520 / 18868 ms / 14345 nodes/ms
hash size r 6 t 1331 
0/16: d6 d7 -6/-1717
1/16: d6 c7 -12/-1725
2/16: b5 b4 -2/-1739
3/16: g5 g4 -2/-1745
4/16: e7 d8 11/-1751
5/16: a6 a5 -1/-1755
6/16: e8 d8 0/-1780
7/16: e8 a8 0/-1782
8/16: e8 b8 0/-1791
9/16: e8 c8 0/-1792
10/16: e7 f8 11/-1793
11/16: d6 c6 -6/-1797
12/16: e8 h8 0/-1799
13/16: e8 g8 0/-1807
14/16: f6 f5 -4/-1905
15/16: e8 f8 0/-1955

8 ....r...
7 ...kbQ..
6 p....p..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
35. Kd7
score: -1717, material: 1674, abs: 4200, pawns: 24, mobility: 15
Depth  1 #searched     8792 bmove: f7 g6 bscore: 1952
Depth  2 #searched    78780 bmove: f7 d5 bscore: 1954
Depth  3 #searched   809944 bmove: f7 g6 bscore: 1957
Depth  4 #searched 11359441 bmove: f7 d5 bscore: 1946
Search total: 282022961 / 19609 ms / 14382 nodes/ms
hash size r 7 t 9405 
0/38: f7 d5 0/1946
1/38: g1 h2 0/1946
2/38: g1 h1 0/1946
3/38: e3 g4 -5/1944
4/38: g1 f2 -18/1944
5/38: f7 f8 0/1944
6/38: f7 b3 0/1944
7/38: f7 h7 0/1943
8/38: b2 b3 2/1942
9/38: a3 a4 1/1940
10/38: e4 e5 7/1939
11/38: d4 d5 6/1891
12/38: f7 g7 0/1879
13/38: g1 f1 -12/1818
14/38: a1 b1 0/1721
15/38: e3 f1 -10/1711
16/38: g1 g2 -12/1709
17/38: e3 c4 0/1709
18/38: e3 d1 -10/1703
19/38: f7 e8 500/1699
20/38: f7 e7 350/1688
21/38: c1 d2 11/1676
22/38: e3 c2 -5/1651
23/38: e3 f5 5/1649
24/38: a1 a2 0/1645
25/38: g3 g4 2/1578
26/38: b2 b4 4/1568
27/38: e2 f4 5/1537
28/38: f7 h5 0/1517
29/38: e3 g2 -5/1501
30/38: e2 c3 5/1498
31/38: f7 a2 0/1485
32/38: e3 d5 5/1443
33/38: f7 e6 0/1313
34/38: f7 g6 0/1137
35/38: f7 f6 104/1062
36/38: f7 g8 0/961
37/38: f7 c4 0/941

8 ....r...
7 ...kb...
6 p....p..
5 .p.Q..p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
35. Qd5+
score: 1946, material: 1674, abs: 4200, pawns: 24, mobility: 15
Depth  1 #searched      784 bmove: d7 c7 bscore: -907
Depth  2 #searched    16911 bmove: d7 c7 bscore: -1965
Depth  3 #searched   135369 bmove: d7 c7 bscore: -1684
Depth  4 #searched   439616 bmove: d7 c7 bscore: -1817
Depth  5 #searched   984314 bmove: d7 c7 bscore: -1717
Depth  6 #searched  6494385 bmove: d7 c7 bscore: -1858
Search total: 288517346 / 20024 ms / 14408 nodes/ms
hash size r 8 t 4906 
0/3: d7 c7 -6/-1858
1/3: d7 c8 -12/-1858
2/3: e7 d6 0/-1862

8 ....r...
7 ..k.b...
6 p....p..
5 .p.Q..p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 R.B...K.
  ABCDEFGH
36. Kc7
score: -1858, material: 1668, abs: 4206, pawns: 24, mobility: 14
Depth  1 #searched    13327 bmove: b2 b3 bscore: 1965
Depth  2 #searched   185338 bmove: d5 a2 bscore: 1684
Depth  3 #searched  1346020 bmove: c1 d2 bscore: 1961
Search total: 289863366 / 20112 ms / 14412 nodes/ms
hash size r 9 t 1152 
0/39: c1 d2 11/1961
1/39: g1 h1 0/1961
2/39: a1 b1 0/1961
3/39: g1 g2 -12/1961
4/39: d5 e6 0/1960
5/39: e3 c4 0/1960
6/39: e2 c3 5/1960
7/39: e2 f4 5/1960
8/39: g1 f1 -12/1960
9/39: b2 b3 2/1960
10/39: a1 a2 0/1959
11/39: e3 c2 -5/1957
12/39: e3 g2 -5/1957
13/39: e3 f5 5/1956
14/39: d5 g8 0/1956
15/39: b2 b4 4/1956
16/39: g1 f2 -18/1956
17/39: d5 f7 0/1955
18/39: g3 g4 2/1955
19/39: e3 g4 -5/1952
20/39: d5 f5 0/1943
21/39: e3 d1 -10/1934
22/39: e3 f1 -10/1925
23/39: d5 g5 104/1861
24/39: a3 a4 1/1836
25/39: d5 a2 0/1817
26/39: d5 b3 0/1817
27/39: e4 e5 7/1814
28/39: d5 d8 0/1304
29/39: d5 d6 0/1304
30/39: d5 e5 0/1298
31/39: d5 c5 0/1198
32/39: d5 b5 104/1173
33/39: d5 a8 0/1059
34/39: d5 c6 0/956
35/39: d5 d7 0/956
36/39: d5 c4 0/941
37/39: g1 h2 0/917
38/39: d5 b7 0/820

8 ....r...
7 ..k.b...
6 p....p..
5 .p.Q..p.
4 ...PP...
3 P...N.P.
2 .P.BN...
1 R.....K.
  ABCDEFGH
36. Bd2
score: 1961, material: 1679, abs: 4217, pawns: 24, mobility: 23
Depth  1 #searched     3876 bmove: c7 b8 bscore: -914
Depth  2 #searched    51855 bmove: c7 b8 bscore: -1956
Depth  3 #searched   551353 bmove: c7 b8 bscore: -1697
Depth  4 #searched  4833194 bmove: c7 b8 bscore: -1856
Search total: 294696560 / 20405 ms / 14442 nodes/ms
hash size r 10 t 3476 
0/20: c7 b8 -18/-1856
1/20: e7 d6 0/-1856
2/20: c7 c8 -6/-1857
3/20: c7 b6 0/-1857
4/20: g5 g4 -2/-1859
5/20: e7 a3 -101/-1860
6/20: e7 b4 0/-1863
7/20: f6 f5 -4/-1866
8/20: e8 d8 0/-1878
9/20: e8 b8 0/-1898
10/20: e8 h8 0/-1922
11/20: e8 f8 0/-1928
12/20: e7 d8 11/-1929
13/20: e7 f8 11/-1963
14/20: e8 c8 0/-1965
15/20: b5 b4 -2/-1966
16/20: a6 a5 -1/-1991
17/20: e7 c5 0/-2044
18/20: e8 g8 0/-2086
19/20: e8 a8 0/-2239

8 .k..r...
7 ....b...
6 p....p..
5 .p.Q..p.
4 ...PP...
3 P...N.P.
2 .P.BN...
1 R.....K.
  ABCDEFGH
37. Kb8
score: -1856, material: 1661, abs: 4235, pawns: 24, mobility: 28
Depth  1 #searched    15474 bmove: d5 e6 bscore: 1959
Depth  2 #searched   142184 bmove: d5 c6 bscore: 1817
Depth  3 #searched  1162984 bmove: d5 d7 bscore: 2063
Search total: 295859544 / 20478 ms / 14447 nodes/ms
hash size r 11 t 792 
0/48: d5 d7 0/2063
1/48: d5 e6 0/2063
2/48: e3 f1 -10/2061
3/48: a1 c1 0/2060
4/48: e3 d1 -10/2060
5/48: d5 c6 0/2049
6/48: e4 e5 7/2015
7/48: e3 f5 5/1999
8/48: e2 f4 5/1997
9/48: e2 c3 5/1993
10/48: b2 b4 4/1992
11/48: d5 f7 0/1991
12/48: g3 g4 2/1991
13/48: d5 f5 0/1990
14/48: g1 h2 0/1989
15/48: a1 a2 0/1989
16/48: e3 g4 -5/1987
17/48: g1 h1 0/1987
18/48: e3 c4 0/1987
19/48: e3 c2 -5/1986
20/48: b2 b3 2/1985
21/48: d2 c3 0/1985
22/48: e2 c1 -5/1984
23/48: d5 a2 0/1983
24/48: d5 b3 0/1983
25/48: d5 c4 0/1983
26/48: e3 g2 -5/1983
27/48: a1 b1 0/1980
28/48: a1 f1 0/1979
29/48: d2 e1 -11/1978
30/48: d2 a5 0/1977
31/48: g1 g2 -12/1977
32/48: g1 f1 -12/1976
33/48: a1 e1 0/1975
34/48: a1 d1 0/1975
35/48: d5 g8 0/1974
36/48: d2 c1 -11/1973
37/48: g1 f2 -18/1972
38/48: d2 b4 0/1966
39/48: d5 c5 0/1963
40/48: a3 a4 1/1848
41/48: d5 g5 104/1177
42/48: d5 b5 104/1037
43/48: d5 d6 0/1037
44/48: d5 e5 0/934
45/48: d5 d8 0/848
46/48: d5 b7 0/840
47/48: d5 a8 0/830

8 .k..r...
7 ...Qb...
6 p....p..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P.BN...
1 R.....K.
  ABCDEFGH
37. Qd7
score: 2063, material: 1661, abs: 4235, pawns: 24, mobility: 28
Depth  1 #searched     2235 bmove: e8 h8 bscore: -2063
Depth  2 #searched    37021 bmove: e8 h8 bscore: -2063
Depth  3 #searched   112034 bmove: e8 h8 bscore: -1263
Depth  4 #searched  7016861 bmove: e8 f8 bscore: -2349
Search total: 302876405 / 20914 ms / 14481 nodes/ms
hash size r 12 t 6108 
0/16: e8 f8 0/-2349
1/16: e8 d8 0/-2359
2/16: e8 g8 0/-2366
3/16: e7 c5 0/-2373
4/16: e7 d6 0/-2419
5/16: e8 c8 0/-2441
6/16: e7 a3 -101/-2453
7/16: e8 h8 0/-2470
8/16: e7 b4 0/-2549
9/16: e7 d8 11/-2574
10/16: e7 f8 11/-2673
11/16: b5 b4 -2/-2673
12/16: g5 g4 -2/-2695
13/16: a6 a5 -1/-2695
14/16: b8 a8 0/-2696
15/16: f6 f5 -4/-2698

8 .k...r..
7 ...Qb...
6 p....p..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P.BN...
1 R.....K.
  ABCDEFGH
38. Rf8
score: -2349, material: 1661, abs: 4235, pawns: 24, mobility: 28
Depth  1 #searched    16810 bmove: d7 e7 bscore: 2069
Depth  2 #searched   127927 bmove: d7 e7 bscore: 2184
Depth  3 #searched  1396765 bmove: d7 e7 bscore: 2184
Search total: 304273170 / 21004 ms / 14486 nodes/ms
hash size r 13 t 1047 
0/48: d7 e7 350/2184
1/48: e3 d5 5/2184
2/48: e3 f5 5/2184
3/48: d7 b5 104/2181
4/48: d7 c8 0/2181
5/48: d2 b4 0/2175
6/48: e4 e5 7/2103
7/48: d4 d5 6/2096
8/48: d7 d5 0/2093
9/48: d7 f5 0/2090
10/48: e2 f4 5/2085
11/48: d7 h3 0/2084
12/48: d7 g4 0/2084
13/48: e2 c3 5/2082
14/48: d2 a5 0/2079
15/48: a3 a4 1/2079
16/48: e3 g4 -5/2078
17/48: a1 f1 0/2078
18/48: g3 g4 2/2078
19/48: b2 b3 2/2077
20/48: d7 e6 0/2076
21/48: d7 c6 0/2076
22/48: b2 b4 4/2076
23/48: e3 c4 0/2075
24/48: a1 b1 0/2075
25/48: a1 c1 0/2074
26/48: d2 c3 0/2074
27/48: a1 e1 0/2074
28/48: a1 d1 0/2074
29/48: a1 a2 0/2071
30/48: e3 c2 -5/2069
31/48: e3 g2 -5/2069
32/48: e2 c1 -5/2067
33/48: g1 g2 -12/2067
34/48: d2 e1 -11/2063
35/48: g1 f1 -12/2062
36/48: e3 f1 -10/2061
37/48: e3 d1 -10/2061
38/48: g1 f2 -18/2060
39/48: d2 c1 -11/2056
40/48: g1 h1 0/1709
41/48: g1 h2 0/1708
42/48: d7 c7 0/1336
43/48: d7 d8 0/1336
44/48: d7 b7 0/1174
45/48: d7 a7 0/1169
46/48: d7 d6 0/1041
47/48: d7 e8 0/836

8 .k...r..
7 ....Q...
6 p....p..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P.BN...
1 R.....K.
  ABCDEFGH
38. Qxe7
score: 2184, material: 2011, abs: 3885, pawns: 24, mobility: 34
Depth  1 #searched     1141 bmove: f8 h8 bscore: -2184
Depth  2 #searched    22595 bmove: f8 h8 bscore: -2184
Depth  3 #searched   219738 bmove: f8 g8 bscore: -2224
Depth  4 #searched  2150211 bmove: f8 h8 bscore: -2313
Search total: 306423381 / 21140 ms / 14494 nodes/ms
hash size r 2 t 2271 
0/12: f8 h8 0/-2313
1/12: f8 c8 0/-2400
2/12: f8 g8 0/-2444
3/12: f8 e8 0/-2579
4/12: f8 f7 0/-2676
5/12: f6 f5 -4/-2697
6/12: b5 b4 -2/-2783
7/12: a6 a5 -1/-2783
8/12: g5 g4 -2/-2795
9/12: f8 d8 0/-2796
10/12: b8 a8 0/-2796
11/12: b8 c8 12/-2801

8 .k.....r
7 ....Q...
6 p....p..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P.BN...
1 R.....K.
  ABCDEFGH
39. Rh8
score: -2313, material: 2011, abs: 3885, pawns: 24, mobility: 28
Depth  1 #searched    12997 bmove: e7 f6 bscore: 2184
Depth  2 #searched    82844 bmove: e7 f6 bscore: 2290
Depth  3 #searched   861636 bmove: e7 f6 bscore: 2290
Depth  4 #searched  9247847 bmove: a1 f1 bscore: 2291
Search total: 315671228 / 21728 ms / 14528 nodes/ms
hash size r 3 t 9855 
0/47: a1 f1 0/2291
1/47: e7 f7 0/2291
2/47: e3 f5 5/2291
3/47: e7 g7 0/2291
4/47: e7 b4 0/2291
5/47: e7 c5 0/2291
6/47: e3 d1 -10/2291
7/47: e7 d6 0/2290
8/47: e3 c4 0/2290
9/47: d2 e1 -11/2290
10/47: e7 f6 104/2289
11/47: g1 f1 -12/2289
12/47: e3 g4 -5/2288
13/47: e3 f1 -10/2288
14/47: e3 d5 5/2288
15/47: e3 g2 -5/2288
16/47: e3 c2 -5/2288
17/47: g1 f2 -18/2287
18/47: d2 c1 -11/2287
19/47: e7 c7 0/2286
20/47: e7 d7 0/2286
21/47: e4 e5 7/2251
22/47: d4 d5 6/2251
23/47: e2 f4 5/2240
24/47: e2 c3 5/2238
25/47: d2 a5 0/2236
26/47: a3 a4 1/2234
27/47: g3 g4 2/2233
28/47: a1 c1 0/2233
29/47: b2 b3 2/2232
30/47: d2 b4 0/2231
31/47: b2 b4 4/2231
32/47: a1 b1 0/2230
33/47: e7 e6 0/2229
34/47: d2 c3 0/2229
35/47: a1 e1 0/2229
36/47: a1 d1 0/2229
37/47: a1 a2 0/2226
38/47: e2 c1 -5/2222
39/47: g1 g2 -12/2222
40/47: e7 h7 0/2189
41/47: e7 d8 0/1707
42/47: e7 f8 0/1686
43/47: e7 e8 0/1655
44/47: e7 b7 0/1439
45/47: e7 a7 0/1396
46/47: e7 e5 0/1382

8 .k.....r
7 ....Q...
6 p....p..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P.BN...
1 .....RK.
  ABCDEFGH
39. Rf1
score: 2291, material: 2011, abs: 3885, pawns: 24, mobility: 30
Depth  1 #searched     2583 bmove: b5 b4 bscore: -2183
Depth  2 #searched    63613 bmove: b8 a8 bscore: -2188
Depth  3 #searched   419668 bmove: b5 b4 bscore: -2295
Depth  4 #searched   608233 bmove: b5 b4 bscore: -2292
Depth  5 #searched 14370017 bmove: a6 a5 bscore: -2400
Search total: 330041245 / 22601 ms / 14602 nodes/ms
hash size r 4 t 14380 
0/18: a6 a5 -1/-2400
1/18: g5 g4 -2/-2400
2/18: h8 h5 0/-2400
3/18: h8 g8 0/-2402
4/18: h8 c8 0/-2402
5/18: b5 b4 -2/-2403
6/18: h8 h3 0/-2405
7/18: b8 c8 12/-2406
8/18: b8 a8 0/-2409
9/18: h8 e8 0/-2418
10/18: h8 d8 0/-2419
11/18: h8 f8 0/-2422
12/18: f6 f5 -4/-2457
13/18: h8 h4 0/-2663
14/18: h8 h6 0/-2669
15/18: h8 h2 -22/-2696
16/18: h8 h7 0/-2698
17/18: h8 h1 0/-2711

8 .k.....r
7 ....Q...
6 .....p..
5 pp....p.
4 ...PP...
3 P...N.P.
2 .P.BN...
1 .....RK.
  ABCDEFGH
40. a5
score: -2400, material: 2010, abs: 3886, pawns: 24, mobility: 30
Depth  1 #searched    15049 bmove: f1 f6 bscore: 2186
Depth  2 #searched    90704 bmove: d2 a5 bscore: 2297
Depth  3 #searched  1026797 bmove: d2 a5 bscore: 2398
Search total: 331068042 / 22662 ms / 14608 nodes/ms
hash size r 2 t 939 
0/49: d2 a5 102/2398
1/49: e7 f6 104/2317
2/49: e7 b4 0/2317
3/49: e7 e5 0/2313
4/49: e7 d6 0/2313
5/49: e7 c5 0/2313
6/49: f1 f6 104/2311
7/49: e7 e6 0/2308
8/49: e7 d7 0/2303
9/49: e3 g4 -5/2261
10/49: e4 e5 7/2236
11/49: d4 d5 6/2225
12/49: a3 a4 1/2214
13/49: e2 c3 5/2212
14/49: b2 b4 4/2211
15/49: g3 g4 2/2210
16/49: b2 b3 2/2209
17/49: e2 f4 5/2209
18/49: e3 d5 5/2208
19/49: f1 f5 0/2208
20/49: f1 f2 0/2208
21/49: f1 f3 0/2208
22/49: f1 f4 0/2208
23/49: d2 b4 0/2208
24/49: e7 g7 0/2208
25/49: e7 f7 0/2207
26/49: e3 f5 5/2205
27/49: e3 c4 0/2205
28/49: d2 c3 0/2204
29/49: d2 e1 -11/2204
30/49: f1 a1 0/2203
31/49: f1 b1 0/2202
32/49: f1 d1 0/2202
33/49: e2 c1 -5/2202
34/49: f1 e1 0/2201
35/49: e3 g2 -5/2201
36/49: e3 c2 -5/2200
37/49: f1 c1 0/2199
38/49: g1 g2 -12/2198
39/49: e7 h7 0/2196
40/49: e3 d1 -10/2196
41/49: d2 c1 -11/2191
42/49: g1 f2 -18/2183
43/49: e7 f8 0/1798
44/49: e7 c7 0/1439
45/49: e7 d8 0/1438
46/49: e7 b7 0/1406
47/49: e7 e8 0/1404
48/49: e7 a7 0/1395

8 .k.....r
7 ....Q...
6 .....p..
5 Bp....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 .....RK.
  ABCDEFGH
40. Bxa5
score: 2398, material: 2112, abs: 3784, pawns: 36, mobility: 33
Depth  1 #searched     2362 bmove: b5 b4 bscore: -2297
Depth  2 #searched    84947 bmove: g5 g4 bscore: -2332
Depth  3 #searched   372140 bmove: b8 a8 bscore: -2329
Depth  4 #searched   900508 bmove: b8 a8 bscore: -2331
Depth  5 #searched  1535968 bmove: b8 a8 bscore: -2331
Search total: 332604010 / 22750 ms / 14619 nodes/ms
hash size r 2 t 2042 
0/17: b8 a8 0/-2331
1/17: h8 c8 0/-2332
2/17: g5 g4 -2/-2390
3/17: h8 g8 0/-2392
4/17: b5 b4 -2/-2396
5/17: f6 f5 -4/-2475
6/17: h8 h3 0/-2477
7/17: h8 h6 0/-2583
8/17: h8 h5 0/-2584
9/17: h8 h4 0/-2665
10/17: h8 h7 0/-2810
11/17: h8 h2 -22/-2813
12/17: h8 h1 0/-2830
13/17: h8 f8 0/-2847
14/17: h8 d8 0/-2897
15/17: h8 e8 0/-2902
16/17: b8 c8 12/-9998

8 k......r
7 ....Q...
6 .....p..
5 Bp....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 .....RK.
  ABCDEFGH
41. Ka8
score: -2331, material: 2112, abs: 3784, pawns: 36, mobility: 34
Depth  1 #searched     9941 bmove: a5 d8 bscore: 2337
Depth  2 #searched    67331 bmove: d4 d5 bscore: 2318
Depth  3 #searched  1500185 bmove: e7 f6 bscore: 2409
Search total: 334104195 / 22837 ms / 14629 nodes/ms
hash size r 3 t 1350 
0/51: e7 f6 104/2409
1/51: f1 f6 104/2404
2/51: g1 f2 -18/2372
3/51: a5 b6 0/2334
4/51: f1 b1 0/2334
5/51: f1 e1 0/2334
6/51: e2 c1 -5/2330
7/51: e3 c2 -5/2329
8/51: g1 g2 -12/2328
9/51: e3 g2 -5/2328
10/51: d4 d5 6/2320
11/51: a5 c7 0/2320
12/51: e7 g7 0/2319
13/51: e3 d1 -10/2319
14/51: e7 e6 0/2317
15/51: e7 d7 0/2315
16/51: e7 c5 0/2315
17/51: e7 d6 0/2315
18/51: e7 b4 0/2315
19/51: e7 c7 0/2314
20/51: f1 f4 0/2313
21/51: e7 f7 0/2310
22/51: b2 b4 4/2308
23/51: a5 d2 0/2307
24/51: a5 b4 0/2307
25/51: e2 c3 5/2305
26/51: a5 c3 0/2305
27/51: a5 d8 0/2305
28/51: f1 f3 0/2302
29/51: e3 d5 5/2301
30/51: f1 c1 0/2301
31/51: g3 g4 2/2301
32/51: f1 f5 0/2299
33/51: f1 f2 0/2299
34/51: e7 e5 0/2298
35/51: e3 g4 -5/2297
36/51: f1 d1 0/2296
37/51: f1 a1 0/2296
38/51: e3 c4 0/2296
39/51: a5 e1 -11/2296
40/51: b2 b3 2/2295
41/51: e3 f5 5/2295
42/51: a3 a4 1/2294
43/51: e7 h7 0/2209
44/51: e2 f4 5/2199
45/51: e4 e5 7/2188
46/51: e7 f8 0/1909
47/51: e7 b7 0/1908
48/51: e7 d8 0/1906
49/51: e7 e8 0/1423
50/51: e7 a7 0/1421

8 k......r
7 ........
6 .....Q..
5 Bp....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 .....RK.
  ABCDEFGH
41. Qxf6
score: 2409, material: 2216, abs: 3680, pawns: 48, mobility: 37
Depth  1 #searched     3060 bmove: h8 h5 bscore: -1497
Depth  2 #searched    23495 bmove: h8 h5 bscore: -2409
Depth  3 #searched   294445 bmove: h8 g8 bscore: -2422
Depth  4 #searched   451389 bmove: h8 g8 bscore: -2416
Depth  5 #searched   868078 bmove: h8 g8 bscore: -2471
Depth  6 #searched  1262602 bmove: h8 g8 bscore: -2471
Search total: 335366797 / 22907 ms / 14640 nodes/ms
hash size r 2 t 1951 
0/18: h8 g8 0/-2471
1/18: h8 h3 0/-2503
2/18: a8 b7 12/-2521
3/18: a8 a7 0/-2521
4/18: h8 c8 0/-2523
5/18: h8 h7 0/-2526
6/18: h8 f8 0/-2542
7/18: a8 b8 0/-2547
8/18: h8 h5 0/-2583
9/18: g5 g4 -2/-2602
10/18: h8 h4 0/-2841
11/18: h8 h2 -22/-2919
12/18: h8 h1 0/-2927
13/18: h8 h6 0/-2943
14/18: b5 b4 -2/-2944
15/18: h8 d8 0/-2954
16/18: h8 e8 0/-3019
17/18: h8 b8 0/-9998

8 k.....r.
7 ........
6 .....Q..
5 Bp....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 .....RK.
  ABCDEFGH
42. Rg8
score: -2471, material: 2216, abs: 3680, pawns: 48, mobility: 42
Depth  1 #searched     7967 bmove: a5 d8 bscore: 2457
Depth  2 #searched    53745 bmove: f6 e5 bscore: 2407
Depth  3 #searched  1409714 bmove: a5 d8 bscore: 2476
Search total: 336776511 / 22989 ms / 14649 nodes/ms
hash size r 3 t 1360 
0/55: a5 d8 0/2476
1/55: e4 e5 7/2475
2/55: d4 d5 6/2473
3/55: a5 b6 0/2472
4/55: a5 c7 0/2464
5/55: e3 d5 5/2458
6/55: f6 h8 0/2457
7/55: b2 b4 4/2457
8/55: f6 e7 0/2457
9/55: g3 g4 2/2456
10/55: g1 h2 0/2456
11/55: a5 b4 0/2456
12/55: f6 a6 0/2455
13/55: f6 d6 0/2455
14/55: f1 c1 0/2455
15/55: b2 b3 2/2455
16/55: g1 h1 0/2455
17/55: f1 f2 0/2452
18/55: f1 f3 0/2451
19/55: f1 d1 0/2450
20/55: f1 a1 0/2450
21/55: f1 b1 0/2449
22/55: f1 e1 0/2449
23/55: e3 g4 -5/2448
24/55: e2 c1 -5/2447
25/55: e3 c2 -5/2446
26/55: e3 g2 -5/2445
27/55: g1 g2 -12/2445
28/55: e3 d1 -10/2436
29/55: f1 f4 0/2435
30/55: g1 f2 -18/2431
31/55: f6 c6 0/2429
32/55: f6 h6 0/2422
33/55: a5 d2 0/2421
34/55: a5 c3 0/2419
35/55: e3 c4 0/2418
36/55: e2 f4 5/2414
37/55: e2 c3 5/2413
38/55: e3 f5 5/2413
39/55: f1 f5 0/2411
40/55: f6 f5 0/2410
41/55: f6 f4 0/2410
42/55: f6 e6 0/2409
43/55: f6 f7 0/2409
44/55: f6 e5 0/2407
45/55: f6 b6 0/2405
46/55: a5 e1 -11/2405
47/55: f6 f3 0/2314
48/55: a3 a4 1/2313
49/55: f6 f2 0/2307
50/55: f6 d8 0/2013
51/55: f6 f8 0/1944
52/55: f6 g5 104/1928
53/55: f6 g6 0/1415
54/55: f6 g7 0/1410

8 k..B..r.
7 ........
6 .....Q..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 .....RK.
  ABCDEFGH
42. Bd8
score: 2476, material: 2216, abs: 3680, pawns: 48, mobility: 40
Depth  1 #searched     1259 bmove: a8 b8 bscore: -2004
Depth  2 #searched    14368 bmove: a8 b8 bscore: -2475
Depth  3 #searched   137289 bmove: a8 b8 bscore: -2389
Depth  4 #searched   306770 bmove: a8 b8 bscore: -2472
Depth  5 #searched  1052941 bmove: a8 b8 bscore: -2424
Search total: 337829452 / 23050 ms / 14656 nodes/ms
hash size r 4 t 1207 
0/11: a8 b8 0/-2424
1/11: a8 b7 12/-2424
2/11: a8 a7 0/-2424
3/11: g8 e8 0/-2424
4/11: b5 b4 -2/-2429
5/11: g5 g4 -2/-2482
6/11: g8 d8 -350/-2603
7/11: g8 h8 0/-2911
8/11: g8 f8 0/-2912
9/11: g8 g6 0/-2916
10/11: g8 g7 0/-2916

8 .k.B..r.
7 ........
6 .....Q..
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 .....RK.
  ABCDEFGH
43. Kb8
score: -2424, material: 2216, abs: 3680, pawns: 48, mobility: 38
Depth  1 #searched    12599 bmove: f6 e5 bscore: 2564
Depth  2 #searched   129326 bmove: f6 e5 bscore: 2564
Depth  3 #searched  1135385 bmove: f6 d6 bscore: 2668
Search total: 338964837 / 23118 ms / 14662 nodes/ms
hash size r 5 t 858 
0/51: f6 d6 0/2668
1/51: f6 b6 0/2668
2/51: f6 f5 0/2597
3/51: e4 e5 7/2590
4/51: d4 d5 6/2590
5/51: d8 c7 0/2585
6/51: d8 e7 0/2585
7/51: a3 a4 1/2584
8/51: f1 f5 0/2583
9/51: g3 g4 2/2583
10/51: e2 c3 5/2582
11/51: e3 d5 5/2578
12/51: e2 f4 5/2577
13/51: f6 f8 0/2576
14/51: f6 h8 0/2576
15/51: b2 b4 4/2576
16/51: f6 e7 0/2576
17/51: g1 h2 0/2575
18/51: f6 g5 104/2574
19/51: b2 b3 2/2574
20/51: g1 h1 0/2574
21/51: e3 c4 0/2574
22/51: f6 f3 0/2573
23/51: f6 f2 0/2573
24/51: f1 f2 0/2571
25/51: f1 f3 0/2570
26/51: f1 f4 0/2570
27/51: f1 d1 0/2569
28/51: f1 a1 0/2569
29/51: e3 g4 -5/2568
30/51: f1 e1 0/2568
31/51: f1 b1 0/2568
32/51: e3 c2 -5/2568
33/51: e3 g2 -5/2567
34/51: e2 c1 -5/2566
35/51: f6 e5 0/2564
36/51: f6 g6 0/2564
37/51: g1 g2 -12/2564
38/51: e3 d1 -10/2558
39/51: g1 f2 -18/2550
40/51: f6 c6 0/2525
41/51: f6 a6 0/2525
42/51: d8 a5 0/2495
43/51: e3 f5 5/2490
44/51: f1 c1 0/2476
45/51: d8 b6 0/2472
46/51: f6 g7 0/2456
47/51: f6 e6 0/2455
48/51: f6 f7 0/2454
49/51: f6 h6 0/2426
50/51: f6 f4 0/1912

8 .k.B..r.
7 ........
6 ...Q....
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 .....RK.
  ABCDEFGH
43. Qd6+
score: 2668, material: 2216, abs: 3680, pawns: 48, mobility: 42
Depth  1 #searched      795 bmove: b8 a7 bscore: -2056
Depth  2 #searched    45595 bmove: b8 a7 bscore: -2564
Depth  3 #searched    94935 bmove: b8 a7 bscore: -2668
Depth  4 #searched   193266 bmove: b8 a7 bscore: -2696
Depth  5 #searched  2092931 bmove: b8 a7 bscore: -2938
Search total: 341057768 / 23237 ms / 14677 nodes/ms
hash size r 6 t 1922 
0/4: b8 a7 0/-2938
1/4: b8 a8 0/-2940
2/4: b8 b7 12/-2940
3/4: b8 c8 12/-9998

8 ...B..r.
7 k.......
6 ...Q....
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 .....RK.
  ABCDEFGH
44. Ka7
score: -2938, material: 2216, abs: 3680, pawns: 48, mobility: 42
Depth  1 #searched    13647 bmove: d6 c5 bscore: 2564
Depth  2 #searched   170243 bmove: d6 c5 bscore: 2668
Depth  3 #searched  1553804 bmove: f1 f7 bscore: 9995
Search total: 342611572 / 23327 ms / 14687 nodes/ms
hash size r 7 t 1319 
0/55: f1 f7 22/9995
1/55: d6 e7 0/9995
2/55: d6 c7 0/9995
3/55: d8 b6 0/9995
4/55: d8 g5 104/2942
5/55: e4 e5 7/2841
6/55: f1 f8 0/2836
7/55: d8 a5 0/2835
8/55: d8 e7 0/2835
9/55: a3 a4 1/2831
10/55: d8 f6 0/2831
11/55: d6 h6 0/2830
12/55: d6 e5 0/2830
13/55: d6 f4 0/2830
14/55: d6 f6 0/2830
15/55: e2 c3 5/2829
16/55: e2 f4 5/2824
17/55: d8 c7 0/2823
18/55: b2 b4 4/2823
19/55: d6 f8 0/2823
20/55: f1 f6 0/2822
21/55: g3 g4 2/2822
22/55: g1 h2 0/2822
23/55: f1 c1 0/2822
24/55: f1 f5 0/2821
25/55: d6 g6 0/2821
26/55: g1 h1 0/2821
27/55: e3 f5 5/2820
28/55: b2 b3 2/2819
29/55: f1 f2 0/2818
30/55: d6 b6 0/2817
31/55: f1 f3 0/2817
32/55: f1 f4 0/2817
33/55: f1 d1 0/2816
34/55: f1 a1 0/2816
35/55: e3 c4 0/2815
36/55: f1 e1 0/2815
37/55: f1 b1 0/2815
38/55: e3 g4 -5/2814
39/55: e2 c1 -5/2813
40/55: e3 c2 -5/2812
41/55: e3 g2 -5/2811
42/55: g1 g2 -12/2811
43/55: e3 d1 -10/2802
44/55: g1 f2 -18/2797
45/55: d6 c5 0/2696
46/55: d6 a6 0/2696
47/55: d6 d7 0/2696
48/55: d6 b4 0/2680
49/55: d6 d5 0/2565
50/55: d4 d5 6/2504
51/55: e3 d5 5/2489
52/55: d6 e6 0/2455
53/55: d6 c6 0/2455
54/55: d6 b8 0/1917

8 ...B..r.
7 k....R..
6 ...Q....
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 ......K.
  ABCDEFGH
44. Rf7+
score: 9995, material: 2238, abs: 3702, pawns: 48, mobility: 46
Depth  1 #searched      204 bmove: a7 a8 bscore: -2084
Depth  2 #searched    13489 bmove: a7 a8 bscore: -2504
Depth  3 #searched    64977 bmove: a7 a8 bscore: -2426
Depth  4 #searched   123164 bmove: a7 a8 bscore: -2506
Depth  5 #searched   225538 bmove: a7 a8 bscore: -2602
Depth  6 #searched   516486 bmove: a7 a8 bscore: -9994
Depth  7 #searched   624409 bmove: a7 a8 bscore: -9994
Depth  8 #searched   754921 bmove: a7 a8 bscore: -9994
Depth  9 #searched   897706 bmove: a7 a8 bscore: -9994
Depth 10 #searched  1055985 bmove: a7 a8 bscore: -9994
Search total: 343667557 / 23386 ms / 14695 nodes/ms
hash size r 8 t 1378 
0/1: a7 a8 0/-9994

8 k..B..r.
7 .....R..
6 ...Q....
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 ......K.
  ABCDEFGH
45. Ka8
score: -9994, material: 2238, abs: 3702, pawns: 48, mobility: 48
Depth  1 #searched    13007 bmove: d6 d5 bscore: 2590
Depth  2 #searched    90185 bmove: d6 d5 bscore: 9997
Depth  3 #searched  1193339 bmove: d6 d5 bscore: 9997
Search total: 344860896 / 23454 ms / 14703 nodes/ms
hash size r 9 t 841 
0/59: d6 d5 0/9997
1/59: d6 c6 0/9997
2/59: d6 a6 0/9997
3/59: e3 d5 5/9997
4/59: d6 e6 0/9997
5/59: d6 c5 0/9997
6/59: e3 c4 0/9997
7/59: d6 e7 0/9997
8/59: d6 c7 0/9997
9/59: d6 b6 0/9997
10/59: d6 d7 0/9997
11/59: f7 b7 0/9997
12/59: d8 b6 0/9997
13/59: f7 a7 0/9995
14/59: d8 g5 104/2935
15/59: d8 e7 0/2850
16/59: d6 f8 0/2845
17/59: g1 h2 0/2839
18/59: g1 h1 0/2839
19/59: d8 f6 0/2838
20/59: d6 f6 0/2836
21/59: d6 h6 0/2836
22/59: d6 e5 0/2836
23/59: d8 a5 0/2827
24/59: f7 f8 -22/2826
25/59: f7 f6 -22/2822
26/59: f7 f5 -22/2821
27/59: f7 f1 -22/2820
28/59: f7 f2 -22/2818
29/59: f7 f3 -22/2817
30/59: f7 f4 -22/2817
31/59: f7 h7 0/2815
32/59: d6 g6 0/2593
33/59: d6 f4 0/2593
34/59: a3 a4 1/2589
35/59: e2 c3 5/2587
36/59: f7 g7 0/2565
37/59: e4 e5 7/2529
38/59: d4 d5 6/2523
39/59: e3 f5 5/2514
40/59: e3 g2 -5/2514
41/59: b2 b4 4/2507
42/59: e3 g4 -5/2507
43/59: e3 f1 -10/2507
44/59: g3 g4 2/2506
45/59: b2 b3 2/2505
46/59: f7 c7 0/2501
47/59: e2 c1 -5/2500
48/59: e2 f4 5/2496
49/59: f7 e7 0/2496
50/59: e3 c2 -5/2495
51/59: g1 g2 -12/2494
52/59: g1 f1 -12/2490
53/59: e3 d1 -10/2489
54/59: g1 f2 -18/2485
55/59: d6 b4 0/2483
56/59: f7 d7 0/2483
57/59: d8 c7 0/2446
58/59: d6 b8 0/1917

8 k..B..r.
7 .....R..
6 ........
5 .p.Q..p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 ......K.
  ABCDEFGH
45. Qd5+
score: 9997, material: 2238, abs: 3702, pawns: 48, mobility: 43
Depth  1 #searched      351 bmove: a8 b8 bscore: -2590
Depth  2 #searched    13643 bmove: a8 b8 bscore: -9998
Depth  3 #searched    32426 bmove: a8 b8 bscore: -9998
Depth  4 #searched    56453 bmove: a8 b8 bscore: -9998
Depth  5 #searched    87725 bmove: a8 b8 bscore: -9998
Depth  6 #searched   122307 bmove: a8 b8 bscore: -9998
Depth  7 #searched   165884 bmove: a8 b8 bscore: -9998
Depth  8 #searched   214136 bmove: a8 b8 bscore: -9998
Depth  9 #searched   263527 bmove: a8 b8 bscore: -9998
Depth 10 #searched   317777 bmove: a8 b8 bscore: -9998
Depth 11 #searched   373229 bmove: a8 b8 bscore: -9998
Depth 12 #searched   433397 bmove: a8 b8 bscore: -9998
Depth 13 #searched   499494 bmove: a8 b8 bscore: -9998
Depth 14 #searched   570006 bmove: a8 b8 bscore: -9998
Depth 15 #searched   645249 bmove: a8 b8 bscore: -9998
Depth 16 #searched   726526 bmove: a8 b8 bscore: -9998
Depth 17 #searched   813645 bmove: a8 b8 bscore: -9998
Depth 18 #searched   900703 bmove: a8 b8 bscore: -9998
Depth 19 #searched   992341 bmove: a8 b8 bscore: -9998
Depth 20 #searched  1085190 bmove: a8 b8 bscore: -9998
Search total: 345946086 / 23514 ms / 14712 nodes/ms
hash size r 10 t 762 
0/1: a8 b8 0/-9998

8 .k.B..r.
7 .....R..
6 ........
5 .p.Q..p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 ......K.
  ABCDEFGH
46. Kb8
score: -9998, material: 2238, abs: 3702, pawns: 48, mobility: 41
Depth  1 #searched    12705 bmove: d5 b7 bscore: 9999
Depth  2 #searched   118170 bmove: d5 b7 bscore: 9999
Depth  3 #searched  1434939 bmove: d5 b7 bscore: 9999
Search total: 347381025 / 23598 ms / 14720 nodes/ms
hash size r 11 t 1093 
0/54: d5 b7 0/9999
1/54: d5 b5 104/9997
2/54: d5 e5 0/9997
3/54: d8 c7 0/9997
4/54: d5 d6 0/9997
5/54: f7 h7 0/9997
6/54: f7 a7 0/9997
7/54: f7 d7 0/9997
8/54: f7 e7 0/9997
9/54: f7 g7 0/9997
10/54: a3 a4 1/9997
11/54: g3 g4 2/9997
12/54: e2 c3 5/9997
13/54: e2 f4 5/9997
14/54: b2 b4 4/9997
15/54: b2 b3 2/9997
16/54: g1 h2 0/9997
17/54: d8 f6 0/9997
18/54: e3 c4 0/9997
19/54: g1 h1 0/9997
20/54: e2 c1 -5/9997
21/54: e3 g4 -5/9997
22/54: e3 c2 -5/9997
23/54: e3 g2 -5/9997
24/54: g1 g2 -12/9997
25/54: e3 d1 -10/9997
26/54: g1 f1 -12/9997
27/54: e3 f1 -10/9997
28/54: g1 f2 -18/9997
29/54: d8 g5 104/9997
30/54: e4 e5 7/9997
31/54: d8 a5 0/9997
32/54: e3 f5 5/9997
33/54: d8 b6 0/9997
34/54: d5 c6 0/9997
35/54: f7 c7 0/9997
36/54: d5 d7 0/9997
37/54: f7 b7 0/9995
38/54: d5 g5 104/2962
39/54: f7 f8 -22/2931
40/54: d8 e7 0/2850
41/54: d5 e6 0/2846
42/54: d5 c5 0/2846
43/54: d5 f5 0/2846
44/54: f7 f6 -22/2831
45/54: d5 b3 0/2831
46/54: d5 a2 0/2831
47/54: f7 f5 -22/2830
48/54: f7 f1 -22/2829
49/54: f7 f2 -22/2827
50/54: f7 f3 -22/2826
51/54: f7 f4 -22/2826
52/54: d5 c4 0/2551
53/54: d5 a8 0/1920

8 .k.B..r.
7 .Q...R..
6 ........
5 .p....p.
4 ...PP...
3 P...N.P.
2 .P..N...
1 ......K.
  ABCDEFGH
46. Qb7#
score: 9999, material: 2238, abs: 3702, pawns: 48, mobility: 39
Search total: 348816684 / 23598 ms / 14781 nodes/ms
hash size r 12 t 0 
1-0
```

