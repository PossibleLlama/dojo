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
	fizz := IntReplacement{
		divisor:     3,
		replacement: "Fizz"}

	output := ""
	output = appendIfDivisible(number, fizz.divisor, output, fizz.replacement)
	output = appendIfDivisible(number, 5, output, "Buzz")

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
