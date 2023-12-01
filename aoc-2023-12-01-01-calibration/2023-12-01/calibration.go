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
	for _, char := range StripNonIntegers(line) {
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
