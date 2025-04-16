#[rustfmt::skip]
pub const BRATKO_KOPEC: [(&str, &str); 24] = [
    //https://www.chessprogramming.org/Bratko-Kopec_Test
    // fen, turn-castling-enpassant, best move
    ("1k1r4/pp1b1R2/3q2pp/4p3/2B5/4Q3/PPP2B2/2K5 b - -", "Qd1+"),
    ("3r1k2/4npp1/1ppr3p/p6P/P2PPPP1/1NR5/5K2/2R5 w - -", "d5"),
    ("2q1rr1k/3bbnnp/p2p1pp1/2pPp3/PpP1P1P1/1P2BNNP/2BQ1PRK/7R b - -", "f5"),
    ("rnbqkb1r/p3pppp/1p6/2ppP3/3N4/2P5/PPP1QPPP/R1B1KB1R w KQkq -", "e6"),
    ("r1b2rk1/2q1b1pp/p2ppn2/1p6/3QP3/1BN1B3/PPP3PP/R4RK1 w - -", "Nd5,a4"),
    ("2r3k1/pppR1pp1/4p3/4P1P1/5P2/1P4K1/P1P5/8 w - -", "g6"),
    ("1nk1r1r1/pp2n1pp/4p3/q2pPp1N/b1pP1P2/B1P2R2/2P1B1PP/R2Q2K1 w - -", "Nf6"),
    ("4b3/p3kp2/6p1/3pP2p/2pP1P2/4K1P1/P3N2P/8 w - -", "f5"),
    ("2kr1bnr/pbpq4/2n1pp2/3p3p/3P1P1B/2N2N1Q/PPP3PP/2KR1B1R w - -", "f5"),
    ("3rr1k1/pp3pp1/1qn2np1/8/3p4/PP1R1P2/2P1NQPP/R1B3K1 b - -", "Ne5"),
    ("2r1nrk1/p2q1ppp/bp1p4/n1pPp3/P1P1P3/2PBB1N1/4QPPP/R4RK1 w - -", "f4"),
    ("r3r1k1/ppqb1ppp/8/4p1NQ/8/2P5/PP3PPP/R3R1K1 b - -", "Bf5"),
    ("r2q1rk1/4bppp/p2p4/2pP4/3pP3/3Q4/PP1B1PPP/R3R1K1 w - -", "b4"),
    ("rnb2r1k/pp2p2p/2pp2p1/q2P1p2/8/1Pb2NP1/PB2PPBP/R2Q1RK1 w - -", "Qd2 Qe1"),
    ("2r3k1/1p2q1pp/2b1pr2/p1pp4/6Q1/1P1PP1R1/P1PN2PP/5RK1 w - -", "Qxg7+"),
    ("r1bqkb1r/4npp1/p1p4p/1p1pP1B1/8/1B6/PPPN1PPP/R2Q1RK1 w kq -", "Ne4"),
    ("r2q1rk1/1ppnbppp/p2p1nb1/3Pp3/2P1P1P1/2N2N1P/PPB1QP2/R1B2RK1 b - -", "h5"),
    ("r1bq1rk1/pp2ppbp/2np2p1/2n5/P3PP2/N1P2N2/1PB3PP/R1B1QRK1 b - -", "Nb3"),
    ("3rr3/2pq2pk/p2p1pnp/8/2QBPP2/1P6/P5PP/4RRK1 b - -", "Rxe4"),
    ("r4k2/pb2bp1r/1p1qp2p/3pNp2/3P1P2/2N3P1/PPP1Q2P/2KRR3 w - -", "g4"),
    ("3rn2k/ppb2rpp/2ppqp2/5N2/2P1P3/1P5Q/PB3PPP/3RR1K1 w - -", "Nh6"),
    ("2r2rk1/1bqnbpp1/1p1ppn1p/pP6/N1P1P3/P2B1N1P/1B2QPP1/R2R2K1 b - -", "Bxe4"),
    ("r1bqk2r/pp2bppp/2p5/3pP3/P2Q1P2/2N1B3/1PP3PP/R4RK1 b kq -", "f6"),
    ("r2qnrnk/p2b2b1/1p1p2pp/2pPpp2/1PP1P3/PRNBB3/3QNPPP/5RK1 w - -", "f4"),
];

#[rustfmt::skip]
pub const KAUFMAN: [(&str, &str); 25] = [
    // https://www.chessprogramming.org/Kaufman_Test
    // fen, turn-castling-enpassant, best move/alt move
    ("1rbq1rk1/p1b1nppp/1p2p3/8/1B1pN3/P2B4/1P3PPP/2RQ1R1K w - -", "Nf6+"),
    ("3r2k1/p2r1p1p/1p2p1p1/q4n2/3P4/PQ5P/1P1RNPP1/3R2K1 b - -", "Nxd4"),
    ("3r2k1/1p3ppp/2pq4/p1n5/P6P/1P6/1PB2QP1/1K2R3 w - -", "Rd1"),
    ("r1b1r1k1/1ppn1p1p/3pnqp1/8/p1P1P3/5P2/PbNQNBPP/1R2RB1K w - -", "Rxb2"),
    ("2r4k/pB4bp/1p4p1/6q1/1P1n4/2N5/P4PPP/2R1Q1K1 b - -", "Qxc1"),
    ("r5k1/3n1ppp/1p6/3p1p2/3P1B2/r3P2P/PR3PP1/2R3K1 b - -", "Rxa2"),
    ("2r2rk1/1bqnbpp1/1p1ppn1p/pP6/N1P1P3/P2B1N1P/1B2QPP1/R2R2K1 b - -", "Bxe4"),
    ("5r1k/6pp/1n2Q3/4p3/8/7P/PP4PK/R1B1q3 b - -", "h6"),
    ("r3k2r/pbn2ppp/8/1P1pP3/P1qP4/5B2/3Q1PPP/R3K2R w KQkq -", "Be2"),
    ("3r2k1/ppq2pp1/4p2p/3n3P/3N2P1/2P5/PP2QP2/K2R4 b - -", "Nxc3"),
    ("q3rn1k/2QR4/pp2pp2/8/P1P5/1P4N1/6n1/6K1 w - -", "Nf5"),
    ("6k1/p3q2p/1nr3pB/8/3Q1P2/6P1/PP5P/3R2K1 b - -", "Rd6"),
    ("1r4k1/7p/5np1/3p3n/8/2NB4/7P/3N1RK1 w - -", "Nxd5"),
    ("1r2r1k1/p4p1p/6pB/q7/8/3Q2P1/PbP2PKP/1R3R2 w - -", "Rxb2"),
    ("r2q1r1k/pb3p1p/2n1p2Q/5p2/8/3B2N1/PP3PPP/R3R1K1 w - -", "Bxf5"),
    ("8/4p3/p2p4/2pP4/2P1P3/1P4k1/1P1K4/8 w - -", "b4"),
    ("1r1q1rk1/p1p2pbp/2pp1np1/6B1/4P3/2NQ4/PPP2PPP/3R1RK1 w - -", "e5"),
    ("q4rk1/1n1Qbppp/2p5/1p2p3/1P2P3/2P4P/6P1/2B1NRK1 b - -", "Qc8"),
    ("r2q1r1k/1b1nN2p/pp3pp1/8/Q7/PP5P/1BP2RPN/7K w - -", "Qxd7"),
    ("8/5p2/pk2p3/4P2p/2b1pP1P/P3P2B/8/7K w - -", "Bg4"),
    ("8/2k5/4p3/1nb2p2/2K5/8/6B1/8 w - -", "Kxb5"),
    ("1B1b4/7K/1p6/1k6/8/8/8/8 w - -", "Ba7"),
    ("rn1q1rk1/1b2bppp/1pn1p3/p2pP3/3P4/P2BBN1P/1P1N1PP1/R2Q1RK1 b - -", "Ba6"),
    ("8/p1ppk1p1/2n2p2/8/4B3/2P1KPP1/1P5P/8 w - -", "Bxc6"),
    ("8/3nk3/3pp3/1B6/8/3PPP2/4K3/8 w - -", "Bxd7"),
];

// Lasker position - test for transposition table - winning move Ka1-b1
pub const LASKER: [(&str, &str); 1] = [("8/k7/3p4/p2P1p2/P2P1P2/8/8/K7 w - -", "Kb1")];

//https://www.chessprogramming.org/The_Nolot_Suite
#[rustfmt::skip]
pub const NOLOT: [(&str,&str);11] = [
    ("r3qb1k/1b4p1/p2pr2p/3n4/Pnp1N1N1/6RP/1B3PP1/1B1QR1K1 w - -", "Nxh6"),        
    ("r4rk1/pp1n1p1p/1nqP2p1/2b1P1B1/4NQ2/1B3P2/PP2K2P/2R5 w - -", "Rxc5"),        
    ("r2qk2r/ppp1b1pp/2n1p3/3pP1n1/3P2b1/2PB1NN1/PP4PP/R1BQK2R w KQkq -", "Nxg5"), 
    ("r1b1kb1r/1p1n1ppp/p2ppn2/6BB/2qNP3/2N5/PPP2PPP/R2Q1RK1 w kq -", "Nxe6"),     
    ("r2qrb1k/1p1b2p1/p2ppn1p/8/3NP3/1BN5/PPP3QP/1K3RR1 w - -", "e5"),             
    ("rnbqk2r/1p3ppp/p7/1NpPp3/QPP1P1n1/P4N2/4KbPP/R1B2B1R b kq -", "axb5"),      
    ("1r1bk2r/2R2ppp/p3p3/1b2P2q/4QP2/4N3/1B4PP/3R2K1 w k -", "Rxd8+"),           
    ("r3rbk1/ppq2ppp/2b1pB2/8/6Q1/1P1B3P/P1P2PP1/R2R2K1 w - -", "Bxh7+"),         
    ("r4r1k/4bppb/2n1p2p/p1n1P3/1p1p1BNP/3P1NP1/qP2QPB1/2RR2K1 w - -", "Ng5"),    
    ("r1b2rk1/1p1nbppp/pq1p4/3B4/P2NP3/2N1p3/1PP3PP/R2Q1R1K w - -","Rxf7"),       
    ("r1b3k1/p2p1nP1/2pqr1Rp/1p2p2P/2B1PnQ1/1P6/P1PP4/1K4R1 w - -", "Rxh6"),
];

//https://www.chessprogramming.org/CCR_One_Hour_Test
#[rustfmt::skip]
pub const CCR: [(&str,&str);25] = [
    ("rn1qkb1r/pp2pppp/5n2/3p1b2/3P4/2N1P3/PP3PPP/R1BQKBNR w KQkq - 0 1",     "Qb3"),
    ("rn1qkb1r/pp2pppp/5n2/3p1b2/3P4/1QN1P3/PP3PPP/R1B1KBNR b KQkq - 1 1",    "Bc8"),
    ("r1bqk2r/ppp2ppp/2n5/4P3/2Bp2n1/5N1P/PP1N1PP1/R2Q1RK1 b kq - 1 10",      "Nh6,Ngxe5"), 
    ("r1bqrnk1/pp2bp1p/2p2np1/3p2B1/3P4/2NBPN2/PPQ2PPP/1R3RK1 w - - 1 12",    "b4"),
    ("rnbqkb1r/ppp1pppp/5n2/8/3PP3/2N5/PP3PPP/R1BQKBNR b KQkq - 3 5",         "e5"), 
    ("rnbq1rk1/pppp1ppp/4pn2/8/1bPP4/P1N5/1PQ1PPPP/R1B1KBNR b KQ - 1 5",      "Bxc3+"),
    ("r4rk1/3nppbp/bq1p1np1/2pP4/8/2N2NPP/PP2PPB1/R1BQR1K1 b - - 1 12",       "Rfb8"),
    ("rn1qkb1r/pb1p1ppp/1p2pn2/2p5/2PP4/5NP1/PP2PPBP/RNBQK2R w KQkq c6 1 6",  "d5"),
    ("r1bq1rk1/1pp2pbp/p1np1np1/3Pp3/2P1P3/2N1BP2/PP4PP/R1NQKB1R b KQ - 1 9", "Nd4"),
    ("rnbqr1k1/1p3pbp/p2p1np1/2pP4/4P3/2N5/PP1NBPPP/R1BQ1RK1 w - - 1 11",     "a4"),
    ("rnbqkb1r/pppp1ppp/5n2/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq f3 1 3",     "d5"),
    ("r1bqk1nr/pppnbppp/3p4/8/2BNP3/8/PPP2PPP/RNBQK2R w KQkq - 2 6",          "Bxf7+"),
    ("rnbq1b1r/ppp2kpp/3p1n2/8/3PP3/8/PPP2PPP/RNBQKB1R b KQ d3 1 5",          "Ne4"), // am 
    ("rnbqkb1r/pppp1ppp/3n4/8/2BQ4/5N2/PPP2PPP/RNB2RK1 b kq - 1 6",           "Nxc4"), // am
    ("r2q1rk1/2p1bppp/p2p1n2/1p2P3/4P1b1/1nP1BN2/PP3PPP/RN1QR1K1 w - - 1 12", "exf6"),
    ("r1bqkb1r/2pp1ppp/p1n5/1p2p3/3Pn3/1B3N2/PPP2PPP/RNBQ1RK1 b kq - 2 7",    "d5"), 
    ("r2qkbnr/2p2pp1/p1pp4/4p2p/4P1b1/5N1P/PPPP1PP1/RNBQ1RK1 w kq - 1 8",     "hxg4"), // am
    ("r1bqkb1r/pp3ppp/2np1n2/4p1B1/3NP3/2N5/PPP2PPP/R2QKB1R w KQkq e6 1 7",   "Bxf6+"),
    ("rn1qk2r/1b2bppp/p2ppn2/1p6/3NP3/1BN5/PPP2PPP/R1BQR1K1 w kq - 5 10",     "Bxe6"),// am
    ("r1b1kb1r/1pqpnppp/p1n1p3/8/3NP3/2N1B3/PPP1BPPP/R2QK2R w KQkq - 3 8",    "Ndb5"), // am
    ("r1bqnr2/pp1ppkbp/4N1p1/n3P3/8/2N1B3/PPP2PPP/R2QK2R b KQ - 2 11",        "Kxe6"), // am
    ("r3kb1r/pp1n1ppp/1q2p3/n2p4/3P1Bb1/2PB1N2/PPQ2PPP/RN2K2R w KQkq - 3 11", "a4"),
    ("r1bq1rk1/pppnnppp/4p3/3pP3/1b1P4/2NB3N/PPP2PPP/R1BQK2R w KQ - 3 7",     "Bxh7+"),
    ("r2qkbnr/ppp1pp1p/3p2p1/3Pn3/4P1b1/2N2N2/PPP2PPP/R1BQKB1R w KQkq - 2 6", "Nxe5"),
    ("rn2kb1r/pp2pppp/1qP2n2/8/6b1/1Q6/PP1PPPBP/RNB1K1NR b KQkq - 1 6",       "Qxb3"), // am
];

//https://www.chessprogramming.org/Eigenmann_Rapid_Engine_Test
//https://glarean-magazin.ch/2017/03/05/computerschach-testaufgaben-engines-eigenmann-rapid-engine-test-eret/
#[rustfmt::skip]
pub const ERET: [(&str,&str);111] = [
    ("r1bqk1r1/1p1p1n2/p1n2pN1/2p1b2Q/2P1Pp2/1PN5/PB4PP/R4RK1 w q -",   "Rxf4"), //; id "ERET 001 - Relief";
    ("r1n2N1k/2n2K1p/3pp3/5Pp1/b5R1/8/1PPP4/8 w - -",                     "Ng6"), //; id "ERET 002 - Zugzwang";
    ("r1b1r1k1/1pqn1pbp/p2pp1p1/P7/1n1NPP1Q/2NBBR2/1PP3PP/R6K w - -",     "f5"), //; id "ERET 003 - Open Line";
    ("5b2/p2k1p2/P3pP1p/n2pP1p1/1p1P2P1/1P1KBN2/7P/8 w - -",              "Nxg5"), //; id "ERET 004 - Endgame";
    ("r3kbnr/1b3ppp/pqn5/1pp1P3/3p4/1BN2N2/PP2QPPP/R1BR2K1 w kq - -",     "Bxf7"), //; id "ERET 005 - Bishop Sacrifice f7";
    ("r2r2k1/1p1n1pp1/4pnp1/8/PpBRqP2/1Q2B1P1/1P5P/R5K1 b - -",           "Nc5"), //; id "ERET 006 - Knight Sacrifice";
    ("2rq1rk1/pb1n1ppN/4p3/1pb5/3P1Pn1/P1N5/1PQ1B1PP/R1B2RK1 b - -",      "Nde5"), //; id "ERET 007 - Bishop Pair";
    ("r2qk2r/ppp1bppp/2n5/3p1b2/3P1Bn1/1QN1P3/PP3P1P/R3KBNR w KQkq -",    "Qxd5"), //; id "ERET 008 - Center";
    ("rnb1kb1r/p4p2/1qp1pn2/1p2N2p/2p1P1p1/2N3B1/PPQ1BPPP/3RK2R w Kkq -", "Ng6"), //; id "ERET 009 - Knight Sacrifice";
    ("5rk1/pp1b4/4pqp1/2Ppb2p/1P2p3/4Q2P/P3BPP1/1R3R1K b - -",            "d4"), //; id "ERET 010 - Passed Pawn";
    ("r1b2r1k/ppp2ppp/8/4p3/2BPQ3/P3P1K1/1B3PPP/n3q1NR w - -",            "dxe5"), //, Nf3; id "ERET 011 - Attacking Castle";
    ("1nkr1b1r/5p2/1q2p2p/1ppbP1p1/2pP4/2N3B1/1P1QBPPP/R4RK1 w - -",      "Nxd5"), //; id "ERET 012 - Relief";
    ("1nrq1rk1/p4pp1/bp2pn1p/3p4/2PP1B2/P1PB2N1/4QPPP/1R2R1K1 w - -",     "Qd2, Bc2"), //; id "ERET 013 - Center";
    ("5k2/1rn2p2/3pb1p1/7p/p3PP2/PnNBK2P/3N2P1/1R6 w - -",                "Nf3"), //; id "ERET 014 - Endgame";
    ("8/p2p4/r7/1k6/8/pK5Q/P7/b7 w - -",                                  "Qd3"), //; id "ERET 015 - Endgame";
    ("1b1rr1k1/pp1q1pp1/8/NP1p1b1p/1B1Pp1n1/PQR1P1P1/4BP1P/5RK1 w - -",   "Nc6"), //; id "ERET 016 - Pos. Sacrifice";
    ("1r3rk1/6p1/p1pb1qPp/3p4/4nPR1/2N4Q/PPP4P/2K1BR2 b - -",             "Rxb2"), //; id "ERET 017 - King Attack";
    ("r1b1kb1r/1p1n1p2/p3pP1p/q7/3N3p/2N5/P1PQB1PP/1R3R1K b kq -",        "Qg5"), //; id "ERET 018 - Development";
    ("3kB3/5K2/7p/3p4/3pn3/4NN2/8/1b4B1 w - -",                           "Nf5"), //; id "ERET 019 - Endgame";
    ("1nrrb1k1/1qn1bppp/pp2p3/3pP3/N2P3P/1P1B1NP1/PBR1QPK1/2R5 w - -",    "Bxh7"), //; id "ERET 020 - Bishop Sacrifice h7";
    ("3rr1k1/1pq2b1p/2pp2p1/4bp2/pPPN4/4P1PP/P1QR1PB1/1R4K1 b - -",       "Rc8"), //; id "ERET 021 - Prophylaxis";
    ("r4rk1/p2nbpp1/2p2np1/q7/Np1PPB2/8/PPQ1N1PP/1K1R3R w - -",           "h4"), //; id "ERET 022 - Passed Pawn";
    ("r3r2k/1bq1nppp/p2b4/1pn1p2P/2p1P1QN/2P1N1P1/PPBB1P1R/2KR4 w - -",   "Ng6"), //; id "ERET 023 - Attacking Castle";
    ("r2q1r1k/3bppbp/pp1p4/2pPn1Bp/P1P1P2P/2N2P2/1P1Q2P1/R3KB1R w KQ -",  "b3"), //; id "ERET 024 - Development";
    ("2kb4/p7/r1p3p1/p1P2pBp/R2P3P/2K3P1/5P2/8 w - -",                    "Bxd8"), //; id "ERET 025 - Endgame";
    ("rqn2rk1/pp2b2p/2n2pp1/1N2p3/5P1N/1PP1B3/4Q1PP/R4RK1 w - -",         "Nxg6"), //; id "ERET 026 - Knight Sacrifice";
    ("8/3Pk1p1/1p2P1K1/1P1Bb3/7p/7P/6P1/8 w - -",                         "g4"), //; id "ERET 027 - Zugzwang";
    ("4rrk1/Rpp3pp/6q1/2PPn3/4p3/2N5/1P2QPPP/5RK1 w - -",                 "Rxb7"), //; id "ERET 028 - Poisoned Pawn";
    ("2q2rk1/2p2pb1/PpP1p1pp/2n5/5B1P/3Q2P1/4PPN1/2R3K1 w - -",           "Rxc5"), //; id "ERET 029 - Exchange Sacrifice";
    ("rnbq1r1k/4p1bP/p3p3/1pn5/8/2Np1N2/PPQ2PP1/R1B1KB1R w KQ -",         "Nh4"), //; id "ERET 030 - Initiative";
    ("4b1k1/1p3p2/4pPp1/p2pP1P1/P2P4/1P1B4/8/2K5 w - -",                  "b4"), //; id "ERET 031 - Endgame";
    ("8/7p/5P1k/1p5P/5p2/2p1p3/P1P1P1P1/1K3Nb1 w - -",                    "Ng3"), //; id "ERET 032 - Zugzwang";
    ("r3kb1r/ppnq2pp/2n5/4pp2/1P1PN3/P4N2/4QPPP/R1B1K2R w KQkq -",        "Nxe5"), //; id "ERET 033 - Initiative";
    ("b4r1k/6bp/3q1ppN/1p2p3/3nP1Q1/3BB2P/1P3PP1/2R3K1 w - -",            "Rc8"), //; id "ERET 034 - Bishop Pair";
    ("r3k2r/5ppp/3pbb2/qp1Np3/2BnP3/N7/PP1Q1PPP/R3K2R w KQkq -",          "Nxb5"), //; id "ERET 035 - Exchange Sacrifice";
    ("r1k1n2n/8/pP6/5R2/8/1b1B4/4N3/1K5N w - -",                          "b7"), //; id "ERET 036 - Endgame";
    ("1k6/bPN2pp1/Pp2p3/p1p5/2pn4/3P4/PPR5/1K6 w - -",                    "Na8"), //; id "ERET 037 - Zugzwang";
    ("8/6N1/3kNKp1/3p4/4P3/p7/P6b/8 w - -",                               "exd5"), //; id "ERET 038 - Endgame";
    ("r1b1k2r/pp3ppp/1qn1p3/2bn4/8/6P1/PPN1PPBP/RNBQ1RK1 w kq -",         "a3"), //; id "ERET 039 - Development";
    ("r3kb1r/3n1ppp/p3p3/1p1pP2P/P3PBP1/4P3/1q2B3/R2Q1K1R b kq -",        "Bc5"), //; id "ERET 040 - King Safety";
    ("3q1rk1/2nbppb1/pr1p1n1p/2pP1Pp1/2P1P2Q/2N2N2/1P2B1PP/R1B2RK1 w - -","Nxg5"), //; - id "ERET 041 - Knight Sacrifice";
    ("8/2k5/N3p1p1/2KpP1P1/b2P4/8/8/8 b - -",                             "Kb7"), //; id "ERET 042 - Endgame";
    ("2r1rbk1/1pqb1p1p/p2p1np1/P4p2/3NP1P1/2NP1R1Q/1P5P/R5BK w - -",      "Nxf5"), //; id "ERET 043 - Knight Sacrifice";
    ("rnb2rk1/pp2q2p/3p4/2pP2p1/2P1Pp2/2N5/PP1QBRPP/R5K1 w - -",          "h4"), //; id "ERET 044 - Open Line";
    ("5rk1/p1p1rpb1/q1Pp2p1/3Pp2p/4Pn2/1R4N1/P1BQ1PPP/R5K1 w - -",        "Rb4"), //; id "ERET 045 - Initiative";
    ("8/4nk2/1p3p2/1r1p2pp/1P1R1N1P/6P1/3KPP2/8 w - -",                   "Nd3"), //; id "ERET 046 - Endgame";
    ("4kbr1/1b1nqp2/2p1p3/2N4p/1p1PP1pP/1PpQ2B1/4BPP1/r4RK1 w - -",       "Nxb7"), //; id "ERET 047 - Relief";
    ("r1b2rk1/p2nqppp/1ppbpn2/3p4/2P5/1PN1PN2/PBQPBPPP/R4RK1 w - -",      "cxd5"), //; id "ERET 048 - Stong Squares";
    ("r1b1kq1r/1p1n2bp/p2p2p1/3PppB1/Q1P1N3/8/PP2BPPP/R4RK1 w kq -",      "f4"), //; id "ERET 049 - Development";
    ("r4r1k/p1p3bp/2pp2p1/4nb2/N1P4q/1P5P/PBNQ1PP1/R4RK1 b - -",          "Nf3"), //; id "ERET 050 - King Attack";
    ("6k1/pb1r1qbp/3p1p2/2p2p2/2P1rN2/1P1R3P/PB3QP1/3R2K1 b - -",         "Bh6"), //; id "ERET 051 - Defence";
    ("2r2r2/1p1qbkpp/p2ppn2/P1n1p3/4P3/2N1BB2/QPP2PPP/R4RK1 w - -",       "b4"), //; id "ERET 052 - Stong Squares";
    ("r1bq1rk1/p4ppp/3p2n1/1PpPp2n/4P2P/P1PB1PP1/2Q1N3/R1B1K2R b KQ -",   "c4"), //; id "ERET 053 - Pos. Sacrifice";
    ("2b1r3/5pkp/6p1/4P3/QppqPP2/5RPP/6BK/8 b - -",                       "c3"), //; id "ERET 054 - Endgame";
    ("r2q1rk1/1p2bpp1/p1b2n1p/8/5B2/2NB4/PP1Q1PPP/3R1RK1 w - -",          "Bxh6"), //; id "ERET 055 - Bishop Sacrifice h6";
    ("r2qr1k1/pp2bpp1/2pp3p/4nbN1/2P4P/4BP2/PPPQ2P1/1K1R1B1R w - -",      "Be2"), //; id "ERET 056 - Zwischenzug";
    ("r2qr1k1/pp1bbp2/n5p1/2pPp2p/8/P2PP1PP/1P2N1BK/R1BQ1R2 w - -",       "d6"), //; id "ERET 057 - Exchange";
    ("8/8/R7/1b4k1/5p2/1B3r2/7P/7K w - -",                                "h4"), //; id "ERET 058 - Endgame";
    ("rq6/5k2/p3pP1p/3p2p1/6PP/1PB1Q3/2P5/1K6 w - -",                     "Qd3"), //; id "ERET 059 - Endgame";
    ("q2B2k1/pb4bp/4p1p1/2p1N3/2PnpP2/PP3B2/6PP/2RQ2K1 b - -",            "Qxd8"), //; id "ERET 060 - King Attack";
    ("4rrk1/pp4pp/3p4/3P3b/2PpPp1q/1Q5P/PB4B1/R4RK1 b - -",               "Rf6"), //; id "ERET 061 - King Attack";
    ("rr1nb1k1/2q1b1pp/pn1p1p2/1p1PpNPP/4P3/1PP1BN2/2B2P2/R2QR1K1 w - -", "g6"), //; id "ERET 062 - Stong Squares";
    ("r3k2r/4qn2/p1p1b2p/6pB/P1p5/2P5/5PPP/RQ2R1K1 b kq -",               "Kf8"), //; id "ERET 063 - Defence";
    ("8/1pp5/p3k1pp/8/P1p2PPP/2P2K2/1P3R2/5r2 b - -",                     "Rxf2"), //; id "ERET 064 - Endgame";
    ("1r3rk1/2qbppbp/3p1np1/nP1P2B1/2p2P2/2N1P2P/1P1NB1P1/R2Q1RK1 b - -", "Qb6"), //; id "ERET 065 - Zwischenzug";
    ("8/2pN1k2/p4p1p/Pn1R4/3b4/6Pp/1P3K1P/8 w - -",                       "Ke1"), //; id "ERET 066 - Endgame";
    ("5r1k/1p4bp/3p1q2/1NpP1b2/1pP2p2/1Q5P/1P1KBP2/r2RN2R b - -",         "f3"), //; id "ERET 067 - Clearance";
    ("r3kb1r/pbq2ppp/1pn1p3/2p1P3/1nP5/1P3NP1/PB1N1PBP/R2Q1RK1 w kq -",   "a3"), //; id "ERET 068 - Open Line";
    ("5rk1/n2qbpp1/pp2p1p1/3pP1P1/PP1P3P/2rNPN2/R7/1Q3RK1 w - -",         "h5"), //; id "ERET 069 - King Attack";
    ("r5k1/1bqp1rpp/p1n1p3/1p4p1/1b2PP2/2NBB1P1/PPPQ4/2KR3R w - -",       "a3"), //; id "ERET 070 - Stong Squares";
    ("1r4k1/1nq3pp/pp1pp1r1/8/PPP2P2/6P1/5N1P/2RQR1K1 w - -",             "f5"), //; id "ERET 071 - Deflection";
    ("q5k1/p2p2bp/1p1p2r1/2p1np2/6p1/1PP2PP1/P2PQ1KP/4R1NR b - -",        "Qd5"), //; id "ERET 072 - Centralization";
    ("r4rk1/ppp2ppp/1nnb4/8/1P1P3q/PBN1B2P/4bPP1/R2QR1K1 w - -",          "Qxe2"), //; id "ERET 073 - Mobility";
    ("1r3k2/2N2pp1/1pR2n1p/4p3/8/1P1K1P2/P5PP/8 w - -",                   "Kc4"), //; id "ERET 074 - Endgame";
    ("6r1/6r1/2p1k1pp/p1pbP2q/Pp1p1PpP/1P1P2NR/1KPQ3R/8 b - -",           "Qf5"), //; id "ERET 075 - Fortress";
    ("r1b1kb1r/1p1npppp/p2p1n2/6B1/3NPP2/q1N5/P1PQ2PP/1R2KB1R w Kkq -",   "Bxf6"), //; id "ERET 076 - Development";
    ("r3r1k1/1bq2ppp/p1p2n2/3ppPP1/4P3/1PbB4/PBP1Q2P/R4R1K w - -",        "gxf6"), //; id "ERET 077 - Attacking Castle";
    ("r4rk1/ppq3pp/2p1Pn2/4p1Q1/8/2N5/PP4PP/2KR1R2 w - -",                "Rxf6"), //; id "ERET 078 - Passed Pawn";
    ("r1bqr1k1/3n1ppp/p2p1b2/3N1PP1/1p1B1P2/1P6/1PP1Q2P/2KR2R1 w - -",    "Qxe8"), //; id "ERET 079 - Queen Sacrifice";
    ("5rk1/1ppbq1pp/3p3r/pP1PppbB/2P5/P1BP4/5PPP/3QRRK1 b - -",           "Bc1"), //; id "ERET 080 - Clearance";
    ("r3r1kb/p2bp2p/1q1p1npB/5NQ1/2p1P1P1/2N2P2/PPP5/2KR3R w - -",        "Bg7"), //; id "ERET 081 - King Attack";
    ("8/3P4/1p3b1p/p7/P7/1P3NPP/4p1K1/3k4 w - -",                         "g4"), //; id "ERET 082 - Endgame";
    ("3q1rk1/7p/rp1n4/p1pPbp2/P1P2pb1/1QN4P/1B2B1P1/1R3RK1 w - -",        "Nb5"), //; id "ERET 083 - Exchange";
    ("4r1k1/1r1np3/1pqp1ppB/p7/2b1P1PQ/2P2P2/P3B2R/3R2K1 w - -",          "Bg7,Bg5"), //; id "ERET 084 - King Attack";
    ("r4rk1/q4bb1/p1R4p/3pN1p1/8/2N3P1/P4PP1/3QR1K1 w - -",               "Ng4"), //; id "ERET 085 - Exchange";
    ("r3k2r/pp2pp1p/8/q2Pb3/2P5/4p3/B1Q2PPP/2R2RK1 w kq -",               "c5"), //; id "ERET 086 - Exchange Sacrifice";
    ("r3r1k1/1bnq1pbn/p2p2p1/1p1P3p/2p1PP1B/P1N2B1P/1PQN2P1/3RR1K1 w - -","e5"), //; id "ERET 087 - Clearance";
    ("8/4k3/p2p2p1/P1pPn2p/1pP1P2P/1P1NK1P1/8/8 w - -",                   "g4"), //; id "ERET 088 - Endgame";
    ("8/2P1P3/b1B2p2/1pPRp3/2k3P1/P4pK1/nP3p1p/N7 w - -",                 "e8N"), //; id "ERET 089 - Underpromotion";
    ("4K1k1/8/1p5p/1Pp3b1/8/1P3P2/P1B2P2/8 w - -",                        "f4"), //; id "ERET 090 - Endgame";
    ("8/6p1/3k4/3p1p1p/p2K1P1P/4P1P1/P7/8 b - -",                         "g6, Kc6"), //; id "ERET 091 - Endgame";
    ("r1b2rk1/ppp3p1/4p2p/4Qpq1/3P4/2PB4/PPK2PPP/R6R b - -",              "Qxg2"), //; id "ERET 092 - Poisoned Pawn";
    ("2b1r3/r2ppN2/8/1p1p1k2/pP1P4/2P3R1/PP3PP1/2K5 w - -",               "Nd6"), //; id "ERET 093 - Endgame";
    ("2k2Br1/p6b/Pq1r4/1p2p1b1/1Ppp2p1/Q1P3N1/5RPP/R3N1K1 b - -",         "Rf6"), //; id "ERET 094 - Queen Sacrifice";
    ("r2qk2r/ppp1b1pp/2n1p3/3pP1n1/3P2b1/2PB1NN1/PP4PP/R1BQK2R w KQkq -", "Nxg5"), //; id "ERET 095 - Queen Sacrifice";
    ("8/8/4p1Pk/1rp1K1p1/4P1P1/1nP2Q2/p2b1P2/8 w - -",                    "Kf6"), //; id "ERET 096 - Endgame";
    ("2k5/p7/Pp1p1b2/1P1P1p2/2P2P1p/3K3P/5B2/8 w - -",                    "c5"), //; id "ERET 097 - Endgame";
    ("8/6pp/5k2/1p1r4/4R3/7P/5PP1/5K2 w - -",                             "Ke2"), //; id "ERET 098 - Endgame";
    ("3q1r1k/4RPp1/p6p/2pn4/2P5/1P6/P3Q2P/6K1 w - -",                     "Re8"), //; id "ERET 099 - Endgame";
    ("rn2k2r/3pbppp/p3p3/8/Nq1Nn3/4B1P1/PP3P1P/R2Q1RK1 w k -",            "Nf5"), //; id "ERET 100 - Initiative";
    ("r1b1kb1N/pppnq1pB/8/3p4/3P4/8/PPPK1nPP/RNB1R3 b q -",               "Ne5"), //; id "ERET 101 - Development";
    ("N4rk1/pp1b1ppp/n3p1n1/3pP1Q1/1P1N4/8/1PP2PPP/q1B1KB1R b K -",       "Nxb4"), //; id "ERET 102 - King Attack";
    ("4k1br/1K1p1n1r/2p2pN1/P2p1N2/2P3pP/5B2/P2P4/8 w - -",               "Kc8"), //; id "ERET 103 - Zugzwang";
    ("r1bqkb1r/ppp3pp/2np4/3N1p2/3pnB2/5N2/PPP1QPPP/2KR1B1R b kq -",      "Ne7"), //; id "ERET 104 - Development";
    ("r3kb1r/pbqp1pp1/1pn1pn1p/8/3PP3/2PB1N2/3N1PPP/R1BQR1K1 w kq -",     "e5"), //; id "ERET 105 - Stong Squares";
    ("r2r2k1/pq2bppp/1np1bN2/1p2B1P1/5Q2/P4P2/1PP4P/2KR1B1R b - -",       "Bxf6"), //; id "ERET 106 - King Safety";
    ("1r1r2k1/2pq3p/4p3/2Q1Pp2/1PNn1R2/P5P1/5P1P/4R2K b - -",             "Rb5"), //; id "ERET 107 - Defence";
    ("8/5p1p/3P1k2/p1P2n2/3rp3/1B6/P4R2/6K1 w - -",                       "Ba4"), //; id "ERET 108 - Endgame";
    ("2rbrnk1/1b3p2/p2pp3/1p4PQ/1PqBPP2/P1NR4/2P4P/5RK1 b - -",           "Qxd4"), //; id "ERET 109 - Relief";
    ("4r1k1/1bq2r1p/p2p1np1/3Pppb1/P1P5/1N3P2/1R2B1PP/1Q1R2BK w - -",     "c5"), //; id "ERET 110 - Passed Pawn";
    ("8/8/8/8/4kp2/1R6/P2q1PPK/8 w - -", "a3"),
];

//https://www.schach-computer.info/wiki/index.php?title=BT-2450
#[rustfmt::skip]
pub const BT2450: [(&str,&str);30] = [
    ("rq2r1k1/5pp1/p7/4bNP1/1p2P2P/5Q2/PP4K1/5R1R w - -",   "Nxg7"),
    ("6k1/2b2p1p/ppP3p1/4p3/PP1B4/5PP1/7P/7K w - -",   "Bxb6"),
    ("5r1k/p1q2pp1/1pb4p/n3R1NQ/7P/3B1P2/2P3P1/7K w - -",   "Re6"),
    ("5r1k/1P4pp/3P1p2/4p3/1P5P/3q2P1/Q2b2K1/B3R3 b - -",  "Qf7"),
    ("3B4/8/2B5/1K6/8/8/3p4/3k4 w - -", "Ka6"),
    ("1k1r4/1pp4p/2n5/P6R/2R1p1r1/2P2p2/1PP2B1P/4K3 b - -", "e3"),
    ("6k1/p3q2p/1nr3pB/8/3Q1P2/6P1/PP5P/3R2K1 b - -", "Rd6"),
    ("2krr3/1p4pp/p1bRpp1n/2p5/P1B1PP2/8/1PP3PP/R1K3B1 w - -", "Rxc6+"),
    ("r5k1/pp2p1bp/6p1/n1p1P3/2qP1NP1/2PQB3/P5PP/R4K2 b - -", "g5"),
    ("2r3k1/1qr1b1p1/p2pPn2/nppPp3/8/1PP1B2P/P1BQ1P2/5KRR w - -", "Rxg7+"),
    ("1br3k1/p4p2/2p1r3/3p1b2/3Bn1p1/1P2P1Pq/P3Q1BP/2R1NRK1 b - -", "Qxh2+"),
    ("8/pp3k2/2p1qp2/2P5/5P2/1R2p1rp/PP2R3/4K2Q b - -", "Qe4"),
    ("3b2k1/1pp2rpp/r2n1p1B/p2N1q2/3Q4/6R1/PPP2PPP/4R1K1 w - -", "Nb4"),
    ("3r1rk1/1p3pnp/p3pBp1/1qPpP3/1P1P2R1/P2Q3R/6PP/6K1 w - -", "Rxh7"),
    ("4k1rr/ppp5/3b1p1p/4pP1P/3pP2N/3P3P/PPP5/2KR2R1 w kq -", "Rg6"),
    ("r1b3k1/ppp3pp/2qpp3/2r3N1/2R5/8/P1Q2PPP/2B3K1 b - -", "g6"),
    ("4r1k1/p1qr1p2/2pb1Bp1/1p5p/3P1n1R/3B1P2/PP3PK1/2Q4R w - -", "Qxf4"),
    ("8/4p3/8/3P3p/P2pK3/6P1/7b/3k4 w - -", "d6"),
    ("3r2k1/pp4B1/6pp/PP1Np2n/2Pp1p2/3P2Pq/3QPPbP/R4RK1 b - -", "f3"),
    ("r4rk1/5p2/1n4pQ/2p5/p5P1/P4N2/1qb1BP1P/R3R1K1 w - -", "Ra2"),
    ("r4rk1/pb3p2/1pp4p/2qn2p1/2B5/6BP/PPQ2PP1/3RR1K1 w - -", "Re6"),
    ("rnb1k2r/pp2qppp/3p1n2/2pp2B1/1bP5/2N1P3/PP2NPPP/R2QKB1R w KQkq -", "a3"),
    ("r1b2rk1/pp1p1pBp/6p1/8/2PQ4/8/PP1KBP1P/q7 w - -", "Qf6"),
    ("R7/3p3p/8/3P2P1/3k4/1p5p/1P1NKP1P/7q w - -", "g6"),
    ("8/8/3k1p2/p2BnP2/4PN2/1P2K1p1/8/5b2 b - -", "Nd3"),
    ("2r3k1/pbr1q2p/1p2pnp1/3p4/3P1P2/1P1BR3/PB1Q2PP/5RK1 w - -", "f5"),
    ("3r2k1/p2r2p1/1p1B2Pp/4PQ1P/2b1p3/P3P3/7K/8 w - -", "e6"),
    ("b2r1rk1/2q2ppp/p1nbpn2/1p6/1P6/P1N1PN2/1B2QPPP/1BR2RK1 w - -", "Ne4"),
    ("r1b4Q/p4k1p/1pp1ppqn/8/1nP5/8/PP1KBPPP/3R2NR w - -", "Ke1"),
    ("2k5/2p3Rp/p1pb4/1p2p3/4P3/PN1P1P2/1P2KP1r/8 w - -", "f4"),
    ];

// https://www.schach-computer.info/wiki/index.php?title=BT-2630
#[rustfmt::skip]
pub const BT2630: [(&str,&str);30] = [
("rq2r1k1/5pp1/p7/4bNP1/1p2P2P/5Q2/PP4K1/5R1R w - -", "Nxg7"),
("6k1/2b2p1p/ppP3p1/4p3/PP1B4/5PP1/7P/7K w - -", "Bxb6"),
("5r1k/p1q2pp1/1pb4p/n3R1NQ/7P/3B1P2/2P3P1/7K w - -", "Re6"),
("5r1k/1P4pp/3P1p2/4p3/1P5P/3q2P1/Q2b2K1/B3R3 w - -", "Qf7"),
("3B4/8/2B5/1K6/8/8/3p4/3k4 w - -", "Ka6"),
("1k1r4/1pp4p/2n5/P6R/2R1p1r1/2P2p2/1PP2B1P/4K3 b - -", "e3"),
("6k1/p3q2p/1nr3pB/8/3Q1P2/6P1/PP5P/3R2K1 b - -", "Rd6"),
("2krr3/1p4pp/p1bRpp1n/2p5/P1B1PP2/8/1PP3PP/R1K3B1 w - -", "Rxc6+"),
("r5k1/pp2p1bp/6p1/n1p1P3/2qP1NP1/2PQB3/P5PP/R4K2 b - -", "g5"),
("2r3k1/1qr1b1p1/p2pPn2/nppPp3/8/1PP1B2P/P1BQ1P2/5KRR w - -", "Rxg7+"),
("1br3k1/p4p2/2p1r3/3p1b2/3Bn1p1/1P2P1Pq/P3Q1BP/2R1NRK1 b - -",  "Qxh2+"),
("8/pp3k2/2p1qp2/2P5/5P2/1R2p1rp/PP2R3/4K2Q b - -", "Qe4"),
("2bq3k/2p4p/p2p4/7P/1nBPPQP1/r1p5/8/1K1R2R1 b - -", "Be6"),
("3r1rk1/1p3pnp/p3pBp1/1qPpP3/1P1P2R1/P2Q3R/6PP/6K1 w - -", "Rxh7"),
("2b1q3/p7/1p1p2kb/nPpN3p/P1P1P2P/6P1/5R1K/5Q2 w - -",  "e5"),
("2krr3/pppb1ppp/3b4/3q4/3P3n/2P2N1P/PP2B1P1/R1BQ1RK1 b - -", "Nxg2"),
("4r1k1/p1qr1p2/2pb1Bp1/1p5p/3P1n1R/3B1P2/PP3PK1/2Q4R w - -", "Qxf4"),
("8/4p3/8/3P3p/P2pK3/6P1/7b/3k4 w - -", "d6"),
("3r2k1/pp4B1/6pp/PP1Np2n/2Pp1p2/3P2Pq/3QPPbP/R4RK1 b - -", "f3"),
("r4rk1/5p2/1n4pQ/2p5/p5P1/P4N2/1qb1BP1P/R3R1K1 w - -", "Ra2"),
("k7/8/PP1b2P1/K2Pn2P/4R3/8/6np/8 w - -", "Re1"),
("rnb1k2r/pp2qppp/3p1n2/2pp2B1/1bP5/2N1P3/PP2NPPP/R2QKB1R w KQkq -", "a3"),
("8/7p/8/p4p2/5K2/Bpk3P1/4P2P/8 w - -", "g4"),
("R7/3p3p/8/3P2P1/3k4/1p5p/1P1NKP1P/7q w - -", "g6"),
("8/8/3k1p2/p2BnP2/4PN2/1P2K1p1/8/5b2 b - -", "Nd3"),
("2r3k1/pbr1q2p/1p2pnp1/3p4/3P1P2/1P1BR3/PB1Q2PP/5RK1 w - -", "f5"),
("3r2k1/p2r2p1/1p1B2Pp/4PQ1P/2b1p3/P3P3/7K/8 w - -", "e6"),
("rnb1k1nr/p2p1ppp/3B4/1p1N1N1P/4P1P1/3P1Q2/PqP5/R4Kb1 w kq -", "e5"),
("r1b1kb1r/pp1n1ppp/2q5/2p3B1/Q1B5/2p2N2/PP3PPP/R3K2R w KQkq -", "O-O-O"),
("2k5/2p3Rp/p1pb4/1p2p3/4P3/PN1P1P2/1P2KP1r/8 w - -", "f4")];

// Not a benchmark - example of en passant leading to check mate
// from Gunnar Gundersen vs Albert H. Faul in 1928
#[rustfmt::skip]
pub const GUNDERSEN_FAUL: [(&str,&str);3] = [
("r1bq1r2/pp2npp1/7k/3pP1NP/1b4Q1/2N5/PP3PP1/R1B1K2R w - -", "Ne6+"),
("r1bq1r2/pp2npp1/4N2k/3pP2P/1b4Q1/2N5/PP3PP1/R1B1K2R b - -", "g5"),
("r1bq1r2/pp2npp1/4N2k/3pP2P/1b4Q1/2N5/PP3PP1/R1B1K2R w - -", "hxg6#")];
