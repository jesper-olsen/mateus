# Puccinia's Checkmate

Puccinia's Checkmate - a rusty chess library:
* Principle variation negamax search with alpha beta pruning
* Transposition table to avoid re-searching cycles
* Evaluation based on material, pawn structure & mobility
* Checks draw by 3x repetition and 50 move rule
* Opening library

References:
* ["An Analysis of Alpha-Beta Pruning", Donald E. Knuth and Ronald W. Moore, Artificial Intelligence 6 (1975), 293-326](http://www-public.telecom-sudparis.eu/~gibson/Teaching/Teaching-ReadingMaterial/KnuthMoore75.pdf) 
* "The History Heuristic and Alpha-Beta Search Enhancements in Practice", Jonathan Schaeffer, IEEE Transactions on Pattern Analysis and Machine Intelligence, Volume: 11, Issue: 11, November 1989, Page(s): 1203 - 1212
* "Computer Chess and Search", T.A. Marsland, ENCYCLOPEDIA OF ARTIFICIAL INTELLIGENCE (2nd Ed.), 1992

Two example apps included - terminal CLI app (src/bin) and browser web application ([examples/spa](https://github.com/jesper-olsen/puccinia_s_checkmate/tree/main/examples/spa)).

Run CLI app like this: 

```
% cargo run --release -- --bin main -h 

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

Run CLI app like this to play white:
```
% cargo run --release -- --bin main -w 

8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ........
3 ........
2 PPPPPPPP
1 RNBQKBNR
  ABCDEFGH
Your Move (White):

8 rnbqkbnr
7 pppppppp
6 ........
5 ........
4 ...P....
3 ........
2 PPP.PPPP
1 RNBQKBNR
  ABCDEFGH
1. d4

8 rnbqkb.r
7 pppppppp
6 .....n..
5 ........
4 ...P....
3 ........
2 PPP.PPPP
1 RNBQKBNR
  ABCDEFGH
2. Nf6
Your Move (White):
```
