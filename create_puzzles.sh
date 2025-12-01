#!/usr/bin/env bash

cd "$(dirname "$0")"

./puzzles.sh $1

day="$(printf '%02d' "$1")"
touch "puzzles/day${day}/example.input"
echo "Creating day${day}.rs"
cp src/day00.rs src/day${day}.rs
sed -e "s/00/${day}/g" -i src/day${day}.rs

PRE_SOLVERS=$(sed '1,/BEGIN_SOLVER_LIST$/!d' src/main.rs)
SOLVERS=$(sed '/BEGIN_SOLVER_LIST$/,/END_SOLVER_LIST$/!d' src/main.rs |
  head -n-1 | tail -n+2 |
  cat - <(echo "$1 => day${day}::Day${day}::try_create(input),") |
  sort -u)
POST_SOLVERS=$(sed '/END_SOLVER_LIST$/,$!d' src/main.rs)

cat <<<$PRE_SOLVERS >src/main.rs
cat <<<$SOLVERS >>src/main.rs
cat <<<$POST_SOLVERS >>src/main.rs

PRE_MODS=$(sed '1,/BEGIN_MOD_LIST$/!d' src/main.rs)
MODS=$(sed '/BEGIN_MOD_LIST$/,/END_MOD_LIST$/!d' src/main.rs |
  head -n-1 | tail -n+2 |
  cat - <(echo "mod day${day};") |
  sort -u)
POST_MODS=$(sed '/END_MOD_LIST$/,$!d' src/main.rs)

cat <<<$PRE_MODS >src/main.rs
cat <<<$MODS >>src/main.rs
cat <<<$POST_MODS >>src/main.rs

cargo fmt
cargo build
git add -A
git commit -m "Day $1 initial commit"
git push
