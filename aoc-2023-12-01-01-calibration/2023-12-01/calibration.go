package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	total := 0

	input := GetInput("../input_2.txt")
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		total += GetFirstAndLastIntegersCombined(
			GetIntegers(line))
	}
	fmt.Printf("Total: %d\n", total)
}

func GetInput(fileName string) string {
	dat, err := os.ReadFile(fileName)
	if err != nil {
		fmt.Printf("Error reading file \"%s\": %s\n", fileName, err.Error())
		os.Exit(1)
	}
	return string(dat)
}

func GetFirstAndLastIntegersCombined(input []int) int {
	if len(input) == 0 {
		return 0
	}
	firstNumber := input[0]
	lastNumber := input[len(input)-1]
	return (firstNumber * 10) + lastNumber
}

func GetIntegers(line string) []int {
	ints := []int{}
	for _, char := range StripNonIntegers(
		ReplaceSpelledOutNumbers(line)) {
		integer, err := strconv.Atoi(string(char))
		if err == nil {
			ints = append(ints, integer)
		}
	}
	return ints
}

func StripNonIntegers(line string) string {
	var result strings.Builder
	for _, c := range line {
		if '0' <= c && c <= '9' {
			result.WriteString(string(c))
		}
	}
	return result.String()
}

func ReplaceSpelledOutNumbers(line string) string {
	changing := true
	for changing {
		changing = false
		for index, c := range line {
			remaining := len(line) - index
			switch c {
			case 'o':
				if remaining >= 3 && line[index:index+3] == "one" {
					line = strings.ReplaceAll(line, "one", "1")
					changing = true
				}
			case 't':
				if remaining >= 3 && line[index:index+3] == "two" {
					line = strings.ReplaceAll(line, "two", "2")
					changing = true
				} else if remaining >= 5 && line[index:index+5] == "three" {
					line = strings.ReplaceAll(line, "three", "3")
					changing = true
				}
			case 'f':
				if remaining >= 4 && line[index:index+4] == "four" {
					line = strings.ReplaceAll(line, "four", "4")
					changing = true
				} else if remaining >= 4 && line[index:index+4] == "five" {
					line = strings.ReplaceAll(line, "five", "5")
					changing = true
				}
			case 's':
				if remaining >= 3 && line[index:index+3] == "six" {
					line = strings.ReplaceAll(line, "six", "6")
					changing = true
				} else if remaining >= 5 && line[index:index+5] == "seven" {
					line = strings.ReplaceAll(line, "seven", "7")
					changing = true
				}
			case 'e':
				if remaining >= 5 && line[index:index+5] == "eight" {
					line = strings.ReplaceAll(line, "eight", "8")
					changing = true
				}
			case 'n':
				if remaining >= 4 && line[index:index+4] == "nine" {
					line = strings.ReplaceAll(line, "nine", "9")
					changing = true
				}
			}
		}
	}
	return line
}
