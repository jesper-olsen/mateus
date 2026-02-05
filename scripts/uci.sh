#!/bin/zsh

# Assumes cutechess is installed:
#     https://github.com/cutechess/cutechess
# Match two uci engines against each other 

BASE_DIR=$(pwd)
UI="$BASE_DIR/../cutechess/build/cutechess-cli" 
MATEUS="$BASE_DIR/target/release/uci"

# Ensure the binary is built before running
cargo build --release --bin ucicli

# TIME MANAGEMENT
INF="inf"
BLITZ="180+2"     # 3 min + 2 sec increment
RAPID="600+5"     # 10 min + 5 sec increment
BULLET="60+1"     # 1 min + 1 sec increment
ULTRA_BULLET=30+0 # 30 seconds total
S0=40/60  # 40 moves in 60 seconds (1.5 sec/move - bullet speed)
S1=40/600 # 40 moves in 10 minutes (15 sec/move - rapid)

TC=$S1

$UI -debug all \
-engine cmd=$MATEUS name=Mateus1 arg="-n" arg="1000000" \
-engine cmd=$MATEUS name=Mateus2 arg="-n" arg="1000000" \
-each proto=uci tc=$TC -games 10 -repeat
