package main

import (
	"fmt"
	"strings"
)

type IntReplacement struct {
	divisor     int
	replacement string
}

func main() {
	fmt.Println("start")
}

func appendIfDivisible(number, divisible int, inputString, appendableString string) string {
	if number%divisible == 0 {
		return inputString + appendableString
	}
	return inputString
}

func GetStringForNumber(number int) string {
	replacements := []IntReplacement{
		{
			divisor:     3,
			replacement: "Fizz"},
		{
			divisor:     5,
			replacement: "Buzz"},
	}

	output := ""
	for _, element := range replacements {
		output = appendIfDivisible(number, element.divisor, output, element.replacement)
	}

	if len(output) > 0 {
		return output
	}
	return fmt.Sprint(number)
}

func ReplaceBetweenRange(start, end int) string {
	output := ""
	for i := start; i <= end; i++ {
		output += GetStringForNumber(i) + " "
	}
	return strings.TrimSpace(output)
}

func FizzBuzz(start, end int) string {
	return ""
}
