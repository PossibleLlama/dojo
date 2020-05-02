package main

import (
	"fmt"
	"strings"
)

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
	output := ""
	output = appendIfDivisible(number, 3, output, "Fizz")
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
