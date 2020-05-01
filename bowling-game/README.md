# Bowling game Kata

## Origin

This kata I have seen in multiple places, although first discovered from [Clean code](https://www.amazon.co.uk/Clean-Code-Handbook-Software-Craftsmanship/dp/0132350882) by Robert C Martin (Uncle Bob). It's also mentioned in some of his other [books]((https://www.amazon.co.uk/Clean-Coder-Conduct-Professional-Programmers/dp/0137081073)), and on [coding dojo](http://codingdojo.org/kata/Bowling/), which I have used as clear way to get the rules of Bowling defined.

## Problem

I want to calculate the final score for a game of Bowling, from a given valid sequence of rolls.

### Inputs

- `X` Strike
- `/` Spare
- `-` Miss

### Rules of bowling

- There are 10 pins in each frame (exception for 10th).
- Each game of bowling contains 10 frames.
- In each frame, the bowler has 2 turns to knock down all pins.
- For each frame, if there are still pins remaining (not all 10 were knocked down), the score is the number of pins knocked over.
- If the bowler knocks all the pins down on the 2nd turn of the frame, this is a spare. The score for that frame is 10 + number of pins knocked down on next turn (not next frame).
- If the bowler knocks all the pins down on the 1st turn of the frame, this is a strike. The turn is over. The score for that frame is 10 + number of pins knocked down on the next 2 turns.
- In the 10th frame, if the bowler gets a spare, they throw an additional 1 ball as part of the same frame.
- In the 10th frame, if the bowler gets a strike, they thrown an additional 2 balls as part of the same frame.

### Examples

``` text
input: - - - - - - - - - - - - - - - - - - - -
output: 0

input: 1 - 1 - 1 - 1 - 1 - 1 - 1 - 1 - 1 - 1 -
output: 10

input: X X X X X X X X X X X X
output: 300

input: - / - / - / - / - / - / - / - / - / - / -
output: 100

input: 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5 / 5
output: 150
```
