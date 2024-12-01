#!/bin/bash

DAY=$1
URL="https://adventofcode.com/2024/day/$DAY"
DIR="day$DAY"

cargo new "$DIR"

echo -e "\nCreating input files ..."

mkdir "$DIR/in"
touch "$DIR/in/example"

curl "https://adventofcode.com/2024/day/$DAY/input" \
	-H "Cookie: session=$AOC_SESSION_COOKIE" \
	-o "$DIR/in/input"
