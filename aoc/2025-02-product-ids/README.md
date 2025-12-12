# Advent of code - 2025 Day 2 Puzzle 1

[Source](https://adventofcode.com/2025/day/2)

We are given a range of IDs of products as the input.

The values between x and y that have a "pattern" to them are invalid.
This pattern is a repetition of at least one value.

## Part 1

The pattern is 2 repeats within the number of a sequence.

For the input range of `99-115`, you have 1 invalid ID of `99`.

None of the numbers have leading `0`'s, so `0101` would be `101`, which is valid.

The objective is to add up all of the invalid product IDs that you find.
For the `input_test.txt` the correct answer is `1227775554`.

## Part 2

The pattern can be any number of repeated numbers in a sequence.

For the input range of `99-115`, you have 2 invalid IDs of `99` and `111`.

The objective is to add up all of the invalid product IDs that you find.
For the `input_test.txt` the correct answer is `4174379265`.
