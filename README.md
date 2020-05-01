# Dojo

## Aim

This is to practice coding and get used to languages/ide specific features over the problems themselves.

## Origins

For each kata, I have tried to give credit to where the problem came from.

This is heavily influenced by Robert C Martin, aka Uncle Bob. Several of the problems have come from what he has written about in various books. I have also got several of the kata's from the [coding dojo](http://codingdojo.org/kata/) website.

## Setup

For each kata, there is an explanation of what the problem is, any rules governing the problem and example inputs/outputs.

Within each kata's folder, there will be a date in `YYYY-MM-DD` format specifying when it was completed, and the source to go along with it. This will let me quickly see when they were completed, and track progress of solutions.

### Scripts

The scripts are expected to be ran from within the `/<kata-name>/<date>` folder.

As this is expected to be used for multiple different lanuages, I will try to include a `run-tests` script, which will run the tests for whichever language is in use for that kata.

There is a script at base level, `test-and-commit`, which when executed from the `/<kata-name>/<date>` folder, will run the test script, and if successful add commit all the changes. You should pass to this script, a string of the language you are using for the kata.

This will not give "useful" commit messages, but is there to allow ease of commiting at each feedback point, so progress through each kata can easily be seen.

The commit messages will be in the following format. `<kata-name>-<language-name> YYYY-mm-DD HH-MM`
