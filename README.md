# Puccinia's Checkmate

Puccinia's Checkmate - a rusty chess library:
* [Negamax search](https://en.wikipedia.org/wiki/Negamax) with alpha beta pruning
* Transposition table to avoid re-searching cycles
* Evaluation based on material, pawn structure & mobility
* Checks draw by 3x repetition and 50 move rule
* Opening library

Two example apps included - terminal CLI app (src/bin) and browser web application ([examples/spa](https://github.com/jesper-olsen/puccinia_s_checkmate/tree/main/examples/spa)).

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

Run CLI app like this to play white:
```
% cargo run --release -- -w 

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
